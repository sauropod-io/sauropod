use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Write;

use crate::openapi_schema::*;

use crate::{Route, RouteData};

pub static API_PREFIX: &str = "/api";

pub fn generate_openapi_schema(
    routes: &[Route],
    schemas: BTreeMap<Cow<'_, str>, serde_json::Value>,
) -> anyhow::Result<()> {
    let output_path = crate::paths::get_api_path("openapi.json");

    // Create paths map using OpenAPI structs
    let mut paths = BTreeMap::new();
    for route in routes {
        let mut path_item = PathItem::default();

        if let Some(get) = &route.get {
            path_item.get = Some(create_operation_from_route_data(get));
        }
        if let Some(post) = &route.post {
            path_item.post = Some(create_operation_from_route_data(post));
        }
        if let Some(delete) = &route.delete {
            path_item.delete = Some(create_operation_from_route_data(delete));
        }

        paths.insert(
            format!("{}{}", API_PREFIX, &route.path.path_without_types()),
            path_item,
        );
    }

    // Create components with schemas
    let mut component_schemas = BTreeMap::new();
    for (name, schema_value) in schemas {
        let schema_ref: ReferenceOr<Schema> = ReferenceOr::Item(
            serde_json::from_value(schema_value)
                .map_err(|e| anyhow::anyhow!("Failed to convert schema: {}", e))?,
        );
        component_schemas.insert(name.to_string(), schema_ref);
    }

    let components = Components {
        schemas: Some(component_schemas),
        ..Components::default()
    };

    // Create the OpenAPI spec
    let spec = OpenApiSpec {
        openapi: "3.0.3".to_string(),
        info: Info {
            title: "Sauropod Scales".to_string(),
            description: None,
            terms_of_service: None,
            contact: None,
            license: Some(License {
                name: "Apache 2.0".to_string(),
                url: Some("https://www.apache.org/licenses/LICENSE-2.0.html".to_string()),
            }),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        servers: Some(vec![Server {
            url: "{protocol}://{host}:{port}".to_string(),
            description: Some("Local server".to_string()),
            variables: Some({
                let mut vars = BTreeMap::new();
                vars.insert(
                    "host".to_string(),
                    ServerVariable {
                        enum_values: None,
                        default: "localhost".to_string(),
                        description: Some("The host".to_string()),
                    },
                );
                vars.insert(
                    "port".to_string(),
                    ServerVariable {
                        enum_values: None,
                        default: "3140".to_string(),
                        description: Some("The port number".to_string()),
                    },
                );
                vars.insert(
                    "protocol".to_string(),
                    ServerVariable {
                        enum_values: None,
                        default: "http".to_string(),
                        description: Some("The protocol".to_string()),
                    },
                );
                vars
            }),
        }]),
        paths,
        components: Some(components),
        tags: None,
        external_docs: None,
    };

    let mut file = std::fs::File::create(output_path)?;
    let json = serde_json::to_string_pretty(&spec)?;
    file.write_all(json.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

fn create_operation_from_route_data(route_data: &RouteData) -> Operation {
    // Create parameters
    let parameters = if route_data.path.parameters().count() > 0 {
        Some(
            route_data
                .path
                .parameters()
                .map(|param| {
                    ReferenceOr::Item(Parameter {
                        name: param.name.to_string(),
                        location: "path".to_string(),
                        required: Some(true),
                        schema: Some(ReferenceOr::Item(Schema {
                            type_name: Some("string".to_string()),
                            ..Schema::default()
                        })),
                        ..Parameter::default()
                    })
                })
                .collect(),
        )
    } else {
        None
    };

    // Create request body if input exists
    let request_body = route_data.input.as_ref().map(|input| {
        ReferenceOr::Item(RequestBody {
            description: None,
            content: {
                let mut content = BTreeMap::new();
                content.insert(
                    "application/json".to_string(),
                    MediaType {
                        schema: Some(ReferenceOr::Reference(Reference {
                            ref_path: format!("#/components/schemas/{}", input.schema_name),
                        })),
                        example: None,
                        examples: None,
                    },
                );
                content
            },
            required: None,
        })
    });

    // Create responses
    let mut responses = BTreeMap::new();
    let success_response = ReferenceOr::Item(Response {
        description: "Successful response".to_string(),
        headers: None,
        content: route_data.output.as_ref().map(|output| {
            let mut content = BTreeMap::new();
            content.insert(
                "application/json".to_string(),
                MediaType {
                    schema: Some(ReferenceOr::Reference(Reference {
                        ref_path: format!("#/components/schemas/{}", output.schema_name),
                    })),
                    example: None,
                    examples: None,
                },
            );
            content
        }),
        links: None,
    });
    responses.insert("200".to_string(), success_response);

    Operation {
        summary: Some(route_data.description.clone()),
        parameters,
        request_body,
        responses,
        ..Operation::default()
    }
}

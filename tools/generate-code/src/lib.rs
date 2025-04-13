use std::io::Write;

use schemars::JsonSchema;

mod config_file;
pub mod database;
pub mod openapi;
pub mod openapi_schema;
pub mod paths;
pub use config_file::generate_code_for_config;
mod docker;
pub use docker::generate_code_for_docker;

pub type Object = serde_json::Map<String, serde_json::Value>;

pub struct TypeInfo {
    pub name: String,
    pub schema_name: String,
}

impl TypeInfo {
    pub fn maybe_new<T: JsonSchema>() -> Option<Self> {
        if std::any::type_name::<T>() == "()" {
            None
        } else {
            let name = std::any::type_name::<T>().replace("alloc::", "std::");
            Some(Self {
                name,
                schema_name: T::schema_name().to_string(),
            })
        }
    }
}

pub struct Parameter {
    pub name: String,
    pub parameter_type: String,
}

/// A segment of a route path.
pub enum RoutePathSegment {
    Literal(String),
    Parameter(Parameter),
}

/// Data for a route path.
pub struct RoutePath {
    /// The path string.
    pub path: String,
    /// The segments of the path.
    pub segments: Vec<RoutePathSegment>,
}

impl RoutePath {
    pub fn new(path: &str) -> Self {
        let segments = path
            .split("/")
            .filter(|x| !x.is_empty())
            .map(|x| {
                if x.starts_with('{') && x.ends_with('}') {
                    match x[1..x.len() - 1].split_once(':') {
                        Some((name, parameter_type)) => RoutePathSegment::Parameter(Parameter {
                            name: name.to_string(),
                            parameter_type: parameter_type.to_string(),
                        }),
                        None => RoutePathSegment::Parameter(Parameter {
                            name: x[1..x.len() - 1].to_string(),
                            parameter_type: "String".to_string(),
                        }),
                    }
                } else {
                    RoutePathSegment::Literal(x.to_string())
                }
            })
            .collect();
        Self {
            path: path.to_string(),
            segments,
        }
    }

    pub fn parameters(&self) -> impl Iterator<Item = &Parameter> {
        self.segments.iter().filter_map(|segment| {
            if let RoutePathSegment::Parameter(parameter) = segment {
                Some(parameter)
            } else {
                None
            }
        })
    }

    pub fn path_without_types(&self) -> String {
        let suffix = self
            .segments
            .iter()
            .map(|segment| match segment {
                RoutePathSegment::Literal(literal) => literal.clone(),
                RoutePathSegment::Parameter(parameter) => format!("{{{}}}", parameter.name),
            })
            .collect::<Vec<String>>()
            .join("/");
        format!("/{}", suffix)
    }

    pub fn loggable_path(&self) -> String {
        let suffix = self
            .segments
            .iter()
            .map(|segment| match segment {
                RoutePathSegment::Literal(literal) => literal.clone(),
                RoutePathSegment::Parameter(parameter) => format!("{{{{{}}}}}", parameter.name),
            })
            .collect::<Vec<String>>()
            .join("/");
        format!("/{}", suffix)
    }
}

/// Data for a method of a route.
pub struct RouteData {
    pub path: RoutePath,
    pub method: String,
    pub description: String,
    pub input: Option<TypeInfo>,
    pub output: Option<TypeInfo>,
}

impl RouteData {
    fn get_method_name(&self) -> String {
        format!(
            "{}_{}",
            &self.method,
            self.path
                .segments
                .iter()
                .map(|segment| {
                    let text = match segment {
                        RoutePathSegment::Literal(literal) => literal.replace('-', "_"),
                        RoutePathSegment::Parameter(parameter) => parameter.name.replace('-', "_"),
                    };

                    // Split camel case into underscored parts
                    let mut result = String::new();
                    let chars: Vec<char> = text.chars().collect();

                    for (i, &c) in chars.iter().enumerate() {
                        if i > 0
                            && c.is_uppercase()
                            && (chars[i - 1].is_lowercase()
                                || (i + 1 < chars.len() && chars[i + 1].is_lowercase()))
                        {
                            result.push('_');
                        }
                        result.push(c.to_lowercase().next().unwrap());
                    }

                    result
                })
                .collect::<Vec<String>>()
                .join("_")
        )
    }

    fn get_doc_comment(&self) -> String {
        let mut comment = String::with_capacity(self.description.len() + 5);
        comment.push_str("/// ");
        comment.push_str(&self.description);
        comment.push('\n');
        comment
    }

    fn make_axum_wrapper(&self) -> String {
        let method_name = self.get_method_name();
        let mut input_args = Vec::with_capacity(2);
        let mut call_args = Vec::with_capacity(2);
        let param_count = self.path.parameters().count();

        let parameters = self.path.parameters().collect::<Vec<_>>();
        match param_count {
            1 => {
                input_args.push(format!(
                    "axum::extract::Path({}): axum::extract::Path<{}>",
                    parameters[0].name, parameters[0].parameter_type
                ));
            }
            param_count if param_count > 1 => {
                let arg_names = parameters
                    .iter()
                    .map(|x| x.name.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                let arg_types = parameters
                    .iter()
                    .map(|x| x.parameter_type.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                input_args.push(format!(
                    "axum::extract::Path(({arg_names})): axum::extract::Path<({arg_types})>"
                ));
            }
            _ => {}
        };
        for parameter in self.path.parameters() {
            call_args.push(parameter.name.to_string());
        }

        if let Some(input) = &self.input {
            input_args.push(format!(
                "axum::extract::Json(input): axum::extract::Json<{}>",
                &input.name
            ));
            call_args.push("input".to_string());
        };
        let input_args_string = input_args.join(", ");
        let call_args_string = call_args.join(", ");

        format!(
            r#"
            {{
                let server_clone = server.clone();
                async move |{input_args_string}| {{
                    tracing::debug!("{0} {1}");
                    let response = server_clone.{method_name}({call_args_string}).instrument(tracing::info_span!("Request", method={0:?}, path={1:?})).await;
                    if let Err(error) = &response {{
                        tracing::error!("Error responding to request: {{:?}}", error);
                    }}
                    crate::create_response_from_result(response)
                }}
            }}
        "#,
            self.method.to_uppercase(),
            &self.path.loggable_path(),
        )
    }

    fn generate_method_signature(&self) -> String {
        let mut input_args = Vec::with_capacity(2);
        input_args.push("&self".to_string());

        for parameter in self.path.parameters() {
            input_args.push(format!("{}: {}", parameter.name, parameter.parameter_type));
        }
        if let Some(input) = &self.input {
            input_args.push(format!("input: {}", &input.name))
        };

        format!(
            "async fn {}({}) -> anyhow::Result<crate::HttpResponse<{}>>",
            self.get_method_name(),
            input_args.join(", "),
            self.output
                .as_ref()
                .map(|x| x.name.to_owned())
                .unwrap_or("()".to_string())
        )
    }

    fn generate_method_declaration(&self) -> String {
        format!(
            "{}{}",
            self.get_doc_comment(),
            self.generate_method_signature()
        )
    }
}

/// Description of a route.
pub struct Route {
    pub path: RoutePath,
    pub get: Option<RouteData>,
    pub post: Option<RouteData>,
    pub delete: Option<RouteData>,
}

impl Route {
    pub fn new(path: &'static str, method: Vec<RouteData>) -> Self {
        let mut get = None;
        let mut post = None;
        let mut delete = None;
        for route_data in method.into_iter() {
            match route_data.method.as_str() {
                "get" => get = Some(route_data),
                "post" => post = Some(route_data),
                "delete" => delete = Some(route_data),
                _ => {}
            }
        }
        Self {
            path: RoutePath::new(path),
            get,
            post,
            delete,
        }
    }
}

#[macro_export]
macro_rules! openapi {
    ($(route $path:literal (
        $($method:ident ( $input:ty ) -> $output:ty : $description:literal)+
     ) )+) => {
        let mut schema_generator = schemars::SchemaGenerator::new(schemars::generate::SchemaSettings::openapi3());
        let mut schema = std::collections::BTreeMap::<_, serde_json::Value>::new();

        // Make sure the schema for the `Error` type is emitted.
        // It's a little special because it's implicitly used in the OpenAPI schema for the error routes.
        schema.insert(<sauropod_schemas::Error as schemars::JsonSchema>::schema_name(), <sauropod_schemas::Error as schemars::JsonSchema>::json_schema(&mut schema_generator).to_value());

        let routes = vec![
            $(
                $crate::Route::new(
                    $path,
                    vec![
                        $(
                            {
                                schema.insert(<$input as schemars::JsonSchema>::schema_name(), <$input as schemars::JsonSchema>::json_schema(&mut schema_generator).to_value());
                                schema.insert(<$output as schemars::JsonSchema>::schema_name(), <$output as schemars::JsonSchema>::json_schema(&mut schema_generator).to_value());

                                $crate::RouteData {
                                    path: $crate::RoutePath::new($path),
                                    method: std::stringify!($method).to_lowercase().to_string(),
                                    description: $description.to_string(),
                                    input: $crate::TypeInfo::maybe_new::<$input>(),
                                    output: $crate::TypeInfo::maybe_new::<$output>(),
                                }
                            }
                        ),+
                    ]
                )
            ),+
        ];

        schema.remove("null"); // Remove () from the schema roots
        for (name, definition) in schema_generator.take_definitions() {
            schema.insert(name.into(), definition);
        }

        $crate::generate_rust_server(&routes)?;
        $crate::openapi::generate_openapi_schema(&routes, schema)?;
    };
}

pub fn generate_rust_server(routes: &[Route]) -> anyhow::Result<()> {
    let mut output_path = crate::paths::get_crate_path("http");
    output_path.push("src");
    output_path.push("generated.rs");
    let mut output = std::fs::File::create(output_path)?;

    writeln!(&mut output, "//! Generated code.")?;
    writeln!(&mut output, "use tracing::Instrument as _;")?;
    writeln!(
        &mut output,
        "pub static API_PREFIX: &str = \"{}\";",
        crate::openapi::API_PREFIX
    )?;

    // Generate the server trait
    writeln!(&mut output, "#[async_trait::async_trait]")?;
    writeln!(&mut output, "pub trait ServerInterface {{")?;
    for route in routes {
        if let Some(get) = &route.get {
            writeln!(&mut output, "{};", get.generate_method_declaration())?;
        }
        if let Some(post) = &route.post {
            writeln!(&mut output, "{};", post.generate_method_declaration())?;
        }
        if let Some(delete) = &route.delete {
            writeln!(&mut output, "{};", delete.generate_method_declaration())?;
        }
    }
    writeln!(&mut output, "}}\n")?;

    // Generate the register route method
    writeln!(
        &mut output,
        r"pub fn register_routes<T: ServerInterface + Sync + Send + 'static>(server: std::sync::Arc<T>) -> axum::Router<()> {{"
    )?;
    writeln!(&mut output, "    axum::Router::new().without_v07_checks()")?;
    for route in routes {
        writeln!(
            &mut output,
            "        .route({:?}, ",
            route.path.path_without_types()
        )?;

        let mut index = 0;
        if let Some(get) = &route.get {
            write!(
                &mut output,
                "axum::routing::get({})",
                &get.make_axum_wrapper()
            )?;
            index += 1;
        }
        if let Some(post) = &route.post {
            if index > 0 {
                write!(&mut output, ".post({})", &post.make_axum_wrapper())?;
            } else {
                write!(
                    &mut output,
                    "axum::routing::post({})",
                    &post.make_axum_wrapper()
                )?;
            }
        }
        if let Some(delete) = &route.delete {
            if index > 0 {
                write!(&mut output, ".delete({})", &delete.make_axum_wrapper())?;
            } else {
                write!(
                    &mut output,
                    "axum::routing::delete({})",
                    &delete.make_axum_wrapper()
                )?;
            }
        }

        writeln!(&mut output, ")")?;
    }
    writeln!(&mut output, "}}")?;

    Ok(())
}

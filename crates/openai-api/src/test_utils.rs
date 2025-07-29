use std::collections::HashSet;

pub fn round_trip_test<T: serde::Serialize + serde::de::DeserializeOwned>(
    example_value: &str,
) -> Result<(), serde_json::Error> {
    let raw_value: serde_json::Value = serde_json::from_str(example_value)?;
    let parsed_value: T = serde_json::from_value(raw_value.clone())?;
    let serialized = serde_json::to_value(&parsed_value)?;
    assert!(
        json_value_eq(&serialized, &raw_value),
        "\n left: {serialized:#?}\nright: {raw_value:#?}"
    );
    Ok(())
}

/// Recursively compare two serde_json::Value, ignoring object key order
fn json_value_eq(a: &serde_json::Value, b: &serde_json::Value) -> bool {
    use serde_json::Value;

    match (a, b) {
        (Value::Object(map_a), Value::Object(map_b)) => {
            let mut non_null_map_a = map_a.clone();
            let mut non_null_map_b = map_b.clone();
            for (k, value) in map_a.iter() {
                if value.is_null() {
                    non_null_map_a.remove(k);
                }
            }
            for (k, value) in map_b.iter() {
                if value.is_null() {
                    non_null_map_b.remove(k);
                }
            }

            if non_null_map_a.len() != non_null_map_b.len() {
                eprintln!("Field mismatch (different map lengths) left: {a:#?}\nright: {b:#?}");
                let keys_a: HashSet<_> = non_null_map_a.keys().collect();
                let keys_b: HashSet<_> = non_null_map_b.keys().collect();
                for key in keys_a.difference(&keys_b) {
                    eprintln!("Extra key in left: {key:#?}");
                }
                for key in keys_b.difference(&keys_a) {
                    eprintln!("Extra key in right: {key:#?}");
                }
                return false;
            }
            for (k, va) in non_null_map_a {
                match non_null_map_b.get(&k) {
                    Some(vb) => {
                        if !json_value_eq(&va, vb) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            true
        }
        (Value::Array(arr_a), Value::Array(arr_b)) => {
            if arr_a.len() != arr_b.len() {
                eprintln!("Field mismatch (different array lengths) left: {a:#?}\nright: {b:#?}");
                return false;
            }
            for (va, vb) in arr_a.iter().zip(arr_b.iter()) {
                if !json_value_eq(va, vb) {
                    return false;
                }
            }
            true
        }
        _ => {
            if a == b {
                true
            } else {
                eprintln!("Field mismatch left: {a:#?}\nright: {b:#?}");
                false
            }
        }
    }
}

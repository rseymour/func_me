use serde_json::json;
use serde_json::Value;
use stringy_fn_derive_macro::tool_json_for_fn;

#[tool_json_for_fn]
fn example(a: i32, b: String) -> Result<(), std::io::Error> {
    dbg!(a);
    dbg!(b);
    // function body
    Ok(())
}

/// * `secret_key` - The secret key used for things
/// * `query` - The query you want to ask
#[tool_json_for_fn]
fn some_other_function(secret_key: String, query: String) -> Result<String, std::io::Error> {
    // function body
    Ok(format!("{} {}", secret_key, query))
}

fn main() {
    example(24, "Hello".to_string()).unwrap();
    // the json_value_ prefix is added by the derive macro, but this will become a proper trait/impl
    let v = json_value_example();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
    let v = json_value_some_other_function();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

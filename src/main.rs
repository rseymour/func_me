use serde_json::json;
use serde_json::Value;
use stringy_fn_derive_macro::json_value;

//#[json_signature]
#[json_value]
fn example(a: i32, b: String) -> Result<(), std::io::Error> {
    dbg!(a);
    dbg!(b);
    // function body
    Ok(())
}

/// * `secret_key` - The secret key used for things
/// * `query` - The query you want to ask
#[json_value]
fn some_other_function(secret_key: String, query: String) -> Result<String, std::io::Error> {
    // function body
    Ok("Hello".to_string())
}

/// `secret_key` - The secret key used for things
/// `query` - The query you want to ask
#[json_value]
fn some_other_f2(secret_key: String, query: String) -> Result<String, std::io::Error> {
    // function body
    Ok("Hello".to_string())
}

fn main() {
    example(24, "Hello".to_string()).unwrap();
    let v = json_value_example();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
    let v = json_value_some_other_function();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

use serde_json::json;
use serde_json::Value;
use stringy_fn_derive_macro::tool_json_for_fn; // Add this line to import the `Value` type

/// `num_iterations` - The number of times to print the message
/// `user_name` - The name to print
#[tool_json_for_fn]
fn example(num_iterations: i32, user_name: String) -> Result<(), std::io::Error> {
    for _i in 0..num_iterations {
        println!("Hello {}", user_name);
    }
    Ok(())
}
fn main() {
    println!("Hello, world!");
    example(3, "Alice".to_string()).unwrap();
    let example_json = &json_value_example();
    // we get the json for posting to an API
    println!(
        "full json:\n{}",
        serde_json::to_string_pretty(example_json).expect("json serializable")
    );
}

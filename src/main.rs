use serde_json::json;
use serde_json::Value;
use stringy_fn_derive_macro::json_signature;
use stringy_fn_derive_macro::json_value;
use stringy_fn_derive_macro::print_signature;

//#[json_signature]
#[json_value]
fn example(a: i32, b: String) -> Result<(), std::io::Error> {
    dbg!(a);
    dbg!(b);
    // function body
    Ok(())
}

fn main() {
    example(24, "Hello".to_string()).unwrap();
    let v = json_value_example();
    println!("{}", serde_json::to_string_pretty(&v).unwrap());
}

use stringy_fn_derive_macro::json_signature;
use stringy_fn_derive_macro::print_signature;

#[json_signature]
fn example(a: i32, b: String) -> Result<(), std::io::Error> {
    dbg!(a);
    dbg!(b);
    // function body
    Ok(())
}

fn main() {
    example(24, "Hello".to_string()).unwrap();
}

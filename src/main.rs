use stringy_fn_derive_macro::print_signature;

#[print_signature]
fn example(a: i32, b: String) -> Result<(), std::io::Error> {
    dbg!(a);
    dbg!(b);
    // function body
    Ok(())
}

fn main() {
    example(24, "Hello".to_string()).unwrap();
}

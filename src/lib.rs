extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn print_signature_bad(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let expanded = quote! {
        fn #name(&self) -> #output {
            println!("Function name: {}", stringify!(#name));
            println!("Input: {:?}", stringify!(#inputs));
            println!("Output: {:?}", stringify!(#output));
            #input
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn print_signature(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    let json_call = format_ident!("json_call_{}", name);

    quote! {
        fn #name(#inputs) #output { #(#stmts)* }
        fn #json_call() -> () {println!("{}", #name);}
    }
    .into()
}

// {
//     "type": "function",
//     "function": {
//       "name": "annotated_function_name",
//       "description": "Description of the function",
//       "parameters": {
//         "type": "object",
//         "properties": {
//           "first_argument_var_name": {
//             "type": "first_arg_type_in_json_style",
//             "description": "description of the first argument"
//           },
//           "second_argument_var_name": {
//             "type": "first_arg_type_in_json_style",
//             "description": "description of second argument",
//             "enum": [1,2]
//           }
//         },
//         "required": ["first_argument_var_name", "second_argument_var_name"]
//       }
//     }
//   }
// create a rust struct out of the above json
struct Function {
    name: String,
    description: String,
    parameters: Parameters,
}

struct Parameters {
    properties: Vec<Property>,
    required: Vec<String>,
}

struct Property {
    name: String,
    type_: String,
    description: String,
    enum_: Option<Vec<String>>,
}

#[proc_macro_attribute]
fn json_signature(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    let json_call = format_ident!("json_call_{}", name);

    quote! {
        fn #name(#inputs) #output { #(#stmts)* }
        fn #json_call() -> () {
            println!("{{");
            println!("  \"type\": \"function\",");
            println!("  \"function\": {{");
            println!("    \"name\": \"{}\",", stringify!(#name));
            println!("    \"description\": \"Description of the function\",");
            println!("    \"parameters\": {{");
            println!("      \"type\": \"object\",");
            println!("      \"properties\": {{");
            #(
                let arg = match #inputs {
                    syn::FnArg::Typed(arg) => arg;
                    _ => panic!("Only support named arguments"),
                };
                let arg_name = match arg.pat.as_ref() {
                    syn::Pat::Ident(ident) => ident.ident.to_string(),
                    _ => panic!("Only support named arguments"),
                };
                let arg_type = &arg.ty;
                quote! {
                    println!("        \"{}\": {{", arg_name);
                    println!("          \"type\": \"{}\",", stringify!(#arg_type));
                    println!("          \"description\": \"Description of the {} argument\"", arg_name);
                    println!("        }},");
                }
            )
            println!("      }},");
            println!("      \"required\": [");
            #(
                let arg = match #inputs {
                    syn::FnArg::Typed(arg) => arg;
                    _ => panic!("Only support named arguments"),
                };
                let arg_name = match arg.pat.as_ref() {
                    syn::Pat::Ident(ident) => ident.ident.to_string(),
                    _ => panic!("Only support named arguments"),
                };
                quote! {
                    println!("        \"{}\",", arg_name);
                }
            )
            println!("      ]");
            println!("    }}");
            println!("  }}");
            println!("}}");
        }
    }
    .into()
}

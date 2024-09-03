extern crate proc_macro;
use derive_quote_to_tokens::ToTokens;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use serde_json::json;
use serde_json::Value;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat, Type};

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
    type_: Type,
    description: String,
    enum_: Option<Vec<TypeP>>,
}

enum TypeP {
    String(String),
    Integer(i32),
    Boolean(bool),
    Object,
    Array,
    Null,
}

#[derive(ToTokens)]

struct Argument {
    name: Ident,
    arg_type: Box<syn::Type>,
    description: String,
}

fn extract_function_raw(func: &ItemFn) -> Vec<Argument> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let arg_name = pat_ident.ident.clone();
                    let arg_type = pat_type.ty.clone();
                    Some(Argument {
                        name: arg_name,
                        arg_type,
                        description: "".to_string(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn rust_type_to_json_schema(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let ret = match segment.ident.to_string().as_str() {
                    "String" => "string",
                    "i32" | "i64" | "isize" => "integer",
                    "f32" | "f64" => "number",
                    "bool" => "boolean",
                    // Add more type mappings as needed
                    _ => "object", // Default to object for complex types
                };
                ret.to_string()
            } else {
                "object".to_string() // Default case
            }
        }
        Type::Reference(type_ref) => {
            // Handle references, possibly recursively
            rust_type_to_json_schema(&type_ref.elem)
        }
        // Handle other Type variants as needed
        _ => "object".to_string(), // Default case
    }
}

fn extract_function_args(func: &ItemFn) -> Vec<(String, String)> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let arg_name = pat_ident.ident.to_string();
                    let arg_type = quote!(#pat_type.ty).to_string();
                    Some((arg_name, arg_type))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

#[proc_macro_attribute]
pub fn json_signature(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    let json_call = format_ident!("json_call_{}", name);

    let args = extract_function_raw(&input);
    let mut fields = Vec::new();
    let mut required = Vec::new();
    for arg in args {
        let name = arg.name;
        let arg_type = rust_type_to_json_schema(&arg.arg_type);
        let desc = arg.description;
        let field = format!(
            "\"{}\": {{ \"type\": \"{}\", \"description\": \"{}\" }}",
            name, arg_type, desc
        );
        fields.push(field);
        required.push(name.to_string());
    }
    quote! {
        fn #name(#inputs) #output { #(#stmts)* }
        fn #json_call() -> () {
            println!("/*{{");
            println!("  \"type\": \"function\",");
            println!("  \"function\": {{");
            println!("    \"name\": \"{}\",", stringify!(#name));
            println!("    \"description\": \"Description of the function\",");
            println!("    \"parameters\": {{");
            println!("      \"type\": \"object\",");
            println!("      \"properties\": {{");
            #(
                println!("{}", #fields);
                //match #args {
                //    syn::FnArg::Typed(arg) => {
                //        let arg_name = match arg.pat.as_ref() {
                //            syn::Pat::Ident(ident) => ident.ident.to_string(),
                //            _ => panic!("Only support named arguments"),
                //        };
                //        let arg_type = &arg.ty;
                //        println!("\"{}\"", stringify!(arg_type));
                //    }
                //    _ => panic!("Only support named arguments"),
                //};
                //quote! {
                //    println!("        \"{}\": {{", arg_name);
                //    println!("          \"type\": \"{}\",", stringify!(#arg_type));
                //    println!("          \"description\": \"Description of the {} argument\"", arg_name);
                //    println!("        }},");
                //}
            )*
            println!("      }},");
            println!("      \"required\": [");
            #(
                    println!("        \"{}\"", stringify!(#required));
            )*
            println!("      ]");
            println!("    }}");
            println!("  }}");
            println!("}}*/");
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn json_value(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    let json_value = format_ident!("json_value_{}", name);

    let args = extract_function_raw(&input);
    let mut fields = serde_json::map::Map::new();
    let mut required = Vec::new();
    for arg in args {
        let name = arg.name;
        let arg_type = rust_type_to_json_schema(&arg.arg_type);
        let desc = arg.description;
        let field = format!(
            "\"{}\": {{ \"type\"; \"{}\", \"description\": \"{}\" }}",
            name, arg_type, desc
        );
        fields.insert(
            name.to_string(),
            json!({"type":arg_type, "description":desc}),
        );
        required.push(name.to_string());
    }
    let fields = Value::Object(fields);
    let fields = serde_json::to_string(&fields).unwrap();
    //let fields = serde_json::value::to_raw_value(&fields).unwrap();

    // Convert Value to RawValue
    quote! {
        fn #name(#inputs) #output { #(#stmts)* }
        fn #json_value() -> Value {
            json!(
                {
                    "type": "function",
                    "function": {
                        "name": stringify!(#name),
                        "description": "Description of the function",
                        "parameters": {
                            "type": "object",
                            "properties": #fields,
                            "required": [#(#required),*]
                        }
                    }
                }
            )
        }
    }
    .into()
}

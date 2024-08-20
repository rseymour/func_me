use derive_quote_to_tokens::ToTokens;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, FnArg, Ident, ItemFn, ItemMod, Pat, Type};

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

#[proc_macro_attribute]
pub fn json_value(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = input.attrs.clone();
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    // this is the function that is inserted into the attributed code
    // ideally we would add a trait and an impl
    let json_value = format_ident!("json_value_{}", name);
    let args = extract_function_raw(&input);

    let mut fields = Vec::new();
    let mut required = Vec::new();
    let mut docs = Vec::new();
    //let mut docs = Vec::new();
    for arg in args {
        let name = arg.name.to_string();
        let arg_type = rust_type_to_json_schema(&arg.arg_type);
        let desc = arg.description;
        let field = quote! {  #name: {"type": #arg_type , "description": #desc} };
        fields.push(field);
        required.push(name);
    }
    for attr in attrs {
        match &attr.meta {
            syn::Meta::Path(_) => (),
            syn::Meta::List(_) => (),
            syn::Meta::NameValue(nv) => {
                let v = nv.value.clone();
                //eprintln!("OPOOKOAKSDFOAKSDOFKOAK {:#?}", quote!(#v));
                docs.push(quote!(#v));
            }
        }
        /*
        if let Ok(doc) = syn::parse::<ItemMod>(attr.into()) {
            let doc = parse_enum_doc_comment(&doc.attrs);
            if let Some(doc) = doc {
                println!("doc: {}", doc);
                docs.push(doc);
            }
        }
         */
        //if let syn::Meta::NameValue(meta) = meta {
        //    //if let syn::Lit::Str(doc) = meta.lit {
        //    use quote::ToTokens;
        //    return Some(meta.value.into_token_stream().to_string());
        //    //}
        //}
    }
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
                            "required": [#(#required),*],
                            "properties": {#(#fields),*},
                            "docs": [#(#docs),*]
                        }
                    }
                }
            )
        }
    }
    .into()
}

fn parse_enum_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        let meta = attr.parse_args().unwrap();
        if let syn::Meta::NameValue(meta) = meta {
            //if let syn::Lit::Str(doc) = meta.lit {
            use quote::ToTokens;
            return Some(meta.value.into_token_stream().to_string());
            //}
        }
    }

    None
}

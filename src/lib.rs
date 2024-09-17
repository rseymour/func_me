use std::collections::HashMap;

use derive_quote_to_tokens::ToTokens;
use quote::{format_ident, quote};
use regex::Regex;
use syn::{parse_macro_input, ItemImpl, LitStr};
use syn::{FnArg, Ident, ItemFn, Pat, Type};

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
                    "i32" | "i64" | "isize" | "u8" | "u128" => "integer",
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
pub fn tool_json_for_fn(
    _attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = input.attrs.clone();
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    // this is the function that is inserted into the attributed code
    // ideally we would add a trait and an impl
    let json_value = format_ident!("json_value_{}", name);
    let args: Vec<Argument> = extract_function_raw(&input);

    // these could/should be sets if iteration order is preserved
    // we rely too much on iteration order
    let mut fields = Vec::new();
    let mut required = Vec::new();
    let mut arg_desc = HashMap::new();
    let re = Regex::new(r".*?`(?<arg_name>.*?)`\W+(?<arg_description>.*)$").unwrap();
    //todo create capture groups and us the regex to get the name and description
    // and push them into a map in raw docs which can be linked/lined up with fields
    for attr in attrs {
        match &attr.meta {
            syn::Meta::NameValue(nv) => {
                let v = nv.value.clone();
                match v {
                    syn::Expr::Lit(s) => match s.lit {
                        syn::Lit::Str(me) => {
                            let haystack = me.value();
                            let arg_caps = re.captures(&haystack).expect("we have doc strings formatted like: /// `arg_name` - arg_description");
                            arg_desc.insert(
                                arg_caps["arg_name"].to_string(),
                                arg_caps["arg_description"].to_string(),
                            );
                        }
                        _ => eprintln!("error in doc string matching code"),
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    for arg in args {
        let name = arg.name.to_string();
        let arg_type = rust_type_to_json_schema(&arg.arg_type);
        //let desc = arg.description;
        let desc = match arg_desc.get(name.as_str()) {
            Some(desc) => desc,
            None => "",
        };
        let field = quote! {  #name: {"type": #arg_type , "description": #desc} };
        fields.push(field);
        required.push(name);
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
                        }
                    }
                }
            )
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn toolbox(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let ty = &input.self_ty;

    let impl_names: Vec<_> = input
        .items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(method) = item {
                Some(method.sig.ident.to_string())
            } else {
                None
            }
        })
        .collect();

    let impl_names_tokens = impl_names.iter().map(|name| quote! { #name });

    // these are the names of the wrapped up functions that go from
    // fn whatever(a: A, b: B) -> x: X to fn_json_whatever(Value) -> Value
    // FIXME make prefixes configurable
    let fn_json_tokens = impl_names
        .iter()
        .map(|name| format_ident!("value_fn_{}", name));
    let impl_values = impl_names
        .iter()
        .map(|name| format_ident!("json_value_{}", name));

    let expanded = quote! {
        #input

        impl #ty {
            pub fn get_impl_names() -> Vec<&'static str> {
                vec![#(#impl_names_tokens),*]
            }
            // value_fn is a name for the function that takes a Value and returns a Value
            // but is really just a wrapper around the original function
            pub fn call_value_fn(name: &str, input: Value) -> Value {
                use std::collections::HashMap;
                use std::iter::zip;
                let matched_func = zip(vec![#(#impl_names),*], vec![#(#ty::#fn_json_tokens),*])
                    .find(|(my_name, fn_json)| my_name == &name );
                // weird to construct on fly but it works?
                match matched_func {
                    Some((_, fn_json)) => fn_json(input),
                    None => json!("function not found")
                }
            }
            pub fn get_impl_json() -> Value {
                let impls = vec![#(#ty::#impl_values),*];
                let mut impl_vals = Vec::new();
                for impl_fn in impls.iter() {
                    impl_vals.push(impl_fn());
                }
                serde_json::json!(impl_vals)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn add_to_toolbox(
    attribute_args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // TODO allow renaming function in the json using an attr?
    // for now just pull out the function description
    // TokenStream [Literal { kind: Str, symbol: "this is a secret third function", suffix: None, span: #0 bytes(881..914) }]
    let function_description = parse_macro_input!(attribute_args as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    let attrs = input.attrs.clone();
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let stmts = &input.block.stmts;
    // this is the function that is inserted into the attributed code
    // ideally we would add a trait and an impl
    let json_value = format_ident!("json_value_{}", name);
    let value_fn_name = format_ident!("value_fn_{}", name);
    let args = extract_function_raw(&input);

    // these could/should be sets if iteration order is preserved
    // we rely too much on iteration order
    let mut fields = Vec::new();
    let mut required = Vec::new();
    let mut value_fn_args = Vec::new();
    let mut arg_desc = HashMap::new();
    let re = Regex::new(r".*?`(?<arg_name>.*?)`\W+(?<arg_description>.*)$").unwrap();
    for attr in attrs {
        match &attr.meta {
            syn::Meta::NameValue(nv) => {
                let v = nv.value.clone();
                match v {
                    syn::Expr::Lit(s) => match s.lit {
                        syn::Lit::Str(me) => {
                            let haystack = me.value();
                            let arg_caps = re.captures(&haystack).expect("we have doc strings formatted like: /// `arg_name` - arg_description");
                            arg_desc.insert(
                                arg_caps["arg_name"].to_string(),
                                arg_caps["arg_description"].to_string(),
                            );
                        }
                        _ => eprintln!("error in doc string matching code"),
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    for arg in args {
        let name = arg.name.to_string();
        let arg_type = rust_type_to_json_schema(&arg.arg_type);
        let desc = match arg_desc.get(name.as_str()) {
            Some(desc) => desc,
            None => "",
        };
        let vfa = quote! { input[#name] };
        let vf_type = &arg.arg_type;
        let vfa = quote! { serde_json::from_value::<#vf_type>(#vfa.clone()).unwrap() };
        value_fn_args.push(vfa);
        let field = quote! {  #name: {"type": #arg_type , "description": #desc} };
        fields.push(field);
        required.push(name);
    }
    quote! {
        pub fn #name(#inputs) #output { #(#stmts)* }
        pub fn #value_fn_name(input: Value) -> Value {
            // FIXME right now the Value of the attributed function is forced to be
            // a Result *and* json serializable this is probably not sustainable
            match Self::#name(#( #value_fn_args),*) {
                Ok(result) => json!(result),
                Err(e) => json!(e.to_string())
            }
        }
        pub fn #json_value() -> Value {
            json!(
                {
                    "type": "function",
                    "function": {
                        "name": stringify!(#name),
                        "description": #function_description,
                        "parameters": {
                            "type": "object",
                            "required": [#(#required),*],
                            "properties": {#(#fields),*},
                        }
                    }
                }
            )
        }
    }
    .into()
}

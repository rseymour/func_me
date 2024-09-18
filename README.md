# func_me

## LLM tool calling with rust attributes and minimal stringiness

Tool calling is a feature of open and closed LLM APIs.
The JSON format posted to each API is near-jsonschema, but most libraries require you to write that schema by hand.
The JSON format returned by each API is relatively simple but requires work to turn into a function call.
`func_me` automates the creation of the json in the request and a wrapper run the returned tool call automatically.
All of this with as much compile time type checking and instant editor feedback as possible.

Give it a try!

## Toolbox mode with function calling

This is the mode currently seen in `examples/ollama_fn.rs`.

Can be run with `cargo run --example ollama_fn`

What happens is that now in addition to generating the JSON to describe the
tooling, this crate now generates a way to call the rust function. Any function with the `#[add_to_toolbox("xyz")]` attribute inside of your impl is now
callable with

```rust
tool_return = MyToolBox::call_value_fn(tool_name, tool_args);
```

where `tool_name` is `.message.tool_calls[].function.name` and `tool_args` is a
`serde_json::Value` of `.message.tool_calls[].function.arguments`

In the example you can see how this allows hands-free (without ever typing the a _stringy_ value of a function name) calling of functions.
Returning the result as JSON is a bit of a hack right now, but it can be done in the future.

## Toolbox mode

This is the mode currently seen in `src/main.rs`.

```rust
struct MyToolChest;

#[toolbox] // this makes the MyToolChest struct into a toolbox giving it the `get_impl_json` associated function
impl MyToolChest {
    #[add_to_toolbox("tightens a bolt")] // this adds the following function to the toolbox with the description "tightens a bolt"
    /// `bolt_location` - Location of bolt in need of tightening
    pub fn bolt_tightener(bolt_location: String) -> Result<String, std::io::Error> {
        // TODO add bolt tightening logic
        Ok(format!("I might have tightend the bolt located here: {}", bolt_location))
    }
}

fn main() {
    // the same json_value_ prefix is added by the derive macro, but inside of the
    // toolbox impl to stay out of the way
    let json_of_tools = MyToolChest::get_impl_json();
    println!("{}", serde_json::to_string_pretty(&json_of_tools).unwrap());
}

```

output, note it is a list since multiple tools can be added using the same pattern above, see main.rs:

```json
[
  {
    "function": {
      "description": "tightens a bolt",
      "name": "bolt_tightener",
      "parameters": {
        "properties": {
          "bolt_location": {
            "description": "Location of bolt in need of tightening",
            "type": "string"
          }
        },
        "required": ["bolt_location"],
        "type": "object"
      }
    },
    "type": "function"
  }
]
```

## Original PoC Mode

This mode works when you just need a `serde_json::Value` for your function and don't mind the namespace pollution
Basic idea is to have a function attribute `#[tool_json_for_fn]` and some doc comments:

```rust
/// `secret_key` - The secret key used for things
/// `query` - The query you want to ask
#[tool_json_for_fn]
fn some_other_function(secret_key: String, query: String) -> Result<String, std::io::Error> {
    Ok(format!("{} {}", secret_key, query))
}
```

Automatically generate:

```json
{
  "function": {
    "description": "Description of the function",
    "name": "some_other_function",
    "parameters": {
      "properties": {
        "query": {
          "description": "The query you want to ask",
          "type": "string"
        },
        "secret_key": {
          "description": "The secret key used for things",
          "type": "string"
        }
      },
      "required": ["secret_key", "query"],
      "type": "object"
    }
  },
  "type": "function"
}
```

NOTE: work in progress/work in public:

general:

- [x] generate a toolbox of functions which can all be turned into json with one call
- [x] generate a function at compile time which outputs a `serde_json::Value` of the "function" schema
- [ ] add a toolbox function to hide all of the json parsing needed to call the `call_value_fn`
- [ ] _rustdoc for the macros_
- [x] write example of how to use this with function calling + ollama and/or other APIs
- [ ] generate a trait (possibly first for [ollama-rs](https://github.com/pepperoni21/ollama-rs))
- [x] do anything turning function output to JSON or utilizing it (I think this may never happen)
- [x] allow function calling from values in the messages JSON (ie the LLM API response JSON)

must:

- [ ] make the docstrings <-> args 1:1 (ie no undoc'd args and no docs for non-existant args
- [x] make a function description syntax
- [x] make an impl ~trait~

soon:

- [ ] examples w/ pyO3/maturin
- [ ] examples that link to ollama-rs, etc

maybe:

- [ ] could try schemars to do the thing, but I think syn is needed so that hack is fine

# Notes

The only similar proc macros I've seen have been in web tools like poem, dropshot, etc.
Dropshot is one I'm somewhat familiar with and it offers 2 types of attributes.
One is function based and the other is trait based.

`func_me` offers a function based attribute for _getting_ the json.

But I _don't_ offer a trait based impl.
Instead I chose 2 attribute style which requires a struct and then a 'plain' impl instead of a trait impl.
Something about traits in this case rubs me as overkill, feeling like twice the work.
With a plain impl, you don't have to define the function twice, and you get the 'namespacing' of the struct for the impl.

I know dropshot had [reasons](https://docs.rs/dropshot/latest/dropshot/#choosing-between-functions-and-traits) for the trait style, but I like this middle ground.
I'd love to hear other folks thoughts on what's wrong with it.

I think there could be a case for traits because of multiple implementations with function calling, but I haven't run into one yet.

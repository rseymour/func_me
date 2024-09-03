# func\_me (WIP)
## LLM tool calling with rust attributes and minimal stringiness

Tool calling is a fun new feature of open and closed LLM APIs. 
The JSON format posted to each API is near-jsonschema, but most libraries require you to write that schema by hand.
This is an attempt to write that json automatically. 

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
        "required": [
          "bolt_location"
        ],
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
      "required": [
        "secret_key",
        "query"
      ],
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
- [ ] *rustdoc for the macros*
- [ ] write example of how to use this with function calling + ollama and/or other APIs
- [ ] generate a trait (possibly first for [ollama-rs](https://github.com/pepperoni21/ollama-rs))
- [ ] do anything turning function output to JSON or utilizing it (I think this may never happen)

must:
- [ ] make the docstrings <-> args 1:1 (ie no undoc'd args and no docs for non-existant args
- [x] make a function description syntax
- [x] make an impl ~trait~

soon:
- [ ] examples w/ pyO3/maturin
- [ ] examples that link to ollama-rs, etc

maybe:
- [ ] could try schemars to do the thing, but I think syn is needed so that hack is fine

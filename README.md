# func\_me
## LLM tool calling json with one rust function attribute macro

Tool calling is a fun new feature of open and closed LLM APIs. 
The JSON format posted to each API is near-jsonschema, but most libraries require you to write that schema by hand.
This is an attempt to write that json automatically. 

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

- [x] generate a function at compile time which outputs a `serde_json::Value` of the "function" schema
- [ ] write example of how to use this with function calling + ollama and/or other APIs
- [ ] generate a trait (possibly first for [ollama-rs](https://github.com/pepperoni21/ollama-rs))
- [ ] do anything turning function output to JSON or utilizing it (I think this may never happen)

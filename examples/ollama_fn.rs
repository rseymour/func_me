use anyhow::Result;
use auto_toolbox::{add_to_toolbox, toolbox};
use reqwest::Client;
use serde_json::json;
use serde_json::Value;

struct MyToolBox;

#[toolbox]
impl MyToolBox {
    #[add_to_toolbox("tightens a lid")] // this adds the following function to the toolbox with the description "tightens a lid"
    /// `rotations` - number of rotations
    pub fn lid_tightener(rotations: f32) -> Result<String, std::io::Error> {
        println!(
            "running some cool rotation code with rotations: {}",
            rotations
        );
        Ok(format!("this many rotations: {}", rotations))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    let json_of_tools = MyToolBox::get_impl_json();

    let payload = json!({
        "model": "llama3.1",
        "messages": [
            {
                "role": "user",
                "content": "How many rotations are needed to tighten the lid of a soda bottle?"
            }
        ],
        "stream": false,
        "tools": &json_of_tools
    });

    let response = client
        .post("http://localhost:11434/api/chat")
        .json(&payload)
        .send()
        .await?;

    let body: Value = response.json().await?;
    // FIXME This needs a lot of help (probably typing the return value of the API)
    for tool_call in body["message"]["tool_calls"].as_array().unwrap() {
        let tool_name = tool_call["function"]["name"].as_str().unwrap();
        let tool_args = &tool_call["function"]["arguments"];
        let tool_return = MyToolBox::call_value_fn(tool_name, tool_args.clone());
        dbg!(tool_return);
    }

    Ok(())
}

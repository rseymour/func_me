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

    let body = response.text().await?;
    println!("{}", body);
    let _x = MyToolBox::call_value_fn("lid_tightener", json!({"rotations": 3.0}));

    Ok(())
}

use auto_toolbox::add_to_toolbox;
use auto_toolbox::toolbox;
use serde_json::json;
use serde_json::Value;

// see https://en.wikipedia.org/wiki/Henry_O._Studley for where the name comes from
// I wish this didn't need its own line, but perhaps it should hold something more
struct StudleyToolChest;

#[toolbox]
impl StudleyToolChest {
    #[add_to_toolbox("a normal function to do stuff")]
    /// `normalcy` - Totally normal thing you trust an LLM to do
    pub fn totally_normal_function(normalcy: String) -> Result<String, std::io::Error> {
        Ok(format!("see this is normal right? {}", normalcy))
    }
    /// * `secret_key` - The secret key used for things
    /// * `query` - The query you want to ask
    #[add_to_toolbox("this is a secret third function")]
    pub fn some_secret_third_function(
        secret_key: String,
        query: String,
    ) -> Result<String, std::io::Error> {
        // function body
        Ok(format!("{} {}", secret_key, query))
    }
    #[add_to_toolbox("tightens a bolt")] // this adds the following function to the toolbox with the description "tightens a bolt"
    /// `bolt_location` - Location of bolt in need of tightening
    pub fn bolt_tightener(bolt_location: String) -> Result<String, std::io::Error> {
        // TODO add bolt tightening logic
        Ok(format!(
            "I might have tightend the bolt located here: {}",
            bolt_location
        ))
    }
    /// `rotations` - number of rotations
    /// `brand` - brand of new tool
    #[add_to_toolbox("this is a new tool")]
    pub fn new_tool(rotations: u32, brand: String) -> Result<u32, std::io::Error> {
        println!("rotations: {}, brand: {}", rotations, brand);
        Ok(rotations)
    }
}

fn main() {
    // the json_value_ prefix is added by the derive macro, but inside of the
    // toolbox impl to stay out of the way
    let json_of_tools = StudleyToolChest::get_impl_json();
    println!("{}", serde_json::to_string_pretty(&json_of_tools).unwrap());
}

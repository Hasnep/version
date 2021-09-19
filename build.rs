use std::collections::HashMap;

fn main() {
    let tools: HashMap<&str, &str> = serde_json::from_str(include_str!("data/tools.json")).unwrap();
    uneval::to_out_dir(tools, "tools.rs");
}

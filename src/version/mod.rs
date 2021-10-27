use std::collections::HashMap;

mod snap;
mod utils;

pub fn get_tool_version(tool_name: &str, version_argument: Option<&&str>) -> Option<String> {
    let is_tool_installed = utils::get_is_tool_installed(tool_name);
    if is_tool_installed {
        match version_argument {
            Some(version_argument) => match utils::get_version_of_tool(tool_name, version_argument)
            {
                Some(version) => return Some(version),
                None => {}
            },
            None => {}
        }
    }

    // Snaps
    if utils::get_is_snap_installed() {
        println!("Checking snap...");
        match snap::check_snap(tool_name) {
            Some(version) => return Some(version.to_string()),
            None => {}
        }
    }

    // If the tool has not been found return nothing
    return None;
}

pub fn print_list_of_tools(tools: &HashMap<&str, &str>) {
    for (tool_name, _) in tools {
        println!("{}", tool_name)
    }
}

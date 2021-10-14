use std::process::Command;

pub fn get_version_of_tool(tool_name: &str, argument: &str) -> Option<String> {
    return match Command::new(tool_name).arg(argument).output() {
        Ok(output) => String::from_utf8(output.stdout).ok(),
        Err(_) => None,
    };
}

pub fn get_is_tool_installed(tool_name: &str) -> bool {
    return match Command::new("which").arg(tool_name).output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    };
}

pub fn get_is_snap_installed() -> bool {
    return get_is_tool_installed("snap");
}

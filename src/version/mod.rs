use regex::Regex;
use std::collections::HashMap;
use std::process::Command;

pub fn print_list_of_tools(tools: &HashMap<&str, &str>) {
    for (tool_name, _) in tools {
        println!("{}", tool_name)
    }
}

pub fn get_tool_version(tool_name: &str, version_argument: Option<&&str>) -> Option<String> {
    let is_tool_installed = get_is_tool_installed(tool_name);
    if is_tool_installed {
        match version_argument {
            Some(version_argument) => match get_version_of_tool(tool_name, version_argument) {
                Some(version) => return Some(version),
                None => {}
            },
            None => {}
        }
    }

    // Snaps
    let is_snap_installed = get_is_snap_installed();
    if is_snap_installed {
        println!("Checking snap...");
        match check_snap(tool_name) {
            Some(version) => return Some(version.to_string()),
            None => {}
        }
    }

    // If the tool has not been found return nothing
    return None;
}

fn get_version_of_tool(tool_name: &str, argument: &str) -> Option<String> {
    return match Command::new(tool_name).arg(argument).output() {
        Ok(output) => String::from_utf8(output.stdout).ok(),
        Err(_) => None,
    };
}

fn get_is_tool_installed(tool_name: &str) -> bool {
    return match Command::new("which").arg(tool_name).output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    };
}

fn get_is_snap_installed() -> bool {
    return get_is_tool_installed("snap");
}

fn extract_version_from_snap_output(tool_name: &str, stdout: &str) -> Option<String> {
    let pattern = Regex::new(&format!(r"{}\s+(\S*)", tool_name)).unwrap();
    return Some(pattern.captures(&stdout)?.get(1)?.as_str().to_string());
}

fn check_snap(tool_name: &str) -> Option<String> {
    let output = Command::new("snap")
        .arg("list")
        .arg(tool_name)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout);
        return match &stdout {
            Ok(s) => extract_version_from_snap_output(tool_name, &s),
            Err(_) => None,
        };
    } else {
        return None;
    }
}

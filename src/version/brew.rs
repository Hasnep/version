use regex::Regex;
use std::process::Command;

pub fn check_brew(tool_name: &str) -> Option<String> {
    let output = Command::new("brew")
        .arg("info")
        .arg(tool_name)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        return extract_version_from_brew_output(tool_name, &stdout);
    } else {
        return None;
    }
}

fn extract_version_from_brew_output(tool_name: &str, stdout: &str) -> Option<String> {
    let pattern = Regex::new(&format!(r"{}: (.*)", tool_name)).unwrap();
    return Some(pattern.captures(&stdout)?.get(1)?.as_str().to_string());
}

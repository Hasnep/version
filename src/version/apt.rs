use regex::Regex;
use std::process::Command;

pub fn check_apt(tool_name: &str) -> Option<String> {
    let output = Command::new("apt")
        .arg("list")
        .arg("--installed")
        .arg(tool_name)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).unwrap();
        return extract_version_from_apt_output(tool_name, &stdout);
    } else {
        return None;
    }
}

fn extract_version_from_apt_output(tool_name: &str, stdout: &str) -> Option<String> {
    let pattern = Regex::new(&format!(r"\n{}/\S*?\s+(\S*)", tool_name)).unwrap();
    return Some(pattern.captures(&stdout)?.get(1)?.as_str().to_string());
}

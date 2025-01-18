use std::process::Command;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct FirewallRule {
    pub name: String,
    pub enabled: bool,
}

// TODO: Use this function to fetch and display firewall rules in the UI
#[allow(dead_code)]
pub fn get_firewall_rules() -> Vec<FirewallRule> {
    let output = Command::new("netsh")
        .args(&["advfirewall", "firewall", "show", "rule", "name=all"])
        .output()
        .expect("Failed to execute netsh command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_firewall_rules(&stdout)
}

// TODO: Internal helper function for parsing firewall rules
#[allow(dead_code)]
fn parse_firewall_rules(output: &str) -> Vec<FirewallRule> {
    let mut rules = Vec::new();
    for line in output.lines() {
        if let Some(name) = line.strip_prefix("Rule Name:") {
            let name = name.trim().to_string();
            // This is a simplified parsing, assuming rules are listed sequentially
            // and you can find the "Enabled" or "Disabled" status in subsequent lines
            let enabled = output.lines().skip_while(|l| !l.contains(&name)).nth(3).map_or(false, |l| l.contains("Enabled"));
            rules.push(FirewallRule { name, enabled });
        }
    }
    rules
}

// TODO: Implement this in the UI for toggling firewall rules
#[allow(dead_code)]
pub fn toggle_rule(rule_name: &str) -> bool {
    let status = if is_rule_enabled(rule_name) { "disable" } else { "enable" };
    let output = Command::new("netsh")
        .args(&["advfirewall", "firewall", "set", "rule", "name=", rule_name, status])
        .output()
        .expect("Failed to execute netsh command to toggle rule");

    output.status.success()
}

// TODO: Use this to check rule status, possibly in the UI or for rule management
#[allow(dead_code)]
fn is_rule_enabled(rule_name: &str) -> bool {
    let output = Command::new("netsh")
        .args(&["advfirewall", "firewall", "show", "rule", "name=", rule_name])
        .output()
        .expect("Failed to check rule status");

    String::from_utf8_lossy(&output.stdout).contains("Enabled")
}

// TODO: Implement in the UI for adding new firewall rules
#[allow(dead_code)]
pub fn add_firewall_rule(rule_name: &str, protocol: &str, local_port: &str, action: &str) -> bool {
    let output = Command::new("netsh")
        .args(&[
            "advfirewall", "firewall", "add", "rule",
            "name=", rule_name,
            "protocol=", protocol,
            "localport=", local_port,
            "action=", action
        ])
        .output()
        .expect("Failed to add new firewall rule");

    output.status.success()
}
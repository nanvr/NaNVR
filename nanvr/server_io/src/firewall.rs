use net_packets::FirewallRulesAction;
use shared::NANVR_LOW_NAME;
use std::{path::PathBuf, process::Command};

// todo: might need to be moved somewhere else
// Errors:
// 1: firewall rule is already set
// 126: pkexec request dismissed
// other: command failed
pub fn firewall_rules(
    action: FirewallRulesAction,
    filesystem_layout: &filepaths::Layout,
) -> Result<(), i32> {
    let action = if matches!(action, FirewallRulesAction::Add) {
        "add"
    } else {
        "remove"
    };
    // run as normal user since we use pkexec to sudo
    let exit_status = Command::new("bash")
        .arg(
            PathBuf::from("../").join(
                filesystem_layout
                    .firewall_script_dir
                    .join(format!("{NANVR_LOW_NAME}_fw_config.sh")),
            ),
        )
        .arg(action)
        .status()
        .map_err(|_| -1)?;

    if exit_status.success() {
        Ok(())
    } else {
        Err(exit_status.code().unwrap())
    }
}

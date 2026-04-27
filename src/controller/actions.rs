use std::process::Command;

use crate::models::PomoKind;

pub fn alert_user(pomo_kind: &PomoKind) {
    let _ = Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("-a")
        .arg("pomo")
        .arg(format!(
            "START {}: {} mins",
            pomo_kind,
            pomo_kind.get_mins()
        ))
        .spawn();

    let _ = Command::new("paplay")
        .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
        .spawn();
}

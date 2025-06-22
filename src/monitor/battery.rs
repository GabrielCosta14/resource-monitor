use anyhow::Result;
use std::process::Command;

#[derive(Default, Clone)]
pub struct Battery {
    pub pct:     u8,
    pub state:   String,            // charging | discharging | charged
    pub time:    Option<String>,    // e.g. "2:44 remaining"
}

pub fn status() -> Result<Battery> {
    let out = Command::new("pmset").args(["-g", "batt"]).output()?;
    if !out.status.success() {
        return Err(anyhow::anyhow!("pmset exit {}", out.status));
    }
    let txt = String::from_utf8_lossy(&out.stdout);
    // find first “… %; <state>; <time>”
    for line in txt.lines() {
        if let Some(idx) = line.find('%') {
            // percentage
            let pct_fragment = line[..idx].split_whitespace().last().unwrap_or("0");
            let pct = pct_fragment.parse::<u8>().unwrap_or(0);

            // rest of the line
            let after = &line[idx + 1..];
            let mut seg = after.split(';').map(|s| s.trim());
            let state = seg.next().unwrap_or("").to_lowercase();     // charging/...
            let t     = seg.next().unwrap_or("").replace("remaining", "").trim().to_string();
            let time  = if t.is_empty() || t.starts_with("(no") { None } else { Some(t) };

            return Ok(Battery { pct, state, time });
        }
    }
    Err(anyhow::anyhow!("battery line not found"))
}

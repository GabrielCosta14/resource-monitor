mod monitor;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use monitor::Monitor;
use std::time::{Duration, Instant};
use ui::UI;

const TICK: Duration = Duration::from_millis(500);

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut ui = UI::new()?;
    ui.clear()?;                        
    let mut mon = Monitor::new();

    loop {
        let start = Instant::now();
        ui.draw(&mon.sample()?)?;

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                if k.code == KeyCode::Char('q') {
                    disable_raw_mode()?;
                    ui.clear()?;
                    return Ok(());
                }
            }
        }
        if let Some(rem) = TICK.checked_sub(start.elapsed()) {
            std::thread::sleep(rem);
        }
    }
}

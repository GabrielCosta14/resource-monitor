use crate::monitor::Snapshot;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Terminal,
};

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(std::io::stdout()))?,
        })
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, s: &Snapshot) -> anyhow::Result<()> {
        self.terminal.draw(|f| {
            let rows = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3); 7])
                .split(f.size());

            // ── CPU gauge ──
            let cpu = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("CPU %"))
                .gauge_style(coloured(s.cpu_pct as u16))
                .percent(s.cpu_pct as u16);
            f.render_widget(cpu, rows[0]);

            // ── CPU sparkline ──
            let cpu_hist: Vec<u64> =
                s.cpu_hist.iter().map(|v| (*v * 100.0) as u64).collect();
            let spark = Sparkline::default()
                .block(Block::default().borders(Borders::ALL).title("Last 60 s"))
                .data(&cpu_hist)
                .max(100);
            f.render_widget(spark, rows[1]);

            // ── Memory gauge ──
            let mem_pct = ((s.mem_used * 100) / s.mem_total) as u16;
            let mem = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "MEM {} / {} MiB",
                    s.mem_used, s.mem_total
                )))
                .gauge_style(coloured(mem_pct))
                .percent(mem_pct);
            f.render_widget(mem, rows[2]);

            // ── Disk gauge ──
            let disk_pct = ((s.disk_used * 100) / s.disk_total.max(1)) as u16;
            let disk = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(format!(
                    "SSD {} / {} GiB",
                    s.disk_used, s.disk_total
                )))
                .gauge_style(coloured(disk_pct))
                .percent(disk_pct);
            f.render_widget(disk, rows[3]);

            // ── FD count ──
            let fd = Paragraph::new(format!("FDs open: {}", s.fds))
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(fd, rows[4]);

            // ── Disk I/O delta ──
            let io = Paragraph::new(format!(
                "Disk Δ  Read {:>6} KB   Write {:>6} KB",
                s.io_read / 1024,
                s.io_written / 1024
            ))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(io, rows[5]);

            // ── Battery ──
            let batt_txt = if let Some(t) = &s.batt_time {
                format!("🔋 {} % {} – {}", s.batt_pct, s.batt_state, t)
            } else {
                format!("🔋 {} % {}", s.batt_pct, s.batt_state)
            };
            let batt = Paragraph::new(batt_txt)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(batt, rows[6]);
        })?;
        Ok(())
    }
}

fn coloured(p: u16) -> Style {
    let fg = match p {
        0..=50 => Color::Green,
        51..=80 => Color::Yellow,
        _ => Color::Red,
    };
    Style::default().fg(fg).add_modifier(Modifier::BOLD)
}

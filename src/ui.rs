use crate::monitor::Snapshot;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl UI {
    pub fn new() -> anyhow::Result<Self> {
        let backend  = CrosstermBackend::new(std::io::stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, snap: &Snapshot) -> anyhow::Result<()> {
        self.terminal.draw(|f| {
            // every widget gets 3 rows so the interior has room
            let areas = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3); 4])   // 4×3-row boxes
                .split(f.size());

            // ── CPU ────────────────────────────────────────
            let cpu = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title("CPU %"))
                .gauge_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .percent(snap.cpu_pct.round() as u16);
            f.render_widget(cpu, areas[0]);

            // ── MEMORY ─────────────────────────────────────
            let mem_pct =
                (snap.mem_used as f64 / snap.mem_total as f64 * 100.0).round() as u16;
            let mem_title = format!("MEM {} / {} MiB", snap.mem_used, snap.mem_total);
            let mem = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(mem_title))
                .gauge_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .percent(mem_pct);
            f.render_widget(mem, areas[1]);

            // ── FD COUNT ───────────────────────────────────
            let fd_text = Paragraph::new(format!("FDs open: {}", snap.fds))
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(fd_text, areas[2]);

            // ── DISK IO Δ/0.5 s ───────────────────────────
            let io_text = Paragraph::new(format!(
                "Disk Δ  Read {:>6} KB   Write {:>6} KB",
                snap.io_read / 1024,
                snap.io_written / 1024
            ))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(io_text, areas[3]);
        })?;
        Ok(())
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.terminal.clear()?;
        Ok(())
    }
}

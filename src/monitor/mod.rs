pub mod cpu;
pub mod memory;
pub mod fd;
pub mod io;
pub mod disk;
pub mod battery;

use anyhow::Result;
use std::collections::VecDeque;

const HIST: usize = 120; // 60 s history @ 500 ms

#[derive(Default, Clone)]
pub struct Snapshot {
    pub cpu_pct:      f64,
    pub cpu_hist:     VecDeque<f64>,
    pub mem_used:     u64,
    pub mem_total:    u64,
    pub disk_used:    u64,
    pub disk_total:   u64,
    pub fds:          usize,
    pub io_read:      u64,
    pub io_written:   u64,
    pub batt_pct:     u8,
    pub batt_state:   String,
    pub batt_time:    Option<String>,
}

pub struct Monitor {
    cpu:     cpu::Cpu,
    memory:  memory::Memory,
    prev_io: Option<io::Io>,
    hist:    VecDeque<f64>,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            memory: memory::Memory::new(),
            prev_io: None,
            hist: VecDeque::with_capacity(HIST),
        }
    }

    pub fn sample(&mut self) -> Result<Snapshot> {
        // CPU
        let cpu_pct = self.cpu.usage();
        if self.hist.len() == HIST { self.hist.pop_front(); }
        self.hist.push_back(cpu_pct);

        // Memory
        let (mem_used, mem_total) = self.memory.used_vs_total_mb();

        // Disk
        let (disk_used, disk_total) = disk::used_vs_total_gib().unwrap_or_default();

        // FDs
        let fds = fd::count().unwrap_or(0);

        // I/O delta
        let io_now  = io::stats()?;
        let io_prev = self.prev_io.replace(io_now).unwrap_or_default();
        let io_read_delta  = io_now.read_bytes  - io_prev.read_bytes;
        let io_write_delta = io_now.write_bytes - io_prev.write_bytes;

        // Battery
        let batt = battery::status().unwrap_or_default();

        Ok(Snapshot {
            cpu_pct,
            cpu_hist: self.hist.clone(),
            mem_used,
            mem_total,
            disk_used,
            disk_total,
            fds,
            io_read:     io_read_delta,
            io_written:  io_write_delta,
            batt_pct:    batt.pct,
            batt_state:  batt.state,
            batt_time:   batt.time,
        })
    }
}

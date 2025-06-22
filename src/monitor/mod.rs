pub mod cpu;
pub mod memory;
pub mod fd;
pub mod io;

use anyhow::Result;

#[derive(Default, Clone)]
pub struct Snapshot {
    pub cpu_pct:     f64,
    pub mem_used:    u64,
    pub mem_total:   u64,
    pub fds:         usize,
    pub io_read:     u64,
    pub io_written:  u64,
}

pub struct Monitor {
    cpu:     cpu::Cpu,
    memory:  memory::Memory,
    prev_io: Option<io::Io>,
}
impl Monitor {
    pub fn new() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            memory: memory::Memory::new(),
            prev_io: None,
        }
    }

    pub fn sample(&mut self) -> Result<Snapshot> {
        let cpu_pct = self.cpu.usage();

        let (mem_used, mem_total) = self.memory.used_vs_total_mb();

        let fds = fd::count().unwrap_or(0);

        let io_now  = io::stats()?;
        let io_prev = self.prev_io.replace(io_now).unwrap_or_default();
        let io_read_delta    = io_now.read_bytes.saturating_sub(io_prev.read_bytes);
        let io_write_delta   = io_now.write_bytes.saturating_sub(io_prev.write_bytes);

        Ok(Snapshot {
            cpu_pct,
            mem_used,
            mem_total,
            fds,
            io_read:    io_read_delta,
            io_written: io_write_delta,
        })
    }
}

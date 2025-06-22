use sysinfo::{System, RefreshKind, MemoryRefreshKind};

pub struct Memory {
    sys: System,
}
impl Memory {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn used_vs_total_mb(&mut self) -> (u64, u64) {
        self.sys.refresh_memory();
        let used  = self.sys.used_memory() / 1_048_576;   // to MiB
        let total = self.sys.total_memory() / 1_048_576;
        (used, total)
    }
}

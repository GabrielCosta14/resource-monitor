use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Cpu {
    sys: System,
}
impl Cpu {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );
        Self { sys }
    }

    pub fn usage(&mut self) -> f64 {
        self.sys.refresh_cpu_usage();
        self.sys.global_cpu_info().cpu_usage() as f64
    }
}

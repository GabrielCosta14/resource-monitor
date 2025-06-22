
# Resource Monitor TUI (macOS)

A lightweight, terminal‑based resource dashboard written in Rust + [Ratatui]  
that works **un‑privileged** on contemporary macOS (Monterey → Sonoma).

```
┌ CPU % ────────────────┐
│██████████▌  56 %      │
└───────────────────────┘
┌ Last 60 s ────────────┐
│▁▂▂▃▄▅▇█▇▅▄▃▂▁         │
└───────────────────────┘
┌ MEM 6123 / 16384 MiB ┐
│██████▏ 37 %          │
└──────────────────────┘
┌ SSD 118 / 512 GiB ───┐
│███▎ 23 %             │
└──────────────────────┘
┌ FDs open: 152 ┐
└───────────────┘
┌ Disk Δ  R  24 KB  W  8 KB ┐
└───────────────────────────┘
┌ 🔋 86 % discharging – 2 h 44 m ┐
└────────────────────────────────┘
```

*(press **`q`** to quit)*

---

## Features
| Widget | Data Source | Refresh |
|--------|-------------|---------|
| CPU gauge & 60 s sparkline | `sysinfo` | 0.5 s |
| Memory gauge | `sysinfo` | 0.5 s |
| Disk‑usage gauge (root fs) | `statfs` | 5 s (same tick) |
| FD count | `/dev/fd` | 0.5 s |
| Disk I/O deltas | `proc_pid_rusage` | 0.5 s |
| Battery paragraph | `pmset -g batt` | 30 s (polled every tick) |

Colour threshold: **Green < 50 %**, **Yellow 50 – 80 %**, **Red > 80 %**.

No root, no entitlements required.

---

## Build & Run

```bash
# Requires Rust ≥ 1.70 and a macOS tool‑chain
git clone <your fork>
cd resource-monitor
cargo run --release      # q to quit
```

Optional: build an optimised binary

```bash
cargo build --release
./target/release/resource-monitor
```

---

## Controls

| Key | Action |
|-----|--------|
| `q` | Quit the monitor |

---

## Internals

* **Ratatui** renders the TUI.  
* **Crossterm** handles raw‑mode and key events.  
* Metrics are gathered in `monitor/`:
  * `cpu.rs` – system‑wide utilisation
  * `memory.rs` – RSS / total
  * `disk.rs` – root filesystem usage
  * `io.rs` – per‑process disk I/O
  * `fd.rs` – open file‑descriptors
  * `battery.rs` – wraps `pmset`
* `monitor::Monitor` produces a `Snapshot`; `ui.rs` draws it.

---

## Tested On

* macOS 14 **Sonoma** (Apple M1)
* macOS 13 **Ventura** (Intel)

---

## License

MIT

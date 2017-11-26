use libc::{sysinfo, SI_LOAD_SHIFT};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::time::Duration;

const SI_LOAD: f64 = (1 << SI_LOAD_SHIFT) as f64;

#[derive(Debug, Default)]
pub struct SystemInfo {
    pub uptime: Duration,
    pub load_1m: f64,
    pub load_5m: f64,
    pub memory_total: u64,
    pub memory_free: u64,
    pub memory_available: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub num_procs: u64,
}

pub fn load() -> SystemInfo {
    let mut si;
    unsafe {
        si = mem::zeroed();
        if sysinfo(&mut si) == -1 {
            return SystemInfo::default();
        }
    }

    SystemInfo {
        uptime: Duration::from_secs(si.uptime as u64),
        load_1m: si.loads[0] as f64 / SI_LOAD,
        load_5m: si.loads[1] as f64 / SI_LOAD,
        memory_total: si.totalram,
        memory_free: si.freeram,
        memory_available: mem_available().unwrap_or(0),
        swap_total: si.totalswap,
        swap_free: si.freeswap,
        num_procs: si.procs as u64,
    }
}

fn mem_available() -> Result<u64, Box<Error>> {
    let meminfo = File::open("/proc/meminfo")?;
    let reader = BufReader::new(meminfo);
    for line in reader.lines().filter_map(|l| l.ok()) {
        if line.starts_with("MemAvailable:") {
            if let Some(value) = line.split_whitespace().skip(1).next() {
                return Ok(value.parse::<u64>()? * 1024);
            }
        }
    }
    Ok(0)
}

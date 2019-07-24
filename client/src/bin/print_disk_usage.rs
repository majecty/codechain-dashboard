#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate parking_lot;
#[macro_use]
extern crate serde_derive;

extern crate core;
#[macro_use]
extern crate crossbeam;
extern crate jsonrpc_core;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate subprocess;
extern crate sysinfo;
extern crate systemstat;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_uds;
extern crate ws;

use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use parking_lot::Mutex;
use sysinfo::{DiskExt, SystemExt};
use systemstat::{CPULoad, DelayedMeasurement, Platform};


#[derive(Debug, Default, Clone)]
pub struct HardwareUsage {
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Default, Clone)]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
}

fn get_disk_usage(sys: &mut sysinfo::System) -> Vec<HardwareUsage> {
    sys.refresh_disk_list();
    sys.refresh_disks();

    let mut total: i64 = 0;
    let mut available: i64 = 0;
    let mut result: Vec<HardwareUsage> = Vec::new();
    for disk in sys.get_disks() {
        total += disk.get_total_space() as i64;
        available += disk.get_available_space() as i64;

        let percentage_used = if total == 0 {
            0f64
        } else {
            (total - available) as f64 / total as f64
        };
        result.push(HardwareUsage {
            total,
            available,
            percentage_used
        });
    }

    result
}

fn main() {
    let mut sysinfo_sys = sysinfo::System::new();
    let usage = get_disk_usage(&mut sysinfo_sys);

    println!("HI");
    println!("Usage: {:#?}", usage);
}

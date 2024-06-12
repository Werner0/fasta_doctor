//Continuously monitor memory usage

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use sysinfo::{System};

pub fn start_memory_monitoring(interval: Duration, is_running: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut system = System::new_all();
        let mut warning_issued = false; 

        while is_running.load(Ordering::SeqCst) {
            system.refresh_all();

            let total_memory = system.total_memory();
            let used_memory = system.used_memory();
            let memory_usage = (used_memory as f32 / total_memory as f32) * 100.0;

            if memory_usage > 90.0 && !warning_issued {
                println!("WARNING: system memory is getting exhausted.");
                warning_issued = true; // Set the flag to true after issuing the warning
            }

            thread::sleep(interval);
        }
    });
}
// use std::{thread::sleep, time::Duration};

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::Duration,
};
use sysinfo::System;

pub static BOOL_PROCCES_PINE: AtomicBool = AtomicBool::new(false);

pub fn cek() {
    loop {
        if let Ok(s) = System::new_all() {
            let found = s
                .processes()
                .values()
                .any(|p| p.name().to_string_lossy().contains("pcsx2-qt"));
            BOOL_PROCCES_PINE.store(found, Ordering::Relaxed);
        }
        sleep(Duration::from_secs(1));
    }
}

use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::Duration,
};
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

pub fn check_pcsx2_process() -> bool {
    static RESULT: AtomicBool = AtomicBool::new(false);
    static IS_RUNNING: AtomicBool = AtomicBool::new(false);

    if !IS_RUNNING.load(Ordering::Relaxed) {
        IS_RUNNING.store(true, Ordering::Relaxed);

        std::thread::spawn(|| {
            let mut s = System::new().unwrap();
            let mut pcsx2_pid = None;

            loop {
                if let Some(pid) = pcsx2_pid {
                    let _ = s.refresh_processes_specifics(
                        ProcessesToUpdate::Some(&[pid]),
                        true,
                        ProcessRefreshKind::nothing(),
                    );

                    if s.process(pid).is_none() {
                        pcsx2_pid = None;
                        RESULT.store(false, Ordering::Relaxed);
                    }
                } else {
                    let _ = s.refresh_processes_specifics(
                        ProcessesToUpdate::All,
                        true,
                        ProcessRefreshKind::nothing(),
                    );

                    pcsx2_pid = s
                        .processes()
                        .iter()
                        .find(|(_, p)| p.name().to_string_lossy().contains("pcsx2-qt"))
                        .map(|(pid, _)| *pid);

                    RESULT.store(pcsx2_pid.is_some(), Ordering::Relaxed);
                }
                sleep(Duration::from_millis(100));
            }
        });
    }

    RESULT.load(Ordering::Relaxed)
}

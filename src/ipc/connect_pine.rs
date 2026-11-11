use pine_ipc::{PINE, PINEBatch, PINECommand, PINEResponse};
use std::{
    net::TcpStream,
    sync::{
        Mutex,
        atomic::{AtomicBool, AtomicU32, Ordering},
    },
    thread::{sleep, spawn},
    time::Duration,
};

pub const TARGET_APP: &str = "pcsx2";
pub const PORT_PINE: u16 = 28011;
static PINE_CONNECTION: Mutex<Option<PINE<TcpStream>>> = Mutex::new(None);

pub fn check_games_is_running() -> (bool, String) {
    static GAME_ID: Mutex<String> = Mutex::new(String::new());
    static IS_RUNNING: AtomicBool = AtomicBool::new(false);
    if !IS_RUNNING.load(Ordering::Relaxed) {
        IS_RUNNING.store(true, Ordering::Relaxed);
        spawn(|| {
            let mut batch = PINEBatch::new();
            batch.add(PINECommand::MsgID);
            loop {
                let mut new_id = String::new();

                if let Ok(mut conn) = PINE_CONNECTION.lock() {
                    if conn.is_none() {
                        *conn = PINE::connect(TARGET_APP, PORT_PINE, true).ok();
                    }
                    if let Some(p) = &mut *conn {
                        if let Ok(res) = p.send(&mut batch) {
                            if let Some(PINEResponse::ResID { id }) = res.first() {
                                new_id = id.clone();
                            }
                        } else {
                            *conn = None;
                        }
                    }
                }

                *GAME_ID.lock().unwrap() = new_id;
                sleep(Duration::from_millis(100));
            }
        });
    }
    let id = GAME_ID.lock().unwrap().clone();
    return (!id.is_empty(), id);
}

pub fn rewrite_value(address: u32, value: u32) {
    if let Ok(mut conn) = PINE_CONNECTION.lock() {
        if conn.is_none() {
            *conn = PINE::connect(TARGET_APP, PORT_PINE, true).ok();
        }
        if let Some(p) = &mut *conn {
            let mut batch = PINEBatch::new();
            batch.add(PINECommand::MsgWrite32 {
                mem: address,
                val: value,
            });
            if p.send(&mut batch).is_err() {
                *conn = None;
            }
        }
    }
}

pub fn rewrite_value_freeze(address: u32, value: u32) {
    static TARGET_ADDRESS: AtomicU32 = AtomicU32::new(0);
    static TARGET_VALUE: AtomicU32 = AtomicU32::new(0);
    static IS_RUNNING: AtomicBool = AtomicBool::new(false);

    TARGET_ADDRESS.store(address, Ordering::Relaxed);
    TARGET_VALUE.store(value, Ordering::Relaxed);

    if !IS_RUNNING.load(Ordering::Relaxed) {
        IS_RUNNING.store(true, Ordering::Relaxed);

        spawn(|| {
            loop {
                rewrite_value(
                    TARGET_ADDRESS.load(Ordering::Relaxed),
                    TARGET_VALUE.load(Ordering::Relaxed),
                );
                sleep(Duration::from_micros(100));
            }
        });
    }
}

pub fn reading_value(address: u32, is_float: bool) -> String {
    if let Ok(mut conn) = PINE_CONNECTION.lock() {
        if let Some(p) = &mut *conn {
            let mut batch = PINEBatch::new();
            batch.add(PINECommand::MsgRead32 { mem: address });
            if let Ok(res) = p.send(&mut batch) {
                if let Some(PINEResponse::ResRead32 { val }) = res.first() {
                    if is_float {
                        return format!("{:.2}", f32::from_bits(*val));
                    } else {
                        return val.to_string();
                    }
                }
            } else {
                *conn = None;
            }
        }
    }
    return String::from("0");
}

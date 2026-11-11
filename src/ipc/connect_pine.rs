use std::{
    net::TcpStream,
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::sleep,
    time::Duration,
};

use pine_ipc::{PINE, PINEBatch, PINECommand, PINEResponse};

pub const TARGET_APP: &str = "pcsx2";
pub const PORT_PINE: u16 = 28011;
static PINE_CONN: Mutex<Option<PINE<TcpStream>>> = Mutex::new(None);

pub fn check_games_is_running() -> (bool, String) {
    static GAME_ID: Mutex<String> = Mutex::new(String::new());
    static IS_RUNNING: AtomicBool = AtomicBool::new(false);
    if !IS_RUNNING.load(Ordering::Relaxed) {
        IS_RUNNING.store(true, Ordering::Relaxed);
        std::thread::spawn(|| {
            let mut batch = PINEBatch::new();
            batch.add(PINECommand::MsgID);
            loop {
                let mut new_id = String::new();

                if let Ok(mut conn) = PINE_CONN.lock() {
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
                sleep(Duration::from_millis(200));
            }
        });
    }
    let id = GAME_ID.lock().unwrap().clone();
    return (!id.is_empty(), id);
}

pub fn rewrite_value(address: u32, value: u32, freeze: bool) {
    static CHECK_REWRITE: AtomicBool = AtomicBool::new(false);
    if !freeze && CHECK_REWRITE.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(mut conn) = PINE_CONN.lock() {
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
            } else if !freeze {
                CHECK_REWRITE.store(true, Ordering::Relaxed);
            }
        }
    }
}

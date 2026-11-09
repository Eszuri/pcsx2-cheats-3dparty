use pine_ipc::{PINE, PINEBatch, PINECommand};
pub const TARGET_APP: &str = "pcsx2";
pub const PORT_PINE: u16 = 28011;

pub fn rewrite_value(address: u32, value: u32) {
    let mut pine = PINE::connect(TARGET_APP, PORT_PINE, true).expect("Failed to connect to PCSX2");
    let mut batch = PINEBatch::new();
    batch.add(PINECommand::MsgWrite32 {
        mem: address,
        val: value,
    });
    pine.send(&mut batch).expect("failed write value");
}

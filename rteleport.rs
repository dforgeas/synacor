use std::io::{prelude::*, SeekFrom};
use std::fs::OpenOptions;
const MAX: u16 = 0x7fff;
const REG_END: u16 = MAX + 8;
const SET_FOUR_BEFORE_CALL: u16 = 5483 + 2;
const ACK_CALL: u16 = 5489;
const NOOP: u16 = 21;
const SAVESTATE_BIN: &str = "savestate.bin";
const REG_8_VALUE: u16 = 25734;
fn main() -> std::io::Result<()> {
    let mut savestate = OpenOptions::new().read(true).write(true).open(SAVESTATE_BIN)?;

    // set 6 into the first register instead, the expected result from ack
    savestate.seek(SeekFrom::Start(2 * SET_FOUR_BEFORE_CALL as u64))?;
    savestate.write(&6u16.to_le_bytes())?;

    // replace the call to ack with noops
    savestate.seek(SeekFrom::Start(2 * ACK_CALL as u64))?;
    for _ in 1..=2 { savestate.write(&NOOP.to_le_bytes())?; };

    // set the eigth register to the correct teleporter value
    savestate.seek(SeekFrom::Start(2 * REG_END as u64))?;
    savestate.write(&REG_8_VALUE.to_le_bytes())?;
    Ok(())
}

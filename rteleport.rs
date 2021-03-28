use std::io::{prelude::*, SeekFrom};
use std::fs::OpenOptions;
const MAX: u16 = 0x7fff;
const REG_END: u16 = MAX + 8;
const SAVESTATE_BIN: &str = "savestate.bin";
const REG_8_VALUE: u16 = 25734;
fn main() -> std::io::Result<()> {
    let mut savestate = OpenOptions::new().read(true).write(true).open(SAVESTATE_BIN)?;
    savestate.seek(SeekFrom::Start(2 * REG_END as u64))?;
    savestate.write(&REG_8_VALUE.to_le_bytes())?;
    Ok(())
}

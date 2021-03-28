use std::io::{prelude::*, SeekFrom};
use std::fs::OpenOptions;
const MAX: u16 = 0x7fff;
const REG_END: u16 = MAX + 8;
const MEM_SIZE: usize = MAX as usize + 1;
const SAVESTATE_BIN: &str = "savestate.bin";
fn main() -> std::io::Result<()> {
    let mut savestate = OpenOptions::new().read(true).write(true).open(SAVESTATE_BIN)?;
    savestate.seek(SeekFrom::Start(REG_END))?;
    savestate.write(25734.to_le_bytes())?;
}

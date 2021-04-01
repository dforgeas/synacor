use std::io::{prelude::*, SeekFrom};
use std::fs::OpenOptions;
const MAX: u16 = 0x7fff;
const REG_END: u16 = MAX + 8;
const SET_FOUR_BEFORE_CALL: u16 = 5483 + 2;
const ACK_CALL: u16 = 5489;
const NOOP: u16 = 21;
const SAVESTATE_BIN: &str = "savestate.bin";
const REG_8_VALUE: u16 = 25734;
fn display_position(s: SeekFrom) -> u64 {
    match s {
        SeekFrom::Start(x) => x / 2,
        _ => 0,
    }
}
fn main() -> std::io::Result<()> {
    let mut savestate = OpenOptions::new().read(true).write(true).open(SAVESTATE_BIN)?;

    let mut buf2 = [0u8; 2];
    // set 6 into the first register instead, the expected result from ack
    let mut position = SeekFrom::Start(2 * SET_FOUR_BEFORE_CALL as u64);
    savestate.seek(position)?;
    savestate.read_exact(&mut buf2)?;
    if u16::from_le_bytes(buf2) == 4 {
        println!("Writing 6 at position {}", display_position(position));
        savestate.seek(position)?;
        savestate.write_all(&6u16.to_le_bytes())?;
    } else {
        println!("Skipping writing at position {}", display_position(position));
    }

    // replace the call to ack with noops
    position = SeekFrom::Start(2 * ACK_CALL as u64);
    savestate.seek(position)?;
    let mut buf4 = [0u8; 4];
    savestate.read_exact(&mut buf4)?;
    // (call, 6027) is (0x11, 0x178b)
    if buf4 == [0x11, 0, 0x8b, 0x17] {
        println!("Writing 2 noops at position {}", display_position(position));
        savestate.seek(position)?;
        for _ in 1..=2 { savestate.write_all(&NOOP.to_le_bytes())?; };
    } else {
        println!("Skipping writing at position {}", display_position(position));
    }

    // set the eigth register to the correct teleporter value
    position = SeekFrom::Start(2 * REG_END as u64);
    savestate.seek(position)?;
    savestate.read_exact(&mut buf2)?;
    if u16::from_le_bytes(buf2) == 0 {
        println!("Writing {} at position {}", REG_8_VALUE, display_position(position));
        savestate.seek(position)?;
        savestate.write_all(&REG_8_VALUE.to_le_bytes())?;
    } else {
        println!("Skipping writing at position {}", display_position(position));
    }

    Ok(())
}

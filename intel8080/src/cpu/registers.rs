use log::{debug, warn};

use super::Cpu;

const NAMES: [&'static str; 6] = ["B", "C", "D", "E", "H", "L"];

impl Cpu {
    /// Stores a byte in a register.
    pub(crate) fn store_byte(&mut self, reg: usize, buf: u8) {
        if reg > 6 {
            warn!("Failed to store byte in register {reg}");
            return;
        }

        let pair = reg / 2;

        if reg % 2 == 0 {
            self.reg[pair].0 = buf;
        } else {
            self.reg[pair].1 = buf;
        }

        debug!("Byte {buf:02x} stored in register {}", NAMES[reg])
    }

    /// Stores a word in a register pair
    pub(crate) fn store_word(&mut self, reg: usize, buf: u16) {
        if reg > 3 {
            warn!("Failed to store word in register pair {reg}");
            return;
        }

        let lsb = (buf & 0xFF_00) >> 8;
        let msb = buf & 0x00_FF;

        self.reg[reg].0 = msb as u8;
        self.reg[reg].1 = lsb as u8;

        debug!("Word {buf:04x} stored in register {}", NAMES[reg * 2]);
    }
}

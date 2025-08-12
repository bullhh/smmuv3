

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub CR2 [
        /// Bits [31:12] Reserved, RES0.
        Reserved12 OFFSET(4) NUMBITS(27) [],
  
        VALID OFFSET(0) NUMBITS(4) [
            defaul = 0b0111,
        ],
    ]
}

/// CR1 register, read-write.
pub type Cr2Reg = ReadWrite<u32, CR2::Register>;

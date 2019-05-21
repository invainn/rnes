use crate::utils::{ get_high_byte, get_low_byte };

enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

enum StatusFlag {
    Carry = 0x01,
    Zero = 0x02,
    IRQDisable = 0x03,
    Decimal = 0x04,  // Useless
    Break = 0x05,
    Push = 0x06,
    Overflow = 0x07,
    Negative = 0x08,
}

struct CPU {
    pub memory: [u8; 0xFFFF],

    // Status register
    pub P: u8,
    // Acc Register
    pub A: u8,

    pub X: u8,
    pub Y: u8,
    pub S: u8,

    pub pc: usize,
    pub sp: usize,

    // Interrupt flags should be here somewhere
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: [0u8; 0xFFFF],

            P: 0,
            A: 0,
            X: 0,
            Y: 0,
            S: 0,

            pc: 0,
            sp: 0,
        }
    }

    // TODO:
    // Implement Opcodes
    // Create method to handle addressing modes
}

use crate::utils::{ get_high_byte, get_low_byte };

use std::{ thread, time };

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
    IRQDisable = 0x04,
    Decimal = 0x08,  // Useless
    Break = 0x10,
    Push = 0x20,
    Overflow = 0x40,
    Negative = 0x80,
}

pub struct CPU {
    pub memory: [u8; 0x10000],

    // Status register
    pub P: u8,
    // Acc Register
    pub A: u8,

    pub X: u8,
    pub Y: u8,
    pub S: usize,

    pub pc: usize,

    // Used for modes absolute etc.
    pub addr: u16,

    // Used for Zero Page and Immediate
    pub val: u8,

    pub cycle_counter: usize,

    // Interrupt flags should be here somewhere
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            memory: [0u8; 0x10000],

            P: 0,
            A: 0,
            X: 0,
            Y: 0,
            S: 0,

            pc: 0,

            addr: 0,
            val: 0,

            cycle_counter: 0,
        }
    }

    // TODO:
    // Implement Opcodes
    // Create method to handle addressing modes
   
    // Execute a CPU cycle
    pub fn emulate_cycle(&mut self) -> usize {
        // Pull an opcode from the address
        let opcode = self.memory[self.pc];
        self.addr = ((self.memory[self.pc + 2] as u16) << 8) | self.memory[self.pc + 1] as u16;
        self.val = self.memory[self.pc + 1];

        println!("{:X?}", self.pc);
        println!("{:X}", self.P);

        // Before executing the next instruction, we need to increment the pc
        // since the 6502 increments the pc before executing the next instruction

        match opcode {
            0x18 => {
                self.pc += 1;
                self.oc_clc();
            }
            0x20 => {
                self.increment_pc(AddressMode::Absolute);
                self.oc_jsr();
            },
            0x38 => {
                self.pc += 1;
                self.oc_sec();
            },
            0x4C => {
                self.increment_pc(AddressMode::Absolute);
                self.oc_jmp(AddressMode::Absolute);
            },
            0x85 => {
                self.increment_pc(AddressMode::ZeroPage);
                self.oc_sta(AddressMode::ZeroPage);
            }
            0x90 => {
                self.increment_pc(AddressMode::Relative);
                self.oc_bcc();
            }
            0x86 => {
                self.increment_pc(AddressMode::ZeroPage);
                self.oc_stx(AddressMode::ZeroPage);
            },
            0xA2 => {
                self.increment_pc(AddressMode::Immediate);
                self.oc_ldx(AddressMode::Immediate);
            },
            0xA9 => {
                self.increment_pc(AddressMode::Immediate);
                self.oc_lda(AddressMode::Immediate);
            },
            0xB0 => {
                self.increment_pc(AddressMode::Relative);
                self.oc_bcs();
            }
            0xD0 => {
                self.increment_pc(AddressMode::Relative);
                self.oc_bne();
            }
            0xEA => {
                self.pc += 1;
                self.oc_nop();
            },
            0xF0 => {
                self.increment_pc(AddressMode::Relative);
                self.oc_beq();
            }
            _    => panic!("Opcode {:X?} not implemented!", opcode)
        }


        self.cycle_counter
    }

    fn increment_pc(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Immediate => self.pc += 2,
            AddressMode::ZeroPage => self.pc += 2,
            AddressMode::ZeroPageX => self.pc += 2,
            AddressMode::ZeroPageY => self.pc += 2,
            AddressMode::Relative => self.pc += 2,
            AddressMode::Absolute => self.pc += 3,
            AddressMode::AbsoluteX => self.pc += 2,
            AddressMode::AbsoluteY => self.pc += 2,
            AddressMode::Indirect => self.pc += 2,
            AddressMode::IndexedIndirect => self.pc += 2,
            AddressMode::IndirectIndexed => self.pc += 2,
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        if rom.len() == 16384 {
            for (i, &byte) in rom.iter().enumerate() {
                self.memory[i + 0xC000] = byte;
            }
        } else {
            for (i, &byte) in rom.iter().enumerate() {
                self.memory[i + 0x8000] = byte;
            }

            for (i, &byte) in rom.iter().enumerate() {
                self.memory[i + 0xC000] = byte;
            }
        }

        self.pc = 0xC000;
        self.P = 0x24;
        self.S = 0xFD;
    }

    fn oc_bit(&mut self) {
        
    }

    fn oc_bcc(&mut self) {
        if (self.P & StatusFlag::Carry as u8) == 0x00 {
            let offset = self.val as i8 as usize;

            self.pc = self.pc + offset;
        }
    }

    fn oc_bcs(&mut self) {
        if (self.P & StatusFlag::Carry as u8) == StatusFlag::Carry as u8 {
            let offset = self.val as i8 as usize;

            self.pc = self.pc + offset;
        }
    }

    fn oc_beq(&mut self) {
        if (self.P & StatusFlag::Zero as u8) == StatusFlag::Zero as u8 {
            let offset = self.val as i8 as usize;

            self.pc = self.pc + offset;
        }
    }

    fn oc_bne(&mut self) {
        if (self.P & StatusFlag::Zero as u8) == 0x00 {
            let offset = self.val as i8 as usize;

            self.pc = self.pc + offset;
        }
    }
    

    fn oc_clc(&mut self) {
        self.P &= !(StatusFlag::Carry as u8);
    }

    fn set_zero_negative_flags(&mut self) {
        if self.val == 0 {
            self.P |= StatusFlag::Zero as u8;
        } else {
            self.P &= !(StatusFlag::Zero as u8);
        }

        if (self.val & 0x80) == 0x80 {
            self.P |= StatusFlag::Negative as u8;
        } else {
            self.P &= !(StatusFlag::Negative as u8);
        }
    }

    fn oc_lda(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Immediate => {
                self.set_zero_negative_flags();
                self.A = self.val;
            },
            _                      => panic!("Mode not covered for LDX")
        }
    }

    fn oc_ldx(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Immediate => {
                self.set_zero_negative_flags();
                self.X = self.val;
            },
            _                      => panic!("Mode not covered for LDX")
        }
    }

    fn oc_jmp(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Absolute => {
                self.pc = self.addr as usize;
            },
            _                     => panic!("Mode not covered for JMP")
        }
    }

    // Mode: Absolute
    fn oc_jsr(&mut self) {
        // Since PC is 16 bytes, we'll need to push each one individually
        let ret_addr = self.pc - 1;
        let pc_high = get_high_byte(ret_addr as u16);
        let pc_low = get_low_byte(ret_addr as u16);

        self.memory[self.S] = pc_high;
        self.S.wrapping_sub(1);

        self.memory[self.S] = pc_low;
        self.S.wrapping_sub(1);

        self.pc = self.addr as usize;
    }

    // Does nothing
    fn oc_nop(&mut self) {}

    // mode: Implied
    fn oc_sec(&mut self) {
        self.P |= StatusFlag::Carry as u8;
    }

    fn oc_sta(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::ZeroPage => {
                self.memory[self.val as usize] = self.A;
            },
            _                      => panic!("Mode not covered for STX")
        }
    }

    fn oc_stx(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::ZeroPage => {
                self.memory[self.val as usize] = self.X;
            },
            _                      => panic!("Mode not covered for STX")
        }
    }
}




























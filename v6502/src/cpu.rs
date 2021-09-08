/*
    Copyright 2021, Andrew C. Young <andrew@vaelen.org>

    This file is part of the v6502 library.

    The v6502 library is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Foobar is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with the v6502 library.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt;

use crate::addressing::Addressing;
use crate::addressing::Addressing::*;
use crate::device::Device;
use crate::device::Rand;
use crate::device::Terminal;
use crate::instruction::Instruction;
use crate::instruction::InstructionType;
use crate::instruction::InstructionType::*;
use crate::opcodes::*;
use crate::memory::Memory;

const MEMORY_SIZE: usize = 0x10000;
const IRQ_VECTOR: u16 = 0xFFFE;
const RESET_VECTOR: u16 = 0xFFFC;

const BRK: Instruction = Instruction {t: Brk, a: Implied };

pub struct Cpu {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8,
    pub memory: [u8;MEMORY_SIZE],
    pub opcodes: [Instruction; 256],
    pub rand: Rand,
    pub terminal: Terminal,
}

impl Memory for Cpu {
    fn get(&mut self, addr: u16) -> u8 {
        match addr {
            0x00FDu16 => self.terminal.get(0),
            0x00FEu16 => self.terminal.get(1),
            0x00FFu16 => self.rand.get(0),
            _ => self.memory[addr as usize],
        }
    }

    fn set(&mut self, addr: u16, v: u8) {
        match addr {
            0x00FDu16 => self.terminal.set(0, v),
            0x00FEu16 => self.terminal.set(1, v),
            0x00FFu16 => self.rand.set(0, v),
            _ => self.memory[addr as usize] = v,
        }
        
    }
}

impl Cpu {
    pub fn new6502() -> Cpu {
        Cpu::new(&OPCODES_6502)
    }

    pub fn new(opcodes: &[(u8, InstructionType, Addressing)]) -> Cpu {
        let mut cpu = Cpu {
            pc: RESET_VECTOR,
            a: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0xFF,
            memory: [0; MEMORY_SIZE],
            opcodes: [BRK; 256],
            rand: Rand::new(),
            terminal: Terminal::new(),
        };
        cpu.load_opcodes(opcodes);
        cpu.reset();
        return cpu;
    }

    pub fn load_opcodes(&mut self, opcodes: &[(u8, InstructionType, Addressing)]) {
        for entry in opcodes {
            let instruction = Instruction {t: entry.1, a: entry.2 };
            self.opcodes[entry.0 as usize] = instruction;
        }
    }

    pub fn clear_memory(&mut self) {
        for i in 0..MEMORY_SIZE {
            self.memory[i] = 0;
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sr = 0;
        self.sp = 0xFF;
        self.jump(Indirect(RESET_VECTOR));
    }

    pub fn get_status_bit(&self, bit: i8) -> bool {
        ((self.sr >> bit) & 1u8) == 1
    }

    pub fn set_status_bit(&mut self, bit: i8) {
        self.sr |= 1u8 << bit;
    }

    pub fn clear_status_bit(&mut self, bit: i8) {
        self.sr &= !(1u8 << bit);
    }

    pub fn is_negative(&self) -> bool {
        self.get_status_bit(7)
    }

    pub fn set_negative(&mut self) {
        self.set_status_bit(7);
    }

    pub fn clear_negative(&mut self) {
        self.clear_status_bit(7);
    }

    pub fn is_overflow(&self) -> bool {
        self.get_status_bit(6)
    }

    pub fn set_overflow(&mut self) {
        self.set_status_bit(6);
    }

    pub fn clear_overflow(&mut self) {
        self.clear_status_bit(6);
    }

    pub fn is_ignored(&self) -> bool {
        self.get_status_bit(5)
    }

    pub fn set_ignored(&mut self) {
        self.set_status_bit(5);
    }

    pub fn clear_ignored(&mut self) {
        self.clear_status_bit(5);
    }

    pub fn is_break(&self) -> bool {
        self.get_status_bit(4)
    }

    pub fn set_break(&mut self) {
        self.set_status_bit(4);
    }

    pub fn clear_break(&mut self) {
        self.clear_status_bit(4);
    }

    pub fn is_decimal(&self) -> bool {
        self.get_status_bit(3)
    }

    pub fn set_decimal(&mut self) {
        self.set_status_bit(3);
    }

    pub fn clear_decimal(&mut self) {
        self.clear_status_bit(3);
    }

    pub fn is_irq_disabled(&self) -> bool {
        self.get_status_bit(2)
    }

    pub fn set_irq_disabled(&mut self) {
        self.set_status_bit(2);
    }

    pub fn clear_irq_disabled(&mut self) {
        self.clear_status_bit(2);
    }

    pub fn is_zero(&self) -> bool {
        self.get_status_bit(1)
    }

    pub fn set_zero(&mut self) {
        self.set_status_bit(1);
    }

    pub fn clear_zero(&mut self) {
        self.clear_status_bit(1);
    }

    pub fn is_carry(&self) -> bool {
        self.get_status_bit(0)
    }

    pub fn set_carry(&mut self) {
        self.set_status_bit(0);
    }

    pub fn clear_carry(&mut self) {
        self.clear_status_bit(0);
    }

    pub fn zero_page(&self) -> &[u8] {
        &self.memory[0x0000..0x0100]
    }

    pub fn stack(&self) -> &[u8] {
        &self.memory[0x0100..0x0200]
    }

    pub fn push(&mut self, v: u8) {
        self.set(0x0100 + self.sp as u16, v);
        self.sp = self.sp.overflowing_sub(1).0;
    }

    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.overflowing_add(1).0;
        let v = self.get(0x0100 + self.sp as u16);
        return v;
    }

    pub fn update_nz(&mut self, v: u8) {
        if v == 0 {
            self.set_zero();
        } else {
            self.clear_zero();
        }
        if ((v & 0x80) >> 7) == 1 {
            self.set_negative();
        } else {
            self.clear_negative();
        }
    }

    pub fn jump(&mut self, a: Addressing) {
        if let Some(addr) = a.address(self) {
            self.pc = addr;
        }
    }

    pub fn next_byte(&mut self) -> u8 {
        let byte = self.get(self.pc);
        self.pc = self.pc.overflowing_add(1).0;
        return byte;
    }

    pub fn next_u16(&mut self) -> u16 {
        let lb = self.next_byte() as u16;
        let hb = self.next_byte() as u16;
        (hb << 8) | lb
    }

    pub fn next_instruction(&mut self) -> Instruction{
        let i = &self.opcodes[self.next_byte() as usize];
        Instruction {
            t: i.t,
            a: match i.a {
                Accumulator => Accumulator,
                Absolute(_) => Absolute(self.next_u16()),
                AbsoluteX(_) => AbsoluteX(self.next_u16()),
                AbsoluteY(_) => AbsoluteY(self.next_u16()),
                Immediate(_) => Immediate(self.next_byte()),
                Implied => Implied,
                Indirect(_) => Indirect(self.next_u16()),
                IndirectX(_) => IndirectX(self.next_byte()),
                IndirectY(_) => IndirectY(self.next_byte()),
                Relative(_) => Relative(self.next_byte() as i8),
                ZeroPage(_) => ZeroPage(self.next_byte()),
                ZeroPageX(_) => ZeroPageX(self.next_byte()),
                ZeroPageY(_) => ZeroPageY(self.next_byte()),
            }
        }
    }

    pub fn execute_next_instruction(&mut self) {
        let i = self.next_instruction();
        self.execute(i);
    }

    pub fn execute(&mut self, i: Instruction) {
        let a = i.a;
        match i.t {
            Adc => {
                // Add with carry
                // A + M + C -> A, C
                let value = a.get(self);
                let acc = self.a;
                let carry_in = self.is_carry();
                let (mut result, mut carry) = acc.overflowing_add(value);
                if carry_in {
                    let (result2, carry2) = result.overflowing_add(1);
                    result = result2;
                    carry |= carry2;
                }
                self.a = result;
                if carry {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                if ((acc ^ result) & (value ^ result) & 0x80) == 0 {
                    self.clear_overflow();
                } else {
                    self.set_overflow();
                }
                self.update_nz(result);
            },
            And => {
                self.a = self.a & a.get(self);
                self.update_nz(self.a);
            },
            Asl => {
                let value = a.get(self);
                let first_bit = value & 0x80 == 0x80;
                let new_value = value << 1;
                a.set(self, new_value);
                if first_bit {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Bcc => {
                if !self.is_carry() {
                    self.jump(a);
                }
            },
            Bcs => {
                if self.is_carry() {
                    self.jump(a);
                }
            },
            Beq => {
                if self.is_zero() {
                    self.jump(a);
                }
            },
            Bit => {
                // Test Bits in Memory with Accumulator
                // bits 7 and 6 of operand are transfered to bit 7 and 6 of SR (N,V);
                // the zero-flag is set to the result of operand AND accumulator.

                // Note: Loading a bit mask in the accumulator before executing
                //       this instruction will result in the Z flag being set
                //       if the mask matched. No mask is needed if one wants
                //       to check the 7th or 6th bit as those are loaded
                //       directly into the 7th and 6th bit of SR.

                let value = a.get(self);
                
                if value & 0x80 != 0 {
                    self.set_negative();
                } else {
                    self.clear_negative();
                }
                
                if value & 0x40 != 0 {
                    self.set_overflow();
                } else {
                    self.clear_overflow();
                }

                if value & self.a == 0 {
                    self.set_zero();
                } else {
                    self.clear_zero();
                }
            },
            Bmi => {
                if self.is_negative() {
                    self.jump(a);
                }
            },
            Bne => {
                if !self.is_zero() {
                    self.jump(a);
                }
            },
            Bpl => {
                if !self.is_negative() {
                    self.jump(a);
                }
            },
            Brk => {
                // Break (Force Interrupt)

                // BRK initiates a software interrupt similar to a hardware
                // interrupt (IRQ). The return address pushed to the stack is
                // PC+2, providing an extra byte of spacing for a break mark
                // (identifying a reason for the break.)
                // The status register will be pushed to the stack with the break
                // flag set to 1. However, when retrieved during RTI or by a PLP
                // instruction, the break flag will be ignored.
                // The interrupt disable flag is not set automatically.

                // NOTE: Because of how the emulator works, we actually push
                //       PC -1 to maintain compatibility.

                let pc = self.pc.overflowing_sub(1).0;
                let pch: u8 = (pc >> 8) as u8;
                let pcl: u8 = pc as u8;
                self.push(pch);
                self.push(pcl);
                self.set_break();
                self.push(self.sr);
                //self.set_irq_disabled();
                self.jump(Indirect(IRQ_VECTOR));
            },
            Bvc => {
                if !self.is_overflow() {
                    self.jump(a);
                }
            },
            Bvs => {
                if self.is_overflow() {
                    self.jump(a);
                }
            },
            Clc => self.clear_carry(),
            Cld => self.clear_decimal(),
            Cli => self.clear_irq_disabled(),
            Clv => self.clear_overflow(),
            Cmp => {
                // Compare Memory with Accumulator
                // A - M
                // Note: Only changes SR
                let value = a.get(self);
                let (new_value, carry) = self.a.overflowing_sub(value);
                if !carry {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Cpx => {
                // Compare Memory with X
                // X - M
                // Note: Only changes SR
                let value = a.get(self);
                let (new_value, carry) = self.x.overflowing_sub(value);
                if !carry {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Cpy => {
                // Compare Memory with Y
                // Y - M
                // Note: Only changes SR
                let value = a.get(self);
                let (new_value, carry) = self.y.overflowing_sub(value);
                if !carry {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Dec => {
                let value = a.get(self);
                let new_value = value.overflowing_sub(1).0;
                a.set(self, new_value);
                self.update_nz(new_value);
            },
            Dex => {
                let value = self.x;
                let new_value = value.overflowing_sub(1).0;
                self.x = new_value;
                self.update_nz(new_value);
            },
            Dey => {
                let value = self.y;
                let new_value = value.overflowing_sub(1).0;
                self.y = new_value;
                self.update_nz(new_value);
            },
            Eor => {
                self.a = self.a ^ a.get(self);
                self.update_nz(self.a);
            },
            Inc => {
                let value = a.get(self);
                let new_value = value.overflowing_add(1).0;
                a.set(self, new_value);
                self.update_nz(new_value);
            },
            Inx => {
                let value = self.x;
                let new_value = value.overflowing_add(1).0;
                self.x = new_value;
                self.update_nz(new_value);
            },
            Iny => {
                let value = self.y;
                let new_value = value.overflowing_add(1).0;
                self.y = new_value;
                self.update_nz(new_value);
            },
            Jmp => self.jump(a),
            Jsr => {
                // Jump to Subroutine
                // Pushes PC + 2 onto the stack then jumps to the given address.

                // The address on the stack will point to the 3rd byte of the
                // instruction, meaning that extra byte will need to be thrown
                // away by the RTS instruction.

                // NOTE: Because of how the emulator works, we actually push
                //       PC -1 to maintain compatibility.

                let pc = self.pc.overflowing_sub(1).0;
                let pch: u8 = (pc >> 8) as u8;
                let pcl: u8 = pc as u8;
                self.push(pch);
                self.push(pcl);
                self.jump(a);
            },
            Lda => self.a = a.get(self),
            Ldx => self.x = a.get(self),
            Ldy => self.y = a.get(self),
            Lsr => { 
                let value = a.get(self);
                let last_bit = value & 0x01 == 0x01;
                let new_value = value >> 1;
                a.set(self, new_value);
                if last_bit {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Nop => { /* NOP */ },
            Ora => {
                self.a = self.a | a.get(self);
                self.update_nz(self.a);
            },
            Pha => self.push(self.a),
            Php => {
                // Push Processor Status on Stack
                // The status register will be pushed with the break
                // flag and bit 5 set to 1.
                let sr = self.sr;
                self.set_break();
                self.set_ignored();
                self.push(self.sr);
                self.sr = sr;
            },
            Pla => {
                self.a = self.pop();
                self.update_nz(self.a);
            },
            Plp => {
                // Pull Processor Status from Stack
                // The status register will be pulled with the break
                // flag and bit 5 ignored.
                let ignored = self.is_ignored();
                let brk = self.is_break();
                self.sr = self.pop();
                if ignored {
                    self.set_ignored();
                } else {
                    self.clear_ignored();
                }
                if brk {
                    self.set_break();
                } else {
                    self.clear_break();
                }
            },
            Rol => {
                // Rotate One Bit Left
                // C <- [76543210] <- C
                let value = a.get(self);
                let first_bit = value & 0x80 == 0x80;
                let mut new_value = value << 1;
                if self.is_carry() {
                    new_value = new_value | 0x01;
                }
                a.set(self, new_value);
                if first_bit {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Ror => {
                // Rotate One Bit Right
                // C -> [76543210] -> C
                let value = a.get(self);
                let last_bit = value & 0x01 == 0x01;
                let mut new_value = value >> 1;
                if self.is_carry() {
                    new_value = new_value | 0x80;
                }
                a.set(self, new_value);
                if last_bit {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                self.update_nz(new_value);
            },
            Rti => {
                // RTI - Return from Interrupt
                // The status register is pulled with the break flag
                // and bit 5 ignored. Then PC is pulled from the stack.
                self.sr = self.pop();
                let pcl = self.pop();
                let pch = self.pop();
                let pc:u16 = ((pch as u16) << 8) | (pcl as u16);
                self.pc = pc.overflowing_add(1).0;
            },
            Rts => {
                // Return from Subroutine
                // Pops PC off the stack and increments it by 1

                // The address on the stack will point to the 3rd byte of the
                // JSR instruction, meaning that extra byte will need to be thrown
                // away by the RTS instruction.

                let pcl = self.pop();
                let pch = self.pop();
                let pc:u16 = ((pch as u16) << 8) | (pcl as u16);
                self.pc = pc.overflowing_add(1).0;
            },
            Sbc => {
                // Subtract with Borrow
                // A - M - !C -> A
                let value = a.get(self);
                let acc = self.a;
                let carry_in = self.is_carry();
                let (mut result, mut carry) = acc.overflowing_sub(value);
                if !carry_in {
                    let (result2, carry2) = result.overflowing_sub(1);
                    result = result2;
                    carry |= carry2;
                }
                self.a = result;
                if !carry {
                    self.set_carry();
                } else {
                    self.clear_carry();
                }
                if ((acc ^ result) & ((255-value) ^ result) & 0x80) == 0 {
                    self.clear_overflow();
                } else {
                    self.set_overflow();
                }
                self.update_nz(self.a);
            },
            Sec => self.set_carry(),
            Sed => self.set_decimal(),
            Sei => self.set_irq_disabled(),
            Sta => a.set(self, self.a),
            Stx => a.set(self, self.x),
            Sty => a.set(self, self.y),
            Tax => {
                self.x = self.a;
                self.update_nz(self.x);
            },
            Tay => {
                self.y = self.a;
                self.update_nz(self.y);
            },
            Tsx => {
                self.x = self.sp;
                self.update_nz(self.x);
            },
            Txa => {
                self.a = self.x;
                self.update_nz(self.a);
            },
            Txs => self.sp = self.x,
            Tya => {
                self.a = self.y;
                self.update_nz(self.a);
            },
        }
    }

    pub fn run(&mut self) {
        while !self.is_break() {
            self.execute_next_instruction();
        }
    }

    fn fmt_memory(data: &[u8], offset: u16, fmt: &mut fmt::Formatter) -> fmt::Result {
        const CHUNK_SIZE: usize = 16;
        let mut row = offset;
        fmt.write_str("       ")?;
        for i in 0..CHUNK_SIZE {
            fmt.write_fmt(format_args!("{:2X} ", i))?;
        }
        fmt.write_str("\n")?;
        for chunk in data.chunks(CHUNK_SIZE) {
            fmt.write_fmt(format_args!("{:04X} : ", row))?;
            for b in chunk {
                fmt.write_fmt(format_args!("{:02X} ", b))?;
            }
            fmt.write_str("\n")?;
            if row < (0xFFFF as usize - CHUNK_SIZE) as u16 {
                row += CHUNK_SIZE as u16;
            }
        }
        Ok(())
    }

    fn registers(&self) -> String {
        format!("A: {:02X} X: {:02X} Y: {:02X} PC: {:04X} SR: {:02X} SP: {:02X}",
            self.a, self.x, self.y, self.pc, self.sr, self.sp)
    } 

}

impl fmt::Debug for Cpu {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("Registers: \n    {}\n", self.registers()))?;
        fmt.write_str("\nZero Page:\n")?;
        Cpu::fmt_memory(self.zero_page(), 0x0000, fmt)?;
        fmt.write_str("\nStack:\n")?;
        Cpu::fmt_memory(self.stack(), 0x0100, fmt)
    }
}

impl fmt::UpperHex for Cpu {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Cpu::fmt_memory(&self.memory[..], 0x0000, fmt)
    }
}
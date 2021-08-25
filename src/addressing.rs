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

use crate::cpu::Cpu;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Addressing {
    Accumulator,
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Immediate(u8),
    Implied,
    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
    Relative(i8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
}

impl Addressing {
    pub fn address(&self, cpu: &Cpu) -> Option<u16> {
        let indirect = |addr: u16| -> u16 {
            let low_byte = cpu.get(addr) as u16;
            let high_byte = cpu.get(addr.overflowing_add(1).0) as u16;
            (high_byte << 8) | low_byte
        };
        match self {
            Addressing::Accumulator => None,
            Addressing::Absolute(addr) => Some(*addr),
            Addressing::AbsoluteX(addr) => Some(addr.overflowing_add(cpu.x as u16).0),
            Addressing::AbsoluteY(addr) => Some(addr.overflowing_add(cpu.y as u16).0),
            Addressing::Immediate(_) => None,
            Addressing::Implied => None,
            Addressing::Indirect(addr) => Some(indirect(*addr)),
            Addressing::IndirectX(addr) => Some(indirect((*addr).overflowing_add(cpu.x).0 as u16)),
            Addressing::IndirectY(addr) => Some(indirect(*addr as u16).overflowing_add(cpu.x as u16).0),
            Addressing::Relative(offset) => {
                let o = *offset;
                if o == 0 {
                    Some(cpu.pc)
                } else if o > 0 {
                    Some(cpu.pc.overflowing_add(o as u16).0)
                } else {
                    Some(cpu.pc.overflowing_sub((-o) as u16).0)
                }
            },
            Addressing::ZeroPage(addr) => Some(*addr as u16),
            Addressing::ZeroPageX(addr) => Some((*addr).overflowing_add(cpu.x).0 as u16),
            Addressing::ZeroPageY(addr) => Some((*addr).overflowing_add(cpu.y).0 as u16),
        }
    }

    pub fn get(&self, cpu: &mut Cpu) -> u8 {
        match self {
            Addressing::Accumulator => cpu.a,
            Addressing::Immediate(v) => *v,
            _ => {
                if let Some(addr) = self.address(cpu) {
                    cpu.get(addr)
                } else {
                    panic!("Invalid addressing mode: {:?}", self);
                }
            }
        }
    }

    pub fn set(&self, cpu: &mut Cpu, value: u8) {
        match self {
            Addressing::Accumulator => cpu.a = value,
            _ => {
                if let Some(addr) = self.address(cpu) {
                    cpu.set(addr, value);
                } else {
                    panic!("Invalid addressing mode: {:?}", self);
                }
            }
        }
    }
}
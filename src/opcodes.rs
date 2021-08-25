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

use crate::instruction::InstructionType;
use crate::instruction::InstructionType::*;
use crate::addressing::Addressing;
use crate::addressing::Addressing::*;

/** Published 6502 Opcodes */
pub const OPCODES_6502: [(u8, InstructionType, Addressing); 151] = [
    (0x00, Brk, Implied),
    (0x01, Ora, IndirectX(0)),
    (0x05, Ora, ZeroPage(0)),
    (0x06, Asl, ZeroPage(0)),
    (0x08, Php, Implied),
    (0x09, Ora, Immediate(0)),
    (0x0A, Asl, Accumulator),
    (0x0D, Ora, Absolute(0)),
    (0x0E, Asl, Absolute(0)),

    (0x10, Bpl, Relative(0)),
    (0x11, Ora, IndirectY(0)),
    (0x15, Ora, ZeroPageX(0)),
    (0x16, Asl, ZeroPageX(0)),
    (0x18, Clc, Implied),
    (0x19, Ora, AbsoluteY(0)),
    (0x1d, Ora, AbsoluteX(0)),
    (0x1e, Asl, AbsoluteX(0)),

    (0x20, Jsr, Absolute(0)),
    (0x21, And, IndirectX(0)),
    (0x24, Bit, ZeroPage(0)),
    (0x25, And, ZeroPage(0)),
    (0x26, Rol, ZeroPage(0)),
    (0x28, Plp, Implied),
    (0x29, And, Immediate(0)),
    (0x2a, Rol, Accumulator),
    (0x2c, Bit, Absolute(0)),
    (0x2d, And, Absolute(0)),
    (0x2e, Rol, Absolute(0)),

    (0x30, Bmi, Relative(0)),
    (0x31, And, IndirectY(0)),
    (0x35, And, ZeroPageX(0)),
    (0x36, Rol, ZeroPageX(0)),
    (0x38, Sec, Implied),
    (0x39, And, AbsoluteY(0)),
    (0x3d, And, AbsoluteX(0)),
    (0x3e, Rol, AbsoluteX(0)),

    (0x40, Rti, Implied),
    (0x41, Eor, IndirectX(0)),
    (0x45, Eor, ZeroPage(0)),
    (0x46, Lsr, ZeroPage(0)),
    (0x48, Pha, Implied),
    (0x49, Eor, Immediate(0)),
    (0x4a, Lsr, Accumulator),
    (0x4c, Jmp, Absolute(0)),
    (0x4d, Eor, Absolute(0)),
    (0x4e, Lsr, Absolute(0)),

    (0x50, Bvc, Relative(0)),
    (0x51, Eor, IndirectY(0)),
    (0x55, Eor, ZeroPageX(0)),
    (0x56, Lsr, ZeroPageX(0)),
    (0x58, Cli, Implied),
    (0x59, Eor, AbsoluteY(0)),
    (0x5d, Eor, AbsoluteX(0)),
    (0x5e, Lsr, AbsoluteX(0)),

    (0x60, Rts, Implied),
    (0x61, Adc, IndirectX(0)),
    (0x65, Adc, ZeroPage(0)),
    (0x66, Ror, ZeroPage(0)),
    (0x68, Pla, Implied),
    (0x69, Adc, Immediate(0)),
    (0x6a, Ror, Accumulator),
    (0x6c, Jmp, Indirect(0)),
    (0x6d, Adc, Absolute(0)),
    (0x6e, Ror, Absolute(0)),

    (0x70, Bvs, Relative(0)),
    (0x71, Adc, IndirectY(0)),
    (0x75, Adc, ZeroPageX(0)),
    (0x76, Ror, ZeroPageX(0)),
    (0x78, Sei, Implied),
    (0x79, Adc, AbsoluteY(0)),
    (0x7d, Adc, AbsoluteX(0)),
    (0x7e, Ror, AbsoluteX(0)),

    (0x81, Sta, IndirectX(0)),
    (0x84, Sty, ZeroPage(0)),
    (0x85, Sta, ZeroPage(0)),
    (0x86, Stx, ZeroPage(0)),
    (0x88, Dey, Implied),
    (0x8a, Txa, Implied),
    (0x8c, Sty, Absolute(0)),
    (0x8d, Sta, Absolute(0)),
    (0x8e, Stx, Absolute(0)),

    (0x90, Bcc, Relative(0)),
    (0x91, Sta, IndirectY(0)),
    (0x94, Sty, ZeroPageX(0)),
    (0x95, Sta, ZeroPageX(0)),
    (0x96, Stx, ZeroPageY(0)),
    (0x98, Tya, Implied),
    (0x99, Sta, AbsoluteY(0)),
    (0x9a, Txs, Implied),
    (0x9d, Sta, AbsoluteX(0)),

    (0xA0, Ldy, Immediate(0)),
    (0xA1, Lda, IndirectX(0)),
    (0xA2, Ldx, Immediate(0)),
    (0xA4, Ldy, ZeroPage(0)),
    (0xA5, Lda, ZeroPage(0)),
    (0xA6, Ldx, ZeroPage(0)),
    (0xA8, Tay, Implied),
    (0xA9, Lda, Immediate(0)),
    (0xAA, Tax, Implied),
    (0xAC, Ldy, Absolute(0)),
    (0xAD, Lda, Absolute(0)),
    (0xAE, Ldx, Absolute(0)),

    (0xB0, Bcs, Relative(0)),
    (0xB1, Lda, IndirectY(0)),
    (0xB4, Ldy, ZeroPageX(0)),
    (0xB5, Lda, ZeroPageX(0)),
    (0xB6, Ldx, ZeroPageY(0)),
    (0xB8, Clv, Implied),
    (0xB9, Lda, AbsoluteY(0)),
    (0xBA, Tsx, Implied),
    (0xBC, Ldy, AbsoluteX(0)),
    (0xBD, Lda, AbsoluteX(0)),
    (0xBE, Ldx, AbsoluteY(0)),

    (0xC0, Cpy, Immediate(0)),
    (0xC1, Cmp, IndirectX(0)),
    (0xC4, Cpy, ZeroPage(0)),
    (0xC5, Cmp, ZeroPage(0)),
    (0xC6, Dec, ZeroPage(0)),
    (0xC8, Iny, Implied),
    (0xC9, Cmp, Immediate(0)),
    (0xCA, Dex, Implied),
    (0xCC, Cpy, Absolute(0)),
    (0xCD, Cmp, Absolute(0)),
    (0xCE, Dec, Absolute(0)),

    (0xD0, Bne, Relative(0)),
    (0xD1, Cmp, IndirectY(0)),
    (0xD5, Cmp, ZeroPageX(0)),
    (0xD6, Dec, ZeroPageX(0)),
    (0xD8, Cld, Implied),
    (0xD9, Cmp, AbsoluteY(0)),
    (0xDD, Cmp, AbsoluteX(0)),
    (0xDE, Dec, AbsoluteX(0)),

    (0xE0, Cpx, Immediate(0)),
    (0xE1, Sbc, IndirectX(0)),
    (0xE4, Cpx, ZeroPage(0)),
    (0xE5, Sbc, ZeroPage(0)),
    (0xE6, Inc, ZeroPage(0)),
    (0xE8, Inx, Implied),
    (0xE9, Sbc, Immediate(0)),
    (0xEA, Nop, Implied),
    (0xEC, Cpx, Absolute(0)),
    (0xED, Sbc, Absolute(0)),
    (0xEE, Inc, Absolute(0)),

    (0xF0, Beq, Relative(0)),
    (0xF1, Sbc, IndirectY(0)),
    (0xF5, Sbc, ZeroPageX(0)),
    (0xF6, Inc, ZeroPageX(0)),
    (0xF8, Sed, Implied),
    (0xF9, Sbc, AbsoluteY(0)),
    (0xFD, Sbc, AbsoluteX(0)),
    (0xFE, Inc, AbsoluteX(0)),
];

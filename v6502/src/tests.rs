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
use crate::addressing::Addressing::*;
use crate::instruction::Instruction;
use crate::instruction::InstructionType::*;
use crate::memory::Memory;

#[test]
fn status_bits() {
    let mut cpu = Cpu::new6502();
    assert!(!cpu.get_status_bit(0), "status bit not empty after initialization");
    cpu.set_status_bit(0);
    assert!(cpu.get_status_bit(0), "status bit not set after set_status_bit(0)");
    assert_eq!(cpu.sr, 0x01, "status register not set correctly for bit 0");
    cpu.clear_status_bit(0);
    assert!(!cpu.get_status_bit(0), "status bit not cleared after clear_status_bit(0)");
    assert_eq!(cpu.sr, 0x00, "status register not cleared correctly for bit 0");
    
    cpu.sr = 0xFF;
    assert!(cpu.get_status_bit(7), "status bit not set after setting sr to 0xFF");
    cpu.clear_status_bit(7);
    assert!(!cpu.get_status_bit(7), "status bit not cleared after clear_status_bit(7)");
    assert_eq!(cpu.sr, 0x7F, "status register not cleared correctly for bit 7");
    cpu.set_status_bit(7);
    assert!(cpu.get_status_bit(7), "status bit not set after set_status_bit(7)");
    assert_eq!(cpu.sr, 0xFF, "status register not set correctly for bit 7");

    cpu.sr = 0x00;
    assert!(!cpu.is_carry(), "carry bit not empty after initialization");
    cpu.set_carry();
    assert!(cpu.is_carry(), "carry bit not set after set_carry()");
    assert_eq!(cpu.sr, 0x01, "status register not set correctly for carry");
    cpu.clear_carry();
    assert!(!cpu.is_carry(), "status bit not cleared after clear_status_bit(0)");
    assert_eq!(cpu.sr, 0x00, "status register not cleared correctly for carry");
}

#[test]
fn addressing() {
    let mut cpu = Cpu::new6502();
    cpu.x = 1;
    cpu.y = 2;
    cpu.set(0x8080, 0xFE);
    cpu.set(0x8081, 0x11);
    cpu.set(0x8082, 0x22);
    cpu.set(0x8083, 0x33);
    cpu.set(0x80, 0x44);
    cpu.set(0x81, 0x45);
    cpu.set(0x82, 0x46);

    assert_eq!(Accumulator.address(&mut cpu), None);
    assert_eq!(Absolute(0x8080).address(&mut cpu), Some(0x8080));
    assert_eq!(AbsoluteX(0x8080).address(&mut cpu), Some(0x8081));
    assert_eq!(AbsoluteY(0x8080).address(&mut cpu), Some(0x8082));
    assert_eq!(Immediate(0x80).address(&mut cpu), None);
    assert_eq!(Implied.address(&mut cpu), None);
    assert_eq!(Indirect(0x8080).address(&mut cpu), Some(0x11FE));
    assert_eq!(IndirectX(0x80).address(&mut cpu), Some(0x4645));
    assert_eq!(IndirectY(0x80).address(&mut cpu), Some(0x4545));
    assert_eq!(Relative(16).address(&mut cpu), Some(0x0010));
    assert_eq!(Relative(-16).address(&mut cpu), Some(0xFFF0));
    assert_eq!(ZeroPage(0x80).address(&mut cpu), Some(0x0080));
    assert_eq!(ZeroPageX(0x80).address(&mut cpu), Some(0x0081));
    assert_eq!(ZeroPageY(0x80).address(&mut cpu), Some(0x0082));
}

#[test]
fn execution() {
    let mut cpu = Cpu::new6502();

    // Status Register Operations

    cpu.execute(Instruction{t: Sec, a: Implied});
    assert!(cpu.is_carry(), "SEC");
    cpu.execute(Instruction{t: Sed, a: Implied});
    assert!(cpu.is_decimal(), "SED");
    cpu.execute(Instruction{t: Sei, a: Implied});
    assert!(cpu.is_irq_disabled(), "SEI");

    cpu.execute(Instruction{t: Clc, a: Implied});
    assert!(!cpu.is_carry(), "CLC");
    cpu.execute(Instruction{t: Cld, a: Implied});
    assert!(!cpu.is_decimal(), "CLD");
    cpu.execute(Instruction{t: Cli, a: Implied});
    assert!(!cpu.is_irq_disabled(), "CLI");
    cpu.set_overflow();
    cpu.execute(Instruction{t: Clv, a: Implied});
    assert!(!cpu.is_overflow(), "CLV");

    // Load and Store

    cpu.execute(Instruction{t: Lda, a: Immediate(0x80)});
    assert_eq!(cpu.a, 0x80, "LDA Immediate");
    cpu.execute(Instruction{t: Ldx, a: Immediate(0x10)});
    assert_eq!(cpu.x, 0x10, "LDX Immediate");
    cpu.execute(Instruction{t: Ldy, a: Immediate(0xA0)});
    assert_eq!(cpu.y, 0xA0, "LDY Immediate");

    cpu.execute(Instruction{t: Sta, a: Absolute(0x8080)});
    assert_eq!(cpu.get(0x8080), cpu.a, "STA Absolute");
    cpu.execute(Instruction{t: Stx, a: Absolute(0x8080)});
    assert_eq!(cpu.get(0x8080), cpu.x, "STX Absolute");
    cpu.execute(Instruction{t: Sty, a: Absolute(0x8080)});
    assert_eq!(cpu.get(0x8080), cpu.y, "STY Absolute");

    // Register Transfers

    cpu.a = 0x80;
    cpu.x = 0x00;
    cpu.y = 0x00;
    cpu.sp = 0xFF;
    cpu.execute(Instruction{t: Tax, a: Implied});
    assert_eq!(cpu.a, cpu.x, "TAX");
    cpu.execute(Instruction{t: Tay, a: Implied});
    assert_eq!(cpu.a, cpu.y, "TAY");
    cpu.execute(Instruction{t: Tsx, a: Implied});
    assert_eq!(cpu.sp, cpu.x, "TSX");
    cpu.execute(Instruction{t: Txa, a: Implied});
    assert_eq!(cpu.x, cpu.a, "TXA");
    cpu.execute(Instruction{t: Tya, a: Implied});
    assert_eq!(cpu.y, cpu.a, "TYA");
    cpu.x = 0x80;
    cpu.execute(Instruction{t: Txs, a: Implied});
    assert_eq!(cpu.y, cpu.a, "TXS");
    
    // Addition

    cpu.clear_carry();
    cpu.a = 0x80;
    cpu.set(0x8000, 0xB0);
    cpu.execute(Instruction{t: Adc, a: Immediate(0x80)});
    assert_eq!(cpu.a, 0x00, "ADC Immediate - Value");
    assert!(cpu.is_carry(), "ADC Immediate - Carry");
    assert!(cpu.is_overflow(), "ADC Immediate - Overflow");
    assert!(!cpu.is_negative(), "ADC Immediate - Negative");
    assert!(cpu.is_zero(), "ADC Immediate - Zero");
    cpu.execute(Instruction{t: Adc, a: Absolute(0x8000)});
    assert_eq!(cpu.a, 0xB1, "ADC Absolute - Value");
    assert!(!cpu.is_carry(), "ADC Absolute - Carry");
    assert!(!cpu.is_overflow(), "ADC Absolute - Overflow");
    assert!(cpu.is_negative(), "ADC Absolute - Negative");
    assert!(!cpu.is_zero(), "ADC Absolute - Zero");

    // Subtraction

    cpu.set_carry();
    cpu.a = 0x50;
    cpu.execute(Instruction{t: Sbc, a: Immediate(0xB0)});
    assert_eq!(cpu.a, 0xA0, "SBC Immediate - Value");
    assert!(!cpu.is_carry(), "SBC Immediate - Carry");
    assert!(cpu.is_overflow(), "SBC Immediate - Overflow");
    assert!(cpu.is_negative(), "SBC Immediate - Negative");
    assert!(!cpu.is_zero(), "SBC Immediate - Zero");

    cpu.clear_carry();
    cpu.a = 0x20;
    cpu.set(0x8000, 0x10);
    cpu.execute(Instruction{t: Sbc, a: Absolute(0x8000)});
    assert_eq!(cpu.a, 0x0F, "SBC Absolute - Value");
    assert!(cpu.is_carry(), "SBC Absolute  - Carry");
    assert!(!cpu.is_overflow(), "SBC Absolute - Overflow");
    assert!(!cpu.is_negative(), "SBC Absolute - Negative");
    assert!(!cpu.is_zero(), "SBC Absolute - Zero");

    // Logical Operations

    cpu.a = 0xF0;
    cpu.set(0x1000, 0x80);
    cpu.execute(Instruction{t: And, a: Absolute(0x1000)});
    assert_eq!(cpu.a, 0x80, "AND");
    assert!(cpu.is_negative(), "AND - Negative Bit");
    assert!(!cpu.is_zero(), "AND - Zero Bit");

    cpu.a = 0x80;
    cpu.set(0x1000, 0x70);
    cpu.execute(Instruction{t: Ora, a: Absolute(0x1000)});
    assert_eq!(cpu.a, 0xF0, "ORA");

    cpu.a = 0xFF;
    cpu.set(0x1000, 0xFF);
    cpu.execute(Instruction{t: Eor, a: Absolute(0x1000)});
    assert_eq!(cpu.a, 0x00, "EOR");

    // Stack Operations

    cpu.a = 0xFF;
    cpu.sp = 0xFF;
    cpu.set(0x01FF, 0x00);
    cpu.execute(Instruction{t: Pha, a: Implied});
    assert_eq!(cpu.get(0x01FF), 0xFF, "PHA Value");
    assert_eq!(cpu.sp, 0xFE, "PHA Stack Pointer");

    cpu.a = 0x00;
    cpu.sp = 0xFF;
    cpu.sr = 0xCF;
    cpu.set(0x01FF, 0x00);
    cpu.execute(Instruction{t: Php, a: Implied});
    assert_eq!(cpu.get(0x01FF), 0xFF, "PHP Value");
    assert_eq!(cpu.sp, 0xFE, "PHP Stack Pointer");

    cpu.a = 0x00;
    cpu.sp = 0xFE;
    cpu.set(0x01FF, 0xFF);
    cpu.execute(Instruction{t: Pla, a: Implied});
    assert_eq!(cpu.a, 0xFF, "PLA Value");
    assert_eq!(cpu.sp, 0xFF, "PLA Stack Pointer");

    cpu.a = 0xFF;
    cpu.sr = 0x00;
    cpu.sp = 0xFE;
    cpu.set(0x01FF, 0xFF);
    cpu.execute(Instruction{t: Plp, a: Implied});
    assert_eq!(cpu.sr, 0xCF, "PHP Value");
    assert_eq!(cpu.sp, 0xFF, "PHP Stack Pointer");

    // Bit Shifting

    cpu.a = 0xFF;
    cpu.sr = 0x00;
    cpu.execute(Instruction{t: Asl, a: Accumulator});
    assert_eq!(cpu.a, 0xFE, "ASL Accumulator - Value");
    assert!(cpu.is_carry(), "ASL Accumulator - Carry");
    assert!(cpu.is_negative(), "ASL Accumulator - Negative");
    assert!(!cpu.is_zero(), "ASL Accumulator - Zero");

    cpu.a = 0xFF;
    cpu.sr = 0x00;
    cpu.set(0x2000, 0x0F);
    cpu.execute(Instruction{t: Asl, a: Absolute(0x2000)});
    assert_eq!(cpu.a, 0xFF, "ASL Absolute - Accumulator");
    assert_eq!(cpu.get(0x2000), 0x1E, "ASL Absolute - Value");
    assert!(!cpu.is_carry(), "ASL Absolute - Carry");
    assert!(!cpu.is_negative(), "ASL Absolute - Negative");
    assert!(!cpu.is_zero(), "ASL Absolute - Zero");

    cpu.a = 0xFF;
    cpu.sr = 0x00;
    cpu.execute(Instruction{t: Lsr, a: Accumulator});
    assert_eq!(cpu.a, 0x7F, "LSR Accumulator - Value");
    assert!(cpu.is_carry(), "LSR Accumulator - Carry");
    assert!(!cpu.is_negative(), "LSR Accumulator - Negative");
    assert!(!cpu.is_zero(), "LSR Accumulator - Zero");

    cpu.a = 0xFF;
    cpu.sr = 0x00;
    cpu.set(0x2000, 0xF0);
    cpu.execute(Instruction{t: Lsr, a: Absolute(0x2000)});
    assert_eq!(cpu.a, 0xFF, "LSR Absolute - Accumulator");
    assert_eq!(cpu.get(0x2000), 0x78, "LSR Absolute - Value");
    assert!(!cpu.is_carry(), "LSR Absolute - Carry");
    assert!(!cpu.is_negative(), "LSR Absolute - Negative");
    assert!(!cpu.is_zero(), "LSR Absolute - Zero");

    // Bit Test

    cpu.a = 0x02;
    cpu.sr = 0x00;
    cpu.set(0x3000, 0xC2);
    cpu.execute(Instruction{t: Bit, a: Absolute(0x3000)});
    assert_eq!(cpu.a, 0x02, "BIT Absolute - Accumulator");
    assert_eq!(cpu.get(0x3000), 0xC2, "BIT Absolute - Value");
    assert!(cpu.is_overflow(), "BIT Absolute - Overflow");
    assert!(cpu.is_negative(), "BIT Absolute - Negative");
    assert!(!cpu.is_zero(), "BIT Absolute - Zero");
    assert!(!cpu.is_carry(), "BIT Absolute - Carry");

    // Comparisons
    cpu.sr = 0x00;
    cpu.a = 0x50;
    cpu.x = 0x00;
    cpu.y = 0x00;
    cpu.execute(Instruction{t: Cmp, a: Immediate(0xB0)});
    assert_eq!(cpu.a, 0x50, "CMP Immediate - Value");
    assert!(!cpu.is_carry(), "CMP Immediate - Carry");
    assert!(!cpu.is_overflow(), "CMP Immediate - Overflow");
    assert!(cpu.is_negative(), "CMP Immediate - Negative");
    assert!(!cpu.is_zero(), "CMP Immediate - Zero");

    cpu.sr = 0x00;
    cpu.a = 0x00;
    cpu.x = 0x50;
    cpu.y = 0x00;
    cpu.execute(Instruction{t: Cpx, a: Immediate(0xB0)});
    assert_eq!(cpu.x, 0x50, "CPX Immediate - Value");
    assert!(!cpu.is_carry(), "CPX Immediate - Carry");
    assert!(!cpu.is_overflow(), "CPX Immediate - Overflow");
    assert!(cpu.is_negative(), "CPX Immediate - Negative");
    assert!(!cpu.is_zero(), "CPX Immediate - Zero");

    cpu.sr = 0x00;
    cpu.a = 0x00;
    cpu.x = 0x00;
    cpu.y = 0x50;
    cpu.execute(Instruction{t: Cpy, a: Immediate(0xB0)});
    assert_eq!(cpu.y, 0x50, "CPX Immediate - Value");
    assert!(!cpu.is_carry(), "CPX Immediate - Carry");
    assert!(!cpu.is_overflow(), "CPX Immediate - Overflow");
    assert!(cpu.is_negative(), "CPX Immediate - Negative");
    assert!(!cpu.is_zero(), "CPX Immediate - Zero");

    // Increment

    cpu.sr = 0x00;
    cpu.set(0x8000, 0x01);
    cpu.execute(Instruction{t: Inc, a: Absolute(0x8000)});
    assert_eq!(cpu.get(0x8000), 0x02, "INC - Value");
    assert!(!cpu.is_negative(), "INC - Negative");
    assert!(!cpu.is_zero(), "INC - Zero");

    cpu.sr = 0x00;
    cpu.x = 0xFF;
    cpu.execute(Instruction{t: Inx, a: Implied});
    assert_eq!(cpu.x, 0x00, "INX - Value");
    assert!(!cpu.is_negative(), "INX - Negative");
    assert!(cpu.is_zero(), "INX - Zero");

    cpu.sr = 0x00;
    cpu.y = 0x7F;
    cpu.execute(Instruction{t: Iny, a: Implied});
    assert_eq!(cpu.y, 0x80, "INY - Value");
    assert!(cpu.is_negative(), "INY - Negative");
    assert!(!cpu.is_zero(), "INY - Zero");

    // Decrement
    
    cpu.sr = 0x00;
    cpu.set(0x8000, 0x01);
    cpu.execute(Instruction{t: Dec, a: Absolute(0x8000)});
    assert_eq!(cpu.get(0x8000), 0x00, "DEC - Value");
    assert!(!cpu.is_negative(), "DEC - Negative");
    assert!(cpu.is_zero(), "DEC - Zero");

    cpu.sr = 0x00;
    cpu.x = 0x00;
    cpu.execute(Instruction{t: Dex, a: Implied});
    assert_eq!(cpu.x, 0xFF, "DEX - Value");
    assert!(cpu.is_negative(), "DEX - Negative");
    assert!(!cpu.is_zero(), "DEX - Zero");

    cpu.sr = 0x00;
    cpu.y = 0x80;
    cpu.execute(Instruction{t: Dey, a: Implied});
    assert_eq!(cpu.y, 0x7F, "DEY - Value");
    assert!(!cpu.is_negative(), "DEY - Negative");
    assert!(!cpu.is_zero(), "DEY - Zero");

    // Roll Left and Right

    cpu.sr = 0x00;
    cpu.clear_carry();
    cpu.a = 0xFF;
    cpu.execute(Instruction{t: Rol, a: Accumulator});
    assert_eq!(cpu.a, 0xFE, "ROL - Value");
    assert!(cpu.is_carry(), "ROL - Carry");
    assert!(cpu.is_negative(), "ROL - Negative");
    assert!(!cpu.is_zero(), "ROL - Zero");

    cpu.sr = 0x00;
    cpu.set_carry();
    cpu.a = 0x01;
    cpu.execute(Instruction{t: Ror, a: Accumulator});
    assert_eq!(cpu.a, 0x80, "ROL - Value");
    assert!(cpu.is_carry(), "ROL - Carry");
    assert!(cpu.is_negative(), "ROL - Negative");
    assert!(!cpu.is_zero(), "ROL - Zero");

    // Branching

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.set_carry();
    cpu.execute(Instruction{t: Bcc, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BCC Shouldn't Branch");
    cpu.clear_carry();
    cpu.execute(Instruction{t: Bcc, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BCC Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.clear_carry();
    cpu.execute(Instruction{t: Bcs, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BCS Shouldn't Branch");
    cpu.set_carry();
    cpu.execute(Instruction{t: Bcs, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BCS Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.clear_zero();
    cpu.execute(Instruction{t: Beq, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BEQ Shouldn't Branch");
    cpu.set_zero();
    cpu.execute(Instruction{t: Beq, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BEQ Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.clear_negative();
    cpu.execute(Instruction{t: Bmi, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BMI Shouldn't Branch");
    cpu.set_negative();
    cpu.execute(Instruction{t: Bmi, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BMI Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.set_zero();
    cpu.execute(Instruction{t: Bne, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BNE Shouldn't Branch");
    cpu.clear_zero();
    cpu.execute(Instruction{t: Bne, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BNE Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.set_negative();
    cpu.execute(Instruction{t: Bpl, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BPL Shouldn't Branch");
    cpu.clear_negative();
    cpu.execute(Instruction{t: Bpl, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BPL Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.set_overflow();
    cpu.execute(Instruction{t: Bvc, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BVC Shouldn't Branch");
    cpu.clear_overflow();
    cpu.execute(Instruction{t: Bvc, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BVC Should Branch");

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.clear_overflow();
    cpu.execute(Instruction{t: Bvs, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0x0000, "BVS Shouldn't Branch");
    cpu.set_overflow();
    cpu.execute(Instruction{t: Bvs, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "BVS Should Branch");

    // Jump

    cpu.sr = 0x00;
    cpu.pc = 0x0000;
    cpu.execute(Instruction{t: Jmp, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "JMP Should Jump");

    // Jump to Subroutine and Return

    cpu.sr = 0x00;
    cpu.sp = 0xFF;
    cpu.pc = 0x1003;
    cpu.execute(Instruction{t: Jsr, a: Absolute(0xFFFF)});
    assert_eq!(cpu.pc, 0xFFFF, "JSR Should Jump");
    assert_eq!(cpu.sp, 0xFD, "JSR - Stack Pointer Should Change");
    assert_eq!(cpu.get(0x01FF), 0x10, "JSR - Stack Should Contain PC High Byte");
    assert_eq!(cpu.get(0x01FE), 0x02, "JSR - Stack Should Contain PC Low Byte");

    cpu.execute(Instruction{t: Rts, a: Implied});
    assert_eq!(cpu.pc, 0x1003, "RTS Should Jump");
    assert_eq!(cpu.sp, 0xFF, "RTS - Stack Pointer Should Change");

    // Break and Return

    cpu.sr = 0x00;
    cpu.sp = 0xFF;
    cpu.pc = 0x1003;
    cpu.set(0xFFFE, 0x04);
    cpu.set(0xFFFF, 0x05);
    cpu.execute(Instruction{t: Brk, a: Implied});
    assert_eq!(cpu.pc, 0x0504, "BRK Should Jump");
    assert_eq!(cpu.sp, 0xFC, "BRK - Stack Pointer Should Change");
    assert_eq!(cpu.get(0x01FF), 0x10, "BRK - Stack Should Contain PC High Byte");
    assert_eq!(cpu.get(0x01FE), 0x02, "BRK - Stack Should Contain PC Low Byte");
    assert_eq!(cpu.get(0x01FD), cpu.sr, "BRK - Stack Should Contain SR");

    cpu.execute(Instruction{t: Rti, a: Implied});
    assert_eq!(cpu.pc, 0x1003, "RTS Should Jump");
    assert_eq!(cpu.sp, 0xFF, "RTS - Stack Pointer Should Change");

}

#[test]
fn opcodes() {
    let mut cpu = Cpu::new6502();
    cpu.set(0x0000, 0xEA);
    cpu.set(0x0001, 0xA5);
    cpu.set(0x0002, 0xFF);
    cpu.set(0x0003, 0xAD);
    cpu.set(0x0004, 0x10);
    cpu.set(0x0005, 0x20);

    let i = cpu.next_instruction();
    assert_eq!(i.t, Nop, "NOP Instruction");
    assert_eq!(i.a, Implied, "NOP Implied Address Type");
    assert_eq!(cpu.pc, 0x0001, "NOP Moves PC by 1");

    let i = cpu.next_instruction();
    assert_eq!(i.t, Lda, "LDA ZP Instruction");
    assert_eq!(i.a, ZeroPage(0xFF), "LDA ZP Address Type");
    assert_eq!(i.a.address(&mut cpu), Some(0x00FFu16), "LDA ZP Address Value");
    assert_eq!(cpu.pc, 0x0003, "LDA ZP Moves PC by 2");

    let i = cpu.next_instruction();
    assert_eq!(i.t, Lda, "LDA Absolute Instruction");
    assert_eq!(i.a, Absolute(0x2010), "LDA Absolute Address Type");
    assert_eq!(i.a.address(&mut cpu), Some(0x2010u16), "LDA Absolute Address Value");
    assert_eq!(cpu.pc, 0x0006, "LDA Absolute Moves PC by 3");
}

#[test]
fn execute_next_instruction() {
    let mut cpu = Cpu::new6502();
    cpu.set(0x0000, 0xEA);
    cpu.set(0x0001, 0xA5);
    cpu.set(0x0002, 0xFF);
    cpu.set(0x0003, 0xAD);
    cpu.set(0x0004, 0x10);
    cpu.set(0x0005, 0x20);
    
    cpu.set(0x00FF, 0x80);
    cpu.set(0x2010, 0x40);

    cpu.execute_next_instruction();
    assert_eq!(cpu.pc, 0x0001, "NOP Moves PC by 1");

    cpu.execute_next_instruction();
    assert_eq!(cpu.a, 0x80, "LDA ZP");
    assert_eq!(cpu.pc, 0x0003, "LDA ZP Moves PC by 2");

    cpu.execute_next_instruction();
    assert_eq!(cpu.a, 0x40, "LDA Absolute");
    assert_eq!(cpu.pc, 0x0006, "LDA Absolute Moves PC by 3");
}
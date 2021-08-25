use std::fmt;

const MEMORY_SIZE: usize = 0xFFFF;

struct CPU {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sr: u8,
    sp: u8,
    memory: [u8;MEMORY_SIZE]
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc: 0xFFFE,
            a: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn reset(&mut self) {
        for i in 0..MEMORY_SIZE {
            self.memory[i] = 0;
        }
        self.pc = 0xFFFE;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sr = 0;
        self.sp = 0;
    }

    pub fn zero_page(&self) -> &[u8] {
        &self.memory[0x0000..0x0100]
    }

    pub fn stack(&self) -> &[u8] {
        &self.memory[0x0100..0x0200]
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
            row += CHUNK_SIZE as u16;
        }
        Ok(())
    }

}

impl fmt::Debug for CPU {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Registers:")
            .field("A", &format_args!("{:02X}", self.a))
            .field("X", &format_args!("{:02X}", self.x))
            .field("Y", &format_args!("{:02X}", self.y))
            .field("PC", &format_args!("{:04X}", self.pc))
            .field("SR", &format_args!("{:02X}", self.sr))
            .field("SP", &format_args!("{:02X}", self.sp))
            .finish()?;
        fmt.write_str("\n")?;
        fmt.write_str("\nZero Page:\n")?;
        CPU::fmt_memory(self.zero_page(), 0x0000, fmt)?;
        fmt.write_str("\nStack:\n")?;
        CPU::fmt_memory(self.stack(), 0x0100, fmt)
    }
}

fn main() {
    println!("Initializing...");
    let mut cpu = CPU::new();
    cpu.reset();
    println!("{:?}", cpu);
}

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

#[cfg(test)]
mod tests;

pub mod addressing;
pub mod instruction;
pub mod opcodes;
pub mod cpu;
pub mod device;
pub mod memory;

use crate::cpu::Cpu;
use crate::memory::Memory;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

/***** main *****/

fn load_hex(cpu: &mut Cpu, filename: &str) {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let buffered = BufReader::new(file);
    for result in buffered.lines() {
        match result {
            Err(why) => panic!("couldn't read from {}: {}", display, why),
            Ok(line) => {
                let parts: Vec<&str> = line.split(":").collect();
                let offset = parts[0];
                let mut pos = match u16::from_str_radix(offset, 16) {
                    Err(why) => panic!("invalid offset {} in {}: {}", offset, display, why),
                    Ok(pos) => pos,
                };
                for h in parts[1].split(" ") {
                    let hex = h.trim();
                    if !hex.is_empty() {
                        let byte = match u8::from_str_radix(hex, 16) {
                            Err(why) => panic!("invalid hex value {} in {}: {}", hex, display, why),
                            Ok(byte) => byte,
                        };
                        cpu.set(pos, byte);
                        pos += 1;
                    }
                }
            }
        }
    }
}

fn main() {
    eprint!("Initializing...");
    let mut cpu = Cpu::new6502();
    eprintln!("Done");
    eprint!("Loading Program...");
    load_hex(&mut cpu, "test.hex");
    cpu.reset();
    eprintln!("Done");
    eprintln!("Initial PC: {:04X}", cpu.pc);
    eprint!("Running...");
    let start_time = Instant::now();
    cpu.run();
    let runtime = start_time.elapsed();
    eprintln!("Done");
    if runtime.as_secs() > 0 {
        eprintln!("Runtime: {} s", runtime.as_secs_f32());
    } else if runtime.as_millis() > 0 {
        eprintln!("Runtime: {} ms", runtime.as_millis());
    } else {
        eprintln!("Runtime: {} μs", runtime.as_micros());
    }
    eprintln!("");
    eprintln!("{:?}", cpu);
    //println!("{:X}", cpu);
}

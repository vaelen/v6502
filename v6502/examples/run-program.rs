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

use std::time::Instant;

use v6502::cpu::Cpu;
use v6502::util::load_hex;

fn main() {
    eprint!("Initializing...");
    let mut cpu = Cpu::new6502();
    eprintln!("Done");
    eprint!("Loading Program...");
    load_hex(&mut cpu, "program.hex");
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
    // print entire memory as hex
    //println!("{:X}", cpu);
}

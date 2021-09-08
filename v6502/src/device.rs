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

use std::{io::{self, Read, Write}, time::{SystemTime, UNIX_EPOCH}};
 
use crate::memory::Memory;
 
 pub trait Device<T=Self>: Memory {
    fn new() -> T;
    fn name() -> String;
    fn status() -> String;
}

pub struct Terminal {
    last_bytes_read: usize,
    pub input: Box<dyn Read>,
    pub output: Box<dyn Write>,
}

impl Memory for Terminal {
    fn get(&mut self, address: u16) -> u8 {
        let mut buf = [0];
        match address {
            0 => {
                self.last_bytes_read = self.input.read(&mut buf).unwrap();
                if self.last_bytes_read == 0 {
                    0
                } else {
                    buf[0]
                }
            },
            1 => self.last_bytes_read as u8,
            _ => 0,
        }
    }

    fn set(&mut self, address: u16, value: u8) {
        match address {
            0 => self.output.write(&[value]).unwrap(),
            _ => 0,
        };
    }
}

impl Device for Terminal{
    fn new() -> Self {
        Terminal { 
            last_bytes_read: 0,
            input: Box::new(io::stdin()),
            output: Box::new(io::stdout()),
        }
    }

    fn name() -> String {
        format!("Terminal")
    }

    fn status() -> String {
        format!("Normal")
    }
}

pub struct Rand {
    w: u32,
    x: u32, 
    y: u32, 
    z: u32, 
}

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;

impl Rand {
    // Borrowed from Wikipedia
    pub fn rand(&mut self) -> u32 {
        let t = self.x^self.x.wrapping_shl(11);
        self.x = self.y; self.y = self.z; self.z = self.w;
        self.w ^= self.w.wrapping_shr(19)^t^t.wrapping_shr(8);
        return self.w;
    }

    pub fn from_seed(seed: u32) -> Rand {
        Rand{
            x: KX^seed, y: KY^seed,
            z: KZ, w: KW
        }
    }
}

impl Device for Rand {
    fn new() -> Self {
        let now = SystemTime::now();
        match now.duration_since(UNIX_EPOCH) {
            Ok(duration) => Rand::from_seed(duration.as_millis() as u32),
            Err(e) => panic!("Couldn't get system time: {}", e),
        }
    }

    fn name() -> String {
        "Random Number Generator".to_string()
    }

    fn status() -> String {
        "Normal".to_string()
    }
}

impl Memory for Rand {
    fn get(&mut self, _: u16) -> u8 {
        self.rand() as u8
    }

    fn set(&mut self, _: u16, _: u8) {

    }
    
}
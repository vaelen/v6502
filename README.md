# v6502

This package contains an emulator for the 6502 microprocessor.

It doesn't support binary decimal (BCD) mode yet.

To run the debug version: `cargo run`

To run the release version: `cargo run --release`

To run tests: `cargo test`

To build a release version: `cargo build --release`

The test program writes a zero page memory address 65,536 times, performing a ROR operation on the accumulator between writes.

6502 References:
- https://www.masswerk.at/6502/6502_instruction_set.html
- http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
- https://llx.com/Neil/a2/opcodes.html
- https://www.pagetable.com/?p=39

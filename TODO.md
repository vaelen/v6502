# TODO List

- Implement BCD Math
- Double check Break and ignored SR bit behavior for PHP, PLP, BRK, and RTI

  Note: The break flag is not an actual flag implemented in a register, and rather
  appears only, when the status register is pushed onto or pulled from the stack.
  When pushed, it will be 1 when transfered by a BRK or PHP instruction, and
  zero otherwise (i.e., when pushed by a hardware interrupt).
  When pulled into the status register (by PLP or on RTI), it will be ignored.
  
  In other words, the break flag will be inserted, whenever the status register
  is transferred to the stack by software (BRK or PHP), and will be zero, when
  transferred by hardware. Since there is no actual slot for the break flag, it
  will be always ignored, when retrieved (PLP or RTI).
  The break flag is not accessed by the Cpu at anytime and there is no internal
  representation. Its purpose is more for patching, to discern an interrupt caused
  by a BRK instruction from a normal interrupt initiated by hardware.

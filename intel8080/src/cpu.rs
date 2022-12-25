pub(crate) mod registers;

pub(crate) struct Cpu {
    counter: u16, // program counter, for keeping track of current execution position in memory
    stack_pointer: u16, // points to the head of the stack
    reg: [(u8, u8); 3], // general purpose register pairs, BC, DE, HL
    temp: (u8, u8), // special W and Z registers, for holding operands of 2 and 3 byte instructions
    status: u8,   // flag register, F
    acc: u8,      // accumulator, used for math, A
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            counter: 0,
            stack_pointer: 0,
            reg: [(0, 0); 3],
            temp: (0, 0),
            status: 0,
            acc: 0,
        }
    }
}

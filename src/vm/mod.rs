use std::ops::{Add, Index, IndexMut, Mul};

#[allow(unused)]
use anyhow::{bail, Context, Result};

use opcode::*;
use std::cell::Cell;
use std::collections::VecDeque;

// We use usize as address since slices indexes use usize
pub type Address = usize;

// The minimal accessible unit. From day 5 it should support negative numbers which
// introduces some complications, by needing to translate between a word value and an address.
pub type Word = i32;

pub type Opcode = u8;

mod opcode {
    use crate::vm::Opcode;

    pub const OPCODE_ADD: Opcode = 1;
    pub const OPCODE_MUL: Opcode = 2;
    pub const OPCODE_STORE_AT: Opcode = 3;
    pub const OPCODE_RET: Opcode = 99;
}

pub struct VM<'a> {
    // memory
    program: &'a mut [Word],

    // program counter (= instruction pointer)
    pc: Address,

    // inputs if any
    inputs: &'a [Word],

    //
    outputs: VecDeque<Word>,
}

impl<'a> VM<'a> {
    pub fn new(program: &'a mut [Word]) -> Self {
        Self {
            program,
            pc: 0,
            inputs: &mut [],
            outputs: VecDeque::new(),
        }
    }

    pub fn with_inputs(program: &'a mut [Word], inputs: &mut [Word]) -> Self {
        unimplemented!()
    }

    pub fn execute(&mut self) -> anyhow::Result<Word> {
        while self.program[self.pc] as u8 != OPCODE_RET {
            let opcode = self.program[self.pc] as u8;

            self.pc = match opcode {
                OPCODE_ADD => binop(Word::add)(self.program, self.pc),
                OPCODE_MUL => binop(Word::mul)(self.program, self.pc),
                _ => bail!("Unable to compute. Rejected program."),
            }
        }

        Ok(self.program[0])
    }
}

fn binop<F>(f: F) -> impl Fn(&mut [Word], Address) -> Address
where
    F: Fn(Word, Word) -> Word,
{
    move |tape: &mut [Word], pc: Address| {
        let pc1 = tape[pc + 1];
        let pc2 = tape[pc + 2];
        let pc3 = tape[pc + 3];

        tape[pc3 as usize] = f(tape[pc1 as usize], tape[pc2 as usize]);

        pc + 4
    }
}

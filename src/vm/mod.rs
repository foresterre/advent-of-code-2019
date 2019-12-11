// TODO:
// * Error handling
// * make an iterator around eval()
// * Transorm Param to InParam and OutParam, which may improve readability
// * Opcodes back to u8 (?) (or too much casting)

use anyhow::{bail, Context, Result};
use opcode::*;
use std::collections::VecDeque;
use std::ops::{Add, Mul};

// We use usize as address since slices indexes use usize
pub type Address = usize;

// The minimal accessible unit. From day 5 it should support negative numbers which
// introduces some complications, by needing to translate between a word value and an address.
pub type Word = i32;
// was u8, but i32 for now means less casting
pub type Opcode = i32;

mod opcode {
    use crate::vm::Opcode;

    pub const OPCODE_ADD: Opcode = 1;
    pub const OPCODE_MUL: Opcode = 2;
    pub const OPCODE_INPUT: Opcode = 3;
    pub const OPCODE_OUTPUT: Opcode = 4;
    pub const OPCODE_JUMP_IF_TRUE: Opcode = 5;
    pub const OPCODE_JUMP_IF_FALSE: Opcode = 6;
    pub const OPCODE_LT: Opcode = 7;
    pub const OPCODE_EQ: Opcode = 8;
    pub const OPCODE_RET: Opcode = 99;
}

const PARAM1: i32 = 1;
const PARAM2: i32 = 10;

// The instruction decoder.
//
// ABCDE
//
// DE - two digit op code
// C - 1st param mode
// B - 2nd param mode
// A - 3rd param mode (currently always an output)
#[derive(Debug, Copy, Clone)]
pub enum InParam {
    Position(Address), // mode 0
    Immediate(Word),   // mode 1
}

impl InParam {
    fn new(opcode: Opcode, param_scale: Word, value: Word) -> Self {
        match opcode / (100 * param_scale) % 10 {
            0 => InParam::Position(value as usize),
            1 => InParam::Immediate(value),
            _ => panic!("param1 . TODO error handling..."),
        }
    }

    // read from the tape, method depends on the mode.
    fn read(&self, tape: &[Word]) -> Word {
        match self {
            InParam::Position(addr) => tape[*addr],
            InParam::Immediate(w) => *w,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OutParam {
    addr: Address,
}

impl OutParam {
    fn new(addr: Address) -> Self {
        OutParam { addr }
    }

    /// Write the tape
    fn write(self, tape: &mut [Word], value: Word) {
        tape[self.addr] = value;
    }
}

#[derive(Debug)]
pub enum Instruction {
    // (operation, first param, second param, third param)
    Binop(BinopInstr, [InParam; 2], OutParam),
    Input(OutParam),
    Output(InParam),
    JumpIfTrue([InParam; 2]),
    JumpIfFalse([InParam; 2]),
    LessThan([InParam; 2], OutParam), // fixme: generalize
    Equals([InParam; 2], OutParam),
    Ret,
}

impl Instruction {
    fn fetch(tape: &[Word], pc: Address) -> Result<Self> {
        let opcode = tape[pc];

        match opcode % 100 {
            OPCODE_ADD => Ok(Instruction::mk_binop(BinopInstr::Add, opcode, tape, pc)),
            OPCODE_MUL => Ok(Instruction::mk_binop(BinopInstr::Mul, opcode, tape, pc)),
            OPCODE_INPUT => Ok(Instruction::Input(OutParam::new(tape[pc + 1] as usize))),
            OPCODE_OUTPUT => Ok(Instruction::Output(InParam::new(
                opcode,
                PARAM1,
                tape[pc + 1],
            ))),
            OPCODE_JUMP_IF_TRUE => Ok(Instruction::JumpIfTrue([
                InParam::new(opcode, PARAM1, tape[pc + 1]),
                InParam::new(opcode, PARAM2, tape[pc + 2]),
            ])),
            OPCODE_JUMP_IF_FALSE => Ok(Instruction::JumpIfFalse([
                InParam::new(opcode, PARAM1, tape[pc + 1]),
                InParam::new(opcode, PARAM2, tape[pc + 2]),
            ])),
            OPCODE_LT => Ok(Instruction::LessThan(
                [
                    InParam::new(opcode, PARAM1, tape[pc + 1]),
                    InParam::new(opcode, PARAM2, tape[pc + 2]),
                ],
                OutParam::new(tape[pc + 3] as usize),
            )),
            OPCODE_EQ => Ok(Instruction::Equals(
                [
                    InParam::new(opcode, PARAM1, tape[pc + 1]),
                    InParam::new(opcode, PARAM2, tape[pc + 2]),
                ],
                OutParam::new(tape[pc + 3] as usize),
            )),
            OPCODE_RET => Ok(Instruction::Ret),
            _ => bail!("Opcode could not be fetched. Opcode may be invalid. "),
        }
    }

    fn eval(&self, vm: &mut VM) -> Option<()> {
        match self {
            Instruction::Binop(op, params, out) => {
                match op {
                    BinopInstr::Add => binop(Word::add)(vm.tape, *params, *out),
                    BinopInstr::Mul => binop(Word::mul)(vm.tape, *params, *out),
                }

                Some(())
            }
            Instruction::Input(out) => {
                let value = vm.inputs.next().expect("eval . TODO");
                out.write(vm.tape, value);

                Some(())
            }
            Instruction::Output(param) => {
                let value = param.read(vm.tape);
                vm.outputs.push_front(value);

                Some(())
            }
            Instruction::JumpIfTrue(params) => {
                //jump_if(Word::eq)(vm, *params), // fixme see below

                let cond = params[0].read(vm.tape);

                if cond != 0 {
                    let jump_addr = params[1].read(vm.tape);
                    vm.pc = jump_addr as usize;
                } else {
                    vm.pc += 3; // fixme: see len()
                }

                Some(())
            }
            Instruction::JumpIfFalse(params) => {
                // jump_if(Word::ne)(vm, *params) // fixme see below
                let cond = params[0].read(vm.tape);

                if cond == 0 {
                    let jump_addr = params[1].read(vm.tape);
                    vm.pc = jump_addr as usize;
                } else {
                    vm.pc += 3; // fixme: see len()
                }

                Some(())
            }
            Instruction::LessThan(params, out) => {
                // jump_if(Word::ne)(vm, *params) // fixme see below
                let this = params[0].read(vm.tape);
                let other = params[1].read(vm.tape);

                if this < other {
                    out.write(vm.tape, 1);
                } else {
                    out.write(vm.tape, 0);
                };

                Some(())
            }
            Instruction::Equals(params, out) => {
                // jump_if(Word::ne)(vm, *params) // fixme see below
                let this = params[0].read(vm.tape);
                let other = params[1].read(vm.tape);

                if this == other {
                    out.write(vm.tape, 1);
                } else {
                    out.write(vm.tape, 0);
                };

                Some(())
            }
            Instruction::Ret => None,
        }
    }

    fn len(&self) -> usize {
        match self {
            Instruction::Binop(_, _, _) => 4,
            Instruction::Input(_) => 2,
            Instruction::Output(_) => 2,
            Instruction::JumpIfTrue(_) => 0, //3, TODO can be different if jumping
            Instruction::JumpIfFalse(_) => 0, //3
            Instruction::LessThan(_, _) => 4,
            Instruction::Equals(_, _) => 4,
            Instruction::Ret => 1,
        }
    }

    fn mk_binop(which: BinopInstr, opcode: Opcode, tape: &[Word], pc: Address) -> Self {
        Instruction::Binop(
            which,
            [
                InParam::new(opcode, PARAM1, tape[pc + 1]),
                InParam::new(opcode, PARAM2, tape[pc + 2]),
            ],
            OutParam::new(tape[pc + 3] as usize),
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinopInstr {
    Add,
    Mul,
}

#[derive(Debug, Copy, Clone)]
pub enum ExecutionOption {
    OutputByAddress(usize),
    OutputByTapeOutput,
}

impl Default for ExecutionOption {
    fn default() -> Self {
        ExecutionOption::OutputByAddress(0)
    }
}

pub struct VM<'a> {
    // our memory
    pub tape: &'a mut [Word],

    // program counter (= instruction pointer)
    pub pc: Address,

    // inputs if any
    pub inputs: Box<dyn Iterator<Item = Word>>,

    // outputs, if any
    pub outputs: VecDeque<Word>,
}

impl<'a> VM<'a> {
    pub fn new(tape: &'a mut [Word]) -> Self {
        Self {
            tape,
            pc: 0,
            inputs: Box::new(VecDeque::new().into_iter()),
            outputs: VecDeque::new(),
        }
    }

    pub fn with_inputs<I>(program: &'a mut [Word], inputs: I) -> Self
    where
        I: IntoIterator<Item = Word>,
        <I as IntoIterator>::IntoIter: 'static,
    {
        Self {
            tape: program,
            pc: 0,
            inputs: Box::new(inputs.into_iter()),
            outputs: VecDeque::new(),
        }
    }

    pub fn execute(&mut self, output_type: ExecutionOption) -> anyhow::Result<Word> {
        while let Ok(fetched) = Instruction::fetch(self.tape, self.pc) {
            self.pc += fetched.len();

            if fetched.eval(self).is_none() {
                break;
            }
        }

        match output_type {
            ExecutionOption::OutputByAddress(n) => Ok(self.tape[n]),
            ExecutionOption::OutputByTapeOutput => Ok(*self
                .outputs
                .front()
                .with_context(|| "output . TODO more than one output")?),
        }
    }
}

fn binop<F>(f: F) -> impl Fn(&mut [Word], [InParam; 2], OutParam)
where
    F: Fn(Word, Word) -> Word,
{
    move |tape: &mut [Word], params: [InParam; 2], out: OutParam| {
        let x = params[0].read(tape);
        let y = params[1].read(tape);

        out.write(tape, f(x, y))
    }
}

// fixme: lifetime for<'r> issues
//fn jump_if<F>(f: F) -> impl Fn(&mut VM, [InParam; 2])
//where
//    F: Fn(Word, Word) -> bool,
//{
//    move |vm: &mut VM, params: [InParam; 2]| {
//        let cond = params[0].read(vm.tape);
//        let jump_addr = params[1].read(vm.tape);
//
//        if f(cond, 0) {
//            vm.pc = jump_addr as usize
//        }
//    }
//}

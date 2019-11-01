use std::rc::Rc;
use std::collections::HashMap;
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use crate::{operators, Operator, OpCode, Stack, Bytecode};

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperation(u8),
    InvalidArguments,
    StackUnderflow,
    StackOverflow,
    UnexpectedEndOfFile,
    SynthesisError,
}


pub struct VirtualMachine<E, CS> where E: Engine, CS: ConstraintSystem<E> {
    stack: Stack<E>,
    opcodes: HashMap<u8, Rc<Box<dyn Operator<E, CS>>>>,
}

impl<E, CS> VirtualMachine<E, CS> where E: Engine, CS: ConstraintSystem<E> {
    pub fn new() -> Self {
        let mut vm = Self {
            stack: Stack::new(),
            opcodes: HashMap::new(),
        };

        vm.opcodes.insert(OpCode::NoOp as u8, Rc::new(Box::new(operators::NoOp)));
        vm.opcodes.insert(OpCode::Push as u8, Rc::new(Box::new(operators::Push)));
        vm.opcodes.insert(OpCode::Pop as u8, Rc::new(Box::new(operators::Pop)));
        vm.opcodes.insert(OpCode::Add as u8, Rc::new(Box::new(operators::Add)));

        vm
    }

    pub fn run(&mut self, cs: &mut CS, bytecode: &mut Bytecode) -> Result<(), RuntimeError> {
        let mut i = 0;
        while !bytecode.is_eof() {
            let code = bytecode.next_byte().ok_or(RuntimeError::UnexpectedEndOfFile)?;
            let operator = self.dispatch(code)?;
            cs.push_namespace(|| format!("{}", i));
            operator.execute(cs, &mut self.stack, bytecode)?;
            cs.pop_namespace();
            i += 1;
        }
        Ok(())
    }

    fn dispatch(&self, code: u8) -> Result<Rc<Box<dyn Operator<E, CS>>>, RuntimeError> {
        match self.opcodes.get(&code) {
            None => Err(RuntimeError::InvalidOperation(code)),
            Some(op) => Ok(op.clone()),
        }
    }

    pub fn log_stack(&self) {
        println!(">>> stack");
        for i in 0..self.stack.len() {
            match self.stack.get(i) {
                None => println!("none"),
                Some(p) => {
                    match p.value {
                        None => println!("none"),
                        Some(fr) => println!("{:?}", fr),
                    }
                }
            }
        }
    }

    pub fn stack(&self) -> &Stack<E> {
        &self.stack
    }
}

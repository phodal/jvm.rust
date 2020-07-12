use crate::instructions::slot::Slot;
use crate::oops::instanced_klass::JMethod;
use crate::rtda::thread::JThread;
use crate::oops::constant_pool::CpEntry;

#[derive(Debug, Clone)]
pub struct LocalVars {
    slots: Vec<Slot>
}

impl LocalVars {
    pub fn new() -> LocalVars {
        LocalVars {
            slots: vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    pub fn new() -> OperandStack {
        OperandStack {
            size: 0,
            slots: vec![]
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub max_locals: u16,
    pub max_stack: u16,
    pub thread: Box<JThread>,
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,
    pub method: JMethod,
}

impl Frame {
    pub fn new(thread: Box<JThread>, method: JMethod) -> Frame {
        Frame {
            max_locals: 0,
            max_stack: 0,
            thread,
            local_vars: LocalVars::new(),
            operand_stack: OperandStack::new(),
            method
        }
    }

    pub fn get_constant_pool(self) -> Vec<CpEntry> {
        return self.method.class.constant_pool_entries
    }
}
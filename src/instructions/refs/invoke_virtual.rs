use crate::instructions::exec::InstructionExec;
use crate::rtda::frame::Frame;
use crate::classfile::class_file_stream::ClassFileStream;

pub struct InvokeVirtual {
    pub index: usize
}

impl InvokeVirtual {
    pub fn new() -> InvokeVirtual {
        InvokeVirtual {
            index: 0
        }
    }
}

impl InstructionExec for InvokeVirtual {
    fn execute(&self, _frame: &Frame) {

    }

    fn fetch_operands(&mut self, reader: &mut ClassFileStream) {
        self.index = reader.read_u16() as usize;
    }
}
use crate::instructions::decoder::decoder;
use crate::instructions::exec::InstructionExec;
use crate::oops::instanced_klass::JMethod;
use crate::rtda::frame::Frame;
use crate::rtda::j_stack::JStack;

#[derive(Debug, Clone)]
pub struct JThread {
    stack: Box<JStack>,
}

impl JThread {
    pub fn new() -> JThread {
        JThread {
            stack: Box::from(JStack::new(0)),
        }
    }

    pub fn push_frame(&mut self, frame: &Frame) {
        self.stack.push(frame)
    }

    pub fn new_frame(self, method: JMethod) -> Frame {
        Frame::new(Box::from(self), method)
    }
}

pub fn execute_method(frame: &mut Frame, instr: Vec<u8>) -> Vec<Box<dyn InstructionExec>> {
    let _length = instr.len();
    let mut vec = decoder(instr.clone());
    for i in 0..vec.len() {
        vec[i].execute(frame);
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::instructions::decoder::decoder;
    use crate::instructions::exec::InstructionExec;
    use crate::rtda::frame::Frame;
    use crate::rtda::heap::runtime::Runtime;
    use crate::rtda::thread::{execute_method, JThread};
    use std::borrow::Borrow;

    #[test]
    fn test_frame() {
        let runtime = Runtime::new();
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.init(string);
        let klass = class_loader.jl_object_class.get(0).unwrap();

        let method = klass.methods.get(1).unwrap();
        let mut thread = JThread::new();
        let mut frame = thread.clone().new_frame(method.clone());
        thread.push_frame(frame.borrow());

        let execs = execute_method(&mut frame, method.clone().code);
        assert_eq!(9, execs.len());
    }
}

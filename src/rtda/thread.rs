use crate::rtda::j_stack::JStack;
use crate::rtda::frame::Frame;
use crate::oops::instanced_klass::JMethod;

#[derive(Debug, Clone)]
pub struct JThread {
    stack: Box<JStack>
}

impl JThread {
    pub fn new() -> JThread {
        JThread {
            stack: Box::from(JStack::new(0))
        }
    }

    pub fn push_frame(&mut self, frame: &Frame) {
        self.stack.push(frame)
    }

    pub fn new_frame(self, method: JMethod) -> Frame {
        Frame::new(Box::from(self), method)
    }
}

#[cfg(test)]
mod tests {
    use crate::rtda::heap::runtime::Runtime;
    use crate::rtda::thread::JThread;
    use std::borrow::Borrow;
    use crate::instructions::decoder::decoder;
    use crate::rtda::frame::Frame;

    fn exec_bytecode_method(frame: &Frame, instr: Vec<u8>) {
        let length = instr.len();
        let vec = decoder(instr);
        for i in 1..length {
            let x = vec.get(0).unwrap();
            x.execute(frame);
        }
        // vec
    }

    #[test]
    fn test_frame() {
        let mut runtime = Runtime::new();
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.init(string);

        let klass = class_loader.jl_object_class.get(0).unwrap();
        let mut method = klass.methods.get(0).unwrap();

        let mut thread = JThread::new();
        let frame = thread.clone().new_frame(method.clone());
        thread.push_frame(frame.borrow());

        exec_bytecode_method(frame.borrow(), method.clone().code);
    }
}
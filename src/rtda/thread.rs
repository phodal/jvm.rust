use crate::instructions::decoder::{decoder, Decode};
use crate::rtda::frame::Frame;
use crate::rtda::heap::j_method::JMethod;
use crate::rtda::heap::runtime::Runtime;
use crate::rtda::jvm_stack::JVMStack;
use std::borrow::Borrow;
use std::sync::{Mutex, Arc};
use crate::rtda::shim_method::{new_shim_frame};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ThreadPool {}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        ThreadPool {}
    }
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub PC: i64,
    pub stack: Box<JVMStack>,
    pub runtime: Box<Runtime>,
    pub lock: Arc<Mutex<ThreadPool>>,
}

impl Thread {
    pub fn new(runtime: Runtime) -> Thread {
        Thread {
            PC: 0,
            runtime: Box::from(runtime),
            stack: Box::from(JVMStack::new(0)),
            lock: Arc::new(Mutex::new(ThreadPool::new())),
        }
    }

    pub fn sleep(&mut self) {
        self.lock.lock();
    }

    pub fn unlock(&mut self) {
        let guard = self.lock.lock().unwrap();
        std::mem::drop(guard);
    }

    pub fn push_frame(&mut self, frame: &Frame) {
        self.stack.push(frame)
    }

    pub fn new_frame(&self, method: JMethod) -> Frame {
        Frame::new(RefCell::new(self.clone()), method)
    }

    pub fn current_frame(&self) -> Option<Frame> {
        self.stack.top()
    }

    pub fn invoke_method_with_shim(&mut self) {
        // let frame = new_shim_frame(RefCell::from(**self));
        // self.push_frame(&frame)
    }
}

pub fn execute_method(frame: &mut Frame, instr: Vec<u8>) -> Vec<Decode> {
    let _length = instr.len();
    let mut vec = decoder(instr.clone());
    for i in 0..vec.len() {
        vec[i].ins.execute(frame);
    }

    vec
}

pub fn create_frame(method: &JMethod, thread: Rc<RefCell<Thread>>) -> Frame {
    let mut ref_mut = thread.borrow_mut();
    let frame = ref_mut.new_frame(method.clone());
    ref_mut.push_frame(frame.borrow());
    frame
}

#[cfg(test)]
mod tests {
    use crate::classpath::class_path::ClassPath;
    use crate::create_main_thread;
    use crate::rtda::heap::runtime::Runtime;
    use crate::rtda::thread::{create_frame, execute_method};
    use std::sync::Arc;
    use std::borrow::BorrowMut;

    #[test]
    fn test_vec() {
        let v = Vec::from("Ljava/lang/Object;");
        let v = Arc::new(v);
        println!("{:?}", v);
    }

    #[test]
    fn test_frame() {
        let runtime = Runtime::new(ClassPath::new());
        let string = String::from("testdata/java8/HelloWorld.Class");
        let mut class_loader = runtime.boot_loader;
        class_loader.add_user_class(string);

        let klass = class_loader.jl_object_class.get(0).unwrap();
        let second = klass.methods.get(1).unwrap();
        let first = klass.methods.get(0).unwrap();

        let jre_home = "/Library/Java/JavaVirtualMachines/jdk1.8.0_202.jdk/Contents/Home/jre";
        let mut thread = create_main_thread(String::from(jre_home), String::from(""));

        let mut frame1 = create_frame(first, thread.clone());
        let first_execs = execute_method(&mut frame1, first.method_data.clone().code);
        assert_eq!(5, first_execs.len());

        let mut frame2 = create_frame(second, thread.clone());
        let execs = execute_method(&mut frame2, second.method_data.clone().code);
        assert_eq!(9, execs.len());
    }
}

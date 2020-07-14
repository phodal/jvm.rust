use crate::rtda::heap::class_loader::ClassLoader;
use crate::classpath::class_path::ClassPath;

pub struct Runtime {
    pub boot_loader: Box<ClassLoader>,
}

impl Runtime {
    pub fn new(cp: ClassPath) -> Runtime {
        let mut loader = ClassLoader::new();

        let runtime = Runtime {
            boot_loader: Box::new(loader.clone()),
        };

        loader.init();
        runtime
    }
}

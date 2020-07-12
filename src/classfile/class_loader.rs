use crate::classfile::class_factory::ClassFactory;
use crate::classfile::class_file_stream::ClassFileStream;
use crate::oops::instanced_klass::InstanceKlass;


use std::fs;
use std::fs::File;
use std::io::Read;

pub struct ClassPathEntry {

}

impl ClassPathEntry {
    pub fn new() -> ClassPathEntry {
        ClassPathEntry {

        }
    }

    pub fn open_stream(self, filename: String) -> ClassFileStream {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        ClassFileStream::new(buffer)
    }
}

pub struct ClassLoader {
    pub jl_object_class: Vec<InstanceKlass>
}

impl ClassLoader {
    pub fn new() -> ClassLoader {
        ClassLoader {
            jl_object_class: vec![]
        }
    }

    pub fn init(&mut self, class_name: String) {
        self.add_user_class(class_name);
    }

    fn add_user_class(&mut self, class_name: String) {
        let klass = self.build_user_class(class_name);
        self.jl_object_class.push(klass);
    }

    pub fn build_user_class(&mut self, class_name: String) -> InstanceKlass {
        let file_name = self.file_name_for_class_name(class_name);
        let entry = ClassPathEntry::new();
        let stream = entry.open_stream(file_name);

        let klass = ClassFactory::create_from_stream(stream);
        klass
    }

    fn file_name_for_class_name(&mut self, class_name: String) -> String {
        class_name
    }
}

#[cfg(test)]
mod tests {
    use crate::classfile::class_loader::ClassLoader;
    use crate::classfile::attribute_info::AttributeInfo;
    
    
    use crate::oops::instanced_klass::InstanceKlass;

    #[test]
    fn test_load_class() {
        let klass = build_klass();

        assert_eq!(0, klass.minor_version);
        assert_eq!(52, klass.major_version);
        assert_eq!(klass.constant_pool_count as usize, klass.constant_pool_entries.len());
        assert_eq!("HelloWorld", klass.klass_name);
        assert_eq!("java/lang/Object", klass.super_klass_name);
        assert_eq!(0, klass.interfaces.len());
        assert_eq!(2, klass.methods.len());
        let info = klass.methods[0].clone();
        assert_eq!(1, info.attribute_table.len());

        let method2 = klass.methods[1].clone();
        assert_eq!(1, method2.attribute_table.len());

        assert_eq!(1, klass.attributes.len());
    }

    fn build_klass() -> InstanceKlass {
        let path = "testdata/java8/HelloWorld.Class";
        let mut class_loader = ClassLoader::new();
        let klass = class_loader.build_user_class(String::from(path));
        klass
    }

    #[test]
    fn test_load_methods() {
        let klass = build_klass();
        assert_eq!(1, klass.methods[0].access_flags);
        assert_eq!(7, klass.methods[0].name_index);
        assert_eq!(8, klass.methods[0].descriptor_index);
        let attribute_info = klass.methods[0].attribute_table[0].clone();
        match attribute_info {
            AttributeInfo::Code(code) => {
                assert_eq!(1, code.max_stack);
                assert_eq!(1, code.max_locals);
                assert_eq!(0, code.exception_table.len());
                assert_eq!(5, code.code.len());
            },
            _ => {
                assert!(false)
            }
        }

        assert_eq!(9, klass.methods[1].access_flags);
        assert_eq!(14, klass.methods[1].name_index);
        assert_eq!(15, klass.methods[1].descriptor_index);
        let attribute_info = klass.methods[1].attribute_table[0].clone();
        match attribute_info {
            AttributeInfo::Code(code) => {
                assert_eq!(2, code.max_stack);
                assert_eq!(1, code.max_locals);
                assert_eq!(0, code.exception_table.len());
                assert_eq!(9, code.code.len());
            },
            _ => {
                assert!(false)
            }
        }
    }

    #[test]
    fn test_load_attr() {
        let klass = build_klass();
        for attr in klass.attributes {
            match attr {
                AttributeInfo::SourceFile(_) => {
                    assert!(true)
                },
                _ => {
                    assert!(false)
                }
            }
        }
    }

    #[test]
    fn test_load_source_file() {
        let klass = build_klass();
        assert_eq!("HelloWorld.java", klass.source_file);
    }

}

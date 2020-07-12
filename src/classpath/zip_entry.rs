use std::iter::Zip;
use std::path::PathBuf;
use crate::classpath::classpath::Entry;

pub struct ZipEntry {
    pub path: PathBuf
}

impl ZipEntry {
    pub fn new(path: PathBuf) -> ZipEntry {
        ZipEntry {
            path
        }
    }
}

impl Entry for ZipEntry {
    fn read_class(&self, class_name: String) {
        unimplemented!()
    }
}
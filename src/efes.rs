use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    write(read(src), dest)
}
pub fn overwrite_file(src: &Path) -> io::Result<()> {
    write(read(src),src)
}

pub fn write(bytes: Vec<u8>, path: &Path) -> io::Result<()> {
    let mut file = File::create(path).expect("Failed to write lock on file.");
    file.write_all(bytes.as_slice()).expect("Failed to write to file.");
    file.sync_all()
}
pub fn read(path: &Path) -> Vec<u8> {
    let file = File::open(path).expect("Failed to read lock on file.");
    let mut buf_reader = BufReader::new(&file);
    let mut contents: Vec<u8> = vec!();
    let result = buf_reader.read_to_end(& mut contents).expect("Failed to read from file.");
    file.sync_all();
    contents
}

use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix;
use std::path::Path;

struct Data {
    contents: Vec<u8>,
    result: Some(io::Result)
}

fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    let to_write: Vec<u8> = match
    writer(reader(src), dest)
}
fn overwrite_file(src: &Path) -> io::Result<()> {
    writer(reader(src),src)
}

fn writer(bytes: Vec<u8>, path: &Path) -> io::Result<()> {
    let mut file = try!(File::create(path));
    file.write_all(bytes.as_slice());
    file.sync_all()
}
fn reader(path: &Path) -> Data {
    let file = try!(File::open(path));
    let mut buf_reader = BufReader::new(file);
    let mut contents: Vec<u8>;
    let result = buf_reader.read_to_end(& mut contents);
    Data{contents: contents, result: result}
}

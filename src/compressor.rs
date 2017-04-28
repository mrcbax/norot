extern crate lz4_compress as lz4;

use std::path::Path;
use efes::read;

pub fn comp(path: &Path) -> Vec<u8> {
    lz4::compress(&(read(path)).as_slice())
}

pub fn decomp(path: &Path) -> Vec<u8> {
    lz4::decompress(&(read(path)).as_slice()).unwrap()
}

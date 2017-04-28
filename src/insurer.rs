use std::path::Path;

use efes::{read,write};
use compressor::{comp,decomp};

fn replicate(src: &Path, dest: &Path, copies: usize) {
    for n in 0 .. copies {
        let out_dest = dest.join(String::from(".") + n.to_string().as_str());
        match write(comp(src),&out_dest) {
            Ok(o) => o,
            Err(e) => panic!(e),
        };
    }
}

fn replace(src: &Path, dest: &Path, copy: usize) {}

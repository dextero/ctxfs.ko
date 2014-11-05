#!/usr/bin/rust run

use std::str::{from_utf8};
use std::path::Path;
use std::os;
use std::io::Command;
use std::io::File;
use std::num::from_str_radix;
use std::vec::Vec;

// argumnum_entries: name of a .ko-file containing 'rust_main'
// result:   relocation section attributes (num_entries, offset)
fn readelf(file: &Path) -> Vec<(uint, uint)> {
    let parse = |s: &str| {
        let mut ret: Vec<(uint, uint)> = Vec::new();

        for line in s.lines() {
            if line.starts_with("Relocation section '.rela.text.")
                    && line.contains("rustfs") {
                let x1 : Vec<&str> = line.words().collect();
                println!("found: {}", x1);
                let num_entries: uint = from_str_radix(x1[7], 10).unwrap();
                let offset: uint = from_str_radix(x1[5].slice_from(2), 16).unwrap();
                ret.push((num_entries, offset));
            }
        }

        return ret;
    };
    let filename = file.as_str().unwrap();
    match Command::new("readelf").arg("-r").arg(filename).output() {
        Err(e)   => fail!("failed to execute readelf: {}", e),
        Ok (out) => from_utf8(out.output.as_slice()).map(parse).unwrap()
    }
}

fn patch(num_entries: uint, offset: uint, buf: &mut [u8]) {
    for i in range (0, num_entries) {
        let rel = offset + 24*i + 8;
        if buf[rel] == 0x4 {
            println!("Fixup: 0x{}", rel);
            buf[rel] = 0x2;
        }
    }
}

pub fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("[usage] fixup [ko]");
        return;
    }
    let filepath = &Path::new(args[1].as_slice());

    let mut buf = match File::open(filepath).read_to_end() {
        Err(e)   => fail!("failed to open output file: {}", e),
        Ok (res) => res
    };

    let sections = readelf(filepath);
    for num_entries_offset_tuple in sections.iter() {
        let (num_entries, offset) = *num_entries_offset_tuple;
        println!("relocating {}, {}", num_entries, offset);
        patch(num_entries, offset, buf.as_mut_slice())
    }

    let mut file = File::create(filepath);
    file.write(buf.as_slice()).unwrap();
}

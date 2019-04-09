use byteorder::{LittleEndian, ReadBytesExt};
use clap::{App, Arg};
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use lines_are_rusty::*;

fn main() {
    let matches = App::new("lines-are-rusty")
        .version("0.1")
        .about("Converts lines files from .rm to SVG.")
        .author("Axel Huebl <axel.huebl@plasma.ninja>")
        .arg(
            Arg::with_name("file")
                .help("The file to read from")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Load the file into a Vec<u8>
    let filename = matches
        .value_of("file")
        .expect("Expected required filename.");
    let mut f = File::open(filename).unwrap();
    let mut line_file = Vec::<u8>::new();
    f.read_to_end(&mut line_file).unwrap();

    let max_size_file = 1024 * 1024; // Bytes
    assert!(max_size_file >= line_file.len());

    // print!("{}", String::from_utf8_lossy(line_file));

    let header = line_file.iter().take(33).cloned().collect::<Vec<u8>>();
    assert_eq!(header, "reMarkable .lines file, version=3".as_bytes());

    let mut numbers = line_file[43..].chunks(4);
    // as std::slice::Windows<[u8;4]>;
    // as &Iterator<Item=&[u8; 4]>;
    /*
    for c in numbers {
        println!("c: {}   {}", read_number_i32(c), read_number_f32(c));
        println!("cr: {:?}", c);
    }
    */

    //if let Some(iter) = numbers {
    let _pages = read_pages(&mut numbers, max_size_file);
    //} else {
    //    let pages = Vec::<Page>::default();
    //}

    // .map(|x|read_next(&x, & mut pc));
    // .collect::<Vec<_>>();

    // println!("{:?}", &numbers[..200]);
}

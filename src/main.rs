use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

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
        .arg(
            Arg::with_name("output")
                .help("The file to save the SVG to")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("no-auto-crop")
                .short("n")
                .long("no-crop")
                .help("Don't crop the page to fit the content")
        )
        .arg(
            Arg::with_name("custom-colors")
                .short("c")
                .long("colors")
                .help("Which colors to use for the layers. Format: L1-black,L1-gray,L1-white;...;L5-black,L5-gray,L5-white")
                .default_value("black,gray,white;black,gray,white;black,gray,white;black,gray,white;black,gray,white")
        )
        .get_matches();
    let filename = matches
        .value_of("file")
        .expect("Expected required filename.");
    let output_filename = matches
        .value_of("output")
        .expect("Expected required filename.");
    let auto_crop = !matches
        .is_present("no-auto-crop");
    let colors = matches
        .value_of("custom-colors").unwrap();


    let layer_colors = lines_are_rusty::LayerColors {
        colors: colors.split(";").map(|layer| {
            let mut it = layer.split(",");
            (it.next().unwrap().to_string(), it.next().unwrap().to_string(), it.next().unwrap().to_string())
        }).collect()
    };

    // Load the file into a Vec<u8>
    let mut f = File::open(filename).unwrap();
    let mut line_file = Vec::<u8>::new();
    f.read_to_end(&mut line_file).unwrap();

    // let max_size_file = 1024 * 1024; // 1mb, or 1024 kilobytes
    // assert!(max_size_file >= line_file.len());

    let page = if &line_file[0..33] == "reMarkable .lines file, version=3".as_bytes() {
        let content = &line_file[43..];
        lines_are_rusty::v3::read_page(&content)
    } else if &line_file[0..33] == "reMarkable .lines file, version=5".as_bytes() {
        println!("{:?}", &line_file[33..43]);
        let content = &line_file[43..];
        lines_are_rusty::v5::read_page(&content).expect("Failed to parse document")
    } else {
        println!("Unsupported line file format:");
        println!("{}", std::str::from_utf8(&line_file[0..33]).unwrap());
        return;
    };

    lines_are_rusty::render_svg(&format!("{}.svg", output_filename), &page, auto_crop, &layer_colors);
    // lines_are_rusty::render_pdf(&format!("{}.pdf", output_filename), &[page]);

    println!("done.");
}

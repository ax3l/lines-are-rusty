use clap::{App, Arg};
use lines_are_rusty::LinesData;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};

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
                .help("Output file name for rendered output. If not present, output is written to stdout.")
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
    let output_filename = matches.value_of("output");
    let auto_crop = !matches.is_present("no-auto-crop");
    let colors = matches.value_of("custom-colors").unwrap();

    let layer_colors = lines_are_rusty::LayerColors {
        colors: colors.split(";").map(|layer| {
            let mut it = layer.split(",");
            (it.next().unwrap().to_string(), it.next().unwrap().to_string(), it.next().unwrap().to_string())
        }).collect()
    };

    // let max_size_file = 1024 * 1024; // 1mb, or 1024 kilobytes
    // assert!(max_size_file >= line_file.len());

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let lines_data = match LinesData::parse(&mut BufReader::new(file)) {
        Ok(lines_data) => lines_data,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut output = BufWriter::new(match output_filename {
        Some(output_filename) => Box::new(File::create(output_filename).unwrap()),
        None => Box::new(io::stdout()) as Box<dyn Write>,
    });

    lines_are_rusty::render_svg(&mut output, &lines_data.pages[0], auto_crop, &layer_colors);
    // lines_are_rusty::render_pdf(&format!("{}.pdf", output_filename), &[page]);

    eprintln!("done.");
}

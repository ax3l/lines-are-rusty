// until rust 2018
extern crate byteorder;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};


#[derive(Default)]
struct Page {
    layers: Vec<Layer>,
}

#[derive(Default)]
struct Layer {
    lines: Vec<Line>,
}

#[derive(Default)]
struct Line {
    brush_type: i32,
    color: i32,
    unknown_line_attribute: i32,
    brush_base_size: f32,
    points: Vec<Point>,
}
impl Line {
    fn new(t: (i32, i32, i32, f32), pts: Vec<Point>) -> Line {
        Line{
            brush_type: t.0,
            color: t.1,
            unknown_line_attribute: t.2,
            brush_base_size: t.3,
            points: pts,
        }
    }
}

#[derive(Default)]
struct Point {
    x: f32,
    y: f32,
    speed: f32,
    direction: f32,
    width: f32,
    pressure: f32,
}
impl Point {
    fn new(f: (f32, f32, f32, f32, f32, f32)) -> Point {
        Point{
            x: f.0,
            y: f.1,
            speed: f.2,
            direction: f.3,
            width: f.4,
            pressure: f.5,
        }
    }
}


#[test]
fn test_read_number_i32() {
    let num = read_number_i32(&[42,0,0,0]);
    assert_eq!(42, num);
}
fn read_number_i32(bytes: &[u8]) -> i32 {
    let mut rdr = Cursor::new(&bytes[0..4]);
    // TODO implement if let Some(...)
    let number = rdr.read_i32::<LittleEndian>().unwrap();
    number
}

fn read_number_f32(bytes: &[u8]) -> f32 {
    let mut rdr = Cursor::new(&bytes[0..4]);
    // TODO implement if let Some(...)
    let number = rdr.read_f32::<LittleEndian>().unwrap();
    number
}

fn parse_line_header(four_bytes: & mut std::slice::Chunks<u8>) -> Option<(i32, i32, i32, f32)> {
    // let mut four_bytes = chunk.clone();
    if let Some(brush_type) = four_bytes.next() {
        if let Some(color) = four_bytes.next() {
            if let Some(unknown_line_attribute) = four_bytes.next() {
                if let Some(brush_base_size) = four_bytes.next() {
                    // TODO verify range of values
                    return Some((
                        read_number_i32(brush_type),
                        read_number_i32(color),
                        read_number_i32(unknown_line_attribute),
                        read_number_f32(brush_base_size)
                    ));
                }
            }
        }
    }
    None
}

fn parse_point_header(four_bytes: & mut std::slice::Chunks<u8>) -> Option<(f32, f32, f32, f32, f32, f32)> {
    // let mut four_bytes = chunk.clone();
    if let Some(x) = four_bytes.next() {
        if let Some(y) = four_bytes.next() {
            if let Some(speed) = four_bytes.next() {
                if let Some(direction) = four_bytes.next() {
                    if let Some(width) = four_bytes.next() {
                        if let Some(pressure) = four_bytes.next() {
                            // TODO verify range of values
                            return Some((
                                read_number_f32(x),
                                read_number_f32(y),
                                read_number_f32(speed),
                                read_number_f32(direction),
                                read_number_f32(width),
                                read_number_f32(pressure),
                            ));
                        }
                    }
                }
            }
        }
    }
    None
}

fn read_points(four_bytes: & mut std::slice::Chunks<u8>, _max_size_file: usize) -> Vec<Point> {
    let mut points = Vec::<Point>::default();
    // let mut four_bytes = chunk.clone();

    if let Some(num_points) = four_bytes.next() {
        for _pt in 0..read_number_i32(num_points) {
            println!("pt: {} / {}", _pt, read_number_i32(num_points));
            if let Some(tuple) = parse_point_header(four_bytes) {
                let new_point = Point::new(tuple);
                points.push(new_point);
            } else {
                break;
            }
        }
    }
    points
}

fn read_lines(four_bytes: & mut std::slice::Chunks<u8>, _max_size_file: usize) -> Vec<Line> {
    let mut lines = Vec::<Line>::default();
    // let mut four_bytes = chunk.clone();

    if let Some(num_lines) = four_bytes.next() {
        for _li in 0..read_number_i32(num_lines) {
            println!("li: {} / {}", _li, read_number_i32(num_lines));
            if let Some(tuple) = parse_line_header(four_bytes) {
                println!("new line!");
                let new_line = Line::new(
                    tuple,
                    read_points(four_bytes, _max_size_file));
                lines.push(new_line);
                println!("new line done!");
            } else {
                break;
            }
        }
    }
    lines
}

fn read_layers(four_bytes: & mut std::slice::Chunks<u8>, _max_size_file: usize) -> Vec<Layer> {
    let mut layers = Vec::<Layer>::default();
    // let mut four_bytes = chunk.clone();

    if let Some(num_layers) = four_bytes.next() {
        for _l in 0..read_number_i32(num_layers) {
            println!("l: {} / {}", _l, read_number_i32(num_layers));
            let new_layer = Layer{
                lines: read_lines(four_bytes, _max_size_file)};
            layers.push(new_layer);
        }
    }
    layers
}

// bytes: &[u8]
fn read_pages(four_bytes: & mut std::slice::Chunks<u8>, _max_size_file: usize) -> Vec<Page>
{
    let mut pages = Vec::<Page>::default();
    // let mut four_bytes = chunk.clone();
    
    let num_pages = 1;
    println!("p: 0 / {}", num_pages);
    let new_page = Page{
        layers: read_layers(four_bytes, _max_size_file)};
    pages.push(new_page);
    pages
}


fn main() {
    let max_size_file = 1024 * 1024; // Bytes
    let line_file = include_bytes!("../aa90b0e7-5c1a-42fe-930f-dad9cf3363cc/0.rm");
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
    let _pages = read_pages(& mut numbers, max_size_file);
    //} else {
    //    let pages = Vec::<Page>::default();
    //}

        // .map(|x|read_next(&x, & mut pc));
        // .collect::<Vec<_>>();
        
    // println!("{:?}", &numbers[..200]);
    

}

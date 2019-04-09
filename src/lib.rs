#![allow(dead_code)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::io::Read;

#[derive(Default)]
pub struct Page {
    layers: Vec<Layer>,
}

#[derive(Default)]
pub struct Layer {
    lines: Vec<Line>,
}

#[derive(Default)]
pub struct Line {
    brush_type: i32,
    color: i32,
    unknown_line_attribute: i32,
    brush_base_size: f32,
    points: Vec<Point>,
}

impl Line {
    pub fn new(t: (i32, i32, i32, f32), pts: Vec<Point>) -> Line {
        Line {
            brush_type: t.0,
            color: t.1,
            unknown_line_attribute: t.2,
            brush_base_size: t.3,
            points: pts,
        }
    }
}

#[derive(Default)]
pub struct Point {
    x: f32,
    y: f32,
    speed: f32,
    direction: f32,
    width: f32,
    pressure: f32,
}

impl Point {
    pub fn new(f: (f32, f32, f32, f32, f32, f32)) -> Point {
        Point {
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
    let num = read_number_i32(&[42, 0, 0, 0]);
    assert_eq!(42, num);
}

pub fn read_number_i32(bytes: &[u8]) -> i32 {
    let mut rdr = Cursor::new(&bytes[0..4]);
    // TODO implement if let Some(...)
    let number = rdr.read_i32::<LittleEndian>().unwrap();
    number
}

pub fn read_number_f32(bytes: &[u8]) -> f32 {
    let mut rdr = Cursor::new(&bytes[0..4]);
    // TODO implement if let Some(...)
    let number = rdr.read_f32::<LittleEndian>().unwrap();
    number
}

pub fn parse_line_header(cursor: &mut Cursor<&[u8]>) -> Option<(i32, i32, i32, f32)> {
    let mut brush_type = [0u8; 4];
    let mut color = [0u8; 4];
    let mut unknown_line_attribute = [0u8; 4];
    let mut brush_base_size = [0u8; 4];

    cursor.read_exact(&mut brush_type).ok()?;
    cursor.read_exact(&mut color).ok()?;
    cursor.read_exact(&mut unknown_line_attribute).ok()?;
    cursor.read_exact(&mut brush_base_size).ok()?;

    // TODO verify range of values
    return Some((
        read_number_i32(&brush_type),
        read_number_i32(&color),
        read_number_i32(&unknown_line_attribute),
        read_number_f32(&brush_base_size),
    ));
}

pub fn parse_point_header(cursor: &mut Cursor<&[u8]>) -> Option<(f32, f32, f32, f32, f32, f32)> {
    let mut x = [0u8; 4];
    let mut y = [0u8; 4];
    let mut speed = [0u8; 4];
    let mut direction = [0u8; 4];
    let mut width = [0u8; 4];
    let mut pressure = [0u8; 4];

    cursor.read_exact(&mut x).ok()?;
    cursor.read_exact(&mut y).ok()?;
    cursor.read_exact(&mut speed).ok()?;
    cursor.read_exact(&mut direction).ok()?;
    cursor.read_exact(&mut width).ok()?;
    cursor.read_exact(&mut pressure).ok()?;

    return Some((
        read_number_f32(&x),
        read_number_f32(&y),
        read_number_f32(&speed),
        read_number_f32(&direction),
        read_number_f32(&width),
        read_number_f32(&pressure),
    ));
}

pub fn read_points(cursor: &mut Cursor<&[u8]>, _max_size_file: usize) -> Vec<Point> {
    let mut points = Vec::<Point>::default();
    let mut num_points = [0u8; 4];
    if let Ok(()) = cursor.read_exact(&mut num_points) {
        for _pt in 0..read_number_i32(&num_points) {
            println!("pt: {} / {}", _pt, read_number_i32(&num_points));
            if let Some(tuple) = parse_point_header(cursor) {
                let new_point = Point::new(tuple);
                points.push(new_point);
            } else {
                break;
            }
        }
    }
    points
}

pub fn read_lines(cursor: &mut Cursor<&[u8]>, _max_size_file: usize) -> Vec<Line> {
    let mut lines = vec![];
    let mut num_lines = [0u8; 4];
    if let Ok(()) = cursor.read_exact(&mut num_lines) {
        for _li in 0..read_number_i32(&num_lines) {
            println!("li: {} / {}", _li, read_number_i32(&num_lines));
            if let Some(tuple) = parse_line_header(cursor) {
                println!("new line!");
                let new_line = Line::new(tuple, read_points(cursor, _max_size_file));
                lines.push(new_line);
                println!("new line done!");
            } else {
                break;
            }
        }
    }
    lines
}

pub fn read_layers(cursor: &mut Cursor<&[u8]>, _max_size_file: usize) -> Vec<Layer> {
    let mut layers = vec![];
    let mut num_layers = [0u8; 4];
    if let Ok(()) = cursor.read_exact(&mut num_layers) {
        for _l in 0..read_number_i32(&num_layers) {
            println!("l: {} / {}", _l, read_number_i32(&num_layers));
            let new_layer = Layer {
                lines: read_lines(cursor, _max_size_file),
            };
            layers.push(new_layer);
        }
    }
    layers
}

// bytes: &[u8]
pub fn read_pages(content: &[u8], _max_size_file: usize) -> Vec<Page> {
    let mut cursor = Cursor::new(content);

    let mut pages = vec![];
    let num_pages = 1;
    println!("p: 0 / {}", num_pages);
    let new_page = Page {
        layers: read_layers(&mut cursor, _max_size_file),
    };
    pages.push(new_page);
    pages
}

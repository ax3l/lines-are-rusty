use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

pub fn parse_line_header(cursor: &mut Cursor<&[u8]>) -> Option<(i32, i32, i32, f32)> {
    let brush_type = cursor.read_i32::<LittleEndian>().ok()?;
    let color = cursor.read_i32::<LittleEndian>().ok()?;
    let unknown_line_attribute = cursor.read_i32::<LittleEndian>().ok()?;
    let brush_base_size = cursor.read_f32::<LittleEndian>().ok()?;

    // TODO verify range of values
    Some((brush_type, color, unknown_line_attribute, brush_base_size))
}

pub fn parse_point_header(cursor: &mut Cursor<&[u8]>) -> Option<(f32, f32, f32, f32, f32, f32)> {
    let x = cursor.read_f32::<LittleEndian>().ok()?;
    let y = cursor.read_f32::<LittleEndian>().ok()?;
    let speed = cursor.read_f32::<LittleEndian>().ok()?;
    let direction = cursor.read_f32::<LittleEndian>().ok()?;
    let width = cursor.read_f32::<LittleEndian>().ok()?;
    let pressure = cursor.read_f32::<LittleEndian>().ok()?;

    Some((x, y, speed, direction, width, pressure))
}

pub fn read_points(cursor: &mut Cursor<&[u8]>) -> Vec<Point> {
    let mut points = Vec::<Point>::default();
    if let Ok(num_points) = cursor.read_i32::<LittleEndian>() {
        for _pt in 0..num_points {
            println!("pt: {} / {}", _pt, num_points);
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

pub fn read_lines(cursor: &mut Cursor<&[u8]>) -> Vec<Line> {
    let mut lines = vec![];
    if let Ok(num_lines) = cursor.read_i32::<LittleEndian>() {
        for _li in 0..num_lines {
            println!("li: {} / {}", _li, num_lines);
            if let Some(tuple) = parse_line_header(cursor) {
                println!("new line!");
                let new_line = Line::new(tuple, read_points(cursor));
                lines.push(new_line);
                println!("new line done!");
            } else {
                break;
            }
        }
    }
    lines
}

pub fn read_layers(cursor: &mut Cursor<&[u8]>) -> Vec<Layer> {
    let mut layers = vec![];
    if let Ok(num_layers) = cursor.read_i32::<LittleEndian>() {
        for _l in 0..num_layers {
            println!("l: {} / {}", _l, num_layers);
            let new_layer = Layer {
                lines: read_lines(cursor),
            };
            layers.push(new_layer);
        }
    }
    layers
}

pub fn read_page(content: &[u8]) -> Page {
    let mut cursor = Cursor::new(content);
    Page {
        layers: read_layers(&mut cursor),
    }
}

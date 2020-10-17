use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io;

pub fn parse_line(cursor: &mut io::Cursor<&[u8]>) -> Result<Line, io::Error> {
    let brush_type = BrushType::try_from(cursor.read_i32::<LittleEndian>()?)
        .expect("Failed to parse brush type");
    let color = Color::try_from(cursor.read_i32::<LittleEndian>()?).unwrap();
    let unknown_line_attribute = cursor.read_i32::<LittleEndian>()?;
    let brush_base_size = cursor.read_f32::<LittleEndian>()?; // width?
    let unknown_line_attribute_2 = cursor.read_i32::<LittleEndian>()?;
    let num_points = cursor.read_i32::<LittleEndian>()?;

    let mut points = Vec::new();
    for _pt in 0..num_points {
        let point = parse_point(cursor)?;
        points.push(point);
    }

    // TODO verify range of values
    Ok(Line {
        brush_type,
        color,
        unknown_line_attribute,
        unknown_line_attribute_2,
        brush_base_size,
        points,
    })
}

pub fn parse_point(cursor: &mut io::Cursor<&[u8]>) -> Result<Point, io::Error> {
    let x = cursor.read_f32::<LittleEndian>()?;
    let y = cursor.read_f32::<LittleEndian>()?;
    let speed = cursor.read_f32::<LittleEndian>()?;
    let direction = cursor.read_f32::<LittleEndian>()?;
    let width = cursor.read_f32::<LittleEndian>()?;
    let pressure = cursor.read_f32::<LittleEndian>()?;

    Ok(Point {
        x,
        y,
        speed,
        direction,
        width,
        pressure,
    })
}

pub fn read_lines(cursor: &mut io::Cursor<&[u8]>) -> Result<Vec<Line>, io::Error> {
    let mut lines = vec![];
    let num_lines = cursor.read_i32::<LittleEndian>()?;
    for _li in 0..num_lines {
        lines.push(parse_line(cursor).expect("Failed to parse line"));
    }
    Ok(lines)
}

pub fn read_layers(cursor: &mut io::Cursor<&[u8]>) -> Result<Vec<Layer>, io::Error> {
    let mut layers = vec![];
    let num_layers = cursor.read_i32::<LittleEndian>()?;
    for _l in 0..num_layers {
        let lines = read_lines(cursor)?;
        layers.push(Layer { lines });
    }
    Ok(layers)
}

pub fn read_page(content: &[u8]) -> Result<Page, io::Error> {
    let mut cursor = io::Cursor::new(content);
    Ok(Page {
        layers: read_layers(&mut cursor)?,
    })
}

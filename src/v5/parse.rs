use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io::{self, Read};

type Endian = LittleEndian;

// TODO: I think we can get rid of this, I remember seeing something in
// the std lib that will do these conversions for us.
pub fn read_number_i32(bytes: &[u8]) -> i32 {
    let mut rdr = io::Cursor::new(&bytes[0..4]);
    // TODO implement if let Some(...)
    let number = rdr.read_i32::<LittleEndian>().unwrap();
    number
}

pub fn read_number_f32(bytes: &[u8]) -> f32 {
    let mut rdr = io::Cursor::new(&bytes[0..4]);
    // TODO implement if let Some(...)
    let number = rdr.read_f32::<LittleEndian>().unwrap();
    number
}

pub fn parse_line(cursor: &mut io::Cursor<&[u8]>) -> Result<Line, io::Error> {
    let brush_type =
        BrushType::try_from(cursor.read_i32::<Endian>()?).expect("Failed to parse brush type");
    let color = Color::try_from(cursor.read_i32::<Endian>()?).unwrap();
    let unknown_line_attribute = cursor.read_i32::<Endian>()?;
    let brush_base_size = cursor.read_f32::<Endian>()?; // width?
    let unknown_line_attribute_2 = cursor.read_i32::<Endian>()?;
    let num_points = cursor.read_i32::<Endian>()?;

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
        brush_base_size,
        points,
    })
}

pub fn parse_point(cursor: &mut io::Cursor<&[u8]>) -> Result<Point, io::Error> {
    let mut x = [0u8; 4];
    let mut y = [0u8; 4];
    let mut speed = [0u8; 4];
    let mut direction = [0u8; 4];
    let mut width = [0u8; 4];
    let mut pressure = [0u8; 4];

    let x = cursor.read_f32::<Endian>()?;
    let y = cursor.read_f32::<Endian>()?;
    let speed = cursor.read_f32::<Endian>()?;
    let direction = cursor.read_f32::<Endian>()?;
    let width = cursor.read_f32::<Endian>()?;
    let pressure = cursor.read_f32::<Endian>()?;

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
    let num_lines = cursor.read_i32::<Endian>()?;
    for _li in 0..num_lines {
        lines.push(parse_line(cursor).expect("Failed to parse line"));
    }
    Ok(lines)
}

pub fn read_layers(cursor: &mut io::Cursor<&[u8]>) -> Result<Vec<Layer>, io::Error> {
    let mut layers = vec![];
    let mut num_layers = [0u8; 4];
    cursor.read_exact(&mut num_layers)?;
    for _l in 0..read_number_i32(&num_layers) {
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

#[test]
fn test_read_number_i32() {
    let num = read_number_i32(&[42, 0, 0, 0]);
    assert_eq!(42, num);
}

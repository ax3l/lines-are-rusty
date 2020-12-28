use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::error;
use std::io;

impl LinesData {
    /// Parses data from an .rm or .lines file to `LinesData`.
    /// Possible errors are `io::Error` and `VersionError`,
    /// Currently, only .rm files of version 3 and 5 are supported.
    pub fn parse(file: &mut dyn io::Read) -> Result<LinesData, Box<dyn error::Error>> {
        let mut buffer = [0; 33];
        file.read_exact(&mut buffer)?;
        let untrimmed_string = String::from_utf8_lossy(&buffer);
        let version_string = untrimmed_string.trim_end();
        let version = match version_string {
            "reMarkable lines with selections and layers" => {
                // early version of the format that is not supported
                return Err(VersionError::boxed(version_string));
            }
            "reMarkable .lines file, version=3" => 3,
            "reMarkable .lines file, version=5" => 5,
            _ => return Err(VersionError::boxed(version_string)),
        };

        if version >= 3 {
            // Newer files have 10 more bytes in the ASCII header. Skip them.
            file.read_exact(&mut [0; 10])?;
        }

        let mut reader = LinesDataReader {
            file: file,
            version: version,
        };

        Ok(LinesData {
            version: version,
            pages: reader.read_pages()?,
        })
    }
}

pub(crate) struct LinesDataReader<'a> {
    pub file: &'a mut dyn io::Read,
    pub version: i32,
}

impl LinesDataReader<'_> {
    fn read_i32(&mut self) -> Result<i32, io::Error> {
        self.file.read_i32::<LittleEndian>()
    }

    fn read_f32(&mut self) -> Result<f32, io::Error> {
        self.file.read_f32::<LittleEndian>()
    }

    pub fn parse_line(&mut self) -> Result<Line, io::Error> {
        let brush_type = BrushType::try_from(self.read_i32()?)
            .expect("Failed to parse brush type");
        let color = Color::try_from(self.read_i32()?).unwrap();
        let unknown_line_attribute = self.read_i32()?;
        let brush_base_size = self.read_f32()?; // width?
        let unknown_line_attribute_2 = if self.version >= 5 {
            self.read_i32()?
        } else {
            0
        };
        let num_points = self.read_i32()?;

        let mut points = Vec::new();
        for _pt in 0..num_points {
            let point = self.parse_point()?;
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

    pub fn parse_point(&mut self) -> Result<Point, io::Error> {
        let x = self.read_f32()?;
        let y = self.read_f32()?;
        let speed = self.read_f32()?;
        let direction = self.read_f32()?;
        let width = self.read_f32()?;
        let pressure = self.read_f32()?;

        Ok(Point {
            x,
            y,
            speed,
            direction,
            width,
            pressure,
        })
    }

    pub fn read_lines(&mut self) -> Result<Vec<Line>, io::Error> {
        let mut lines = vec![];
        let num_lines = self.read_i32()?;
        for _li in 0..num_lines {
            lines.push(self.parse_line()?);
        }
        Ok(lines)
    }

    pub fn read_layers(&mut self) -> Result<Vec<Layer>, io::Error> {
        let mut layers = vec![];
        let num_layers = self.read_i32()?;
        for _l in 0..num_layers {
            let lines = self.read_lines()?;
            layers.push(Layer { lines });
        }
        Ok(layers)
    }

    pub fn read_pages(&mut self) -> Result<Vec<Page>, Box<dyn error::Error>> {
        // From version 3(?) on, only a single page is stored per file.
        // The number of pages is not stored in the lines file any more.
        let num_pages = if self.version >= 3 {
            1
        } else {
            self.read_i32()?
        };
        (0..num_pages)
            .map(|_| {
                Ok(Page {
                    layers: self.read_layers()?,
                })
            })
            .collect()
    }
}
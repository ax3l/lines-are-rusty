use crate::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use std::io;

impl LinesData {
    /// Parses data from an .rm or .lines file to `LinesData`.
    /// Possible errors are `io::Error` and `VersionError`,
    /// Currently, only .rm files of version 3 and 5 are supported.
    pub fn parse(file: &mut dyn io::Read) -> Result<LinesData, LinesError> {
        let mut buffer = [0; 33];
        file.read_exact(&mut buffer)?;
        let untrimmed_string = String::from_utf8_lossy(&buffer);
        let version_string = untrimmed_string.trim_end();
        let version = match version_string {
            "reMarkable lines with selections and layers" => {
                // early version of the format that is not supported
                return Err(LinesError::UnsupportedVersion(version_string.to_string()));
            }
            "reMarkable .lines file, version=3" => 3,
            "reMarkable .lines file, version=5" => 5,
            _ => return Err(LinesError::UnsupportedVersion(version_string.to_string())),
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

    fn read_line(&mut self) -> Result<Line, LinesError> {
        Ok(Line {
            brush_type: BrushType::try_from(self.read_i32()?)?,
            color: Color::try_from(self.read_i32()?)?,
            unknown_line_attribute: self.read_i32()?,
            brush_base_size: self.read_f32()?, // width
            unknown_line_attribute_2: if self.version >= 5 {
                self.read_i32()?
            } else {
                0
            },
            points: self.read_points()?,
        })
    }

    fn read_points(&mut self) -> Result<Vec<Point>, io::Error> {
        let num_points = self.read_i32()?;
        (0..num_points).map(|_| self.read_point()).collect()
    }

    fn read_point(&mut self) -> Result<Point, io::Error> {
        Ok(Point {
            x: self.read_f32()?,
            y: self.read_f32()?,
            speed: self.read_f32()?,
            direction: self.read_f32()?,
            width: self.read_f32()?,
            pressure: self.read_f32()?,
        })
    }

    fn read_lines(&mut self) -> Result<Vec<Line>, LinesError> {
        let num_lines = self.read_i32()?;
        (0..num_lines).map(|_| self.read_line()).collect()
    }

    fn read_layers(&mut self) -> Result<Vec<Layer>, LinesError> {
        let num_layers = self.read_i32()?;
        (0..num_layers)
            .map(|_| {
                Ok(Layer {
                    lines: self.read_lines()?,
                })
            })
            .collect()
    }

    pub fn read_pages(&mut self) -> Result<Vec<Page>, LinesError> {
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

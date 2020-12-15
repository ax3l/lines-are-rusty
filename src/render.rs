use crate::*;
use pdf_canvas::graphicsstate::{self, CapStyle, JoinStyle, Matrix};
use pdf_canvas::Pdf;

const BASE_LINE_WIDTH: f32 = 4.;

pub fn line_to_svg_color(line: &Line, layer_id: usize) -> &'static str {
    match line.brush_type {
        BrushType::Highlighter => "rgb(240, 220, 40)",
        _ => match layer_id {
            1 => match line.color {
                Color::Black => "red",
                Color::Grey => "magenta",
                Color::White => "white",
            },
            2 => match line.color {
                Color::Black => "blue",
                Color::Grey => "cyan",
                Color::White => "white",
            },
            3 => match line.color {
                Color::Black => "limegreen",
                Color::Grey => "yellow",
                Color::White => "white",
            },
            4 => match line.color {
                Color::Black => "darkorchid",
                Color::Grey => "darkorange",
                Color::White => "white",
            },
            _ => match line.color {
                Color::Black => "black",
                Color::Grey => "grey",
                Color::White => "white",
            },
        },
    }
}

pub fn render_highlighter_line(line: &Line, min_x: f32, min_y: f32, layer_id: usize) -> svg::node::element::Path {
    let first_point = &line.points[0];

    let mut data = svg::node::element::path::Data::new().move_to((first_point.x-min_x, first_point.y-min_y));
    for point in line.points.iter() {
        data = data.line_to((point.x-min_x, point.y-min_y));
    }

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", line_to_svg_color(line, layer_id))
        .set("stroke-width", first_point.width * 0.8)
        .set("stroke-linecap", "round")
        .set("stroke-opacity", 0.25)
        .set("d", data)
}

pub fn render_fineliner_line(line: &Line, min_x: f32, min_y: f32, layer_id: usize) -> svg::node::element::Path {
    let first_point = &line.points[0];

    let mut data = svg::node::element::path::Data::new().move_to((first_point.x-min_x, first_point.y-min_y));
    for point in line.points.iter() {
        data = data.line_to((point.x-min_x, point.y-min_y));
    }

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", line_to_svg_color(line, layer_id))
        .set("stroke-width", first_point.width * 0.8)
        .set("stroke-linecap", "round")
        .set("d", data)
}

pub fn crop(page: &Page) -> (f32, f32, f32, f32) {
    let mut min_x = 1404_f32;
    let mut min_y = 1872_f32;
    let mut max_x = 0_f32;
    let mut max_y = 0_f32;
    for layer in page.layers.iter() {
        for line in layer.lines.iter() {
            for point in line.points.iter() {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            }
        }
    }
    (min_x, min_y, max_x, max_y)
}

pub fn render_svg(path: &str, page: &Page, auto_crop: bool) {
    let (min_x, min_y, max_x, max_y) = if auto_crop {
        crop(page)
    } else {
        (0_f32, 0_f32, 1404_f32, 1872_f32)
    };
    let mut doc = svg::Document::new();
    for (layer_id, layer) in page.layers.iter().enumerate() {
        for line in layer.lines.iter() {
            if line.points.is_empty() {
                continue;
            }
            let color = line_to_svg_color(&line, layer_id);
            match line.brush_type {
                BrushType::Highlighter => doc = doc.add(render_highlighter_line(line, min_x, min_y, layer_id)),
                BrushType::Fineliner => doc = doc.add(render_fineliner_line(line, min_x, min_y, layer_id)),
                BrushType::EraseArea => (),
                BrushType::Eraser => (),
                _ => {
                    let mut prev_point = &line.points[0];
                    for point in line.points.iter() {
                        let data = svg::node::element::path::Data::new()
                            .move_to((prev_point.x - min_x, prev_point.y - min_y))
                            .line_to((point.x - min_x, point.y - min_y));
                        let (width, opacity) = match line.brush_type {
                            BrushType::BallPoint => (point.width, point.pressure.powf(5.0) + 0.7),
                            BrushType::Marker => (point.width, 1.0),
                            BrushType::Fineliner => panic!("Should have been handled above"),
                            BrushType::SharpPencil => (point.width, 1.0),
                            BrushType::TiltPencil => (point.width, 1.0),
                            BrushType::Brush => (point.width, 1.0),
                            BrushType::Calligraphy => (point.width, 1.0),
                            BrushType::Highlighter => panic!("Should have been handled above"),
                            BrushType::Eraser => panic!("Should have been handled above"),
                            BrushType::EraseArea => panic!("Should have been handled above"),
                        };

                        if opacity != 1.0 {
                            doc = doc.add(
                                svg::node::element::Path::new()
                                    .set("fill", "none")
                                    .set("stroke", color)
                                    .set("stroke-width", width * 0.8)
                                    .set("stroke-linecap", "round")
                                    .set("stroke-opacity", opacity)
                                    .set("d", data),
                            );
                        } else {
                            doc = doc.add(
                                svg::node::element::Path::new()
                                    .set("fill", "none")
                                    .set("stroke", color)
                                    .set("stroke-width", width * 0.8)
                                    .set("stroke-linecap", "round")
                                    .set("d", data),
                            );
                        }
                        prev_point = point;
                    }
                }
            }
        }
    }
    let doc = doc.set("viewBox", (0, 0, max_x-min_x, max_y-min_y));
    svg::save(path, &doc).expect("Failed to save svg doc");
}

/// Create a `mandala.pdf` file.
pub fn render_pdf(path: &str, pages: &[Page]) {
    // Open our pdf document.
    let mut document = Pdf::create(path).expect("Create PDF file");

    // Only one page to consider.
    let page = &pages[0];

    // Render a page with something resembling a mandala on it.
    document
        .render_page(1404.0, 1872.0, |c| {
            // Inverse Y coordinate system.
            c.concat(Matrix::scale(1., -1.))?;
            c.concat(Matrix::translate(0., -1872.))?;

            c.set_stroke_color(graphicsstate::Color::gray(0))?;

            for layer in &page.layers {
                for line in &layer.lines {
                    if line.points.is_empty() {
                        continue;
                    }
                    let first_point = &line.points[0];
                    c.move_to(first_point.x, first_point.y)?;
                    for point in &line.points {
                        c.set_line_width(point.pressure * BASE_LINE_WIDTH)?;
                        c.set_line_cap_style(CapStyle::Round)?;
                        c.set_line_join_style(JoinStyle::Round)?;
                        c.line_to(point.x, point.y)?;
                    }
                    c.stroke()?;
                }
            }

            Ok(())
        })
        .unwrap();
    document.finish().unwrap();
}

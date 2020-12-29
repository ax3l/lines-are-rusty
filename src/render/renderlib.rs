use crate::Page;

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

use tui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

use crate::puget_sound::{PUGET_SOUND_HIGH_RESOLUTION, PUGET_SOUND_LOW_RESOLUTION};

#[derive(Debug, Clone, Copy)]
pub enum MapResolution {
    Low,
    High,
}

impl MapResolution {
    fn data(self) -> &'static [(f64, f64)] {
        match self {
            MapResolution::Low => &PUGET_SOUND_LOW_RESOLUTION,
            MapResolution::High => &PUGET_SOUND_HIGH_RESOLUTION,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
#[derive(Debug, Clone)]
pub struct Map {
    pub resolution: MapResolution,
    pub color: Color,
}

impl Default for Map {
    fn default() -> Map {
        Map {
            resolution: MapResolution::Low,
            color: Color::Reset,
        }
    }
}

// from Painter
// fn get_point(x_bounds: [f64; 2], y_bounds: [f64; 2], resolution: (f64, f64), x: f64, y: f64) -> Option<(usize, usize)> {
/// Convert the (x, y) coordinates to location of a point on the grid
///
/// # Examples:
/// ```
/// use tui::{symbols, widgets::canvas::{Painter, Context}};
///
/// let mut ctx = Context::new(2, 2, [1.0, 2.0], [0.0, 2.0], symbols::Marker::Braille);
/// let mut painter = Painter::from(&mut ctx);
/// let point = painter.get_point(1.0, 0.0);
/// assert_eq!(point, Some((0, 7)));
/// let point = painter.get_point(1.5, 1.0);
/// assert_eq!(point, Some((1, 3)));
/// let point = painter.get_point(0.0, 0.0);
/// assert_eq!(point, None);
/// let point = painter.get_point(2.0, 2.0);
/// assert_eq!(point, Some((3, 0)));
/// let point = painter.get_point(1.0, 2.0);
/// assert_eq!(point, Some((0, 0)));
/// ```
fn get_point(x: f64, y: f64) -> Option<(usize, usize, Color)> {
    let x = x.abs();
    let y = y.abs();
    let x_bounds: [f64; 2] = [122.0, 123.0];
    let y_bounds: [f64; 2] = [47.0, 48.0];

    // "zoom" factor
    let resolution: (f64, f64) =  (
        // f64::from(x_bounds[1]) * 2.0 - 1.0,
        // f64::from(y_bounds[1]) * 4.0 - 1.0,
        f64::from(180) * 2.9   - 1.0,
        f64::from(180) * 2.9  - 1.0,
        // 359.0/2.0, 719.0/4.0
        // 180.0 / 16.0,360.0/32.0
        // f64::from(x_bounds[1]/2.0) - 1.0, f64::from(y_bounds[1]) / 2.0 - 1.0
    );
    let left = x_bounds[0];
    let right = x_bounds[1];
    let top = y_bounds[1];
    let bottom = y_bounds[0];
    if x < left || x > right || y < bottom || y > top {
        println!("x{x:} < l{left} | x{x} > r{right} | y{y} < b{bottom} | y{y} > t{top} ");
        panic!("ayy");
        // return None;
    }
    let width = (x_bounds[1] - x_bounds[0]).abs();
    let height = (y_bounds[1] - y_bounds[0]).abs();
    if width == 0.0 || height == 0.0 {
        println!("{width:} w or h zero {height}");
        return None;
    }

    // println!("{:} - {:}= {width:} w or h zero {height}", x_bounds[0], x_bounds[1]);

    let color = match (x,y) {
        (122.33,47.60) => {
            // println!("Seattle!");
            Color::Red
        },
        (122.62,47.56) => {
            // println!("Bremerton!");
            Color::Blue
        },
        (122.62,_) => {
            // println!("Bremerton!");
            Color::Green
        },
        _ => {
            // println!("{y} Not Seattle!");
            Color::White
        }
    };
    
    // print!("from {x},{y} => ");
    // let x = ((x - left) * resolution.0 / width) as usize;
    // let y = ((top - y) * resolution.1 / height) as usize;
    let x = ((x - left) * resolution.0 ) as usize;
    let y = ((top - y) * resolution.1 ) as usize;
    // println!("to {x},{y}");

    // location within grid
    Some((x+90, y-125, color))

    // println!("resolution {:}, {:}", resolution.0, resolution.1);

    // Some((x as usize, y as usize))
}

impl Shape for Map {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.resolution.data() {
            if let Some((x, y, color)) = get_point(*x, *y) {
                // println!("x {}, y {}", x, y);
                // return
                // painter.paint(122, 47, self.color);
                // painter.paint(y,x, self.color);
                // painter.paint(x,y, self.color);
                painter.paint(x,y, color);
            }
        }
        // painter.paint(0, 1, self.color);
        // painter.paint(0, 180, self.color);
        // painter.paint(320, 180, Color::Cyan);
        // painter.paint(320, 0, Color::Red);
    }
}

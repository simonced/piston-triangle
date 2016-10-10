extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

/// points used to build the triangle and plot points on the canvas
pub struct Point {
    x: u32,
    y: u32,
}

/// width of the png image
pub const WIDTH:  u32 = 800;
/// height of the png image
pub const HEIGHT: u32 = 600;

// Main entry point
fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [WIDTH, HEIGHT])
        .exit_on_esc(true).build().unwrap();
    println!("press ESC to quit");

    // default dot color
    let dots_color = [0.0, 1.0, 0.0, 1.0];

    // draw loop
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            //clear([1.0; 4], g);
            let point = Point {
                x: rand::thread_rng().gen_range(0, WIDTH),
                y: rand::thread_rng().gen_range(0, HEIGHT)
            };
            draw_dot(point, dots_color, c.transform, g);
        });
    }
}

pub fn draw_dot <G> (
    p: Point,
    color: types::Color,
    transform: math::Matrix2d,
    g: &mut G
)
    where G: Graphics
{
    // temporary setting:
    let size = 1.0;

    rectangle(
        color, // Color
        [p.x as f64, p.y as f64, size, size], // line points?
        transform,
        g
    )
}

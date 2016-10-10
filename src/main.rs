extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

/// points used to build the triangle and plot points on the canvas
/// the following derive thing was found: https://doc.rust-lang.org/std/marker/trait.Copy.html
#[derive(Debug, Copy, Clone)]
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

    // triangle points
    let pts: [Point; 3] = [
        Point {x: WIDTH / 2, y: 0},
        Point {x: 0, y: HEIGHT},
        Point {x: WIDTH, y: HEIGHT},
    ];

    // no clue yet of why this point has to be random in the canvas
    let mut point = Point {
        x: rand::thread_rng().gen_range(0, WIDTH),
        y: rand::thread_rng().gen_range(0, HEIGHT)
    };

    // loops count
    let mut loops: u32 = 10_000;

    // draw loop
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            //clear([1.0; 4], g);
            if loops > 0 {
                loops -= 1;
                let num = rand::thread_rng().gen_range(0,3);
                point.x = (point.x + pts[num].x) / 2;
                point.y = (point.y + pts[num].y) / 2;
                draw_dot(point, dots_color, c.transform, g);

                println!("Loops remaining: {}", loops);
            }
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

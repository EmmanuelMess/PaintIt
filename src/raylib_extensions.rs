use raylib::math::{Rectangle, Vector2};



/// Generate a rectangle based on a pair of points, given in any order
pub fn generate_rectangle(p0: Vector2, p1: Vector2) -> Rectangle {
    let start = if p0.x <= p1.x && p0.y <= p1.y { p0 } else { p1 };
    let end = if p0.x <= p1.x && p0.y <= p1.y { p1 } else { p0 };

    let size = end - start;

    Rectangle {
        x: start.x,
        y: start.y,
        width: size.x.abs(),
        height: size.y.abs(),
    }
}
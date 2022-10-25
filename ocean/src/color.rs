#[derive(Eq, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn new_red() -> Color {
        Color::new(255, 0, 0)
    }

    pub fn new_green() -> Color {
        Color::new(0, 255, 0)
    }

    pub fn new_blue() -> Color {
        Color::new(0, 0, 255)
    }

    /**
     * Returns a new `Color` whose components are the sum of `c1` and `c2`'s components, modulo 256.
     *
     * First, try writing this function the "obvious" way with arithmetic operations. The test for
     * this method (which you can run with `cargo test part1_color` will fail) with a panic.
     *
     * Note which line of the test is causing the panic: why not the other?
     *
     * Then, look through the documentation for `u8` and see if there is a method that will help you.
     * https://doc.rust-lang.org/std/primitive.u8.html
     */
    pub fn cross(c1: &Color, c2: &Color) -> Color {
        let r = ((c1.r as u32)+(c2.r as u32))%256;
        let r = r as u8;
        let g = ((c1.g as u32)+(c2.g as u32))%256;
        let g = g as u8;
        let b = ((c1.b as u32)+(c2.b as u32))%256;
        let b = b as u8;
        Color::new(r,g,b)
    }
}

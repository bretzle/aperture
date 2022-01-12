pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub const WHITE: Self = Color::broadcast(1.0);
    pub const BLACK: Self = Color::broadcast(0.0);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn broadcast(r: f32) -> Self {
        Self {
            r,
            g: r,
            b: r,
            a: r,
        }
    }
}

pub use mitchell_netravali::MitchellNetravali;

mod mitchell_netravali;

pub trait Filter {
    fn weight(&self, x: f32, y: f32) -> f32;
    fn width(&self) -> f32;
    fn inv_width(&self) -> f32;
    fn height(&self) -> f32;
    fn inv_height(&self) -> f32;
}

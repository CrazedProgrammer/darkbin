use util::Vec2;

#[derive(Clone, PartialEq)]
pub struct Viewport {
    pub window_size: Vec2,
    pub position: Vec2,
    pub zoom: f32,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            window_size: Vec2::zero(),
            position: Vec2::zero(),
            zoom: 1f32,
        }
    }
}

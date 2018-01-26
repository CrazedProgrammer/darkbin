use util::Vec2;

#[derive(Clone, PartialEq)]
pub struct Viewport {
    pub position: Vec2,
    pub zoom: f32,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            position: Vec2::new(0f32, 0f32),
            zoom: 1f32,
        }
    }
}

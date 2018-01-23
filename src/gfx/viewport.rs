#[derive(Clone)]
pub struct Viewport {
    pub position: (f32, f32),
    pub zoom: f32,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            position: (0f32, 0f32),
            zoom: 1f32,
        }
    }
}

use game::entity::{Entity, EntityShape, EntityType};
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use gfx::assets::Asset;
use util::Vec2;
use vek::ops::Clamp;

#[derive(Clone)]
pub struct Camera {
    target_pos: Vec2,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            target_pos: Vec2::zero(),
        }
    }
}

impl Entity for Camera {
    fn do_action(&mut self, action: &EntityAction, state: &GameState, shape: &mut EntityShape) -> Vec<Event> {
        let mut actions = vec![];
        match action {
            &EntityAction::Init => {
                shape.entity_type = EntityType::Camera;
                shape.texture = Asset::Empty;
            },
            &EntityAction::Update(d_time) => {
                let zoom_step = 0.5f32;
                let catchup_speed = 40f32;
                let mouse_adjust = 0.5f32;

                let mut viewport = state.viewport.clone();
                viewport.zoom += state.input.mouse_wheel as f32 * zoom_step;
                if viewport.zoom <= 0f32 {
                    viewport.zoom = 0.01f32;
                }
                let catchup_ratio = (d_time * catchup_speed).clamped(0f32, 1f32);

                match state.search_first_type(EntityType::Player) {
                    Some(id) => {
                        let player_shape = state.shapes.get(&id).unwrap();
                        self.target_pos = player_shape.position + (state.input.mouse_pos - state.input.window_size / 2f32) * mouse_adjust / state.viewport.zoom;
                    },
                    None => { },
                }

                shape.position = shape.position * (1f32 - catchup_ratio) + self.target_pos * catchup_ratio;
                viewport.position = shape.position;
                if state.viewport != viewport {
                    actions.push(Event::new(0f32, Action::ChangeViewport(viewport)));
                }
            },
        }
        actions
    }
}

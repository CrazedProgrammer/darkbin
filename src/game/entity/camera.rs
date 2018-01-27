use game::entity::{Entity, EntityShape, EntityType};
use game::event::{Event, EntityAction, Action};
use game::state::GameState;
use gfx::assets::Asset;

#[derive(Clone)]
pub struct Camera {
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
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
            &EntityAction::Update(_d_time) => {
                let mut viewport = state.viewport.clone();
                viewport.zoom += state.input.mouse_wheel as f32 * 0.5f32;
                if viewport.zoom <= 0f32 {
                    viewport.zoom = 0.01f32;
                }

                match state.search_first_type(EntityType::Player) {
                    Some(id) => {
                        let player_shape = state.shapes.get(&id).unwrap();
                        shape.position = player_shape.position;
                        viewport.position = shape.position;
                    },
                    None => { },
                }

                if state.viewport != viewport {
                    actions.push(Event::new(0f32, Action::ChangeViewport(viewport)));
                }
            },
        }
        actions
    }
}

use game::entity::Entity;
use std::rc::Rc;
use gfx::viewport::Viewport;

#[derive(Clone)]
pub struct Event {
    pub time: f32,
    pub action: Action,
}

impl Event {
    pub fn new(time: f32, action: Action) -> Event {
        Event {
            time: time,
            action: action,
        }
    }
}

#[derive(Clone)]
pub enum Action {
    AddEntity(Rc<Entity>),
    DoEntity(u64, EntityAction),
    RemoveEntity(u64),
    ChangeViewport(Viewport),
}

#[derive(Clone)]
pub enum EntityAction {
    Init,
    Update(f32),
}

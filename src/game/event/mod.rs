use game::entity::Entity;
use std::rc::Rc;

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
    QuitGame,
    AddEntity(Rc<Entity>),
    RemoveEntity(u64),
    DoEntity(u64, EntityAction),
}

#[derive(Clone)]
pub enum EntityAction {
    Update(f32),
}

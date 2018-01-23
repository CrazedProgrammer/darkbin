pub struct Event {
    time: f32,
    action: EventAction
}

pub enum EventAction {
    QuitGame,
    RemoveEntity(u64),
}

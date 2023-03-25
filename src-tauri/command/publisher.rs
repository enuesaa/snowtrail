// internal. hard coding
struct EventPublisher {
    name: String,
    description: String,
}

pub struct EventMeta {
    name: String,
    value: String,
}
pub struct Event {
    name: String,
    publisher: EventPublisher,
    meta: Vec<EventMeta>,
}
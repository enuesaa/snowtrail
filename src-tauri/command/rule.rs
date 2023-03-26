struct EventTemplate {
    name: String,
    status: EventStatus,
    runtime: EventRuntime,
    value: Vec<EventValue>,
    meta: Vec<EventMeta>,
}
struct PubsubRule {
    method: String, // like eq
    path: String,
    value: String,
}
struct Pubsub {
    name: String,
    description: String,
    rule: Vec<PubsubRule>,
    trigger: EventTemplate,
}

enum EventStatus {
    processing,
    end,
    exception,
}
enum EventRuntime {
    shell,
    createfile,
    python3,
    wasm,
}
pub struct EventValue {
    name: String,
    value: String,
}
pub struct EventMeta {
    name: String,
    value: String,
}
pub struct Event {
    name: String,
    status: EventStatus,
    runtime: EventRuntime,
    value: Vec<EventValue>, // like Note { name, dscription, project, save path }
    meta: Vec<EventMeta>,
}

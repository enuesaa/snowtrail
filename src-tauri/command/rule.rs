struct Transformer {
    script: EventScript, // this is also events, only tsnode
    value: Vec<EventValue>,
    meta: Vec<EventMeta>,
}
struct Subscribe {
    name: String,
    description: String,
    rule: Vec<String>, // can subscribe snowtrail event
    transformer: Transformer,
}

enum EventStatus {
    process,
    success,
    error,
}
enum EventRuntime {
    shell,
    tsnode,
}
struct EventScript {
    runtime: EventRuntime,
    script: String, // createfile
}
pub struct EventValue {
    name: String,
    value: String,
}
pub struct Event {
    name: String,
    status: EventStatus,
    script: EventScript, // 場合によっては配列に
    value: Vec<EventValue>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}

// snowtrail event
// subscribe
// transform
// publish event
// subscribe ...

struct Transformer {
    runtime: EventRuntime, // this is also events, only ts node
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
// docker で.. volume を共有
enum EventRuntime {
    shell,
    tsnode,
    createfile,
    // rust,
    // go,
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


// snowtrail event
// subscribe
// transform
// publish event
// subscribe ...

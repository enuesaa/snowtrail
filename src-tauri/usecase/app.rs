use crate::repository::rocks::RocksRepository;
use crate::repository::runcommand::RuncommandRepository;
use crate::service::project::{Project, ProjectService};
use crate::service::script::{Script, ScriptService};
use crate::service::subscribe::{Subscribe, SubscribeService};
use crate::service::event::{Event, EventService};

pub struct AppUsecase {
    rocks: RocksRepository,
    runcommand: RuncommandRepository,
}
impl AppUsecase {
    pub fn new() -> Self {
        AppUsecase {
            rocks: RocksRepository::new(),
            runcommand: RuncommandRepository::new(),
        }
    }

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    fn runcommand(&self) -> RuncommandRepository {
        self.runcommand.clone()
    }

    pub fn list_projects(&self) -> Vec<Project> {
        ProjectService::new(self.rocks()).list()
    }
    
    pub fn get_project(&self, id: &str) -> Project {
        ProjectService::new(self.rocks()).get(id)
    }
    
    pub fn create_project(&self, project: Project) {
        ProjectService::new(self.rocks()).create(project)
    }

    pub fn delete_project(&self, id: &str) {
        ProjectService::new(self.rocks()).delete(id)
    }

    pub fn list_scripts(&self, project_name: String) -> Vec<Script> {
        ScriptService::new(self.rocks(), self.runcommand()).list_in_project(project_name)
    }
    
    pub fn get_script(&self, id: &str) -> Script {
        ScriptService::new(self.rocks(), self.runcommand()).get(id)
    }
    
    pub fn create_script(&self, script: Script) {
        ScriptService::new(self.rocks(), self.runcommand()).create(script)
    }

    pub fn delete_script(&self, id: &str) {
        let script = ScriptService::new(self.rocks(), self.runcommand()).get(id);
        ScriptService::new(self.rocks(), self.runcommand()).delete(script)
    }

    pub fn run_script(&self, id: &str) {
        ScriptService::new(self.rocks(), self.runcommand()).run(id);
        let event = Event::new("snowtrail:command:run");
        EventService::new(self.rocks()).create(event);
    }

    pub fn list_events(&self) -> Vec<Event> {
        EventService::new(self.rocks()).list()
    }

    pub fn get_event(&self, id: &str) -> Event {
        EventService::new(self.rocks()).get(id)
    }

    pub fn create_event(&self, event: Event) -> String {
        // trigger event
        let id = EventService::new(self.rocks()).create(event);
        // publish
        // trigger event
        id
    }

    pub fn delete_event(&self, id: &str) {
        EventService::new(self.rocks()).delete(id)
    }

    pub fn list_subscribes(&self) -> Vec<Subscribe> {
        SubscribeService::new(self.rocks()).list()
    }

    pub fn get_subscribe(&self, id: &str) -> Subscribe {
        SubscribeService::new(self.rocks()).get(id)
    }

    pub fn create_subscribe(&self, subscribe: Subscribe) -> String {
        SubscribeService::new(self.rocks()).create(subscribe)
    }

    pub fn delete_subscribe(&self, id: &str) {
        SubscribeService::new(self.rocks()).delete(id);
    }
}


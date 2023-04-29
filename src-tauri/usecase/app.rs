use crate::repository::rocks::RocksRepository;
use crate::service::event::{EventService, Event};

pub struct AppUsecase {
    rocks: RocksRepository,
}
impl AppUsecase {
    pub fn new() -> Self {
        AppUsecase {
            rocks: RocksRepository::new(),
        }
    }

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    pub fn create_event(&self, event: Event) {
        // trigger event
        EventService::create(self.rocks(), event);
        // publish
        // trigger event
    }
}
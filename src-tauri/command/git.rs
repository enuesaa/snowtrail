use crate::service::git::GitHistories;
use crate::repository::command::Runcommand;
use crate::service::oldevnt::Event;

#[tauri::command]
pub fn git_histories() -> GitHistories {
    GitHistories::fetch(Runcommand::new())
}

#[tauri::command]
pub fn push_git_histories_to_event() {
    let historis = GitHistories::fetch(Runcommand::new());
    historis.items.iter().for_each(|h| {
        let event = Event::new(&h.hash);
        event.create();
    });
}

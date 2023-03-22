use crate::service::git::GitHistories;
use crate::repository::command::Runcommand;
use crate::service::event::Event;

#[tauri::command]
#[allow(non_snake_case)]
pub fn gitHistories() -> GitHistories {
    GitHistories::fetch(Runcommand::new())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn pushGitHistoriesToEvent() {
    let historis = GitHistories::fetch(Runcommand::new());
    historis.items.iter().for_each(|h| {
        let event = Event::new(&h.hash);
        event.create();
    });
}

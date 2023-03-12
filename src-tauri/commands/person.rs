use crate::services::person::{add_person as add_person_to_db, Person};

#[tauri::command]
pub async fn add_person() {
    let person = Person::new("aaa");
    add_person_to_db(person).await;
}

use crate::inspection::InspectionDate;
use crate::Database;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ViewReadyInspection {
    name: String,
    duedate: String,
}

#[derive(Serialize)]
struct ViewBag {
    inspections: Vec<ViewReadyInspection>,
}

#[get("/")]
pub fn index(db: Database) -> Template {
    let inspections = db
        .get_inspections()
        .iter()
        .map(|i| ViewReadyInspection {
            name: i.name.clone(),
            duedate: "todo".to_string(),
        })
        .collect::<Vec<ViewReadyInspection>>();

    Template::render("index", &ViewBag { inspections })
}

#[derive(Serialize)]
struct EditViewBag {
    name: String,
    repetition: String,
    last_confirmation_time: String,
    due_date: String,
}

#[get("/inspection/<inspection>")]
pub fn edit(inspection: String, db: Database) -> Template {
    let inspection = db
        .get_inspections()
        .into_iter()
        .filter(|i| i.name == inspection)
        .nth(0)
        .unwrap();
    println!("{:#?}", inspection);
    Template::render(
        "edit",
        &EditViewBag {
            name: inspection.name.clone(),
            repetition: inspection.repetition.to_string(),
            last_confirmation_time: inspection.date.to_string(),
            due_date: format_duration(inspection.next_time()),
        },
    )
}

fn format_duration(duration: chrono::Duration) -> String {
    format!(
        "{} d. {:>02}:{:>02}:{:>02}",
        duration.num_days(),
        duration.num_hours() % 24,
        duration.num_minutes() % 60,
        duration.num_seconds() % 60,
    )
}

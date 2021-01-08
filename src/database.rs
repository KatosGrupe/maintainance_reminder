use crate::inspection::InspectionDate;
use crate::Repetition;
use chrono::DateTime;
use chrono::Utc;

pub struct Database {
    connection: sqlite::Connection,
}

impl Database {
    pub fn open(url: &str) -> Database {
        Database {
            connection: sqlite::open(&url)
                .expect(&format!("Failed to load the database ({})", url)),
        }
    }

    pub fn get_inspections(&self) -> Vec<InspectionDate> {
        let mut cursor = self.connection
        .prepare(
            "
select i.id,
       i.repetition,
       coalesce(max(id.completed_on), '0000-01-01T00:00:00+00:00') as last_completion_time from inspections i
left join inspection_date id on i.id = id.inspection_id
group by i.id, i.repetition;
",
        )
        .expect("Failed to prepare query")
        .cursor();
        let mut inspections = vec![];

        while let Some(row) = cursor.next().unwrap() {
            inspections.push(InspectionDate {
                id: row[0].as_integer().unwrap(),
                repetition: Repetition::from_string(row[1].as_string().unwrap())
                    .expect("Failed to parse"),
                date: match DateTime::parse_from_rfc3339(row[2].as_string().unwrap()) {
                    Ok(value) => value,
                    Err(error) => panic!(
                        "Failed to parse ({}): {}",
                        error,
                        row[2].as_string().unwrap()
                    ),
                },
            })
        }

        inspections
    }
}
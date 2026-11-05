use chrono::{Local, NaiveDate};
#[derive(Debug)]

pub struct Film {
    pub title: String,
    pub description: String,
    pub price: f64,
    pub created_at: NaiveDate,
    pub films: Vec<Film>,
}

impl Film {
    pub fn add(data: (String, String, f64)) -> Self {
        Self {
            title: (data.0),
            description: (data.1),
            price: (data.2),
            created_at: Local::now().date_naive(),
            films: Vec::new(),
        }
    }
}

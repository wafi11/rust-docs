struct Ticket {
    pub id: i32,
    pub title: String,
    pub price: f32,
}

use std::sync::Mutex;

static TICKETS: Mutex<Vec<Ticket>> = Mutex::new(Vec::new());

impl Ticket {
 
}

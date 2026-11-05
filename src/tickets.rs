#[derive(Debug)]
pub struct Tickets {
    pub title: String,
    pub price: f64,
    // ini digunkan untuk penyimpanan array
    pub tickets_data: Vec<Tickets>,
}

impl Tickets {
    pub fn get_data(&self) -> &Vec<Tickets> {
        // ambil values dari struct
        &self.tickets_data
    }

    pub fn show_values(&self) {
        for data in self.get_data() {
            println!("Ticket {} harganya {}", data.title, data.price);
        }
    }

    pub fn create_ticket(&mut self) {
        let ticket = Tickets {
            title: self.title.clone(),
            price: self.price,
            tickets_data: Vec::new(),
        };
        self.tickets_data.push(ticket);
    }

    pub fn new(data: (String, f64)) -> Self {
        Self {
            title: (data.0),
            price: (data.1),
            tickets_data: Vec::new(),
        }
    }
}

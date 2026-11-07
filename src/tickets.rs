#[derive(Debug, Clone)]
pub struct Ticket {
    pub title: String,
    pub price: f64,
}

impl Ticket {
    pub fn new(title: impl Into<String>, price: f64) -> Self {
        Self {
            title: title.into(),
            price,
        }
    }
}

#[derive(Debug, Default)]
pub struct TicketService {
    tickets: Vec<Ticket>,
}

impl TicketService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_ticket(&mut self, title: impl Into<String>, price: f64) {
        self.tickets.push(Ticket::new(title, price));
    }

    pub fn list(&self) -> &[Ticket] {
        &self.tickets
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, index: usize) -> Option<Ticket> {
        if index < self.tickets.len() {
            Some(self.tickets.remove(index))
        } else {
            None
        }
    }

    pub fn total_price(&self) -> f64 {
        self.tickets.iter().map(|t| t.price).sum()
    }
}

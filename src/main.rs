mod film;
mod read_file_csv;
mod tickets;

use film::Film;
use tickets::Tickets;

fn main() {
    let mut ticket: Tickets = Tickets {
        title: String::from("ticket pertama"),
        price: 100.0,
        tickets_data: Vec::new(),
    };

    tickets::Tickets::show_values(&ticket);

    let tiket1 = tickets::Tickets::new((String::from("tiket pertama"), 100.00));

    println!("{:?}", tiket1);

    ticket = Tickets {
        title: String::from("ticket kedua"),
        price: 10.0,
        tickets_data: Vec::new(),
    };

    tickets::Tickets::create_ticket(&mut ticket);

    tickets::Tickets::show_values(&ticket);

    let film_1 = film::Film::add((
        String::from("Avangers"),
        String::from("Avangers Film Yang bagus"),
        1000.00,
    ));
    println!("{:?}", film_1);
}

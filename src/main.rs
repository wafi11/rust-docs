mod config;
mod jobs;
mod tickets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let data_ticket = tickets::Ticket {
    //     price: 1000.0,
    //     title: "Konser".to_string(),
    // };

    // let mut svc = tickets::TicketService::new();
    // svc.create_ticket(data_ticket.title, data_ticket.price);
    // svc.create_ticket("Bioskop", 2000_0.0);
    // println!("{:#?}\n", svc.list());
    // println!("Total : {:?}", svc.total_price());

    let saldo_const = "SALDO_AWAL_STOK";
    let data_brg = jobs::queue::SaldoAwalStock {
        barang: "pensil",
        qty: 1,
    };
    let data_saldo: Vec<jobs::queue::State> = vec![jobs::queue::State {
        gudang: "001".to_string(),
        data: vec![data_brg],
    }];

    let mut jobs = jobs::queue::QueueService::new();
    jobs.create_queue(saldo_const, data_saldo);
    jobs.queue().await?;
    let worker = jobs.worker().await?;

    println!("{:#?}", jobs.queues);
    jobs.choice_queue("jajakajaja");

    println!("worker berjalan, tekan Ctrl+C untuk berhenti");
    tokio::signal::ctrl_c().await?;
    worker.close(5000).await?;

    Ok(())
}

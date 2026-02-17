use std::net::TcpStream;

fn scan_port(ip :&str, port : u16) -> bool{
    // get address combine ip:port
    let addr = format!("{}:{}", ip, port);
    // return values port on ip is open
    TcpStream::connect(addr).is_ok()
}

pub fn scanning_port(){
    let args :Vec<String> = vec!["192.168.1.20","1-1000"].into_iter().map(|s| s.to_owned()).collect();

    // if args.len() < 1 {
    //     println!("Usage: {} <ip> <start_port-end_port>", args[0]);
    //     println!("Example: {} 192.168.1.1 1-1000", args[0]);
    //     return;
    // }
    let ip = &args[0];
    println!("Scanning ip '{}'", ip);
    let range: Vec<&str> = args[1].split('-').collect();
    println!("Scanning range '{:?}", range);

    let start: u16 = range[0].parse().expect("Invalid start port");
    println!("Scanning start '{:?}", start);

    let end: u16 = range[1].parse().expect("Invalid end port");
    println!("Scanning end '{:?}", end);

    println!("Scanning {}:{}-{}...\n", ip, start, end);

    for port in start..=end {
        if scan_port(ip, port) {
            println!("Port {:5} - OPEN", port);
        }
    }

    println!("\nDone!");
}




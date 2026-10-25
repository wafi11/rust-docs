use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub fn read_file() -> std::io::Result<()> {
    let start = Instant::now();
    let file = File::open("data.csv")?;

    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, file);

    let mut count = 0u64;
    let mut line = String::new();
    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        count += 1;
    }
    println!("{count} baris dalam {:?}", start.elapsed());
    Ok(())

}

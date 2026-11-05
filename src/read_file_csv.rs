use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub fn read_file() -> std::io::Result<()> {
    let start = Instant::now();
    let file = File::open("data.csv")?;

    let mut reader = BufReader::with_capacity(1 * 1024 * 1024, file);

    let mut count = 0u64;
    let mut line = String::new();

    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        count += 1;

        // if count % 1000 == 0 {
        //     println!("baris {} per 1000 sudah dilewatin dalam waktu {:?}",count,start.elapsed())
        // }
        
    }

    println!("{count} baris dalam {:?}", start.elapsed());
    Ok(())

}

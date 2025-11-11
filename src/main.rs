use std::fs::{OpenOptions};
use std::io::Write;
use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{engine::general_purpose, Engine as _};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:4000")
    .expect("couldn't bind to address");

    println!("Listening on 0.0.0.0:4000...");

    // Open (or create) the CSV file for appending
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("packets.csv")?;

    // Write header if file is empty
    if file.metadata()?.len() == 0 {
        writeln!(file, "timestamp,source_ip,base64_data")?;
    }

    loop {
        let mut buf = [0u8; 1500];
        let (num_bytes, src_addr) = socket
        .recv_from(&mut buf)
        .expect("failed to receive data");

        let data = &buf[..num_bytes];
        let encoded = general_purpose::STANDARD.encode(data);

        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();

        let source_ip = src_addr.ip();

        writeln!(file, "{},{},{}", timestamp, source_ip, encoded)?;

        file.flush()?;

        println!(
            "Saved {} bytes from {} at timestamp {}",
            num_bytes, source_ip, timestamp
        );
        let parsed = gdl90::read_raw(data);
        if parsed.is_err() {
            continue;
        }
        println!("{:#?}", parsed.unwrap());
    }
}

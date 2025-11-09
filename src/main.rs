use std::net::UdpSocket;

use gdl90::datalink::Gdl90DatalinkMessage;

fn main() {
    // hardcoded, but you should read this from transponder
    let socket = UdpSocket::bind("0.0.0.0:4000").expect("couldn't bind to address");
    loop {
        let mut buf = [0; 1500];
        let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf)
                                                .expect("Didn't receive data");
        let filled_buf = &buf[..number_of_bytes];
        println!("{:?}", filled_buf);
        let parsed = gdl90::read_raw(filled_buf);
        if parsed.is_err() {
            continue;
        }
        match &parsed.as_ref().unwrap().message_data {
            Gdl90DatalinkMessage::Heartbeat { status_byte_1, status_byte_2, uat_timestamp, message_counts } => {
                //println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::Initialization { configuration_byte_1, configuration_byte_2 } => {
                println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::UplinkData { time_of_reception, payload } => {
                println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::HeightAboveTerrain { hat } => {
                println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::OwnshipReport { report } => {
                println!("{:#?}", report);
            },
            Gdl90DatalinkMessage::TrafficReport { report } => {
                println!("{:#?}", report);
            },
            Gdl90DatalinkMessage::OwnshipGeoometricAltitude { ownship_geo_altitude, vertical_metrics } => {
                println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::BasicReport() => {
                println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::LongReport() => {
                println!("{:#?}", parsed.unwrap());
            },
            Gdl90DatalinkMessage::Unknown => {
                println!("{:#?}", parsed.unwrap());
            },
        }
        //println!("{:#?}", parsed.unwrap());
    }
}

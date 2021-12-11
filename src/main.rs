use clap::{App, Arg};
use human_bytes::human_bytes;
use std::fs::File;
use std::io::stdout;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

#[derive(Clone, Copy, Debug)]
struct NetworkStatus {
    bytes_sent: u64,
    bytes_received: u64,
}

impl NetworkStatus {
    fn new() -> NetworkStatus {
        NetworkStatus {
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_network_status(interface: &str) -> NetworkStatus {
    let mut status = NetworkStatus::new();
    let mut found = false;

    if let Ok(lines) = read_lines("/proc/net/dev") {
        for line in lines {
            if let Ok(line) = line {
                let fields = line
                    .split(" ")
                    .filter(|&x| !x.is_empty())
                    .collect::<Vec<&str>>();
                if fields.len() < 12 {
                    continue;
                }
                if fields[0] != (interface.to_string() + &":".to_string()) {
                    continue;
                }
                status.bytes_received = fields[1].parse().unwrap();
                status.bytes_sent = fields[9].parse().unwrap();
                found = true;
                break;
            }
        }
    }
    if !found {
        println!("network interface {} not found", interface);
        std::process::exit(1);
    }

    status
}

fn get_unix_timestamp() -> u64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn evalutate_log(file: &str) {
    const DAYS: usize = 30;
    let mut days = vec![NetworkStatus::new(); DAYS];
    let secs_per_day = 3600 * 24;
    let now = get_unix_timestamp();
    let start = now - (secs_per_day * DAYS) as u64;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line) = line {
                let fields = line.split(" ").collect::<Vec<&str>>();
                if fields.len() < 3 {
                    continue;
                }
                let timestamp: u64 = fields[0].parse().unwrap();
                if timestamp < start {
                    continue;
                }
                let sent: u64 = fields[1].parse().unwrap();
                let received: u64 = fields[2].parse().unwrap();
                let day = (timestamp - start) / (secs_per_day as u64);
                if day < days.len() as u64 {
                    days[day as usize].bytes_sent += sent;
                    days[day as usize].bytes_received += received;
                }
            }
        }
    }
    let mut sum_sent: u64 = 0;
    let mut sum_received: u64 = 0;
    for (i, day) in days.iter().enumerate() {
        println!(
            "day: {}, sent: {}, received: {}",
            i + 1,
            human_bytes(day.bytes_sent as f64),
            human_bytes(day.bytes_received as f64)
        );
        sum_sent += day.bytes_sent;
        sum_received += day.bytes_received;
    }
    println!(
        "sum: sent: {}, received: {}",
        human_bytes(sum_sent as f64),
        human_bytes(sum_received as f64)
    );
}

fn create_log(interface: &str) {
    // log network traffic every minute
    println!("{} 0 0", get_unix_timestamp());
    stdout().flush().unwrap();
    let mut last_status = get_network_status(interface);
    loop {
        thread::sleep(time::Duration::from_secs(60));
        let status = get_network_status(interface);
        let sent = status.bytes_sent - last_status.bytes_sent;
        let received = status.bytes_received - last_status.bytes_received;
        println!("{} {} {}", get_unix_timestamp(), sent, received);
        stdout().flush().unwrap();
        last_status = status;
    }
}

fn main() {
    let mut app = App::new("networktraffic")
        .version("0.1")
        .author("Frank Buss <fb@frank-buss.de>")
        .about("Logs network traffic and prints summary information")
        .arg(
            Arg::with_name("file")
                .conflicts_with("interface")
                .short("f")
                .long("file")
                .help("Config file to read, for showing summary information")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interface")
                .conflicts_with("file")
                .short("i")
                .long("interface")
                .help("Network interface name, for example eth0")
                .takes_value(true),
        );
    let matches = app.clone().get_matches();

    let file = matches.value_of("file").unwrap_or("");
    let interface = matches.value_of("interface").unwrap_or("");
    if file != "" {
        evalutate_log(file);
    } else if interface != "" {
        create_log(interface);
    } else {
        app.print_help().unwrap();
        println!("");
    }
}

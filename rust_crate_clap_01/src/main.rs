use clap::{arg, command, value_parser, Arg, ArgAction};
use std::net::IpAddr;

fn main() {
    // real world clap example

    let kind_choices = ["http1", "http2", "http3"];
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(--hostname <HOSTNAME> "Server hostname")
                .default_value("127.0.0.1")
                .value_parser(value_parser!(IpAddr)),
        )
        /*
        .arg(
            arg!(-p --port <PORT> "Server port")
                .default_value("6161")
                .value_parser(value_parser!(u16)),
        )
        */
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("6161")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            arg!(-k --kind <KIND> "Server kind")
                .value_parser(kind_choices)
                .default_value(kind_choices[0]),
        )
        .arg(arg!(-w --weak_checks "Enable weak checks").action(ArgAction::SetTrue))
        .arg(arg!([message] "Message to send").required(true))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count),
        )
        .get_matches();

    if let Some(hostname) = matches.get_one::<IpAddr>("hostname") {
        println!("Value for server hostname: {hostname}");
    }
    if let Some(port) = matches.get_one::<u16>("port") {
        println!("Value for server port: {port}");
    }
    println!(
        "Value for server kind: {:?}",
        matches.get_one::<String>("kind")
    );
    println!(
        "Value for weak checks: {:?}", // type: Option<&bool>
        matches.get_one::<bool>("weak_checks")
    );
    println!(
        "Value for message: {:?}",
        matches.get_one::<String>("message")
    );
    println!("Value for verbose: {}", matches.get_count("verbose"));
}

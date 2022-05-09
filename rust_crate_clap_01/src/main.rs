// #[macro_use]
// extern crate clap;

use clap::{Arg, App, ArgGroup, crate_version, value_t};

fn main() {

    // simple clap example
    /*
    let args = App::new("app")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .help("Config file name")
                 .takes_value(true))
        .get_matches();

    let config = args.value_of("config").unwrap_or("default.conf");
    println!("config: {}", config);
    */

    // real world clap example

    let kind_choices = &["http1", "http2", "http3"];
    let args = App::new("app")
        // .help_heading("Server") // only in clap 3 :-/
        .version(crate_version!())
        .arg(Arg::with_name("hostname")
                 .short("h")
                 .long("hostname")
                 .help("Server hostname")
                 // .default_value("127.0.0.1")
             // .required(true))
        )
        .arg(Arg::with_name("port")
                 .short("p")
                 .long("port")
                 .help("Server port")
                 .default_value("8080")
             // .required(true))
        )
        .arg(Arg::with_name("kind")
            .long("kind")
            .possible_values(kind_choices)
            .help("Server kind")
            .default_value(kind_choices[0])
        )
        .arg(Arg::with_name("enable_weak_checks")
            .long("enable_weak_checks")
            .help("Enable wc")
        )
        .arg(
            Arg::with_name("key_msg")
                .long("key_msg")
                .help("Message key (0 to disable)")
        )
        // .group(ArgGroup::with_name("server").args(&["hostname", "port"]))
        // .group(ArgGroup::with_name("keys").args(&["key_msg"]))
        .get_matches();

    let hostname = args.value_of("hostname").unwrap_or("127.0.0.1");
    // let port = args.value_of("port").unwrap_or("8080");
    // let port = args.value_of("port").unwrap(); // safe to use unwrap as default value

    // let port = value_t!(args, "port", u32).unwrap_or_else(|e| e.exit());
    let port = value_t!(args, "port", u32).unwrap_or_else(|e| e.exit());

    println!("hostname: {} - port: {}", hostname, port);

    let kind = args.value_of("kind").unwrap();
    let e_wc = args.is_present("enable_weak_checks");

    println!("kind: {} - enable weak checks: {}", kind, e_wc);

}

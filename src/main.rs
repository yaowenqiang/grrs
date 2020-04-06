//export RUST_LOG=error
use log::{info, warn};
extern crate env_logger;
use std:: thread;
use std::time::Duration;
#[macro_use]  extern crate log;
use structopt::StructOpt;
#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    }).expect("Error setting Ctrl+C handler!");

thread::sleep(Duration::from_secs(2));
env_logger::init();
info!("starting up.");
warn!("Oops! nothing implemented!");
    /*
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = CLI {
        pattern: pattern,
        path: std::path::PathBuf::from(path)
    };
    */
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        //println!("{}", i);
        //pb.println(format!("[+] finished: #{}", i));
        pb.inc(1);


    }
    pb.finish_with_message("done.");

    let args = Cli::from_args();
    println!("{:?}", args);
    let contents = std::fs::read_to_string(&args.path).expect("could not load file ");

    for line in contents.lines() {
       if(line.contains(&args.pattern))  {
           println!("{:?}", line);
       }
    }

    println!("{:?}", args);

    println!("Hello, world!");
}

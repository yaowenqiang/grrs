use structopt::StructOpt;
#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    /*
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = CLI {
        pattern: pattern,
        path: std::path::PathBuf::from(path)
    };
    */

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

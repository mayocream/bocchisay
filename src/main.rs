use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'f', long = "name", default_value = "hitori")]
    name: Option<String>,

    message: Option<String>,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name.unwrap());
    if let Some(message) = args.message {
        println!("{}", message);
    }
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // session configuration file path
    #[arg(short, long)]
    config_path: String,

    // flag to check if it is ran as a zsh post cmd hook
    #[arg(short, long, default_value_t = false)]
    zhook: bool,
}
fn main() {

    let args = Args::parse();

    println!("{}", args.config_path);

    if args.config_path.is_empty() {
        println!("meu deus")
    }
}

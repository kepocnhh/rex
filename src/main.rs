fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        panic!("Arguments error!");
    }
    match rex::on_args(&args[1..]) {
        Ok(message) => {
            println!("{message}");
        }
        Err(message) => {
            println!("{message}");
            std::process::exit(1);
        }
    }
}

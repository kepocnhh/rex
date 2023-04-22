fn print_help() {
    println!("Usage: rex [options...]");
    let message = vec![
        (vec!["-u", "--url"], "url like \"https://github.com/\""),
        (vec!["-m", "--method"], "method like \"GET\",\"POST\", etc. Default is \"GET\"."),
    ].into_iter()
        .map(|(args, message)| {
            assert!(!args.is_empty());
            assert!(!message.is_empty());
            return format!("{:<16}| {message}", args.join(", "));
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{message}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        panic!("Arguments error!");
    }
    match rex::on_args(&args[1..]) {
        Ok(message) => println!("{message}"),
        Err(it) => {
            match it {
                rex::Error::Before(message) => {
                    println!("{message}\n");
                    print_help();
                    std::process::exit(1);
                }
                rex::Error::After(message) => {
                    println!("{message}");
                    std::process::exit(2);
                }
            }
        }
    }
}

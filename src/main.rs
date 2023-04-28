use rex::internal::entity::{Error, Success};

fn print_help() {
    // todo help
    // todo version
    println!("Usage: rex {{url}}");
    println!(" when url like \"https://github.com/\" is requested with default params");
    println!();
    println!("Or usage: rex [options...]");
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
        Ok(it) => {
            match it {
                Success::Output(message) => {
                    println!("{message}");
                }
                Success::Silent => {
                    // noop
                }
            }
        },
        Err(it) => {
            match it {
                Error::Request(message) => {
                    println!("{message}\n");
                    print_help();
                    std::process::exit(1);
                }
                Error::Response(message) => {
                    println!("{message}");
                    std::process::exit(2);
                }
            }
        }
    }
}

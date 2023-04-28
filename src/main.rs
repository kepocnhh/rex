use rex::internal::entity::{Action, Error, Method, Success};

fn print_help() {
    println!("Usage: rex {{url}}");
    println!(" when url like \"https://github.com/\" is requested with default params");
    println!();
    println!("Or usage: rex --help");
    println!(" prints this help info");
    println!();
    println!("Or usage: rex --version");
    println!(" prints version of rex (current is \"{}\")", rex::VERSION);
    println!();
    println!("Or usage: rex [args...]");
    println!("{:<16}| {:<11}| Description", "Arg", "Default");
    println!("{:<16}+-{:<11}+{}", "-".repeat(16), "-".repeat(11), "-".repeat(16));
    let message = vec![
        (vec!["-u", "--url"], "", "url like \"https://github.com/\""),
        (vec!["-m", "--method"], Method::default().to_string(), "method like \"GET\",\"POST\", etc."),
    ].into_iter()
        .map(|(args, default, message)| {
            assert!(!args.is_empty());
            assert!(!message.is_empty());
            return format!("{:<16}| {:<11}| {message}", args.join(", "), default);
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
                Success::Action(action) => {
                    match action {
                        Action::PrintHelp => {
                            print_help();
                        }
                    }
                }
                Success::Output(message) => {
                    println!("{message}");
                }
                Success::Silent => {
                    // noop
                }
            }
            std::process::exit(0);
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

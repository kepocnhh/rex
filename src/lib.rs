fn print_help() {
    println!("Usage: rex [options...]");
}

pub fn on_args(args: &[String]) -> Result<&str, &str> {
    if args.is_empty() {
        print_help();
        return Err("No arguments!")
    }
    if args.len() % 2 != 0 {
        print_help();
        return Err("Arguments error!")
    }
    let mut url: Option<String> = None;
    for i in 0..(args.len() / 2) {
        if args[i] == "-u" {
            match url {
                None => {
                    let value = args[i + 1].clone();
                    if value.is_empty() {
                        return Err("Url is empty!")
                    }
                    url = Some(value)
                }
                Some(_) => {
                    return Err("Url is already set!")
                }
            }
        }
    }
    let url = url.ok_or("Url is empty!")?;
    return Err("Unknown error!")
}

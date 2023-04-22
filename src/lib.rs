fn print_help() {
    println!("Usage: rex [options...]");
}

fn err<T>(message: &str) -> Result<T, String> {
    return Err(String::from(message));
}

fn ok<T>(message: &str) -> Result<String, T> {
    return Ok(String::from(message));
}

fn insert_filled_or(actual: &mut Option<String>, tag: &str, value: String) -> Result<(), String> {
    if actual.is_some() {
        return Err(format!("Value \"{tag}\" is already set!"));
    }
    let _ = actual.insert(filled(tag, value)?);
    return Ok(());
}

fn insert_or(actual: &mut Option<String>, value: String) -> Result<(), String> {
    if actual.is_some() {
        return err("Value is already set!");
    }
    let _ = actual.insert(value);
    return Ok(());
}

fn filled(tag: &str, it: String) -> Result<String, String> {
    if it.is_empty() {
        return Err(format!("Value \"{tag}\" is empty!"));
    }
    return Ok(it);
}

pub fn on_args(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        print_help();
        return err("No arguments!");
    }
    if args.len() % 2 != 0 {
        print_help();
        return err("Arguments error!");
    }
    let mut url: Option<String> = None;
    for i in 0..(args.len() / 2) {
        let arg = args[i].as_str();
        match arg {
            "-u" => {
                insert_filled_or(&mut url, arg, args[i + 1].clone())?;
            }
            _ => {
                return Err(format!("Unknown arg {arg}!"));
            }
        }
    }
    let url = url.ok_or("Url is empty!")?;
    return err("Unknown error!")
}

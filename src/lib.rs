use ureq::OrAnyStatus;

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

fn call(request: ureq::Request) -> Result<ureq::Response, String> {
    return err("Unknown error!")
}

fn ureq_error(error: ureq::Transport) -> String {
    return format!("Transport error: {error:?}");
}

fn io_error(error: std::io::Error) -> String {
    return format!("IO error: {error:?}");
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
    let agent: ureq::Agent = ureq::AgentBuilder::new().build();
    let method = "GET"; // todo
    let response = agent.request(method, url.as_str())
        .call()
        .or_any_status()
        .map_err(|it| ureq_error(it))?;
    return Ok(response.into_string().map_err(|it| io_error(it))?);
}

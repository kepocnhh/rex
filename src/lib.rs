use ureq::OrAnyStatus;

fn print_help() {
    let message = vec![
        "Usage: rex [options...]",
        " -u url, like https://github.com/",
    ].join("\n");
    println!("{message}");
}

fn err<T>(message: &str) -> Result<T, String> {
    return Err(String::from(message));
}

fn ok<T>(message: &str) -> Result<String, T> {
    return Ok(String::from(message));
}

fn insert_filled_or(actual: &mut Option<String>, tag: &str, value: String) -> Result<(), String> {
    return insert_or(actual, tag, filled(tag, value)?);
}

fn insert_or<T>(actual: &mut Option<T>, tag: &str, value: T) -> Result<(), String> {
    if actual.is_some() {
        return Err(format!("Value \"{tag}\" is already set!"));
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
    return vec![
        String::from("Transport error!"),
        format!("kind: {}", error.kind()),
        String::from(error.message().unwrap_or("")),
    ].into_iter()
        .filter(|it| !it.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
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
                // todo parse url
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
    // todo check response
    return Ok(response.into_string().map_err(|it| io_error(it))?);
}

use ureq::OrAnyStatus;
use url::Url;

fn insert_or<T>(actual: &mut Option<T>, tag: &str, value: T) -> Result<(), Error> {
    if actual.is_some() {
        return Error::Before(format!("Value \"{tag}\" is already set!")).to_result();
    }
    let _ = actual.insert(value);
    return Ok(());
}

fn filled(tag: &str, it: String) -> Result<String, Error> {
    if it.is_empty() {
        return Error::Before(format!("Value \"{tag}\" is empty!")).to_result();
    }
    return Ok(it);
}

fn ureq_error(error: ureq::Transport) -> Error {
    let message = vec![
        String::from("Transport error!"),
        format!("kind: {}", error.kind()),
        String::from(error.message().unwrap_or("")),
    ].into_iter()
        .filter(|it| !it.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    return Error::After(message);
}

fn io_error(error: std::io::Error) -> Error {
    return Error::After(format!("IO error: {error}"));
}

fn parse_error(error: url::ParseError) -> Error {
    return Error::Before(format!("Parse error: {error}"));
}

pub enum Error {
    Before(String),
    After(String),
}

impl Error {
    fn to_result<T>(self) -> Result<T, Error> {
        return Err(self);
    }

    fn before(message: &str) -> Error {
        return Error::Before(String::from(message));
    }
}

pub fn on_args(args: &[String]) -> Result<String, Error> {
    if args.is_empty() {
        return Error::before("No arguments!").to_result();
    }
    if args.len() % 2 != 0 {
        return Error::before("Arguments error!").to_result();
    }
    let mut url: Option<Url> = None;
    for i in 0..(args.len() / 2) {
        let arg = args[i].as_str();
        match arg {
            "-u" | "--url" => {
                insert_or(
                    &mut url,
                    arg,
                    Url::parse(&filled(arg, args[i + 1].clone())?)
                        .map_err(parse_error)?
                )?;
            }
            _ => {
                return Error::Before(format!("Unknown arg {arg}!")).to_result();
            }
        }
    }
    let url = url.ok_or("Url is empty!").map_err(Error::before)?;
    let agent: ureq::Agent = ureq::AgentBuilder::new().build();
    let method = "GET"; // todo
    let response = agent.request(method, url.as_str())
        .call()
        .or_any_status()
        .map_err(ureq_error)?;
    // todo check response
    return Ok(response.into_string().map_err(io_error)?);
}

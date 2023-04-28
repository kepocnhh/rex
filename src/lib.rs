use ureq::OrAnyStatus;
use url::Url;

trait Inserted<T> {
    fn set_arg_or_else<F>(&mut self, tag: &str, input: String, transform: F) -> Result<(), Error> where F: FnOnce(String) -> Result<T, Error>;
    fn set_arg_or(&mut self, tag: &str, input: String, transform: fn(String) -> T) -> Result<(), Error>;
}

impl<T> Inserted<T> for Option<T> {
    fn set_arg_or_else<F>(&mut self, tag: &str, input: String, transform: F) -> Result<(), Error> where F: FnOnce(String) -> Result<T, Error> {
        if self.is_some() {
            return Error::Before(format!("Value \"{tag}\" is already set!")).to_result();
        }
        if input.is_empty() {
            return Error::Before(format!("Value \"{tag}\" is empty!")).to_result();
        }
        let value = transform(input)?;
        let _ = self.insert(value);
        return Ok(());
    }

    fn set_arg_or(&mut self, tag: &str, input: String, transform: fn(String) -> T) -> Result<(), Error> {
        return self.set_arg_or_else(tag, input, |it| Ok(transform(it)));
    }
}

fn filled<F, E>(it: String, on_error: F) -> Result<String, E>
    where F: FnOnce() -> E {
    if it.is_empty() {
        return Err(on_error());
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

enum Method {
    GET,
    POST,
}

impl Method {
    fn default() -> Method {
        return Method::GET;
    }

    fn from(it: String) -> Result<Method, Error> {
        let method = match it.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => {
                return Error::Before(format!("Method \"{it}\" is not supported!")).to_result();
            }
        };
        return Ok(method);
    }

    fn to_string(&self) -> &str {
        return match self {
            Method::GET => "GET",
            Method::POST => "POST",
        };
    }
}

struct Request {
    url: Url,
    method: Method,
}

fn get_request(args: &[String]) -> Result<Request, Error> {
    // todo unit test
    if args.is_empty() {
        return Error::before("No arguments!").to_result();
    }
    if args.len() % 2 != 0 {
        return Error::before("Arguments error!").to_result();
    }
    let mut url: Option<Url> = None;
    let mut method: Option<Method> = None;
    for i in 0..(args.len() / 2) {
        let index = i * 2;
        let arg = args[index].as_str();
        match arg {
            "-u" | "--url" => {
                url.set_arg_or_else(arg, args[index + 1].clone(), |input| {
                    Url::parse(&input).map_err(parse_error)
                })?;
            }
            "-m" | "--method" => {
                method.set_arg_or_else(arg, args[index + 1].clone(), Method::from)?;
            }
            _ => {
                return Error::Before(format!("Unknown arg {arg}!")).to_result();
            }
        }
    }
    let url = url.ok_or("No url!").map_err(Error::before)?;
    let method = method.unwrap_or(Method::default());
    return Ok(Request { url, method });
}

pub fn on_args(args: &[String]) -> Result<String, Error> {
    if args.len() == 1 {
        match args[0].as_str() {
            "-h" | "--help" => {
                todo!();
            }
            "-v" | "--version" => {
                todo!();
            }
            it if it.is_empty() => {
                return Error::Before(format!("Value \"url\" is empty!")).to_result();
            }
            it => {
                let url = Url::parse(&it).map_err(parse_error)?;
                let agent: ureq::Agent = ureq::AgentBuilder::new().build();
                let response = agent.request(Method::default().to_string(), url.as_str())
                    .call()
                    .or_any_status()
                    .map_err(ureq_error)?;
                // todo check response
                return Ok(response.into_string().map_err(io_error)?);
            }
        }
    }
    let request = get_request(args)?;
    let agent: ureq::Agent = ureq::AgentBuilder::new().build();
    let response = agent.request(request.method.to_string(), request.url.as_str())
        .call()
        .or_any_status()
        .map_err(ureq_error)?;
    // todo check response
    return Ok(response.into_string().map_err(io_error)?);
}

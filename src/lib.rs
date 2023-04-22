use ureq::OrAnyStatus;
use url::Url;

trait Inserted<T> {
    fn insert_or(&mut self, tag: &str, value: T) -> Result<(), Error>;
}

impl <T> Inserted<T> for Option<T> {
    fn insert_or(&mut self, tag: &str, value: T) -> Result<(), Error> {
        if self.is_some() {
            return Error::Before(format!("Value \"{tag}\" is already set!")).to_result();
        }
        let _ = self.insert(value);
        return Ok(());
    }
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
        }
    }
}

struct Request {
    url: Url,
    method: Method,
}

fn get_request(args: &[String]) -> Result<Request, Error> {
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
                let input = filled(arg, args[index + 1].clone())?;
                let value = Url::parse(&input).map_err(parse_error)?;
                url.insert_or(arg, value)?;
            }
            "-m" | "--method" => {
                let input = filled(arg, args[index + 1].clone())?;
                let value = Method::from(input)?;
                method.insert_or(arg, value)?;
            }
            _ => {
                return Error::Before(format!("Unknown arg {arg}!")).to_result();
            }
        }
    }
    let url = url.ok_or("Url is empty!").map_err(Error::before)?;
    let method = method.unwrap_or(Method::default());
    return Ok(Request { url, method })
}

pub fn on_args(args: &[String]) -> Result<String, Error> {
    let request = get_request(args)?;
    let agent: ureq::Agent = ureq::AgentBuilder::new().build();
    let response = agent.request(request.method.to_string(), request.url.as_str())
        .call()
        .or_any_status()
        .map_err(ureq_error)?;
    // todo check response
    return Ok(response.into_string().map_err(io_error)?);
}

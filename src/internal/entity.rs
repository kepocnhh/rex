use url::Url;

pub enum Error {
    Before(String),
    After(String),
}

impl Error {
    pub(crate) fn to_result<T>(self) -> Result<T, Error> {
        return Err(self);
    }

    pub(crate) fn before(message: &str) -> Error {
        return Error::Before(String::from(message));
    }

    pub(crate) fn after(message: &str) -> Error {
        return Error::After(String::from(message));
    }
}

pub(crate) enum Method {
    GET,
    POST,
}

impl Method {
    pub(crate) fn default() -> Method {
        return Method::GET;
    }

    pub(crate) fn from(it: String) -> Result<Method, String> {
        let method = match it.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => {
                return Err(format!("Method \"{it}\" is not supported!"));
            }
        };
        return Ok(method);
    }

    pub(crate) fn to_string(&self) -> &str {
        return match self {
            Method::GET => "GET",
            Method::POST => "POST",
        };
    }
}

pub(crate) struct Environment {
    pub url: Url,
    pub method: Method,
}

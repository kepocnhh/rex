use url::Url;

pub enum Error {
    Request(String),
    Response(String),
}

impl Error {
    pub(crate) fn to_result<T>(self) -> Result<T, Error> {
        return Err(self);
    }

    pub(crate) fn request(message: &str) -> Error {
        return Error::Request(String::from(message));
    }

    pub(crate) fn response(message: &str) -> Error {
        return Error::Response(String::from(message));
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

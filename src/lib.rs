pub mod internal;

use internal::entity::Environment;
use internal::entity::Error;
use internal::entity::Method;
use internal::entity::Success;
use internal::request::call;
use internal::util;
use url::Url;
use crate::internal::entity::Action;

fn get_request(args: &[String]) -> Result<Environment, Error> {
    // todo unit test
    if args.is_empty() {
        return Error::request("No arguments!").to_result();
    }
    if args.len() % 2 != 0 {
        return Error::request("Arguments error!").to_result();
    }
    let mut url: Option<Url> = None;
    let mut method: Option<Method> = None;
    for i in 0..(args.len() / 2) {
        let index = i * 2;
        let arg = args[index].as_str();
        match arg {
            "-u" | "--url" => {
                util::set_or_else(&mut url, arg, args[index + 1].clone(), |input| {
                    Url::parse(&input).map_err(|it| format!("Parse error: {it}"))
                }).map_err(Error::Request)?;
            }
            "-m" | "--method" => {
                util::set_or_else(&mut method, arg, args[index + 1].clone(), Method::from).map_err(Error::Request)?;
            }
            _ => {
                return Error::Request(format!("Unknown arg {arg}!")).to_result();
            }
        }
    }
    let url = url.ok_or("No url!").map_err(Error::request)?;
    let method = method.unwrap_or(Method::default());
    return Ok(Environment { url, method });
}

pub fn on_args(args: &[String]) -> Result<Success, Error> {
    if args.len() == 1 {
        match args[0].as_str() {
            "-h" | "--help" => {
                return Success::Action(Action::PrintHelp).to_result();
            }
            "-v" | "--version" => {
                todo!();
            }
            it if it.is_empty() => {
                return Error::Request(format!("Value \"url\" is empty!")).to_result();
            }
            it => {
                let url = Url::parse(&it)
                    .map_err(|it| Error::Request(format!("Parse error: {it}")))?;
                return call(Environment { url, method: Method::default() }).map(Success::Output);
            }
        }
    }
    return call(get_request(args)?).map(Success::Output);
}

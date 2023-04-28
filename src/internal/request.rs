use ureq::OrAnyStatus;
use crate::internal::entity::Environment;
use crate::internal::entity::Error;

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

pub(crate) fn request(env: Environment) -> Result<String, Error> {
    let agent: ureq::Agent = ureq::AgentBuilder::new().build();
    let response = agent.request(env.method.to_string(), env.url.as_str())
        .call()
        .or_any_status()
        .map_err(ureq_error)?;
    // todo check response
    return Ok(response.into_string().map_err(io_error)?);
}

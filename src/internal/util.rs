fn filled_or_else<F, E>(it: String, err: F) -> Result<String, E> where F: FnOnce() -> E {
    if it.is_empty() {
        return Err(err());
    }
    return Ok(it);
}

pub(crate) fn set_or_else<T, F>(
    value: &mut Option<T>,
    name: &str,
    input: String,
    transform: F,
) -> Result<(), String> where F: FnOnce(String) -> Result<T, String> {
    if value.is_some() {
        return Err(format!("Value \"{name}\" is already set!"));
    }
    if input.is_empty() {
        return Err(format!("Value \"{name}\" is empty!"));
    }
    let _ = value.insert(transform(input)?);
    return Ok(());
}

fn set_or<T>(
    value: &mut Option<T>,
    name: &str,
    input: String,
    transform: fn(String) -> T
) -> Result<(), String> {
    return set_or_else(value, name, input, |it| Ok(transform(it)));
}

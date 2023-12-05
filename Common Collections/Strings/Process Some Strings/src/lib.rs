pub fn trim_me(input: &str) -> String {
    /* TODO: Remove whitespace from both ends of a string! */
    input.trim().to_string()
}

pub fn compose_me(input: &str) -> String {
    /* TODO: Add " world!" to the string! There's multiple ways to do this! */
    format!("{}{}", input, " world!")
}

pub fn replace_me(input: &str) -> String {
    /* TODO: Replace "cars" in the string with "balloons"! */
    input.replace("cars", "balloons")
}
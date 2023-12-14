pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub mod transformer {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>/* Write type of input */) -> Vec<String>/* Write type of result */ {
        let mut output = vec![];
        // TODO: Complete the function body!
        for (string, command) in input {
            output.push(match command {
                Command::Append(taille) => format!("{}{}",string, "bar".repeat(taille).to_string()),
                Command::Trim => string.trim().to_string(),
                Command::Uppercase => string.to_uppercase().to_string()
            })
        }
        output
    }
}
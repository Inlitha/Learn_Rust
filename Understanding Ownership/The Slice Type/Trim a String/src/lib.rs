pub fn trimmed_space(str: &str) -> &str{
    let chars = str.chars();

    let mut first_non_space = str.len();
    // let nb_espace = str.matches(' ').count();
    let mut last_non_space = 0;
    // println!("{}", str);
    // println!("first_non_space : {} - last_non_space : {}", first_non_space, last_non_space);
    // println!("nombre d'espace : {}", nb_espace);
    // if nb_espace == 0{
        // return &str[..];
    // }
    for (i, ch) in chars.enumerate() {
        if ch != ' '{
            if first_non_space == str.len() {
                first_non_space = i;
                // println!("FP");
            }
        // }else if ch == ' ' && last_non_space == 0 {
            // if str[i..].find(' ') == None{
                // last_non_space = str.len();
                // println!("lp");
                // continue;
            // }else if str[i..].matches(' ').count() == str.len() - i {
            last_non_space = i;
                // println!("LP");
            // }
        }
    }
    // println!("first_non_space : {} - last_non_space : {}", first_non_space, last_non_space);
    // if first_non_space == str.len() && last_non_space == 0{
        // return &str[..];
    // }
    &str[first_non_space..=last_non_space]
}

use bon;

// modules
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Parser {
    input: String,
    pos: usize,
    read_pos: usize,
    curr_char: char,
}

#[bon::bon]
impl Parser {
    #[builder]
    pub fn new() -> Parser {
        return Parser {
            input: String::from(""),
            pos: 0,
            read_pos: 0,
            curr_char: 'a',
        };
    }
}

use std::fmt;

pub struct EmptyStringError;

impl fmt::Display for EmptyStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Empty input")
    }
}

#[derive(Debug)]
pub struct CharStream {
    input: String,
    current_position: usize,
}

impl CharStream {
    pub fn new(input: String) -> Result<CharStream, EmptyStringError> {
        if input.is_empty() {
            return Err(EmptyStringError);
        }
        Ok(CharStream {
            input,
            current_position: 0,
        })
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.current_position += 1;
        return self.input.chars().nth(self.current_position - 1);
    }

    pub fn putback(&mut self) {
        self.current_position -= 1;
    }

    pub fn current_pos(&self) -> usize {
        return self.current_position;
    }
}


pub struct Token<'a> {
    pub chars: &'a str,
}

pub struct Lexer<'a>{
    cursor: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer<'a> {
        Self { cursor: src.trim_start() }
         
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }
        for (offset, c) in self.cursor.char_indices() {
            if c.is_whitespace() {
                let token = Token {
                    chars: &self.cursor[..offset],
                };
                self.cursor = &self.cursor[offset..].trim_start();
                return Some(token);
            }
        }
        let token =Token {
            chars: self.cursor,
        };
        self.cursor = &self.cursor[self.cursor.len()..];
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tokenize() {
        let lexer = super::Lexer::new("   10\n   20 ");
        for each in lexer {
            println!("{}", each.chars);
        }
    }
}
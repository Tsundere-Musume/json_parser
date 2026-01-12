use std::collections::HashMap;

#[derive(Debug)]
enum JsonValue {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
    Array(Vec<JsonValue>),
    Obj(HashMap<String, JsonValue>),
}

#[derive(Debug)]
struct Parser {
    src: String,
    pos: usize,
}

impl Parser {
    fn done(&self) -> bool {
        self.pos >= self.src.len()
    }
    fn new(src: String) -> Parser {
        Parser { src, pos: 0 }
    }

    fn parse(&mut self) -> Option<JsonValue> {
        self.skip_whitespace();
        match self.peek()? {
            '"' => self.parse_string(),
            _ => None,
        }
    }

    fn consume(&mut self, to_match: char) -> bool {
        //TODO: check return type
        if self.peek() == Some(to_match) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_string(&mut self) -> Option<JsonValue> {
        if !self.consume('"') {
            return None;
        }
        let mut result = String::new();
        loop {
            match self.peek()? {
                '"' => {
                    self.advance();
                    return Some(JsonValue::String(result));
                }

                ch => {
                    result.push(ch);
                    self.advance();
                }
            }
        }
    }

    fn parse_null(&mut self) -> Option<JsonValue> {
        if self.pos + 4 <= self.src.len() && (&self.src[self.pos..self.pos + 4] == "null") {
            self.pos += 4;
            Some(JsonValue::Null)
        } else {
            None
        }
    }

    fn parse_number(&mut self) -> Option<JsonValue> {
        todo!()
    }
    fn parse_array(&mut self) -> Option<JsonValue> {
        todo!()
    }
    fn parse_object(&mut self) -> Option<JsonValue> {
        todo!()
    }

    //TODO: Consider &str instead of String
    fn peek(&self) -> Option<char> {
        self.src.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.src.chars().nth(self.pos - 1)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                '\t' | '\n' | ' ' | '\r' => self.advance(),
                _ => break,
            };
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let input = "Testing parse_string()";
        let mut parser = Parser::new(format!("\"{}\"", input));
        match parser.parse_string() {
            Some(JsonValue::String(value)) => assert_eq!(value, input),
            _ => panic!("Expected String"),
        };
    }

    #[test]
    fn test_skip_whitespaces() {
        let mut parser = Parser::new("      \t\n ".to_string());
        parser.skip_whitespace();
        assert!(matches!(parser.peek(), None));

        let mut parser = Parser::new("      \t\n  a".to_string());
        parser.skip_whitespace();
        assert!(matches!(parser.peek(), Some(ch) if ch == 'a'));
    }

    #[test]
    fn test_parse_null() {
        let mut parser = Parser::new("null".to_string());
        assert!(matches!(parser.parse_null(), Some(JsonValue::Null)));

        let mut parser = Parser::new("nul".to_string());
        assert!(matches!(parser.parse_null(), None));
    }
}

use std::fmt;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Number(f64),
    String(String),
    Bool(bool),
    Array(Vec<JsonValue>),
    Obj(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonValue::Obj(map) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in map.iter() {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug)]
struct Parser {
    src: String,
    pos: usize,
}

impl Parser {
    fn new(src: String) -> Parser {
        Parser { src, pos: 0 }
    }

    fn parse(&mut self) -> Option<JsonValue> {
        self.skip_whitespace();
        match self.peek()? {
            '"' => self.parse_string(),
            '0'..='9' => self.parse_number(),
            't' | 'f' => self.parse_bool(),
            '[' => self.parse_array(),
            'n' => self.parse_null(),
            '{' => self.parse_object(),
            _ => None, //FIXME: probably panic or resturn result since all valid json cases have
                       //already been handled
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

    // TODO: Return result maybe
    fn expect(&mut self, to_match: char) {
        if self.peek() != Some(to_match) {
            panic!("Expected {}", to_match);
        }
        self.advance();
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

    fn consume_word(&mut self, word: &str) -> bool {
        let l = word.len();
        if self.pos + l <= self.src.len() && (&self.src[self.pos..self.pos + l] == word) {
            self.pos += l;
            true
        } else {
            false
        }
    }

    fn parse_null(&mut self) -> Option<JsonValue> {
        if self.consume_word("null") {
            Some(JsonValue::Null)
        } else {
            None
        }
    }

    fn parse_number(&mut self) -> Option<JsonValue> {
        //TODO: add for floats or other number representations
        let idx = self.pos;
        loop {
            match self.peek() {
                Some(ch) if ch >= '0' && ch <= '9' => {
                    self.advance();
                }
                _ => break,
            }
        }

        if idx == self.pos {
            None
        } else {
            let value = &self.src[idx..self.pos].parse::<f64>().unwrap();
            Some(JsonValue::Number(*value))
        }
    }

    fn parse_bool(&mut self) -> Option<JsonValue> {
        if self.consume_word("true") {
            Some(JsonValue::Bool(true))
        } else if self.consume_word("false") {
            Some(JsonValue::Bool(false))
        } else {
            None
        }
    }

    fn parse_array(&mut self) -> Option<JsonValue> {
        if self.consume('[') {
            let mut result = vec![];
            loop {
                match self.parse() {
                    Some(v) => result.push(v),
                    None => break,
                }

                self.skip_whitespace();

                if !self.consume(',') {
                    break;
                }
            }
            self.expect(']');
            Some(JsonValue::Array(result))
        } else {
            None
        }
    }

    fn parse_object(&mut self) -> Option<JsonValue> {
        if self.consume('{') {
            let mut map = HashMap::new();
            loop {
                let key = match self.parse() {
                    Some(JsonValue::String(v)) => v,
                    //FIXME: move to result types
                    Some(_) => panic!("Expected a string value as a key"),
                    None => break,
                };

                self.skip_whitespace();
                self.expect(':');

                let value = match self.parse() {
                    Some(v) => v,
                    None => panic!("Expected a value for the key {}", key),
                };
                map.insert(key, value);
                self.skip_whitespace();
                if !self.consume(',') {
                    break;
                }
            }
            self.expect('}');
            Some(JsonValue::Obj(map))
        } else {
            None
        }
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
    let data = fs::read_to_string("todos.json").expect("Failed to read from a file");
    let mut parser = Parser::new(data);
    let result = parser.parse().unwrap();
            println!("{}", result);
    match result {
        JsonValue::Obj(v) => {
            println!("\n{:?}", &v["total"]);
        }
        _ => (),
    };
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

    #[test]
    fn test_parse_bool() {
        let mut parser = Parser::new("true".to_string());
        match parser.parse_bool() {
            Some(JsonValue::Bool(value)) => assert_eq!(value, true),
            _ => panic!("Expected True"),
        };

        let mut parser = Parser::new("false".to_string());
        match parser.parse_bool() {
            Some(JsonValue::Bool(value)) => assert_eq!(value, false),
            _ => panic!("Expected False"),
        };

        let mut parser = Parser::new("fale".to_string());
        assert!(matches!(parser.parse_bool(), None));
    }

    #[test]
    fn test_parse_number() {
        let mut parser = Parser::new("01234 abc".to_string());
        match parser.parse_number() {
            Some(JsonValue::Number(value)) => assert_eq!(value, 1234_f64),
            _ => panic!("Expected number: 1234"),
        };

        let mut parser = Parser::new("false".to_string());
        assert!(matches!(parser.parse_number(), None));
    }

    #[test]
    fn test_parse_array() {
        let mut parser = Parser::new("[1,32,\"abc\", null  ]".to_string());
        let expected_result = vec![
            JsonValue::Number(1_f64),
            JsonValue::Number(32_f64),
            JsonValue::String("abc".to_string()),
            JsonValue::Null,
        ];
        match parser.parse_array() {
            Some(JsonValue::Array(value)) => assert_eq!(value, expected_result),
            _ => panic!("Expected array"),
        };

        let mut parser = Parser::new("false".to_string());
        assert!(matches!(parser.parse_array(), None));

        let mut parser = Parser::new("[]".to_string());
        match parser.parse_array() {
            Some(JsonValue::Array(value)) => assert_eq!(value, vec![]),
            _ => panic!("Expected an empty array"),
        }
    }

    #[test]
    fn test_parse_object() {
        let mut parser = Parser::new(
            r#"{
                "one" : 2,
                "two" : [2, null, false],
                "three": "third value",
            }"#
            .to_string(),
        );

        let mut expected_result = HashMap::new();
        expected_result.insert("one".to_string(), JsonValue::Number(2.0));
        expected_result.insert(
            "two".to_string(),
            JsonValue::Array(vec![
                JsonValue::Number(2.0),
                JsonValue::Null,
                JsonValue::Bool(false),
            ]),
        );
        expected_result.insert(
            "three".to_string(),
            JsonValue::String("third value".to_string()),
        );

        match parser.parse_object() {
            Some(JsonValue::Obj(value)) => assert_eq!(value, expected_result),
            _ => panic!("Expected array"),
        };

        let mut parser = Parser::new("false".to_string());
        assert!(matches!(parser.parse_object(), None));

        let mut parser = Parser::new("{}".to_string());
        match parser.parse_object() {
            Some(JsonValue::Obj(value)) => assert_eq!(value, HashMap::new()),
            _ => panic!("Expected an empty map"),
        }
    }
}

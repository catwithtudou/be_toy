use std::collections::HashMap;

use crate::dom::{AttrMap, comment, elem, Node, text};

pub fn parse(source: String) -> Node {
    let mut nodes = Parser::new(0, source).parse_nodes();

    if nodes.len() == 1 {
        return nodes.swap_remove(0);
    }

    return elem("html".to_string(), HashMap::new(), nodes);
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn new(pos: usize, input: String) -> Parser {
        Parser {
            pos,
            input,
        }
    }


    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn next_two_char(&self) -> (char, char) {
        let mut chars = self.input[self.pos..].chars();
        (chars.next().unwrap(), chars.next().unwrap())
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    fn consume_while<T>(&mut self, cond: T) -> String
        where T: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && cond(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|a: char| { a.is_whitespace() });
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> Node {
        match self.next_two_char() {
            ('<', '!') => self.parse_comment(),
            ('<', _) => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Node {
        text(self.consume_while(|c| c != '<'))
    }

    fn parse_comment(&mut self) -> Node {
        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '!');
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '-');
        let content = self.consume_while(|c| c != '-');
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '-');
        assert_eq!(self.consume_char(), '>');

        return comment(content);
    }

    fn parse_element(&mut self) -> Node {
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        return elem(tag_name, attrs, children);
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert_eq!(self.consume_char(), open_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes: HashMap<String, String> = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
}

#[cfg(test)]
mod test_html {
    use crate::dom::NodeType;
    use crate::html::{parse, Parser};

    #[test]
    fn test_parse() {
        let source = "<html><body id=\"name\">Hello World</body></html>".to_string();
        let nodes = parse(source);
        match nodes.node_type {
            NodeType::Element(data) => {
                assert_eq!(data.tag_name, "html".to_string());
                assert_eq!(data.attributes.len(), 0);
            }
            _ => panic!("error"),
        }
        for node in nodes.children {
            match node.node_type {
                NodeType::Element(data) => {
                    assert_eq!(data.tag_name, "body".to_string());
                    assert_eq!(data.attributes.contains_key("id"), true);
                    assert_eq!(data.attributes.get("id").unwrap(), "name");
                    println!("tag_name:{}", data.tag_name);
                    let (key, value) = data.attributes.get_key_value("id").unwrap();
                    println!("attribute:key={},value={}", key, value);
                }
                _ => panic!("error")
            }
        }
    }

    #[test]
    fn test_parse_with_comment() {
        let source = "<html><!-- 这是一行注释! --><body id=\"name\">Hello World</body></html>".to_string();
        let nodes = parse(source);
        match nodes.node_type {
            NodeType::Element(data) => {
                assert_eq!(data.tag_name, "html".to_string());
                assert_eq!(data.attributes.len(), 0);
            }
            _ => panic!("error"),
        }
        println!("{}", nodes.children.len());
        for node in nodes.children {
            match node.node_type {
                NodeType::Element(data) => {
                    assert_eq!(data.tag_name, "body".to_string());
                    assert_eq!(data.attributes.contains_key("id"), true);
                    assert_eq!(data.attributes.get("id").unwrap(), "name");
                    println!("tag_name:{}", data.tag_name);
                    let (key, value) = data.attributes.get_key_value("id").unwrap();
                    println!("attribute:key={},value={}", key, value);
                }
                NodeType::Comment(data) => {
                    assert_eq!(data, " 这是一行注释! ")
                }
                _ => panic!("error")
            }
        }
    }


    #[test]
    fn test_next_two_char() {
        let node = Parser {
            pos: 0,
            input: "abc".to_string(),
        };
        assert_eq!(node.next_two_char(), ('a', 'b'));
        match node.next_two_char() {
            ('a', 'b') => assert!(true),
            ('a', _) => panic!("error"),
            _ => panic!("error"),
        }
    }
}


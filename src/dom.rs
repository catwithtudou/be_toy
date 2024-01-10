use std::collections::HashMap;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;


pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, cd: Vec<Node>) -> Node {
    Node {
        children: cd,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data),
    }
}
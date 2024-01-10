use std::collections::HashMap;
use crate::dom::NodeType::Element;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;


fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn elem(name: String, attrs: AttrMap, cd: Vec<Node>) -> Node {
    Node {
        children: cd,
        node_type: Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}
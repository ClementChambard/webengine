use std::collections::HashMap;

pub struct Node {
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

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn text(data: String) -> Self {
        Self {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Self {
        Self {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            }),
        }
    }

    pub fn print(&self, indent: i32) {
        let mut indent_str = String::new();
        for _ in 0..indent {
            indent_str.push(' ');
        }
        match &self.node_type {
            NodeType::Text(s) => println!("{indent_str}{s}"),
            NodeType::Element(edata) => {
                let mut attr_str = String::new();
                for (k, v) in edata.attributes.iter() {
                    attr_str.push_str(&format!(" {k}=\"{v}\""));
                }
                if self.children.len() == 0 {
                    println!("{indent_str}<{}{attr_str} />", edata.tag_name);
                } else {
                    println!("{indent_str}<{}{attr_str}>", edata.tag_name);
                    for c in &self.children {
                        c.print(indent + 2);
                    }
                    println!("{indent_str}</{}>", edata.tag_name);
                }
            }
        }
    }
}

use std::collections::{HashMap, HashSet};

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    pub tag_name: String,
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

    pub fn print_self(&self) {
        match &self.node_type {
            NodeType::Text(s) => println!("{s}"),
            NodeType::Element(edata) => {
                let mut attr_str = String::new();
                for (k, v) in edata.attributes.iter() {
                    attr_str.push_str(&format!(" {k}=\"{v}\""));
                }
                println!("<{}{attr_str} />", edata.tag_name);
            }
        }
    }

    pub fn _print(&self, indent: i32) {
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
                        c._print(indent + 2);
                    }
                    println!("{indent_str}</{}>", edata.tag_name);
                }
            }
        }
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.get_attr("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }

    pub fn get_attr(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }
}

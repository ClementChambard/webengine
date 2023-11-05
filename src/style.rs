use std::collections::HashMap;

use crate::dom::{ElementData, Node, NodeType};

fn matches(elem: &ElementData, selector: &crate::css::Selector) -> bool {
    match *selector {
        crate::css::Selector::Simple(ref simple_selector) => {
            matches_simple_selector(elem, simple_selector)
        }
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &crate::css::SimpleSelector) -> bool {
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    let elem_classes = elem.classes();
    if selector
        .class
        .iter()
        .any(|class| !elem_classes.contains(&**class))
    {
        return false;
    }
    return true;
}

type PropertyMap = HashMap<String, crate::css::Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

impl<'a> StyledNode<'a> {
    pub fn print(&self) {
        self.node.print_self();
        for (k, v) in self.specified_values.iter() {
            println!(" {}: {};", k, v.to_string());
        }
        for c in &self.children {
            c.print();
        }
    }
}

type MatchedRule<'a> = (crate::css::Specificity, &'a crate::css::Rule);

fn match_rule<'a>(elem: &ElementData, rule: &'a crate::css::Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(
    elem: &ElementData,
    stylesheet: &'a crate::css::StyleSheet,
) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(elem, rule))
        .collect()
}

fn specified_values(elem: &ElementData, stylesheet: &crate::css::StyleSheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a crate::css::StyleSheet) -> StyledNode<'a> {
    let mut sn = StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    };
    if let NodeType::Element(elem) = &sn.node.node_type {
        if let Some(s) = elem.get_attr("style") {
            for d in &crate::css::Parser::parse(format!("rule{}{s}{}", '{', '}').as_str())
                .rules
                .iter()
                .next()
                .unwrap()
                .declarations
            {
                sn.specified_values.insert(d.name.clone(), d.value.clone());
            }
        }
    }
    sn
}

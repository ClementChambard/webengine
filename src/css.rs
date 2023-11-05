use std::format;

pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

impl StyleSheet {
    pub fn _print(&self) {
        for r in &self.rules {
            for (i, s) in r.selectors.iter().enumerate() {
                let Selector::Simple(s) = s;
                if let Some(t) = &s.tag_name {
                    print!("{}", t);
                }
                for c in &s.class {
                    print!(".{}", c);
                }
                if let Some(i) = &s.id {
                    print!("#{}", i);
                }
                if i < r.selectors.len() - 1 {
                    print!(", ");
                }
            }
            println!(" {}", '{');
            for d in &r.declarations {
                print!("    {}: {};", d.name, d.value.to_string());
            }
            println!("{}", '}');
        }
    }
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub enum Selector {
    Simple(SimpleSelector),
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}

pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

impl Value {
    pub fn to_string(&self) -> String {
        match &self {
            Value::Length(l, u) => format!("{}{}", l, u.to_string()),
            Value::ColorValue(c) => c.to_string(),
            Value::Keyword(s) => s.clone(),
        }
    }
}

#[derive(Clone)]
pub enum Unit {
    Px,
}

impl From<String> for Unit {
    fn from(s: String) -> Self {
        match s.as_str() {
            "px" => Unit::Px,
            _ => panic!("unknown unit {}", s),
        }
    }
}

impl Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Px => "px".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn parse(input: &str) -> StyleSheet {
        let mut parser = Parser {
            pos: 0,
            input: input.to_string(),
        };
        let mut rules = Vec::new();
        parser.consume_whitespace();
        while !parser.eof() {
            rules.push(parser.parse_rule());
            parser.consume_whitespace();
        }
        StyleSheet { rules }
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list.", c),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn hex_to_dec(&self, c1: &char, c2: Option<&char>) -> u8 {
        let n1 = hex(c1);
        if let Some(c2) = c2 {
            n1 * 16 + hex(c2)
        } else {
            ((n1 as u32) * 255 / 15) as u8
        }
    }

    fn parse_color(&mut self) -> Value {
        self.consume_char();
        let mut chars = Vec::new();
        loop {
            match self.next_char() {
                '0'..='9' | 'a'..='f' | 'A'..='F' => chars.push(self.consume_char()),
                _ => break,
            }
        }

        match chars.len() {
            3 => Value::ColorValue(Color {
                r: self.hex_to_dec(chars.iter().nth(0).unwrap(), None),
                g: self.hex_to_dec(chars.iter().nth(1).unwrap(), None),
                b: self.hex_to_dec(chars.iter().nth(2).unwrap(), None),
                a: 255,
            }),
            4 => Value::ColorValue(Color {
                r: self.hex_to_dec(chars.iter().nth(0).unwrap(), None),
                g: self.hex_to_dec(chars.iter().nth(1).unwrap(), None),
                b: self.hex_to_dec(chars.iter().nth(2).unwrap(), None),
                a: self.hex_to_dec(chars.iter().nth(3).unwrap(), None),
            }),
            6 => Value::ColorValue(Color {
                r: self.hex_to_dec(
                    chars.iter().nth(0).unwrap(),
                    Some(chars.iter().nth(1).unwrap()),
                ),
                g: self.hex_to_dec(
                    chars.iter().nth(2).unwrap(),
                    Some(chars.iter().nth(3).unwrap()),
                ),
                b: self.hex_to_dec(
                    chars.iter().nth(4).unwrap(),
                    Some(chars.iter().nth(5).unwrap()),
                ),
                a: 255,
            }),
            8 => Value::ColorValue(Color {
                r: self.hex_to_dec(
                    chars.iter().nth(0).unwrap(),
                    Some(chars.iter().nth(1).unwrap()),
                ),
                g: self.hex_to_dec(
                    chars.iter().nth(2).unwrap(),
                    Some(chars.iter().nth(3).unwrap()),
                ),
                b: self.hex_to_dec(
                    chars.iter().nth(4).unwrap(),
                    Some(chars.iter().nth(5).unwrap()),
                ),
                a: self.hex_to_dec(
                    chars.iter().nth(6).unwrap(),
                    Some(chars.iter().nth(7).unwrap()),
                ),
            }),
            _ => panic!("unknown color"),
        }
    }

    fn parse_num(&mut self) -> Value {
        let num = self.consume_while(|c| match c {
            '0'..='9' | '.' | '-' | '+' => true,
            _ => false,
        });
        if let Ok(num) = num.parse() {
            Value::Length(num, Unit::from(self.parse_identifier()))
        } else {
            panic!("not a number");
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '#' => self.parse_color(),
            '-' | '+' | '0'..='9' => self.parse_num(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_identifier();
        self.consume_whitespace();
        assert!(self.consume_char() == ':');
        self.consume_whitespace();
        let val = self.parse_value();
        self.consume_whitespace();
        assert!(self.consume_char() == ';');
        Declaration { name, value: val }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert!(self.consume_char() == '{');
        self.consume_whitespace();
        if self.next_char() == '}' {
            self.consume_char();
            return Vec::new();
        }
        let mut declarations = Vec::new();
        loop {
            declarations.push(self.parse_declaration());
            self.consume_whitespace();
            if self.next_char() == '}' {
                break;
            }
        }
        assert!(self.consume_char() == '}');
        declarations
    }
}

fn valid_identifier_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '-' | '0'..='9' => true,
        _ => false,
    }
}

fn hex(c: &char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => 0,
    }
}

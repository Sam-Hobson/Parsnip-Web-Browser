use crate::css::{Rule, Selector, SimpleSelector, Declaration, Value};
use crate::parsing::parser::{valid_standard_char, Parser};

pub struct CssParser {
    p: Parser,
}

impl CssParser {
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        // TODO: Make this not error prone.
        while !self.p.eof() {
            match self.p.next_char() {
                '#' => {
                    self.p.consume_char();
                    selector.id = Some(self.p.parse_standard_word());
                }
                '.' => {
                    self.p.consume_char();
                    selector.class.push(self.p.parse_standard_word());
                }
                '*' => {
                    self.p.consume_char();
                }
                c if valid_standard_char(c) => {
                    selector.tag_name = Some(self.p.parse_standard_word());
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
            self.p.consume_whitespace();

            match self.p.next_char() {
                ',' => {
                    self.p.consume_char();
                    self.p.consume_whitespace();
                }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c),
            }
        }

        selectors.sort_by(|x, y| y.specificity().cmp(&x.specificity()));
        return selectors;
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
    }

    fn parse_declaration(&mut self) -> Declaration {
        let key = self.p.parse_standard_word();
        self.p.consume_whitespace();
        assert_eq!(self.p.consume_char(), ':');
        self.p.consume_whitespace();
        let value = self.parse_value();
        self.p.consume_whitespace();
        assert_eq!(self.p.consume_char(), ';');

        Declaration {
            name: key,
            value: value
        }
    }

    fn parse_value(&mut self) -> Value {
    }
}

use super::new_tokenizer::Token;
use crate::{bytes::CharIterator, css3::new_tokenizer::Tokenizer};
use std::convert::From;

#[derive(Debug, PartialEq)]
struct Function {
    name: String,
    value: Vec<ComponentValue>,
}

#[derive(Debug, PartialEq)]
enum SimpleBlockTokenKind {
    Curly,
    Bracket,
    Paren,
}

impl From<Token> for SimpleBlockTokenKind {
    fn from(token: Token) -> SimpleBlockTokenKind {
        match token {
            _ if token.is_left_paren() => SimpleBlockTokenKind::Paren,
            _ if token.is_left_curl() => SimpleBlockTokenKind::Curly,
            _ if token.is_left_bracket() => SimpleBlockTokenKind::Bracket,
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct SimpleBlock {
    kind: SimpleBlockTokenKind,
    value: Vec<ComponentValue>,
}

#[derive(Debug, PartialEq)]
struct AtRule {
    name: String,
    prelude: Vec<ComponentValue>,
    block: Option<SimpleBlock>,
}

#[derive(Debug, PartialEq)]
struct QualifiedRule {
    prelude: Vec<ComponentValue>,
    block: SimpleBlock,
}

impl Default for QualifiedRule {
    fn default() -> Self {
        QualifiedRule {
            prelude: Vec::new(),
            block: SimpleBlock {
                kind: SimpleBlockTokenKind::Curly,
                value: Vec::new(),
            },
        }
    }
}

impl QualifiedRule {
    pub fn set_block(&mut self, block: SimpleBlock) {
        self.block = block;
    }

    pub fn add_prelude(&mut self, value: ComponentValue) {
        self.prelude.push(value)
    }
}

enum Rule {
    QualifiedRule(QualifiedRule),
    AtRule(AtRule),
}

#[derive(Debug, PartialEq)]
struct Declaration {
    name: String,
    value: Vec<ComponentValue>,
    important: bool,
}

#[derive(Debug, PartialEq)]
enum ComponentValue {
    /// Any token expect for `<function-token>`, `<{-token>`, `<(-token>`, and `<[-token>` (which are consumed in other higher-level objects)
    ///
    /// Note: `<}-token>`, `<)-token>`, `<]-token>`, `<bad-string-token>`, and `<bad-url-token>` are always parse errors.
    Token(Token),
    Function(Function),
    SimpleBlock(SimpleBlock),
}

// Parser output: at-rules, qualified rules, and/or declarations
pub struct CSS3Parser<'stream> {
    tokenizer: Tokenizer<'stream>,
}

impl<'stream> CSS3Parser<'stream> {
    pub fn new(tokenizer: Tokenizer) -> CSS3Parser {
        CSS3Parser { tokenizer }
    }

    pub fn from_input_stream(ci: &mut CharIterator) -> CSS3Parser {
        CSS3Parser::new(Tokenizer::new(ci))
    }

    /// [5.3.1. Parse something according to a CSS grammar](https://www.w3.org/TR/css-syntax-3/#parse-grammar)
    fn parse() {
        todo!()
    }

    /// [5.3.2. Parse A Comma-Separated List According To A CSS Grammar](https://www.w3.org/TR/css-syntax-3/#parse-comma-list)
    fn parse_comma_separated_list() {
        todo!()
    }

    /// [5.4.1. Consume a list of rules](https://www.w3.org/TR/css-syntax-3/#consume-list-of-rules)
    fn consume_rules_list(&mut self, is_top_level: bool) -> Vec<Rule> {
        let mut rules = Vec::new();

        loop {
            if self.current_token().is_whitespace() {
                self.tokenizer.consume();
                continue;
            }

            if self.current_token().is_eof() {
                break; // return rules list
            }

            if self.current_token().is_cdo() || self.current_token().is_cdc() {
                self.tokenizer.consume();

                if is_top_level {
                    continue; // do nothing
                }

                if let Some(rule) = self.consuem_qualified_rules() {
                    rules.push(Rule::QualifiedRule(rule));
                    continue;
                }
            }

            if self.current_token().is_at_keyword() {
                rules.push(Rule::AtRule(self.consume_at_rule()));
                continue;
            }

            if let Some(rule) = self.consuem_qualified_rules() {
                rules.push(Rule::QualifiedRule(rule));
                continue;
            }
        }

        rules
    }

    /// [5.4.2. Consume an at-rule](https://www.w3.org/TR/css-syntax-3/#consume-at-rule)
    fn consume_at_rule(&mut self) -> AtRule {
        let name = self.tokenizer.consume().to_string();
        let mut prelude = Vec::new();
        let mut block = None;

        loop {
            // eof: parser error
            if self.current_token().is_semicolon() || self.current_token().is_eof() {
                break; // return the block
            }

            if self.current_token().is_left_curl() {
                let token = self.tokenizer.consume();
                block = Some(self.consume_simple_block(&token));
                break; // return the block
            }

            prelude.push(self.consume_component_value());
        }

        AtRule {
            name,
            prelude,
            block,
        }
    }

    /// [5.4.3. Consume a qualified rule](https://www.w3.org/TR/css-syntax-3/#consume-qualified-rule)
    fn consuem_qualified_rules(&mut self) -> Option<QualifiedRule> {
        let mut rule = QualifiedRule::default();

        loop {
            // eof: parser error
            if self.current_token().is_eof() {
                return None;
            }

            if self.current_token().is_left_curl() {
                let token = self.tokenizer.consume();
                rule.set_block(self.consume_simple_block(&token));
                return Some(rule);
            }

            rule.add_prelude(self.consume_component_value());
        }
    }

    /// [5.4.4. Consume a style block’s contents](https://www.w3.org/TR/css-syntax-3/#consume-style-block)
    fn consume_style_block_content(&mut self) {
        // let declarations = Vec::new();
        // let rules = Vec::new();

        loop {
            let token = self.current_token();

            if token.is_whitespace() || token.is_semicolon() {
                self.tokenizer.consume();
                continue;
            }

            if token.is_eof() {
                // Extend decls with rules, then return decls.
            }

            if token.is_at_keyword() {
                // todo: consume at-rule
            }

            if token.is_ident() {
                // todo
            }

            if token == Token::Delim('&') {
                // todo: consume qualified rules
            }

            // anything else is a parser error
            // clean up: consume a component value and do nothing
            while !self.current_token().is_eof() && self.current_token().is_semicolon() {
                self.consume_component_value();
            }
        }
    }

    /// [5.4.5. Consume a list of declarations](https://www.w3.org/TR/css-syntax-3/#consume-list-of-declarations)
    fn consume_declaration_list(&mut self) -> Vec<Declaration> {
        let declarations = Vec::new();
        loop {
            let token = self.current_token();

            if token.is_whitespace() || token.is_semicolon() {
                self.tokenizer.consume();
                continue;
            }

            if token.is_eof() {
                break;
            };

            if token.is_at_keyword() {
                //todo: consume an at-rule
            }

            if token.is_ident() {
                let _list = vec![self.tokenizer.consume()];

                while !self.current_token().is_semicolon() && !self.current_token().is_eof() {
                    // todo: consume a component value
                }
            }
        }

        declarations
    }

    /// [5.4.6. Consume a declaration](https://www.w3.org/TR/css-syntax-3/#consume-declaration)
    fn consume_declaration(&mut self) -> Option<Declaration> {
        let name = self.tokenizer.consume().to_string();
        let mut value = Vec::new();

        while self.current_token().is_whitespace() {
            self.tokenizer.consume();
        }

        // parser error
        if !self.current_token().is_semicolon() {
            return None;
        }

        self.tokenizer.consume();

        while self.current_token().is_whitespace() {
            self.tokenizer.consume();
        }

        while !self.current_token().is_eof() {
            value.push(self.consume_component_value())
        }

        let len = value.len();

        let important = len > 2
            && value.last()
                == Some(&ComponentValue::Token(Token::Ident(
                    "important".to_string(),
                )))
            && value.get(len - 2) == Some(&ComponentValue::Token(Token::Delim('!')));

        Some(Declaration {
            name,
            value,
            important,
        })
    }

    /// [5.4.7. Consume a component value](https://www.w3.org/TR/css-syntax-3/#consume-a-component-value)
    fn consume_component_value(&mut self) -> ComponentValue {
        let token = self.tokenizer.consume();

        match token {
            t if t.is_left_curl() || t.is_left_bracket() || t.is_left_paren() => {
                ComponentValue::SimpleBlock(self.consume_simple_block(&t))
            }
            t if t.is_function() => ComponentValue::Function(self.consume_function()),
            t => ComponentValue::Token(t),
        }
    }

    /// [5.4.8. Consume a simple block](https://www.w3.org/TR/css-syntax-3/#consume-a-simple-block)
    fn consume_simple_block(&mut self, ending: &Token) -> SimpleBlock {
        let mut value = Vec::new();

        loop {
            // eof: parser error
            if self.current_token().is(ending) || self.current_token().is_eof() {
                break;
            }

            value.push(self.consume_component_value())
        }

        SimpleBlock {
            kind: SimpleBlockTokenKind::from(ending.clone()),
            value,
        }
    }

    /// [5.4.9. Consume a function](https://www.w3.org/TR/css-syntax-3/#consume-function)
    fn consume_function(&mut self) -> Function {
        let name = self.tokenizer.consume().to_string();
        let mut value = Vec::new();

        loop {
            let token = self.current_token();

            if token.is_left_paren() || token.is_eof() {
                // consume `(` or `EOF`
                self.tokenizer.consume();
                break;
            }

            value.push(self.consume_component_value());
        }

        Function { name, value }
    }

    fn current_token(&self) -> Token {
        self.tokenizer.lookahead(0)
    }

    fn next_token(&self) -> Token {
        self.tokenizer.lookahead(1)
    }
}

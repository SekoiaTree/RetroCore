use std::fmt::Debug;

pub mod simple_rules;

pub struct Tokenizer<Token, Error, Code, Parser, State>
where
    Parser: Fn(String, &mut State) -> Result<Vec<Token>, Error>,
{
    splitters: Vec<char>,
    parser: Parser,
    rules: Vec<(
        Box<dyn Fn(&Vec<Token>, &mut State) -> bool>,
        Box<dyn Fn(Vec<Token>, &mut State) -> Result<Vec<Code>, Error>>,
    )>,
}

impl<Token, Error, Code, Parser, State>
    Tokenizer<Token, Error, Code, Parser, State>
where
    Parser: Fn(String, &mut State) -> Result<Vec<Token>, Error>,
    Token: Debug
{
    pub fn new(parser: Parser) -> Self {
        Self {
            splitters: Vec::new(),
            parser,
            rules: Vec::new(),
        }
    }

    pub fn add_rule<X, Y>(mut self, condition: X, parse: Y) -> Self
    where
        X: Fn(&Vec<Token>, &mut State) -> bool + 'static,
        Y: Fn(Vec<Token>, &mut State) -> Result<Vec<Code>, Error> + 'static,
    {
        self.rules.push((Box::new(condition), Box::new(parse)));
        self
    }

    pub fn add_splitter(mut self, splitter: char) -> Self {
        self.splitters.push(splitter);
        self
    }

    fn process_line(&self, input: Vec<Token>, state: &mut State) -> Result<Vec<Code>, Error> {
        for rule in &self.rules {
            if rule.0(&input, state) {
                return rule.1(input, state);
            }
        }
        panic!("No rule has accepted the line {:?}!", input);
    }

    pub fn process(self, input: String, mut initial_state: State) -> Result<Vec<Code>, Error> {
        let mut output = Vec::new();
        for i in input.lines() {
            let mut token_list = vec![];
            for j in i.split(|c| self.splitters.contains(&c)) {
                token_list.append(&mut (self.parser)(j.to_string(), &mut initial_state)?);
            }
            output.append(&mut self.process_line(token_list, &mut initial_state)?)
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizing::Tokenizer;

    #[test]
    fn basic_test() {
        enum Token {
            Number(i32),
            Text(String),
        }
    }
}

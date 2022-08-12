pub mod simple_rules;

pub struct Tokenizer<T, E, C, P, N> where P : Fn(String) -> Result<Vec<T>, E>, N : Fn(Vec<T>) -> E {
    splitters : Vec<char>,
    parser : P,
    rules : Vec<(Box<dyn Fn(&Vec<T>) -> bool>, Box<dyn Fn(Vec<T>) -> Result<Vec<C>, E>>)>,
    no_match_fn : N,
}

impl<T, E, C, P, N> Tokenizer<T, E, C, P, N> where P : Fn(String) -> Result<Vec<T>, E>, N : Fn(Vec<T>) -> E {
    pub fn new(parser : P, no_match_fn : N) -> Self {
        Self {
            splitters : Vec::new(),
            parser,
            rules : Vec::new(),
            no_match_fn,
        }
    }

    pub fn add_rule<X, Y>(mut self, condition : X, parse : Y) -> Self where
        X : Fn(&Vec<T>) -> bool + 'static, Y : Fn(Vec<T>) -> Result<Vec<C>, E> + 'static {
        self.rules.push((Box::new(condition), Box::new(parse)));
        self
    }

    pub fn add_splitter(mut self, splitter : char) -> Self {
        self.splitters.push(splitter);
        self
    }

    fn process_line(&self, input : Vec<T>) -> Result<Vec<C>, E> {
        for rule in &self.rules {
            if rule.0(&input) {
                return rule.1(input);
            }
        }
        Err((self.no_match_fn)(input))
    }

    pub fn process(self, input : String) -> Result<Vec<C>, E> {
        let mut output = Vec::new();
        for i in input.lines() {
            for j in i.split(|c| self.splitters.contains(&c)) {
                output.append(&mut self.process_line((self.parser)(j.to_string())?)?);
            }
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
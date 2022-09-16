pub fn first_matches_condition<T>(required: T) -> impl Fn(&Vec<T>) -> bool where T: PartialEq {
    return move |input: &Vec<T>| input.first() == Some(&required);
}

pub fn matches_condition_at_index<T>(index: usize, required: T) -> impl Fn(&Vec<T>) -> bool where T: PartialEq {
    return move |input: &Vec<T>| input.get(index) == Some(&required);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasicToken {
    Number(i32),
    Text(String),
    Register(usize),
    Symbol(char),
}

pub fn parse_basic_token(_input: String) -> Result<Vec<BasicToken>, String> {
    let output = Vec::new();

    Ok(output)
}
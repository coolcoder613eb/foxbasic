use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct BasicParser;

fn main() {
    let bas = std::fs::read_to_string("test.bas").expect("Failed to read file.");
    let parse = BasicParser::parse(Rule::CONST, &bas);
    println!("{:#?}", parse);
}

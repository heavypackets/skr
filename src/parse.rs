use pest::Parser;

#[derive(Parser)]
#[grammar = "skr.pest"]
pub struct SkrParser;

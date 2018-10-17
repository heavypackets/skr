extern crate pest;
extern crate skr;

use pest::Parser;
use skr::parse::Rule;
use skr::parse::SkrParser;

fn main() {
    let unparsed_file = std::fs::read_to_string("examples/types.skr").expect("cannot read file");
    let file = SkrParser::parse(Rule::file, &unparsed_file)
        .unwrap_or_else(|e| panic!("{}", e))
        .next()
        .unwrap();

    for expression in file.into_inner() {
        if let Rule::type_def = expression.as_rule() {
            for inner in expression.into_inner() {
                match inner.as_rule() {
                    Rule::type_def_curry => {
                        let mut tokens = inner.into_inner();
                        let mut type_name = "";
                        let mut base_type = "";

                        for token in tokens {
                            match token.as_rule() {
                                Rule::type_name => type_name = token.as_str(),
                                Rule::base_type => base_type = token.as_str(),
                                _ => (),
                            }
                        }
                        println!("Type: {} as {}", type_name, base_type);
                    }
                    Rule::type_def_block => {}
                    _ => (),
                }
            }
        }
    }
}

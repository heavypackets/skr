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
        // Type declarations
        if let Rule::type_def = expression.as_rule() {
            for inner in expression.into_inner() {
                match inner.as_rule() {
                    // Simple type curry declaration
                    Rule::type_def_curry => {
                        let tokens = inner.into_inner();
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
                    // Complex type declaration
                    Rule::type_def_block => {
                        let block_elements = inner.into_inner();
                        for block_element in block_elements {
                            match block_element.as_rule() {
                                // Type name
                                Rule::type_name => {
                                    println!("Type: {} as", block_element.as_str());
                                }
                                // Enum block
                                Rule::type_enum_block => {
                                    let mut tokens = block_element.into_inner();
                                    let mut variant_names = Vec::new();

                                    // Enum type variants
                                    for token in tokens {
                                        if let Rule::type_name = token.as_rule() {
                                            variant_names.push(token.as_str());
                                        }
                                    }
                                    println!("   Variants: {:?}", variant_names);
                                }
                                // Composite type block ->
                                Rule::type_block => {
                                    for block_exp in block_element.into_inner() {
                                        match block_exp.as_rule() {
                                            // Subtype declaration
                                            Rule::type_name => {
                                                let type_name = block_exp.as_str();
                                                println!("   Subtype: {}", type_name);
                                            }
                                            // Atrribute assignment
                                            Rule::type_attribute_assigment => {
                                                let tokens = block_exp.into_inner();
                                                let mut type_name = "";
                                                let mut type_attribute = "";

                                                for token in tokens {
                                                    match token.as_rule() {
                                                        Rule::type_name => {
                                                            type_name = token.as_str()
                                                        }
                                                        Rule::type_attribute => {
                                                            type_attribute = token.as_str()
                                                        }
                                                        _ => (),
                                                    }
                                                }
                                                println!(
                                                    "   Type Attribute: {} = {}",
                                                    type_attribute, type_name
                                                );
                                            }
                                            // Subtype curry declaration
                                            Rule::subtype_def_curry => {
                                                let tokens = block_exp.into_inner();
                                                let mut type_name = "";
                                                let mut base_type = "";

                                                for token in tokens {
                                                    match token.as_rule() {
                                                        Rule::type_name => {
                                                            type_name = token.as_str()
                                                        }
                                                        Rule::base_type => {
                                                            base_type = token.as_str()
                                                        }
                                                        _ => (),
                                                    }
                                                }
                                                println!(
                                                    "   Subtype: {} as {}",
                                                    type_name, base_type
                                                );
                                            }
                                            // Subtype block
                                            Rule::subtype_block => {
                                                let mut tokens = block_exp.into_inner();
                                                let mut type_names = Vec::new();
                                                let base_type = match tokens.nth(0) {
                                                    Some(t) => t.as_str(),
                                                    None => unreachable!(),
                                                };

                                                // Subtype names
                                                for token in tokens {
                                                    if let Rule::type_name = token.as_rule() {
                                                        type_names.push(token.as_str())
                                                    }
                                                }
                                                println!(
                                                    "   Subtypes: {:?} as {}",
                                                    type_names, base_type
                                                );
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

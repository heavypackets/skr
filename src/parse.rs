use super::namespace::*;
use super::types::*;
use failure::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[derive(Parser)]
#[grammar = "skr.pest"]
pub struct SkrParser;

pub fn populate_namespace(input: &str, namespace: &mut Namespace) -> Result<(), Error> {
    let file = SkrParser::parse(Rule::file, input).unwrap().next().unwrap();

    for expression in file.into_inner() {
        if Rule::type_def == expression.as_rule() {
            parse_types(namespace, expression)?
        }
    }

    Ok(())
}

fn parse_types(namespace: &mut Namespace, pair: Pair<'_, Rule>) -> Result<(), Error> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_def_curry => {
                let parsed_type = parse_simple_type(namespace, inner);
                namespace.define_type(parsed_type)?;
            }
            Rule::type_def_block => {
                let parsed_type = parse_complex_type(namespace, inner);
                namespace.define_type(parsed_type)?;
            }
            _ => continue,
        }
    }

    Ok(())
}

fn parse_simple_type(namespace: &mut Namespace, pair: Pair<'_, Rule>) -> Type {
    let mut tokens = pair.into_inner();

    // Subtype definitions do not start with type keyword
    if Rule::kw_type == tokens.clone().nth(0).unwrap().as_rule() {
        tokens.next();
    }

    debug_assert_eq!(tokens.clone().nth(0).unwrap().as_rule(), Rule::type_name);
    let type_name = tokens.nth(0).unwrap().as_str();
    let type_class;

    let next = tokens.nth(1).unwrap();
    match next.as_rule() {
        Rule::type_list => {
            debug_assert_eq!(next.as_rule(), Rule::type_list);
            debug_assert_eq!(
                next.clone().into_inner().nth(1).unwrap().as_rule(),
                Rule::base_type_name
            );

            let list_type = next.into_inner().nth(1).unwrap().as_str();
            namespace.declare_type(&list_type);
            type_class = TypeClass::List(list_type.to_owned());
        }
        Rule::base_type_name => {
            debug_assert_eq!(next.as_rule(), Rule::base_type_name);
            let derived_type = next.as_str();
            namespace.declare_type(derived_type);
            type_class = TypeClass::Derived(derived_type.to_owned());
        }
        _ => unreachable!(),
    }

    let attributes = parse_type_attributes(namespace, tokens);

    Type {
        name: type_name.to_owned(),
        type_class,
        attributes,
    }
}

fn parse_type_attributes(
    namespace: &mut Namespace,
    pairs: Pairs<'_, Rule>,
) -> Option<TypeAttributes> {
    let mut attributes = TypeAttributes::new();
    for elements in pairs {
        if Rule::type_attribute_assigment == elements.as_rule() {
            let mut tokens = elements.into_inner();

            debug_assert_eq!(
                tokens.clone().nth(0).unwrap().as_rule(),
                Rule::type_attribute
            );
            let attribute_name = tokens.nth(0).unwrap().as_str();
            namespace.declare_type(attribute_name);

            debug_assert_eq!(
                tokens.clone().nth(0).unwrap().as_rule(),
                Rule::type_identifier
            );
            let attribute_value = tokens.nth(0).unwrap().as_str();
            namespace.declare_attribute_type(attribute_value);
            attributes.insert(attribute_name.to_owned(), attribute_value.to_owned());
        }
    }
    if attributes.is_empty() {
        None
    } else {
        Some(attributes)
    }
}

fn parse_complex_type(namespace: &mut Namespace, pair: Pair<'_, Rule>) -> Type {
    let mut tokens = pair.into_inner();

    debug_assert_eq!(tokens.clone().nth(1).unwrap().as_rule(), Rule::type_name);
    let type_name = tokens.nth(1).unwrap().as_str();

    for token in tokens {
        match token.as_rule() {
            Rule::type_enum_block => return parse_enum_type(namespace, type_name, token),
            Rule::type_block => return parse_composite_type(namespace, type_name, token),
            _ => continue,
        }
    }

    unreachable!()
}

fn parse_composite_type(namespace: &mut Namespace, type_name: &str, pair: Pair<'_, Rule>) -> Type {
    let tokens = pair.into_inner();

    let attributes = parse_type_attributes(namespace, tokens.clone());
    let mut derived_types = Types::new();
    let mut subtypes = TypeCollection::new();

    for expression in tokens {
        match expression.as_rule() {
            // Derived types
            Rule::type_name => {
                let type_name = expression.as_str();
                namespace.declare_type(type_name);
                derived_types.insert(type_name.to_owned());
            }
            // Subtype definition
            Rule::subtype_def_curry => {
                let new_type = parse_simple_type(namespace, expression);
                subtypes.insert(new_type.name.clone(), new_type);
            }
            // Subtype block definitions
            Rule::subtype_block => {
                let mut tokens = expression.into_inner();
                let base_token = tokens.nth(0).unwrap();
                let type_class = match base_token.as_rule() {
                    Rule::base_type_name => {
                        let base_type = base_token.as_str();
                        namespace.declare_type(base_type);
                        TypeClass::Derived(base_type.to_owned())
                    }
                    Rule::type_list => {
                        debug_assert_eq!(
                            base_token.clone().into_inner().nth(1).unwrap().as_rule(),
                            Rule::base_type_name
                        );
                        let base_type = base_token.into_inner().nth(1).unwrap().as_str();
                        namespace.declare_type(base_type);

                        TypeClass::List(base_type.to_owned())
                    }
                    _ => unreachable!(),
                };

                for token in tokens {
                    if Rule::type_name == token.as_rule() {
                        let mut subtype_name = token.as_str();
                        subtypes.insert(
                            subtype_name.to_owned(),
                            Type {
                                name: subtype_name.to_owned(),
                                attributes: None,
                                type_class: type_class.clone(),
                            },
                        );
                    }
                }
            }
            _ => continue,
        }
    }

    Type {
        name: type_name.to_owned(),
        attributes,
        type_class: TypeClass::Composite {
            derived_types,
            subtypes,
        },
    }
}

fn parse_enum_type(namespace: &mut Namespace, type_name: &str, pair: Pair<'_, Rule>) -> Type {
    let tokens = pair.into_inner();
    let mut variant_names = EnumVariants::new();

    let attributes = parse_type_attributes(namespace, tokens.clone());

    for token in tokens {
        if Rule::type_enum_variant == token.as_rule() {
            variant_names.insert(token.as_str().to_owned());
        }
    }

    Type {
        name: type_name.to_owned(),
        type_class: TypeClass::Enum(variant_names),
        attributes,
    }
}

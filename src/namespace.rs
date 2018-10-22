use super::error::TypeError;
use super::types::*;
use failure::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Namespace {
    name: String,
    types: TypeCollection,
}

impl Namespace {
    pub fn new(name: &str) -> Namespace {
        Namespace {
            name: name.to_owned(),
            types: TypeCollection::new(),
        }
    }

    pub fn declare_attribute_type(&mut self, name: &str) {
        /* TODO - In the future, attributes can be assigned enum variants -- we'll need to check if name points to appropriate variant for
        the attribute, and if so, differenciate it from attribute value that simple points to a type */
        self.declare_type(name);
    }

    pub fn declare_type(&mut self, name: &str) {
        if !self.types.contains_key(name) {
            self.types.insert(
                name.to_owned(),
                Type {
                    name: name.to_owned(),
                    type_class: TypeClass::Unknown,
                    attributes: None,
                },
            );
        }
    }

    pub fn define_type(&mut self, new_type: Type) -> Result<(), Error> {
        if let Some(replaced) = self.types.insert(new_type.name.clone(), new_type) {
            if replaced.type_class != TypeClass::Unknown {
                Err(TypeError::MultipleDefinitions {
                    type_name: replaced.name,
                })?
            }
        }

        Ok(())
    }

    pub fn make_type<F>(&mut self, name: &str, make_type: F)
    where
        F: Fn() -> Type,
    {
        if !self.types.contains_key(name) {
            self.types.insert(name.to_owned(), make_type());
        }
    }
}

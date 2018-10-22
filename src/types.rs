use std::collections::{HashMap, HashSet};

pub type TypeName = String;
pub type TypeAttributeKey = String;
pub type TypeAtrributeValue = String;

pub type TypeAttributes = HashMap<TypeAttributeKey, TypeAtrributeValue>;
pub type TypeCollection = HashMap<TypeName, Type>;
pub type Types = HashSet<TypeName>;
pub type EnumVariants = HashSet<TypeName>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum TypeClass {
    Native(NativeType),
    Derived(TypeName),
    Enum(EnumVariants),
    Composite {
        derived_types: Types,
        subtypes: TypeCollection,
    },
    List(TypeName),
    Unknown,
}

// TODO - Store span of where type was defined
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Type {
    pub name: TypeName,
    pub attributes: Option<TypeAttributes>,
    pub type_class: TypeClass,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum NativeType {
    Number,
    Float,
    String,
    Boolean,
    Date,
    Time,
}

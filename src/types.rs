use std::collections::HashMap;

type TypeIdentifier = String;
type TypeAttributeKey = String;
type TypeAtrributeValue = String;

type TypeAttributes = HashMap<TypeAttributeKey, TypeAtrributeValue>;
type SubTypes = Vec<SimpleType>;

struct SimpleType {
    name: TypeIdentifier,
    attributes: TypeAttributes,
    base_type: BaseType,
}

struct CompositeType {
    name: TypeIdentifier,
    attributes: TypeAttributes,
    sub_types: SubTypes,
}

enum BaseType {
    Number,
    String,
    Boolean,
    List(TypeIdentifier),
}

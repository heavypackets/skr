use super::types::TypeName;

#[derive(Fail, Debug)]
pub enum ParseError {
    #[fail(display = "IO error: {}", error)]
    IoError { error: ::std::io::Error },
}

#[derive(Fail, Debug)]
pub enum TypeError {
    #[fail(display = "Type {} defined multiple times", type_name)]
    MultipleDefinitions { type_name: TypeName },
}

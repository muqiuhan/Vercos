/// Copyright (C) 2023 Muqiu Han
use crate::error::Log;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Object {
    Malformed(String, usize),
    UnknownType(String, String),
}

impl Log for Object {
    fn fmt(&self) -> String {
        match self {
            Object::Malformed(sha, len) => {
                format!("Malformed object `{}`: bad length `{}`", sha, len)
            }
            Object::UnknownType(typ, sha) => {
                format!("Unknown type `{}` for object `{}`", typ, sha)
            }
        }
    }
}

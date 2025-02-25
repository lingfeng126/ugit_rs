use std::fmt::Formatter;

use crate::data;

use super::object_type::ObjectTypes;

pub struct Blob{
    content: Vec<u8>
}

impl std::fmt::Display for Blob{
    fn fmt(&self, f:&mut Formatter) -> std::fmt::Result{
        write!(f, "{}", String::from_utf8_lossy(&self.content))
    }
}

impl Blob{
    pub fn new(content: Vec<u8>) -> Self{
        Blob{
            content
        }
    }

    pub fn from_oid(oid: &String) -> Self{
        let content = data::get_object(oid, ObjectTypes::Blob);
        Self::new(content)
    }
}
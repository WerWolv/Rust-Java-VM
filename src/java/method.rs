#![allow(dead_code)]

use crate::java;
use crate::java::Attribute;

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub descriptor: String,

    pub attributes: Vec<java::Attribute>
}

impl Method {

    pub fn new(class_file: &java::ClassFile, method_info: &java::class::MethodInfo) -> Option<Self> {
        if let Some(name) = class_file.get_constant_pool_string(method_info.name_index as usize) {
            if let Some(descriptor) = class_file.get_constant_pool_string(method_info.descriptor_index as usize) {
                let mut attributes = vec![];

                for attribute in &method_info.attributes {
                    if let Some(attribute) = Attribute::new(&class_file, &attribute) {
                        attributes.push(attribute);
                    }
                }

                return Some(Method {
                    name,
                    descriptor,
                    attributes
                })
            }
        }

        None
    }

}
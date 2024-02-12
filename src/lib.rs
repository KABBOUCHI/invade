use std::{any::Any, str};

pub use invade_derive::*;

pub type AccessHelper = for<'a, 'b> fn(&'a dyn Any) -> dyn Any;

pub struct Field<'a, Element: Sized> {
    pub name: String,
    pub get_ptr: Option<fn(&'a Element) -> &'a dyn Any>,
    pub set_ptr: Option<fn(&'a mut Element, Box<dyn Any>)>,
}

pub struct Invaded<'a, Element: Sized> {
    value: Element,
    fields: Vec<Field<'a, Element>>,
}

impl<'a, Element> Invaded<'a, Element> {
    pub fn new(value: Element, fields: Vec<Field<'a, Element>>) -> Self {
        Self { value, fields }
    }

    pub fn set<T: Any + Send + Sync>(&'a mut self, name: &str, value: T){
        let field = self.fields.iter().find(|field| field.name == name);

        if let Some(field) = field {
            if let Some(ptr) =  field.set_ptr {
                ptr(&mut self.value, Box::new(value));
            }
        }
    }

    pub fn get<T: Any + Send + Sync + Clone>(&'a self, name: &str) -> Option<T> {
        let field = self.fields.iter().find(|field| field.name == name);

        if let Some(field) = field {
            if let Some(ptr) =  field.get_ptr {
                let value = ptr(&self.value);
                let value = value.downcast_ref::<T>();

                return value.cloned();
            }
        }

        None
    }

    pub fn call<T: Any + Send + Sync>(&mut self, name: &str, args: T) {
        todo!("call the method with the given key")
    }

    pub fn restore(self) -> Element {
        self.value
    }
}

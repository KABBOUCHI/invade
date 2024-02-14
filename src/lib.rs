use std::{any::Any, str};

pub use invade_derive::*;

pub type AccessHelper = for<'a, 'b> fn(&'a dyn Any) -> dyn Any;

#[derive(Debug)]
pub struct Field<'a, Element: Sized> {
    pub name: String,
    pub get_ptr: Option<fn(&'a Element) -> &'a dyn Any>,
    pub set_ptr: Option<fn(&'a mut Element, Box<dyn Any>)>,
}

#[derive(Debug)]
pub struct Method<'a, Element: Sized + InvadedMethods<'a, Element>> {
    pub name: String,
    pub ptr: Option<fn(&'a mut Element, Vec<Box<dyn std::any::Any + Send + Sync>>)>,
}

pub trait InvadedMethods<'a, Element: Sized + InvadedMethods<'a, Element>> {
    fn invaded_methods(&self) -> Vec<Method<'a, Element>>;
}

pub struct Invaded<'a, Element: Sized + InvadedMethods<'a, Element>> {
    value: Element,
    fields: Vec<Field<'a, Element>>,
}

impl<'a, Element: Sized + InvadedMethods<'a, Element>> Invaded<'a, Element> {
    pub fn new(value: Element, fields: Vec<Field<'a, Element>>) -> Self {
        Self { value, fields }
    }

    pub fn set<T: Any + Send + Sync>(&'a mut self, name: &str, value: T) {
        let field = self.fields.iter().find(|field| field.name == name);

        if let Some(field) = field {
            if let Some(ptr) = field.set_ptr {
                ptr(&mut self.value, Box::new(value));
            }
        }
    }

    pub fn get<T: Any + Send + Sync + Clone>(&'a self, name: &str) -> Option<T> {
        let field = self.fields.iter().find(|field| field.name == name);

        if let Some(field) = field {
            if let Some(ptr) = field.get_ptr {
                let value = ptr(&self.value);
                let value = value.downcast_ref::<T>();

                return value.cloned();
            }
        }

        None
    }

    pub fn call(&'a mut self, name: &str, args: Vec<Box<dyn std::any::Any + Send + Sync>>) {
        let y: &dyn InvadedMethods<Element> = &self.value;
        let methods = &y.invaded_methods();
        let method = methods.iter().find(|method| method.name == name);
        
        if let Some(method) = method {
            if let Some(ptr) = method.ptr {
                ptr(&mut self.value, args);
            }
        }
    }

    pub fn restore(self) -> Element {
        self.value
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     struct Test {
//         name: String,
//         age: i32,
//     }

//     impl Test {
//         fn grow(&mut self) {
//             self.age += 1;
//         }
//     }

//     #[test]
//     fn test_field() {
//         let mut test = Test {
//             name: "test".to_string(),
//             age: 20,
//         };
//         let invaded = Invaded::new(
//             &mut test,
//             vec![
//                 Field {
//                     name: "name".to_string(),
//                     get_ptr: Some(|test| &test.name as &dyn Any),
//                     set_ptr: Some(|test, value| test.name = *value.downcast::<String>().unwrap()),
//                 },
//                 Field {
//                     name: "age".to_string(),
//                     get_ptr: Some(|test| &test.age as &dyn Any),
//                     set_ptr: Some(|test, value| test.age = *value.downcast::<i32>().unwrap()),
//                 },
//             ],
//         );

//         assert_eq!(invaded.get::<String>("name"), Some("test".to_string()));
//         assert_eq!(invaded.get::<i32>("age"), Some(20));
//     }

//     // #[test]
//     // fn test_methods() {
//     //     let mut test = Test {
//     //         name: "test".to_string(),
//     //         age: 20,
//     //     };

//     //     let mut invaded = Invaded::new(
//     //         &mut test,
//     //         vec![Field {
//     //             name: "age".to_string(),
//     //             get_ptr: Some(|test| &test.age as &dyn Any),
//     //             set_ptr: Some(|test, value| test.age = *value.downcast::<i32>().unwrap()),
//     //         }],
//     //         vec![Method {
//     //             name: "grow".to_string(),
//     //             ptr: Some(|test, _| {
//     //                 test.grow();
//     //             }),
//     //         }],
//     //     );

//     //     invaded.call("grow", vec![]);

//     //     let invaded = Invaded::new(
//     //         &mut test,
//     //         vec![
//     //             Field {
//     //                 name: "name".to_string(),
//     //                 get_ptr: Some(|test| &test.name as &dyn Any),
//     //                 set_ptr: Some(|test, value| test.name = *value.downcast::<String>().unwrap()),
//     //             },
//     //             Field {
//     //                 name: "age".to_string(),
//     //                 get_ptr: Some(|test| &test.age as &dyn Any),
//     //                 set_ptr: Some(|test, value| test.age = *value.downcast::<i32>().unwrap()),
//     //             },
//     //         ],
//     //         vec![],
//     //     );

//     //     assert_eq!(invaded.get::<i32>("age"), Some(21));
//     // }
// }

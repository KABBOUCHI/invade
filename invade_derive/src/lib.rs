extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Invade)]
pub fn derive_invade(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_invade(&ast)
}

fn impl_invade(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => f.named.to_owned(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

   
    let mut field_items = proc_macro2::TokenStream::new();

    for field in fields {
        let field_name = field.ident.unwrap(); // Field name
        let field_str = field_name.to_string();
        let field_type = field.ty; // Field type

        let token = quote!(
            invade::Field {
                name: #field_str.to_string(),
                get_ptr: Some(|s| -> &dyn std::any::Any {
                    &s.#field_name as &dyn std::any::Any
                }),
                set_ptr: Some(|s, v| {
                    if let Some(v) = v.downcast_ref::<#field_type>() {
                        s.#field_name = (*v).clone();
                    }
                }),
            },
        );
        field_items.extend(token);
    }
   
    let gen = quote! {
        impl #name {
            fn invade(&mut self) -> invade::Invaded<&mut Self> {
                invade::Invaded::new(
                    self,
                    vec![
                        #field_items
                    ],
                    vec![
                        invade::Method {
                            name: "inc".to_string(),
                            ptr: Some(|s, _| {
                                s.inc();
                            }),
                        }
                    ],
                )
            }

            fn invade_get<T: std::any::Any + Send + Sync + Clone>(&mut self, field: &str) -> Option<T> {
                self.invade().get::<T>(field)
            }

            pub fn invade_set<T: std::any::Any + Send + Sync + Clone>(&mut self, field: &str, value: T) {
                self.invade().set(field, value);
            }

            pub fn invade_call(&mut self, method: &str) {
                self.invade().call(method);
            }
        }
    };
    gen.into()
}

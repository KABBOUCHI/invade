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
    let gen = quote! {
        impl #name {
            fn invade(&mut self) -> invade::Invaded<&mut Self> {
                invade::Invaded::new(
                    self,
                    vec![
                        invade::Field {
                            name: "count".to_string(),
                            get_ptr: Some(|s| -> &dyn std::any::Any {
                                &s.count as &dyn std::any::Any
                            }),
                            set_ptr: Some(|s, v| {
                                if let Some(v) = v.downcast_ref::<u32>() {
                                    s.count = *v;
                                }
                            }),
                        }
                    ]
                )
            }

            fn invade_get<T: std::any::Any + Send + Sync + Clone>(&mut self, field: &str) -> Option<T> {
                self.invade().get::<T>(field)
            }

            pub fn invade_set<T: std::any::Any + Send + Sync + Clone>(&mut self, field: &str, value: T) {
                self.invade().set(field, value);
            }
        }
    };
    gen.into()
}

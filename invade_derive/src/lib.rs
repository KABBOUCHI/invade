use quote::quote;
use syn::parse_macro_input;

fn impl_invade(
    _attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let orig_item: proc_macro2::TokenStream = item.clone().into();
    if orig_item.to_string().contains("impl") {
        let input = parse_macro_input!(item as syn::ItemImpl);
        let mut methods = proc_macro2::TokenStream::new();

        for item in input.items {
            if let syn::ImplItem::Fn(method) = item {
                let method_name = method.sig.ident;
                let token = quote!(
                    invade::Method {
                        name: stringify!(#method_name).to_string(),
                        ptr: Some(|s, _| {
                            s.#method_name();
                        }),
                    },
                );
                methods.extend(token);
            }
        }

        return quote!(
            #orig_item

            impl<'a> invade::InvadedMethods<'a, &'a mut Counter> for &'a mut Counter {
                fn invaded_methods(&self) -> Vec<invade::Method<'a, &'a mut Counter>> {
                    vec![
                        #methods
                     ]
                }
            }
        )
        .into();
    }

    let input = parse_macro_input!(item as syn::DeriveInput);

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
        #orig_item

        // impl<'a> invade::InvadedMethods<'a, &'a mut Counter> for &'a mut Counter {
        //     fn invaded_methods(&self) -> Vec<invade::Method<'a, &'a mut Counter>> {
        //         vec![]
        //     }
        // }

        impl #name {
            fn invade(&mut self) -> invade::Invaded<&mut Self> {
                invade::Invaded::new(
                    self,
                    vec![
                        #field_items
                    ]
                )
            }

            pub fn invade_get<T: std::any::Any + Send + Sync + Clone>(&mut self, field: &str) -> Option<T> {
                self.invade().get::<T>(field)
            }

            pub fn invade_set<T: std::any::Any + Send + Sync + Clone>(&mut self, field: &str, value: T) {
                self.invade().set(field, value);
            }

            pub fn invade_call(&mut self, method: &str, args: Vec<Box<dyn std::any::Any + Send + Sync>>) {
                self.invade().call(method, args);
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn invade(
    attrs: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let res = impl_invade(attrs.into(), item.into());

    res
}

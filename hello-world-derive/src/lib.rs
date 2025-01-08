extern crate proc_macro;
// extern crate syn;

use syn;
use syn::{parse_macro_input, DeriveInput, Field};

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld, attributes(arg))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    println!(" {s}");

    // Parse the string representation
    // let ast = syn::parse_derive_input(&s).unwrap();
    let ast = parse_macro_input!(input as DeriveInput);
    // println!("--- ast.attrs     {:#?}", ast.attrs);
    // println!("--- ast.ident     {:#?}", ast.ident);
    // println!("--- ast.data      {:#?}", ast.data);
    // println!("--- ast.body      {:#?}", ast.body);
    // println!("--- ast.generics  {:#?}", ast.generics);

    let struct_name = &ast.ident;
    let mut field_name = None;

    if let syn::Data::Struct(ref data_struct) = ast.data {
        for field in &data_struct.fields {

            // For the "arg" macro.
            if field.attrs.iter().any(|attr| attr.path().is_ident("arg")) {

                // Find the field identifier.
                if let Some(field_ident) = &field.ident {

                    // Get the type of the field
                    if let syn::Type::Path(type_path) = &field.ty {

                        // If the type is i32 do something...
                        if type_path.path.is_ident("i32") {
                            println!("------ i32 {}", field_ident);
                            return i32_handler(field, &ast);
                        }

                        // If the type is string, do something else.
                        if type_path.path.is_ident("String") {
                            println!("------ string {}", field_ident);
                        }

                        let val = type_path.path.get_ident();
                        println!("------- type: {:#?}", val);
                    }

                    field_name = Some(field_ident);
                }
            }
        }
    }

    // If we found a field with #[arg], generate the method
    if let Some(field_ident) = field_name {
        let gen;

        gen = quote! {
            impl #struct_name {

                pub fn getTest(&self) {
                    println!("---- {}", #field_ident);
                }
            }
        };

        // let gen = quote! {
        //     impl #struct_name {
        //         pub fn getTest(&self) {
        //             println!("{} = {} --", stringify!(#field_ident), self.#field_ident);
        //         }
        //     }
        // };
        gen.into()
    } else {
        // If no #[arg] attribute is found, return an empty TokenStream
        TokenStream::new()
    }

    // Build the impl
    // let gen = impl_hello_world(&ast);
    //
    // // Return the generated impl
    // gen.parse().unwrap()
}

// fn impl_hello_world(ast: &syn::DeriveInput) -> quote::ToTokens {
//     let name = &ast.ident;
//     quote! {
//         impl HelloWorld for #name {
//             fn hello_world() {
//                 println!("Hello, World! My name is {}", stringify!(#name));
//             }
//         }
//     }
// }

fn i32_handler(field: &Field, ast: &DeriveInput) ->  TokenStream {
    let struct_name = &ast.ident;

    // Find the field identifier.
    if let Some(field_ident) = &field.ident {

        // Get the type of the field
        if let syn::Type::Path(type_path) = &field.ty {

            // If the type is i32 do something...
            if type_path.path.is_ident("i32") {
                // println!("------ i32 {} {}", field_ident, self.#field_ident);

                return quote! {
                    impl #struct_name {
                        pub fn getTest(&mut self) {
                            self.#field_ident = 69;
                            println!("{} = {} -- new", stringify!(#field_ident), self.#field_ident);
                        }
                    }
                }.into();
            }
        }
    }

    TokenStream::new()
}


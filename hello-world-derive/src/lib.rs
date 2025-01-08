extern crate proc_macro;

use syn::{parse_macro_input, DeriveInput, Field, Type};

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld, attributes(arg))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

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
        let mut gen = quote! {};

        gen.extend(quote! {
            impl #struct_name {

                pub fn getTest(&self) {
                    println!("---- {}", #field_ident);
                }
            }
        });

        gen.extend(quote! {
            impl #struct_name {
                pub fn getTest(&self) {
                    println!("{} = {} --", stringify!(#field_ident), self.#field_ident);
                }
            }
        });

        gen.into()
    } else {
        // If no #[arg] attribute is found, return an empty TokenStream
        TokenStream::new()
    }
}

fn i32_handler(field: &Field, ast: &DeriveInput) ->  TokenStream {
    let struct_name = &ast.ident;

    // Find the field identifier.
    if let Some(field_ident) = &field.ident {

        // Get the type of the field
        if let syn::Type::Path(type_path) = &field.ty {

            // If the type is i32 do something...
            if type_path.path.is_ident("i32") {

                // Create the macro with the field to modify
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


#[proc_macro_derive(Hola, attributes(arg))]
pub fn hola(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);
    impl_parse(&ast)
}

fn impl_parse(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Collect the field names and types
    let mut field_inits = Vec::new(); // To store field initializations
    let mut field_names = Vec::new();  // To store field names for `call` method

    if let syn::Data::Struct(ref data_struct) = ast.data {

        // Iterate over the fields of the struct
        for field in &data_struct.fields {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;

            // Store the field name for the `call` method
            field_names.push(field_name);

            // Generate code to initialize the field
            let init_value = match field_type {
                // If the field is `i32`, we initialize it to `10`
                Type::Path(type_path) if type_path.path.is_ident("i32") => quote! { 10 },
                // If the field is `String`, we initialize it to `String::from("hello")`
                Type::Path(type_path) if type_path.path.is_ident("String") => quote! { String::from("hello") },
                // Handle other types if needed (you can extend this part)
                _ => quote! { Default::default() }, // Default initialization for other types
            };

            // Add the field initialization to the list
            field_inits.push(quote! { #field_name: #init_value });
        }
    }

    let mut gen = quote! {};

    gen.extend(quote! {
        impl #name {
            pub fn parse() -> Self {
                Self {
                    #(#field_inits),*
                }
            }
        }
    });

    gen.extend(quote! {
        impl #name {
            pub fn call(&self) {
                println!("---- test ");
            }
        }
    });

    gen.into()
}


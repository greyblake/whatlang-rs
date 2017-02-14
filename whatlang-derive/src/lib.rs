extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumFromString)]
pub fn from_string_builder(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_from_string(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_from_string(ast: &syn::MacroInput) -> quote::Tokens {
    let enum_name = &ast.ident;

    let variants = match ast.body {
        syn::Body::Enum(ref variants) => variants,
        _ => panic!("#[derive(EnumFromString)] can only be used with enums")
    };

    // generate match expression line by line
    let mut matcher: quote::Tokens = quote::Tokens::new();
    let mut variants_names: Vec<String> = Vec::with_capacity(variants.len());
    for variant in variants {
        let variant_name = variant.ident.to_string();
        let match_tokens = quote! {
            #variant_name => Ok(#enum_name::#variant),
        };
        matcher.append(match_tokens.as_str());
        variants_names.push(variant_name);
    }

    quote! {
        impl ::std::str::FromStr for #enum_name {
            type Err = ();
            fn from_str(s: &str) -> Result<#enum_name, ()> {
                match s {
                    #matcher
                    _ => Err(())
                }
            }
        }

        impl ::std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, f)
            }
        }

        impl #enum_name {
            fn get_variants() -> Vec<&'static str> {
                vec!(#(#variants_names),*)
            }
        }
    }
}

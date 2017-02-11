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
    let name = &ast.ident;

    let fields = match ast.body {
        syn::Body::Enum(ref fields) => fields,
        syn::Body::Struct(_) => panic!("#[derive(EnumFromString)] can only be used with enums")
    };

    // generate match expression line by line
    let mut matcher: quote::Tokens = quote::Tokens::new();
    for field in fields {
        let field_name = field.ident.to_string();
        let match_tokens = quote! {
            #field_name => Some(#name::#field),
        };
        matcher.append(match_tokens.as_str());
    }

    quote! {
        impl EnumFromString for #name {
            type EnumType = #name;
            fn from_string<S: Into<String>>(s: S) -> Option< #name > {
                match s.into().as_ref() {
                    #matcher
                    _ => None
                }
            }
        }
    }
}

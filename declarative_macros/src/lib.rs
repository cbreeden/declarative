#![allow(warnings)]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;

use syn::Body;
use syn::VariantData;

mod attributes;

#[proc_macro_derive(Declarative)]
pub fn declarative_init(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let input = syn::parse_derive_input(&source).expect("failed to parse derive input");
    let expanded = match declarative(input) {
        Ok(expanded) => {
            panic!("{}", expanded.to_string());
            expanded.parse()
        }
        Err(err) => panic!("declarative failed: {}", err),
    };

    expanded.unwrap()
    // unimplemented!()
}

fn declarative(input: syn::DeriveInput) -> Result<Tokens, String> {
    let ident = input.ident;
    let (lifetimes, ty_params, where_clause) = (
        input.generics.lifetimes,
        input.generics.ty_params,
        input.generics.where_clause,
    );

    let fields = match input.body {
        Body::Struct(VariantData::Struct(fields)) => fields,
        _ => return Err("declarative is only implemented for struct-type structs".into()),
    };

    let parse_body = decl_fields(&fields);
    let return_type = decl_return(&fields);
    let parse = quote! {
        impl<'buf> Declarative<'buf> for OffsetTable<'buf> {
            fn parse(mut buffer: &'buf [u8]) -> DeclResult<'buf, Self> {
                #parse_body
                #return_type
            }
        }
    };

    Ok(parse)
}

fn decl_fields(fields: &[syn::Field]) -> Tokens {
    // Ignore references
    let fields = fields.iter().filter(|field| match field.ty {
        syn::Ty::Rptr(_, _) => false,
        _ => true,
    });

    let body = fields.map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;

        let attrs = attributes::decl_field_attributes(&field.attrs);
        let parameters = attrs.parameters;
        let params = quote!( ( #parameters ,) );
        let params = match attrs.array_length {
            Some(len) => quote!( (#len, #params) ),
            None => params,
        };

        quote!(
            let #ident = buffer.parse_with::<#ty>(#params);
        )
    });

    quote!( #(#body)* )
}

fn decl_return(fields: &[syn::Field]) -> Tokens {
    let fields = fields.iter().map(|field| &field.ident);
    quote! {
        Ok((
            OffsetTable {
                #(#fields),*
            },
            buffer,
        ))
    }
}

// // Generate implementation
// impl<'buf> Declarative<'buf> for OffsetTable<'buf> {
//     fn parse(mut buffer: &'buf [u8]) -> DeclResult<'buf, Self> {

//          // Parse Data
//         let version = buffer.parse::<Version>()?;
//         let num_tables = buffer.parse::<u16>()?;
//         let search_range = buffer.parse::<u16>()?;
//         let entry_selector = buffer.parse::<u16>()?;
//         let range_shift = buffer.parse::<u16>()?;
//         let tables = buffer.parse_array::<TableRecord>(num_tables as usize)?;

//         // Return type
//         Ok((
//             OffsetTable {
//                 buffer,
//                 version,
//                 num_tables,
//                 search_range,
//                 entry_selector,
//                 range_shift,
//                 tables,
//             },
//             buffer,
//         ))
//     }
// }

use crate::valar::{DeriveField, DeriveStruct, get_fields_from_derive_struct};
use darling::{
    FromMeta,
    ast::{Fields, Style},
    util::WithOriginal,
};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone, FromMeta)]
#[darling(and_then = "NewAttr::validate")]
pub(crate) struct NewAttr {
    into_iter: Option<bool>,
    default: Option<bool>,
    // Let see if we could improve this
    value: Option<String>,
    into: Option<bool>,
}

impl NewAttr {
    fn validate(self) -> darling::Result<Self> {
        Ok(self)
    }
}

pub fn generate_new(derive_struct: &DeriveStruct) -> TokenStream {
    let fields = get_fields_from_derive_struct(derive_struct);
    let (imp, ty, whr) = derive_struct.generics.split_for_impl();
    let derive_struct_ident = &derive_struct.ident;
    let token_stream = match fields.style {
        Style::Tuple => generate_for_tuple(fields, derive_struct),
        Style::Struct => generate_for_struct(fields, derive_struct),
        _ => unreachable!(),
    };

    quote! {
        impl #imp #derive_struct_ident #ty #whr {
            #token_stream
        }
    }
}

fn generate_for_struct(fields: &Fields<WithOriginal<DeriveField, syn::Field>>, derive_struct: &DeriveStruct) -> TokenStream {
    let token_stream: TokenStream = fields
        .fields
        .iter()
        .fold(TokenStream::new(), |accumulator, current_field| {
            let WithOriginal { parsed, original: _ } = current_field;
            let field_name = &parsed.ident;
            let field_type = &parsed.ty;
            let field_with_type_token = quote! { #field_name : #field_type };

            todo!()
        });

    // ield_with_type_token: Option<(TokenStream, Vec<Ident>)> = fields
    //     .fields
    //     .iter()
    //     .fold(None, |accumulator, current_field| {

    //         if let Some((mut field_with_type_token_stream, mut field_name_only)) = accumulator {
    //             // field_with_type_token_stream.extend(field_with_type_token);
    //             // field_name_only.push(field_name);
    //             // return Some(token);

    //             todo!()
    //         } else {
    //             // Some((field_with_type_token, vec![field_name]))

    //             todo!()
    //         }
    //     });

    // field_with_type_token
    //     .map(|(fields_with_token, fieldnames)| {

    //     })
    //     .unwrap_or(
    //         darling::Error::custom("no fields were processed: the struct has no fields or all fields are marked with `#[access(skip)]`")
    //             .write_errors()
    //             .into(),
    //     )

    quote! {
        fn new(names:types) -> Self {
            Self { names }
        }
    }
}

fn generate_for_tuple(fields: &Fields<WithOriginal<DeriveField, syn::Field>>, derive_struct: &DeriveStruct) -> TokenStream {
    quote! {}
}

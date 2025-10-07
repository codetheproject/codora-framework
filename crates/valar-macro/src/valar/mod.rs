use crate::valar::{accessor::AccessAttr, new::NewAttr};
use darling::{FromDeriveInput, FromField, FromMeta, util::WithOriginal};
use proc_macro2::TokenStream;
use syn::{Generics, Ident, Type};

mod accessor;
mod new;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(valar))]
pub(crate) struct DeriveStruct {
    pub ident: Ident,
    pub generics: Generics,
    pub data: darling::ast::Data<(), WithOriginal<DeriveField, syn::Field>>,
    // attributes
    pub new: Option<DeriveStructNewAttr>,

    // if this is none we expect the user to add those data
    pub access: Option<DeriveStructAccessAttr>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct DeriveStructAccessAttr {
    pub refrence: Option<bool>,
    pub mutable: Option<bool>,
    pub refmut: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct DeriveStructNewAttr {
    visibility: Option<Visibility>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct Visibility {
    private: Option<bool>,
    path: Option<String>,
}

#[derive(Debug, FromField)]
#[darling(attributes(valar))]
pub(crate) struct DeriveField {
    pub ident: Option<Ident>,
    pub ty: Type,
    // attributes
    pub new: Option<NewAttr>,
    pub access: Option<AccessAttr>,
}

pub fn derive(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _token_stream = token_stream.clone();
    let derive_input = syn::parse_macro_input!(_token_stream as syn::DeriveInput);

    let derive_struct: DeriveStruct = match FromDeriveInput::from_derive_input(&derive_input) {
        Ok(accessor) => accessor,
        Err(e) => return e.write_errors().into(),
    };

    println!("{:#?}", derive_struct);

    // let constructor_result = derive_struct
    //     .new
    //     .as_ref()
    //     .map(|_| new::generate_new(&derive_struct))
    //     .unwrap_or_default();

    // [constructor_result, accessor::generate_accessor(&derive_struct)]
    //     .into_iter()
    //     .collect::<TokenStream>()
    //     .into()

    TokenStream::new().into()
}

pub(crate) fn get_fields_from_derive_struct(derive_struct: &DeriveStruct) -> &darling::ast::Fields<WithOriginal<DeriveField, syn::Field>> {
    match &derive_struct.data {
        darling::ast::Data::Struct(fields) => fields,
        // Safety: This is has beeen checked already
        _ => unreachable!(),
    }
}

#[allow(unused)]
fn work_with_derive_input(derive_input: syn::DeriveInput) {
    todo!()
}

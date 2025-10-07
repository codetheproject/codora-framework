use crate::valar::{DeriveField, DeriveStruct, get_fields_from_derive_struct};
use darling::{FromMeta, util::WithOriginal};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Ident;

#[derive(Debug, FromMeta)]
#[darling(and_then = "AccessAttr::validate")]
pub(crate) struct AccessAttr {
    pub rename_as: Option<String>,
    pub refrence: Option<bool>,
    pub mutable: Option<bool>,
    pub refmut: Option<bool>,
    pub skip: Option<bool>,
}

impl AccessAttr {
    fn validate(self) -> darling::Result<Self> {
        let has_refmut = self.refmut == Some(true);
        let has_mutable = self.mutable == Some(true);
        let enabled_accessor_flags = has_refmut as u8 + has_mutable as u8;
        if enabled_accessor_flags > 1 {
            return Err(darling::Error::custom("only one of `reference`, `mutable`, or `refmut` may be enabled"));
        }
        // If the field is skipped, no accessor-related attributes are allowed
        let has_accessor_attrs = has_refmut || has_mutable || self.rename_as.is_some();
        if self.skip == Some(true) && has_accessor_attrs {
            return Err(darling::Error::custom("field is skipped; remove accessor-related attributes"));
        }

        Ok(self)
    }
}

// pub fn generate_accessor(derive_struct: &DeriveStruct) -> TokenStream {
//     let fields = get_fields_from_derive_struct(derive_struct);

//     let functions: Option<TokenStream> = fields
//         .fields
//         .iter()
//         .filter(|field| field.parsed.skip.map_or(true, |v| !v))
//         .fold(None, |mut accumulator, current_field| {
//             const IS_MUTABLE: bool = true;
//             let WithOriginal { parsed, original: _ } = current_field;
//             let token_stream = accumulator.get_or_insert_default();
//             // it's safe to continue
//             if let Some(value) = parsed.refmut
//                 && value
//             {
//                 let mut_fn = generate_accessor_function(derive_struct.ident.span(), parsed, IS_MUTABLE);
//                 let ref_fn = generate_accessor_function(derive_struct.ident.span(), parsed, !IS_MUTABLE);

//                 token_stream.extend(quote! {
//                     #mut_fn
//                     #ref_fn
//                 });
//             }

//             // Refrence Only
//             if let Some(value) = parsed.refrence
//                 && value
//             {
//                 token_stream.extend(generate_accessor_function(derive_struct.ident.span(), parsed, !IS_MUTABLE));
//             }

//             // Mutable Only
//             if let Some(value) = parsed.mutable
//                 && value
//             {
//                 token_stream.extend(generate_accessor_function(derive_struct.ident.span(), parsed, IS_MUTABLE));
//             }
//             accumulator
//         });

//     let (imp, ty, whr) = derive_struct.generics.split_for_impl();
//     let derived_struct_ident = &derive_struct.ident;
//     functions
//         .map(|token| {
//             quote! {
//                 impl #imp #derived_struct_ident #ty #whr {
//                     #token
//                 }
//             }
//         })
//         .unwrap_or(
//             darling::Error::custom("no fields were processed: the struct has no fields or all fields are marked with `#[access(skip)]`")
//                 .write_errors()
//                 .into(),
//         )
// }

// fn generate_accessor_function(fallback_span: Span, field: &DeriveField, mutable: bool) -> TokenStream {
//     let span = field
//         .ident
//         .as_ref()
//         .map(|i| i.span())
//         .unwrap_or(fallback_span);

//     let base_name = field
//         .rename_as
//         .as_ref()
//         .map(|name| Ident::new(name, span))
//         .or_else(|| field.ident.clone())
//         .unwrap_or_else(|| Ident::new("value", span));

//     let fn_ident = if mutable {
//         format_ident!("{}_mut", base_name)
//     } else {
//         base_name
//     };

//     // field.ident would only be none if the derived struct is an unnamed
//     let field_access = field
//         .ident
//         .as_ref()
//         .map(|f| quote! { #f })
//         .unwrap_or(quote! { 0 });

//     let ty = &field.ty;

//     let self_param = if mutable {
//         quote! { &mut self }
//     } else {
//         quote! { &self }
//     };

//     let return_type = if mutable {
//         quote! { &mut #ty }
//     } else {
//         quote! { &#ty }
//     };

//     quote! {
//         #[inline(always)]
//         pub fn #fn_ident(#self_param) -> #return_type {
//             #self_param.#field_access
//         }
//     }
// }

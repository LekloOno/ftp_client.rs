use proc_macro::TokenStream;
use quote::{quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
/// Auto-implements the response conversion to FtpResult
/// for the provided Response enum. <br/>
/// 
/// # Structural Requirements
/// For errors, an `FtpResult::Err::From` coercion must exist for this enum.
/// 
/// # Notes
/// For compatibility reasons, the maccro expects that
/// the `FtpResult` trait is locally imported.
///
/// # Arguments
/// * is_error - Defines if this response enum should be
///     converted to a FtpResult::Err or FtpResult::Ok.
/// 
/// # Examples
/// ```
/// use ftp_client::response::FtpResult;
/// use ftp_result_derive::ftp_result;
/// 
/// #[ftp_result(true)]
/// pub enum FtpPermanentError {
///     NotLoggedIn {res: String},
/// }
/// ```
pub fn ftp_result(attr: TokenStream, item: TokenStream) -> TokenStream {
    let is_error = parse_macro_input!(attr as syn::LitBool).value;
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let from_impl = if is_error {
        quote! {
            impl From<#name> for FtpResult {
                fn from(v: #name) -> Self {
                    Self::Err(v.into())
                }
            }
        }
    } else {
        quote! {
            impl From<#name> for FtpResult {
                fn from(v: #name) -> Self {
                    Self::Ok(v)
                }
            }
        }
    };

    let expanded = quote! {
        #input
        #from_impl
    };

    TokenStream::from(expanded)
}
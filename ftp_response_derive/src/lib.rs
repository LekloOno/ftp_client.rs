use proc_macro::TokenStream;
use quote::{quote};
use syn::{Data, DeriveInput, Fields, LitInt, parse_macro_input};

#[proc_macro_derive(FtpResponseEnum, attributes(response_code))]
/// Auto-implements FtpResponse and Display for the provided
/// Response enum to ensure coherence.
/// 
/// # Structural Requirements
/// - The enum must have **at least one variant**.
/// - Its variants must have **exactly one `String` field**.
/// - Variants of the same enum can't share **the same `response_code`**.
///
/// # Notes
/// For compatibility reasons, the maccro expects that
/// `FtpResponse`, `FtpResponseCode` and `body_string` are locally imported.
/// 
/// # Variant Attributes
/// * `response_code` - the decimal code of the error represented by this variant. 
/// 
/// # Generated code
/// - `FtpResponse::from_bytes` simply matches the `response_code` on Variants.
/// - `Display::fmt` results in `{response_code} {VariantName}: {response_msg}`
///
/// # Examples
/// ```
/// use ftp_client::response::{
///     FtpResponse,
///     FtpResponseCode,
///     body_string
/// };
/// 
/// use ftp_response_derive::FtpResponseEnum;
///
/// #[derive(FtpResponseEnum)]
/// pub enum FtpIntermediateReply {
///     #[response_code(331)]
///     UserNameOkPswdNeeded {msg: String}
/// }
/// ```
pub fn ftp_response_enum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut from_bytes_arms = Vec::new();
    let mut display_arms = Vec::new();

    if let Data::Enum(data_enum) = input.data {
        for variant in data_enum.variants {
            let var_name = variant.ident;
            let mut code_value = None;

            for attr in variant.attrs {
                if attr.path().is_ident("response_code") {
                    let lit_int: LitInt = attr.parse_args().expect(
                        "response_code attribute must be in the form #[response_code = 123]",
                    );
                    code_value = Some(lit_int);
                }
            }

            let code = match code_value {
                Some(v) => v,
                None => panic!("Variant {} missing #[response_code = ...]", var_name),
            };

            match variant.fields {
                Fields::Named(fields_named) => {
                    let field = &fields_named.named.get(0).unwrap().ident;

                    from_bytes_arms.push(quote! {
                        #code => Some(Self::#var_name { #field: body_string(body) }),
                    });

                    display_arms.push(quote! {
                        Self::#var_name { #field } => write!(f, "{} {}: {}", #code, stringify!(#var_name), #field),
                    });
                }
                _ => panic!("Variant {} must have named fields", var_name),
            }
        }
    } else {
        panic!("FtpResponseEnum can only be derived for enums");
    }

    let expanded = quote! {
        impl FtpResponse for #name {
            fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
                match code.into() {
                    #(#from_bytes_arms)*
                    _ => None
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
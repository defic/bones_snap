use proc_macro::TokenStream;

#[proc_macro]
pub fn bones_snap(input: TokenStream) -> TokenStream {
    snap_core::bones_snap(input.into()).into()
}

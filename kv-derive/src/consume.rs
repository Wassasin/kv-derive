use proc_macro2::TokenStream;
use quote::quote;

use crate::Field;

pub(crate) fn generate_match_field_consumer(field: &Field) -> TokenStream {
    let ident = field.get_ident();
    let key = field.get_key();
    quote! {
        #key => { kv_derive_impl::consumer::Consumer::consume(&mut this.#ident, value)?; }
    }
}

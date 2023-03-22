use proc_macro2::{Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

use syn::{self, Error, Ident, LitStr};

pub fn parse_lit_to_tokenstream(lit: &LitStr) -> Result<TokenStream, Error> {
    let str = lit.value();
    let tokens: TokenStream = str.parse().map_err(|err| syn::Error::from(err))?;
    Ok(quote! { (#tokens) })
}

pub fn get_crate_name(internal: bool) -> TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("surrealdb-orm") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "surrealdb_orm".to_string(),
        };
        TokenTree::from(Ident::new(&name, Span::call_site())).into()
    }
}

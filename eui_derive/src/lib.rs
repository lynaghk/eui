mod schema;

#[proc_macro_derive(Schema)]
pub fn derive_schema(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    schema::do_derive_schema(item)
}

#[proc_macro_attribute]
pub fn eui(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output = quote::quote! {
        #[derive(eui::eui_derive::Schema, eui::serde::Serialize, eui::serde::Deserialize)]
        #[serde(crate = "self::serde")]
        #input
    };

    output.into()
}

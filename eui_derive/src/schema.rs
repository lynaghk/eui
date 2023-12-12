use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput, Fields, GenericParam,
    Generics,
};

pub fn do_derive_schema(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let span = input.span();
    let name = input.ident;

    // Add a bound `T: Schema` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ty = generate_type(&input.data, span, name.to_string())
        .unwrap_or_else(syn::Error::into_compile_error);

    let expanded = quote! {
        impl #impl_generics ::eui::schema::Schema for #name #ty_generics #where_clause {
            const SCHEMA: &'static ::eui::schema::NamedType = #ty;
        }
    };

    expanded.into()
}

fn generate_type(data: &Data, span: Span, name: String) -> Result<TokenStream, syn::Error> {
    let ty = match data {
        Data::Struct(data) => generate_struct(&data.fields),
        Data::Enum(data) => {
            let name = data.variants.iter().map(|v| v.ident.to_string());
            let ty = data.variants.iter().map(|v| generate_variants(&v.fields));

            quote! {
                &::eui::schema::SdmTy::Enum(&[
                    #( &::eui::schema::NamedVariant { name: #name, ty: #ty } ),*
                ])
            }
        }
        Data::Union(_) => {
            return Err(syn::Error::new(
                span,
                "unions are not supported by `eui::schema`",
            ))
        }
    };

    Ok(quote! {
        &::eui::schema::NamedType {
            name: #name,
            ty: #ty,
        }
    })
}

fn generate_struct(fields: &Fields) -> TokenStream {
    match fields {
        syn::Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| {
                let ty = &f.ty;
                let name = f.ident.as_ref().unwrap().to_string();
                quote_spanned!(f.span() => &::eui::schema::NamedValue { name: #name, ty: <#ty as ::eui::schema::Schema>::SCHEMA })
            });
            quote! { &::eui::schema::SdmTy::Struct(&[
                #( #fields ),*
            ]) }
        }
        syn::Fields::Unnamed(fields) => {
            let fields = fields.unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote_spanned!(f.span() => <#ty as ::eui::schema::Schema>::SCHEMA)
            });
            quote! { &::eui::schema::SdmTy::TupleStruct(&[
                #( #fields ),*
            ]) }
        }
        syn::Fields::Unit => {
            quote! { &::eui::schema::SdmTy::UnitStruct }
        }
    }
}

fn generate_variants(fields: &Fields) -> TokenStream {
    match fields {
        syn::Fields::Named(fields) => {
            let fields = fields.named.iter().map(|f| {
                let ty = &f.ty;
                let name = f.ident.as_ref().unwrap().to_string();
                quote_spanned!(f.span() => ::eui::schema::NamedValue { name: #name, ty: <#ty as ::eui::schema::Schema>::SCHEMA })
            });
            quote! { &::eui::schema::SdmTy::StructVariant(&[
                #( #fields ),*
            ]) }
        }
        syn::Fields::Unnamed(fields) => {
            let fields = fields.unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote_spanned!(f.span() => <#ty as ::eui::schema::Schema>::SCHEMA)
            });
            quote! { &::eui::schema::SdmTy::TupleVariant(&[
                #( #fields ),*
            ]) }
        }
        syn::Fields::Unit => {
            quote! { &::eui::schema::SdmTy::UnitVariant }
        }
    }
}

/// Add a bound `T: MaxSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::eui::schema::Schema));
        }
    }
    generics
}
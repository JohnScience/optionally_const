use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

fn find_const_type_attr(
    attrs: &[syn::Attribute],
) -> &syn::Attribute {
    attrs.iter()
        .find(|attr| attr.path().get_ident()
            .map_or(false, |ident| ident == "const_type"))
        .unwrap_or_else(|| {
            panic!("Expected #[const_type(ConstTypeName)] attribute");
        })
}

fn const_type_ident(attrs: &[syn::Attribute]) -> syn::Ident {
    let const_type_name_attr: &syn::Attribute = find_const_type_attr(&attrs);
    let meta: &syn::Meta = &const_type_name_attr
        .meta;
    let syn::Meta::List(list) = meta else {
        panic!("Expected #[const_type(ConstTypeName)] attribute to be a list");
    };
    let syn::MetaList {
        path: _const_type,
        delimiter: _parens,
        tokens,
    } = list;
    syn::parse2(tokens.clone())
        .unwrap_or_else(|_| {
            panic!("Expected #[const_type(ConstTypeName)] attribute to contain a single identifier");
        })
}

fn assert_fieldless_enum(data_enum: &syn::DataEnum) {
    for variant in data_enum.variants.iter() {
        if !matches!(variant.fields, syn::Fields::Unit) {
            panic!("Expected fieldless enum variant, found non-fieldless variant {}", variant.ident);
        }
    }
}

/// Derives the const type for a [fieldless enum] as well as the implementations
/// of the `Const` and `OptionallyConst` traits for the parameterizations
/// of the const type that represent the enum variants.
/// 
/// # Example
/// 
/// ```rust
/// use optionally_const::OptionallyConst;
/// use optionally_const_macros::FieldlessEnumConstType;
/// 
/// #[derive(FieldlessEnumConstType, Debug)]
/// #[const_type(ConstTypeName)]
/// enum FieldlessEnum {
///     A,
///     B,
///     C,
/// }
/// 
/// fn print_fieldless_enum<T>(value: T)
/// where
///     T: OptionallyConst<FieldlessEnum>,
/// {
///     if let Some(value) = T::MAYBE_CONST {
///         println!("Const value: {:?}", value);
///     } else {
///         let value: FieldlessEnum = T::into_value(value);
///         println!("Non-const value: {:?}", value);
///     }
/// }
/// 
/// fn main() {
///     print_fieldless_enum(FieldlessEnum::A);
///     print_fieldless_enum(FieldlessEnum::B);
///     print_fieldless_enum(FieldlessEnum::C);
///     print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::A as usize }>);
///     print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::B as usize }>);
///     print_fieldless_enum(ConstTypeName::<{ FieldlessEnum::C as usize }>);
/// }
/// ```
/// 
/// [fieldless enum]: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.fieldless
#[proc_macro_derive(FieldlessEnumConstType, attributes(const_type))]
pub fn derive_fieldless_enum_const_type(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        attrs,
        // Should we care about the visibility?
        vis,
        ident,
        generics: _no_generics,
        data
    } = input;

    // The identifier of the generic type whose parameterizations will be used to
    // represent the const values of the enum variants.
    let const_type_ident: syn::Ident = const_type_ident(&attrs);

    let syn::Data::Enum(data_enum) = data else {
        panic!("#[derive(FieldlessEnumConstType)] can only be used on enums.");
    };

    assert_fieldless_enum(&data_enum);

    let variants = data_enum.variants.iter().map(|variant| &variant.ident);

    let const_type_defn: proc_macro2::TokenStream = quote! {
        #vis struct #const_type_ident<const DISCRIMINANT: usize>;
    };

    let const_impls: proc_macro2::TokenStream = {
        let variants = variants.clone();
        quote! {
            #(
                impl ::optionally_const::Const<#ident> for #const_type_ident<{#ident::#variants as usize}> {
                    const VALUE: #ident = #ident::#variants;
                }
            )*
        }
    };

    let optionally_const_impls: proc_macro2::TokenStream = quote! {
        #(
            impl ::optionally_const::OptionallyConst<#ident> for #const_type_ident<{#ident::#variants as usize}> {
                const MAYBE_CONST: Option<#ident> = Some(#ident::#variants);
                
                fn into_value(self) -> #ident {
                    #ident::#variants
                }
            }
        )*
    };

    let output: proc_macro2::TokenStream = quote! {
        #const_type_defn
        #const_impls
        #optionally_const_impls
    };

    let output: TokenStream = output.into();

    output
}

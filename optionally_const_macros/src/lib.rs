use proc_macro::TokenStream;

use derive_syn_parse::Parse;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

fn find_const_type_attr(attrs: &[syn::Attribute]) -> &syn::Attribute {
    attrs
        .iter()
        .find(|attr| {
            attr.path()
                .get_ident()
                .is_some_and(|ident| ident == "const_type")
        })
        .unwrap_or_else(|| {
            panic!("Expected #[const_type(ConstTypeName)] attribute");
        })
}

#[derive(Parse)]
struct ConstTypeSyntax {
    #[call(syn::Attribute::parse_outer)]
    attrs: Vec<syn::Attribute>,
    name: syn::Ident,
}

fn const_type_syntax(attrs: &[syn::Attribute]) -> ConstTypeSyntax {
    let const_type_name_attr: &syn::Attribute = find_const_type_attr(attrs);
    let meta: &syn::Meta = &const_type_name_attr.meta;
    let syn::Meta::List(list) = meta else {
        panic!("Expected #[const_type(ConstTypeName)] attribute to be a list");
    };
    let syn::MetaList {
        path: _const_type,
        delimiter: _parens,
        tokens,
    } = list;

    syn::parse2(tokens.clone()).unwrap_or_else(|_| {
        panic!("Expected #[const_type(ConstTypeName)] attribute to contain a single identifier");
    })
}

fn assert_fieldless_enum(data_enum: &syn::DataEnum) {
    for variant in &data_enum.variants {
        assert!(
            matches!(variant.fields, syn::Fields::Unit),
            "Expected fieldless enum variant, found non-fieldless variant {}",
            variant.ident
        );
    }
}

/// Derives the [const type] for a [fieldless enum] as well as the implementations
/// of the [`Const`] and [`OptionallyConst`] traits for the parameterizations
/// of the [const type] that represent the enum variants.
///
/// The fieldless enum also must derive the [`Clone`] and [`Copy`] traits.
///
/// # Example
///
/// ```rust
/// use optionally_const::OptionallyConst;
/// use optionally_const_macros::FieldlessEnumConstType;
///
/// // Clone and Copy derives on the enum are required for the derive macro to work.
/// #[derive(FieldlessEnumConstType, Debug, Clone, Copy)]
/// #[const_type(
///     // You can use any outer attributes you want here.
///     // They will be placed verbatim on the generated type.
///     #[derive(Clone, Copy)]
///     ConstTypeName
/// )]
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
/// [const type]: https://github.com/JohnScience/optionally_const/tree/main/optionally_const#const-type
/// [`Const`]: https://docs.rs/optionally_const/latest/optionally_const/trait.Const.html
/// [`OptionallyConst`]: https://docs.rs/optionally_const/latest/optionally_const/trait.OptionallyConst.html
#[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
#[proc_macro_derive(FieldlessEnumConstType, attributes(const_type))]
pub fn derive_fieldless_enum_const_type(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        attrs,
        // Should we care about the visibility?
        vis,
        ident,
        generics: _no_generics,
        data,
    } = input;

    // The identifier of the generic type whose parameterizations will be used to
    // represent the const values of the enum variants.
    let ConstTypeSyntax {
        attrs: const_type_attrs,
        name: const_type_ident,
    } = const_type_syntax(&attrs);

    let syn::Data::Enum(data_enum) = data else {
        panic!("#[derive(FieldlessEnumConstType)] can only be used on enums.");
    };

    assert_fieldless_enum(&data_enum);

    let variants = data_enum.variants.iter().map(|variant| &variant.ident);

    let const_type_defn: proc_macro2::TokenStream = quote! {
        #[doc =
            concat!(
                "A [const type] for the [fieldless enum] [`",stringify!(#ident), "`].\n\
                \n\
                This is a code-generated type that was derived with the \
                [`#[derive(", stringify!(FieldlessEnumConstType), ")]`]\
                (::optionally_const::", stringify!(FieldlessEnumConstType),") \
                derive macro.\n\
                \n\
                This type is supposed to be parameterized by the enum variant's [discriminant]s \
                converted to a `usize`.\n\
                \n\
                For example, `", stringify!(#const_type_ident), "<{",stringify!(#ident),"::Variant as usize}>`.\n\
                \n\
                [const type]: https://github.com/JohnScience/optionally_const/tree/main/optionally_const#const-type
                [fieldless enum]: https://doc.rust-lang.org/reference/items/enumerations.html#r-items.enum.fieldless
                [discriminant]: https://doc.rust-lang.org/reference/items/enumerations.html#discriminants
                "
        )]
        #(
            #const_type_attrs
        )*
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

    // Originally, the signature of this function was
    //
    // ```
    // #vis fn try_into_const_type_instance<T>
    // (
    //     self
    // ) -> Option<#const_type_ident<{T::VALUE as usize}>>
    // where
    //     T: ::optionally_const::Const<#ident>,
    // ```
    let try_into_const_type_instance_impls_on_enum: proc_macro2::TokenStream = quote! {
        impl #ident {
            #[doc =
                concat!(
                    "Converts the enum variant into a [const type] instance.\n\
                    \n\
                    This is a code-generated function that was derived with the \
                    [`#[derive(", stringify!(FieldlessEnumConstType), ")]`]\
                    (::optionally_const::", stringify!(FieldlessEnumConstType),") \
                    derive macro.\n\
                    \n\
                    This function is supposed to be parameterized by the enum variant's discriminants \
                    converted to a `usize`.\n\
                    \n\
                    For example, `", stringify!(try_into_const_type_instance), "::<{",stringify!(#ident),"::Variant as usize}>()`.\n\
                    \n\
                    # Errors\n\
                    \n\
                    This function returns the original enum variant wrapped in [`Err`] if the \
                    discriminant of the enum variant does not match the discriminant of the const type instance.\n\
                    \n\
                    This function is defined on the enum rather than implemented as a trait method \
                    because at the time of writing this code, it's impossible to make the trait method `const`.\n\
                    \n\
                    [const type]: https://github.com/JohnScience/optionally_const/tree/main/optionally_const#const-type"
            )]
            #vis const fn try_into_const_type_instance<const DISCRIMINANT: usize>
            (
                self
            ) -> ::core::result::Result<#const_type_ident<DISCRIMINANT>, Self>
            where
                #const_type_ident<DISCRIMINANT>: ::optionally_const::Const<#ident>,
            {
                if self as usize == DISCRIMINANT {
                    Ok(#const_type_ident::<DISCRIMINANT>)
                } else {
                    Err(self)
                }
            }
        }
    };

    let optionally_const_impls: proc_macro2::TokenStream = quote! {
        #(
            impl ::optionally_const::OptionallyConst<#ident> for #const_type_ident<{#ident::#variants as usize}> {
                const MAYBE_CONST: Option<#ident> = Some(#ident::#variants);

                fn into_value(self) -> #ident {
                    #ident::#variants
                }

                fn try_from_value(value: #ident) -> Result<Self, #ident> {
                    if matches!(<Self as ::optionally_const::Const<#ident>>::VALUE, value) {
                        Ok(#const_type_ident)
                    } else {
                        Err(value)
                    }
                }
            }
        )*
    };

    let output: proc_macro2::TokenStream = quote! {
        #try_into_const_type_instance_impls_on_enum
        #const_type_defn
        #const_impls
        #optionally_const_impls
    };

    let output: TokenStream = output.into();

    output
}

//! Provide a `ff!` macro to write Fully Qualified Syntax without nesting
//! turbofishes.
//! 
//! Which can be usefull in very generic code when the chain of traits and
//! associated types can be long and verbose, or in macros generating chains of
//! traits.
//!
//! Syntax is
//!
//! ```ignore
//! ff!(Type | Trait1::Item | Trait2::Item ...)
//! ```
//!
//! which desugars to
//!
//! ```ignore
//! <... <<T as Trait1>::Item as Trait2>::Item ...>
//! ```

use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, spanned::Spanned, Token},
};

/// Provide an alternative syntax to write Fully Qualified Syntax without
/// nesting turbofishes.
///
/// The content of `ff!(...)` starts with the source type, followed by a
/// chain of `| Trait::Item`, `Trait` being the trait containing `Item`
/// associated type. Both `Trait` and `Item` can have generics. The last `Item`
/// can also qualify an associated constant or function.
///
/// # Exemple
///
/// Given the following traits and structs:
///
/// ```
/// use flatfish::ff;
///
/// trait Level1 {
///     type Foo<T>: Level2<T>;
/// }
///
/// trait Level2<T> {
///     type Baz;
/// }
///
/// struct Impl1;
/// impl Level1 for Impl1 {
///     type Foo<T> = Impl2;
/// }
///
/// struct Impl2;
/// impl<T> Level2<T> for Impl2 {
///     type Baz = u32;
/// }
/// ```
///
/// The following types are equivalent:
/// - `ff!(T | Level1::Foo<u32> | Level2<u32>::Baz)`
/// - `<<T as Level1>::Foo<u32> as Level2<u32>>::Baz`
#[proc_macro]
pub fn ff(tokens: TokenStream) -> TokenStream {
    let FlatfishBody { source_type, chain } = parse_macro_input!(tokens as FlatfishBody);

    let mut out = quote! { #source_type };

    for step in chain {
        let FlatfishItem { trait_, item } = step;

        out = quote!(< #out as #trait_ >:: #item)
    }

    TokenStream::from(out)
}

struct FlatfishBody {
    source_type: syn::TypePath,
    chain: Vec<FlatfishItem>,
}

impl syn::parse::Parse for FlatfishBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let source_type = input.parse()?;
        let mut chain = vec![];

        loop {
            let lookahead = input.lookahead1();
            if !lookahead.peek(Token![|]) {
                break;
            }

            chain.push(input.parse()?);
        }

        Ok(Self { source_type, chain })
    }
}

struct FlatfishItem {
    trait_: syn::TypePath,
    item: syn::PathSegment,
}

impl syn::parse::Parse for FlatfishItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: Token![|] = input.parse()?;
        let mut trait_: syn::TypePath = input.parse()?;

        let error_msg = "Syntax is `path::to::Trait::Item`";

        // Item in trait is the last part of the path, which should at least
        // contain too segments: the trait then the item. More segments
        // corresponds to the path of the trait.
        let Some(item) = trait_.path.segments.pop() else {
            return Err(syn::Error::new(trait_.span(), error_msg));
        };

        let item = item.into_value();
        if trait_.path.segments.is_empty() {
            return Err(syn::Error::new(trait_.span(), error_msg));
        }

        let _ = trait_.path.segments.pop_punct();

        Ok(Self { trait_, item })
    }
}

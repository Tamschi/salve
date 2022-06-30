use catch::CatchExt;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote_spanned;
use salve::ParseOrForgeExt;
use syn::{parse::ParseStream, Error, Token};

pub fn parse_triple(input: ParseStream, errors: &mut Vec<Error>) -> (Ident, Token![,], Ident) {
	(
		// Parsing will continue here regardless of errors, yielding dummy values.
		// Errors are collected for later use.
		input.parse_or_forge().catch_item(errors),
		input.parse_or_forge().catch_item(errors),
		input.parse_or_forge().catch_item(errors),
	)
}

pub fn macro_transform(input: ParseStream) -> TokenStream {
	let mut errors = vec![];
	let (first, _, second) = parse_triple(input, &mut errors);
	if !input.is_empty() {
		// Added last, since this is often incidental.
		errors.push(Error::new_spanned(
			input.parse::<TokenStream>().expect("infallible"),
			"Unexpected tokens.",
		))
	}
	let errors = errors.into_iter().map(Error::into_compile_error);
	quote_spanned! {Span::mixed_site()=>
		#(#errors)* // Emit parsing errors first, for better visibility.

		// Even if `second` is unavailable, `first` may be present, reducing errors elsewhere.
		#[automatically_derived]
		struct #first;

		// Even if `first` is unavailable, `seconds` may be present, reducing errors elsewhere.
		#[automatically_derived]
		struct #second;
	}
}

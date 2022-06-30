//! Greasy parsing extensions for [`syn`], to soften proc macro errors.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fsalve)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fsalve)

#![doc(html_root_url = "https://docs.rs/salve/0.0.1")]
#![warn(clippy::pedantic, missing_docs)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::sync::atomic::{AtomicUsize, Ordering};

use proc_macro2::{Span, TokenTree};
use syn::{
	parse::{Parse, ParseBuffer},
	token::{
		Abstract, Add, AddEq, And, AndAnd, AndEq, As, Async, At, Auto, Await, Bang, Become, Box,
		Brace, Bracket, Break, Caret, CaretEq, Colon, Colon2, Comma, Const, Continue, Crate,
		Default, Div, DivEq, Do, Dollar, Dot, Dot2, Dot3, DotDotEq, Dyn, Else, Enum, Eq, EqEq,
		Extern, FatArrow, Final, Fn, For, Ge, Group, Gt, If, Impl, In, LArrow, Le, Let, Loop, Lt,
		Macro, Match, Mod, Move, MulEq, Mut, Ne, Or, OrEq, OrOr, Override, Paren, Pound, Priv, Pub,
		Question, RArrow, Ref, Rem, RemEq, Return, SelfType, Semi, Shl, ShlEq, Shr, ShrEq, Star,
		Static, Struct, Sub, SubEq, Super, Tilde, Trait, Try, Type, Typeof, Underscore, Union,
		Unsafe, Unsized, Use, Virtual, Where, While, Yield,
	},
	Error, Ident,
};
use this_is_fine::Fine;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

static FORGERY_COUNTER: AtomicUsize = AtomicUsize::new(0);

trait Next {
	fn next(&self) -> usize;
}
impl Next for AtomicUsize {
	fn next(&self) -> usize {
		match self.fetch_add(1, Ordering::Relaxed) {
			usize::MAX => panic!("salve::FORGERY_COUNTER exhausted."),
			i => i,
		}
	}
}

/// For forgeable elements of the syntax.
///
/// The forged value's [`Span`] should have the given location but resolve at [`Span::mixed_site()`](`Span::mixed_site`).
pub trait Forge: Sized {
	/// Forges a new, if possible unique, instance of `Self`.
	///
	/// The forged value's [`Span`] should have the given location but resolve at [`Span::mixed_site()`](`Span::mixed_site`).
	fn forge(span: Span) -> Self;
}

/// Extends [`ParseBuffer`] with [`parse_or_forge::<T: Parse + Forge>(&self)`](`ParseOrForgeExt::parse_or_forge`).
pub trait ParseOrForgeExt {
	/// Parses a `T` from `self` or, if that fails, forges it with the current [`self.span()`](`ParseBuffer::span`).
	///
	/// The result is [`Fine`] either way, but will carry an [`Error`] iff `T` was forged.
	fn parse_or_forge<T: Parse + Forge>(&self) -> Fine<T, Error>;

	/// Parses a `T` from `self` or, if that fails, forges it with the current [`self.span()`](`ParseBuffer::span`).
	///
	/// The result is [`Fine`] either way, but will carry an [`Error`] iff `T` was forged.
	///
	/// Additionally, iff `T` was forged, one [`TokenTree`] is consumed, iff the input was not yet empty.
	fn parse_or_forge_and_skip_tt<T: Parse + Forge>(&self) -> Fine<T, Error>;
}
impl ParseOrForgeExt for ParseBuffer<'_> {
	fn parse_or_forge<T: Parse + Forge>(&self) -> Fine<T, Error> {
		match self.parse() {
			Ok(parsed) => (parsed, Ok(())),
			Err(error) => (T::forge(self.span()), Err(error)),
		}
	}

	fn parse_or_forge_and_skip_tt<T: Parse + Forge>(&self) -> Fine<T, Error> {
		let fine = self.parse_or_forge();
		if fine.1.is_err() {
			self.parse::<TokenTree>().ok();
		}
		fine
	}
}

impl Forge for Ident {
	fn forge(span: Span) -> Self {
		Ident::new(
			&format!("__fallback_ident_{}", FORGERY_COUNTER.next()),
			span.resolved_at(Span::mixed_site()),
		)
	}
}

macro_rules! forge_tokens_span {
	($($Name:ident),*$(,)?) => {$(
		impl Forge for $Name {
			fn forge(span: Span) -> Self {
				$Name {
					span: span.resolved_at(Span::mixed_site()),
				}
			}
		}
	)*};
}

const fn spread<T: Copy, const N: usize>(value: T) -> [T; N] {
	[value; N]
}

macro_rules! forge_tokens_spans {
	($($Name:ident),*$(,)?) => {$(
		impl Forge for $Name {
			fn forge(span: Span) -> Self {
				$Name {
					spans: spread(span.resolved_at(Span::mixed_site())),
				}
			}
		}
	)*};
}

forge_tokens_span!(
	Abstract, As, Async, Auto, Await, Become, Box, Brace, Bracket, Break, Const, Continue, Crate,
	Default, Do, Dyn, Else, Enum, Extern, Final, Fn, For, Group, If, Impl, In, Let, Loop, Macro,
	Match, Mod, Move, Mut, Override, Paren, Priv, Pub, Ref, Return, SelfType, Static, Struct,
	Super, Trait, Try, Type, Typeof, Union, Unsafe, Unsized, Use, Virtual, Where, While, Yield
);

forge_tokens_spans!(
	Add, AddEq, And, AndAnd, AndEq, At, Bang, Caret, CaretEq, Colon, Colon2, Comma, Div, DivEq,
	Dollar, Dot, Dot2, Dot3, DotDotEq, Eq, EqEq, FatArrow, Ge, Gt, LArrow, Le, Lt, MulEq, Ne, Or,
	OrEq, OrOr, Pound, Question, RArrow, Rem, RemEq, Semi, Shl, ShlEq, Shr, ShrEq, Star, Sub,
	SubEq, Tilde, Underscore,
);

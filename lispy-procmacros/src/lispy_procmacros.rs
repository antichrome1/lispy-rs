//! This crate exports some procmacros which are then re-exported by the `lispy` crate.

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use quote::quote;

include!("cond.rs");

//! Procedural macros for the Substrate analog circuit generator framework.
#![warn(missing_docs)]

mod pdk;

use darling::FromDeriveInput;
use pdk::layers::{LayerInputReceiver, LayersInputReceiver};
use pdk::supported_pdks::supported_pdks_impl;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Enumerates PDKs supported by a certain layout implementation of a block.
///
/// Automatically implements the appropriate trait for all specified PDKs given a process-portable
/// implementation in a single PDK.
///
/// # Examples
///
/// ```
#[doc = include_str!("../../docs/api/code/prelude.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/layer.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/layers.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/several_pdks.md.hidden")]
#[doc = include_str!("../../docs/api/code/block/inverter.md.hidden")]
#[doc = include_str!("../../docs/api/code/block/buffer.md.hidden")]
#[doc = include_str!("../../docs/api/code/layout/inverter_multiprocess.md")]
#[doc = include_str!("../../docs/api/code/layout/buffer_multiprocess.md")]
/// ```
#[proc_macro_attribute]
pub fn supported_pdks(args: TokenStream, input: TokenStream) -> TokenStream {
    supported_pdks_impl(args, input)
}

/// Derives a layer implementation on a struct.
///
/// # Examples
///
/// ```
#[doc = include_str!("../../docs/api/code/prelude.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/layer.md")]
/// ```
#[proc_macro_derive(Layer, attributes(layer, value, id))]
pub fn derive_layer(input: TokenStream) -> TokenStream {
    let receiver =
        LayerInputReceiver::from_derive_input(&parse_macro_input!(input as DeriveInput)).unwrap();
    quote!(
        #receiver
    )
    .into()
}

/// Derives a layer set implementation on a struct.
///
/// # Examples
///
/// ```
#[doc = include_str!("../../docs/api/code/prelude.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/layer.md.hidden")]
#[doc = include_str!("../../docs/api/code/pdk/layers.md")]
/// ```
#[proc_macro_derive(Layers, attributes(layer, value, alias))]
pub fn derive_layers(input: TokenStream) -> TokenStream {
    let receiver =
        LayersInputReceiver::from_derive_input(&parse_macro_input!(input as DeriveInput)).unwrap();
    quote!(
        #receiver
    )
    .into()
}

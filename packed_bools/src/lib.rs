#[doc = include_str!("../README.md")]
use proc_macro::TokenStream;

mod packed_bools;

#[proc_macro_derive(PackedBooleans, attributes(pack_bools))]
pub fn packed_bools(input: TokenStream) -> TokenStream {
    packed_bools::packed_bools(input)
}

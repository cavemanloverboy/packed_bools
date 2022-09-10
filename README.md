# packed_bools: a lightweight ergonomic bitwise manipulation library

Want to stick several `bool` values in a struct? Using `core`/`std`, you will pay the price of 1 byte of space for each value so that each `bool` can be addressable. For memory/storage sensitive applications, we'd really like to have an alternative. Enter `packed_bools`.

There are already bitwise manipulation libraries on crates.io. A quick search will reveal half a dozen or so popular ones. What makes this one different? It is dev-friendly and ergonomic. When I speak of (or to) my cat, I don't refer to him by his spatial coordinates. I refer to him by his name, "Ron" (or "Ronald" if he just threw my earphones on the floor). Likewise, I don't want to have to refer to individual bits as "bit 2" or "bit 5" of this or that type... and have to document it somewhere so that I don't forget which bit is which... and then also implement bitwise operations to actually carry out operations. I want to refer to the booleans by my chosen flag name -- I want to name my bits.

This library provides a derive macro `PackedBooleans` with which you can mark a (or several) `core::primitive::u8` with `#[pack_bools(..)]`, in which you can provide the names of up to 8 boolean flags (per `u8`). This library introduces no new types. It simply implements a new method on your struct that manipulates and retrieves your bit by name.


# Example Usage
```rust
use packed_bools::PackedBooleans;

#[derive(Default, PackedBooleans)]
pub struct FooStruct<'a, T=()> {
    #[pack_bools(active, admin, discount, premium, og, frozen, kyc, tos_agree)]
    pub booleans: u8,
    pub an_option_with_generics: Option<&'a T>,
}


// Initialize your struct
let mut foo_struct = FooStruct::<()>::default();

// Set the values of each bit individually
foo_struct.set_active(true);
foo_struct.set_admin(false);
foo_struct.set_discount(true);
foo_struct.set_premium(false);
foo_struct.set_og(false);
foo_struct.set_frozen(true);
foo_struct.set_kyc(false);
foo_struct.set_tos_agree(true);

// Read the values of each bit individually
assert_eq!(foo_struct.get_active(), true);
assert_eq!(foo_struct.get_admin(), false);
assert_eq!(foo_struct.get_discount(), true);
assert_eq!(foo_struct.get_premium(), false);
assert_eq!(foo_struct.get_og(), false);
assert_eq!(foo_struct.get_frozen(), true);
assert_eq!(foo_struct.get_kyc(), false);
assert_eq!(foo_struct.get_tos_agree(), true);
```
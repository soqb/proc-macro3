# proc-macro3
defines a macro that imports either from `proc_macro` or `proc-macro2` depending on the `proc-macro2` feature flag.

## Examples
```rust
proc_macro3::import!(); // creates a module named 'proc_macro'
use proc_macro::TokenStream; // uses from the newly created module

fn uses_token_steam(input: TokenStream) -> TokenStream {
    in_some_way(input)
}
```
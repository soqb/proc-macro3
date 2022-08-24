# proc-macro3
context-based cfg for proc macros - all the benefits of proc-macro2 with none of the slow compile times

## Examples
```rust
proc_macro3::import!(); // creates a module named 'proc_macro'
use proc_macro::TokenStream; // uses from the newly created module

fn uses_token_steam(input: TokenStream) -> TokenStream {
    in_some_way(input)
}
```
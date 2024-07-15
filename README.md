# crabtrap

Rust porting of the golang library [mousetrap](https://github.com/inconshreveable/mousetrap)

This is a tiny crate with a minimal footprint, that solves a simple problem:

`Was the binary launched from the terminal or by double clicking on it?`

This crate gives an answer in the Windows world, keeping the question open in all other operating systems (suggestions are welcome).

## Example

```rust
use crabtrap::started_by_explorer;

fn main(){
    if started_by_explorer(){
        println!("I'm from a GUI");
    } else{
        println!("I'm from the terminal");
    }
}
```

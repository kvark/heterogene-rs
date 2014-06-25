/*!
    Heterogeneous Stack
    
    No need for any code - Rust can already do that.

# Example
```rust
    let s = ();
    let s = (1u, s);     //append
    let s = ('c', s);
    let (ch, s) = s;    //consume
    let (num, s) = s;
    println!("Stack: {} {} {}", ch, num, s);
```
*/

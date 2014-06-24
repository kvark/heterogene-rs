/*!
    Heterogeneous Queue

# Example
```rust
    use heterogene::queue::{Q0,Q1,Q2,Q3};
    let q = ();
    let q = q.append(1u);
    let q = q.append('c');
    let (num, q) = q.consume();
    let (ch, q) = q.consume();
    println!("Queue: {} {} {}", num, ch, q);
```
*/

pub trait Q0 {
    fn append<T1>(self, t: T1) -> (T1,);
}
impl Q0 for () {
    fn append<T1>(self, t: T1) -> (T1,) {
        (t,)
    }
}

// The need for the special case of Q1 could be eliminated
// if Rust allowed trailing commas in generic parameters:
// trait Q1<T1,>
pub trait Q1<T1> {
    fn append<T2>(self, t: T2) -> (T1,T2);
    fn consume(self) -> (T1,());
}
impl<T1> Q1<T1> for (T1,) {
    fn append<T2>(self, t: T2) -> (T1,T2) {
        let (t1,) = self;
        (t1, t)
    }
    fn consume(self) -> (T1,()) {
        let (t1,) = self;
        (t1,())
    }
}


macro_rules! queue(
    ($Q:ident $B0:ident $($B:ident)+) => (
        pub trait $Q< $B0, $($B),+ > {
            fn append<Tn>(self, t: Tn) -> ($B0, $(($B),)+ Tn);
            fn consume(self) -> ($B0, ($(($B),)+));
        }
        impl<$B0,$($B),+> $Q<$B0,$($B),+> for ($B0, $($B),+) {
            fn append<Tn>(self, t: Tn) -> ($B0, $($B,)+ Tn) {
                let ($B0, $($B),+) = self;
                ($B0, $($B),+, t)
            }
            fn consume(self) -> ($B0, ($(($B),)+)) {
                let ($B0, $($B),+) = self;
                ($B0, ($($B),+,))
            }
        }
    )
)

queue!(Q2 T1 T2)
queue!(Q3 T1 T2 T3)
queue!(Q4 T1 T2 T3 T4)
queue!(Q5 T1 T2 T3 T4 T5)
queue!(Q6 T1 T2 T3 T4 T5 T6)
queue!(Q7 T1 T2 T3 T4 T5 T6 T7)
queue!(Q8 T1 T2 T3 T4 T5 T6 T7 T8)
queue!(Q9 T1 T2 T3 T4 T5 T6 T7 T8 T9)

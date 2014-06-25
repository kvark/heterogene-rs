/*!

Heterogeneous Queue (alternative)
    
This version is hand-written (no macros) but has a simpler architecture
that allows implicit consumption by deconstruction on assignment.

# Example

```rust
    use heterogene::queue_alt::{Q0,Q1,Q2};
    let q = ();
    let q = q.append(1u);
    let q = q.append('c');
    let (num, q) = q;
    let (ch, q) = q;
    println!("Queue-alt: {} {} {}", num, ch, q);
```
*/

pub trait Q0 {
    fn append<T1>(self, t1: T1) -> (T1,());
}
impl Q0 for () {
    fn append<T1>(self, t1: T1) -> (T1,()) {
        (t1,())
    }
}

pub trait Q1<T1> {
    fn append<T2>(self, t2: T2) -> (T1,(T2,()));
}
impl<T1> Q1<T1> for (T1,()) {
    fn append<T2>(self, t2: T2) -> (T1,(T2,())) {
        let (t1,_) = self;
        (t1,(t2,()))
    }
}

pub trait Q2<T1,T2> {
    fn append<T3>(self, t3: T3) -> (T1,(T2,(T3,())));
}
impl<T1,T2> Q2<T1,T2> for (T1,(T2,())) {
    fn append<T3>(self, t3: T3) -> (T1,(T2,(T3,()))) {
        let(t1,(t2,_)) = self;
        (t1,(t2,(t3,())))
    }
}

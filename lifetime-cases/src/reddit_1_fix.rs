// TAKEAWAYS:
// 1. &T and &self in trait impl are different references (because of autoref - implicit borrowing - feature)
// 2. API Design: In general, Trait<'a> denotes that:
//  - Implementer might contain (or might not) 'a refs internally (main case)
//  - Implementer itself will be 'a reference (shared or exclusive, rare case imho)
// 3. Trait bounds are applied not only to T, but &T, &mut T, [T], and other...
trait Hello<'a> {
    fn hi(&self);
}

// no refs here
impl<'a> Hello<'a> for &'a () {     // NOTE: may replace last 'a with '_ - lifetime inference
                                    // (or omitted entirely, but it would be different semantics)

    // self is &&'a () here
    fn hi(&self) {
        println!("Hi");
    }
}

struct Wrapper<T>(T);
impl<'a, T> Hello<'a> for &'a Wrapper<T>
where
    &'a T: Hello<'a>,
{
    //fix: lifetimes are not interconnected here anymore
    fn hi(&self) {
        let inner = &self.0;
        inner.hi();
    }
}

pub fn reddit1_demo_fix() {
    let w = Wrapper(());
    (&w).hi();
}
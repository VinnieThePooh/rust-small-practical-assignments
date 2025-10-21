// We say that trait implementer might contain (or might not) some ref with lifetime 'a,
 trait Hello<'a> {
    fn hi(&self);
}

// no refs here
impl<'a> Hello<'a> for &'a () {
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
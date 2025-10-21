// ref: https://www.reddit.com/r/learnrust/comments/1o58a9d/why_does_introducing_a_seemingly_harmless/
trait Hello<'a> {
    // the problem is LIFETIME PRESENCE in method signature in current case
    fn hi(&'a self);
}

impl<'a> Hello<'a> for &'a () {
    fn hi(&'a self) {
        println!("Hi");
    }
}

struct Wrapper<T>(T);

// The reason: explicitly denoting with 'a, we say that container owns some internal reference with
// 'a lifetime, and instance must NOT outlive that internal reference
impl<'a, T> Hello<'a> for &'a Wrapper<T>
where
    &'a T: Hello<'a>,
{
    
    fn hi(&'a self) {
        // you actually define timespan of lifetime 'a itself here - being inside current method body,
        // but it thus means that the ref to instance itself will outlive that lifetime
        // leading to the fact that inner reference would be invalid after method call complete
        // but current instance outlives it (what is PROHIBITED)
        // NOTE: compiler does not know that inner ref is ephemeral though
        let inner = &self.0;
        // NOTE: not compiled
        // inner.hi();
    }
}

pub fn reddit1_demo() {
    let a = &();
    // NOTE: self is &'a &'a here,
    a.hi();

    let wrapper = &Wrapper(a);
    //COOL ERROR: the method `hi` exists for reference `&reddit_1::Wrapper<&()>`, but its trait bounds were not satisfied
    //FIX: pass T directly, not &T
    // wrapper.hi();
}

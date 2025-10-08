use std::marker::PhantomData;

struct Primary;
impl Primary {
    fn split(&mut self) -> (Secondary1<'_>, Secondary2<'_>) {
        (
            Secondary1 {
                data: PhantomData
            },
            Secondary2 {
                data: PhantomData
            }
        )
    }
}

#[derive(Debug)]
struct Secondary1<'a> {
    data: PhantomData<&'a i32>,
}

#[derive(Debug)]
struct Secondary2<'a> {
    data: PhantomData<&'a i32>,
}

fn main() {
    let mut p = Primary;
    let (s1, s2) = p.split();

    // it works
    // p.split();

    dbg!(s1);
    _ = p.split();
}

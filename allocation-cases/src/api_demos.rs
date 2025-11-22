
pub fn clone_from_may_avoid_allocations_demo() {
    let mut v1 = Vec::with_capacity(99);
    let v2 = vec![1, 2, 3];
    // v1's allocation is reused
    v1.clone_from(&v2);

    dbg!(&v1);
    dbg!(v1.capacity());
    assert_eq!(v1.capacity(), 99);
}

pub fn cow_usage_samples() {
    use std::borrow::Cow;

    let mut errors: Vec<Cow<'static, str>> = vec![];
    errors.push(Cow::Borrowed("something went wrong"));
    errors.push(Cow::Owned(format!("something went wrong on line {}", 100)));

    let cow = Cow::from("something else went wrong");

    match &cow {
        Cow::Borrowed(s) => println!("borrowed: {}", s),
        Cow::Owned(s) => println!("owned: {}", s),
    }

    dbg!(&cow);
    errors.push(cow);

    let cow : Cow<str> = format!("something else went wrong on line {}", 101).into();
    dbg!(&cow);
    match &cow {
        Cow::Borrowed(s) => println!("borrowed: {}", s),
        Cow::Owned(s) => println!("owned: {}", s),
    }

    errors.push(cow);
}

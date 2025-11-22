use std::fs::File;
use std::io::Read;

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


pub fn boxed_slices() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // size_of_val is used for ?Size types (trait objects or [T])
    dbg!(size_of_val(&v));
    assert_eq!(size_of_val(&v), 3 * size_of::<usize>());



    let bs: Box<[i32]> = v.into_boxed_slice();

    dbg!(size_of_val(&bs));
    assert_eq!(size_of_val(&bs), 2 * size_of::<usize>());

    // boxed slice can ber constructed directly from an iterator with Iterator::collect
    // avoiding the need for reallocation
    let bs: Box<[u32]> = (1..3).collect();

    // a boxed slice can be converted to a vector without any cloning or reallocation
    let vector = bs.into_vec();
}


pub fn vector_api() {
    // Very fast due to OS assistance for this type of initialization (C)
    let zeroed = vec![0u8; 10];
}


pub fn iterator_chain_demo() -> Result<(), Box<dyn std::error::Error>> {
    let f1 = File::open("./foo.txt")?;
    let f2 = File::open("./bar.txt")?;

    let mut handle = f1.chain(f2);
    let mut buffer = String::new();

    // read the value into a String. We could use any Read method here,
    // this is just one example.
    handle.read_to_string(&mut buffer)?;
    dbg!(buffer);
    Ok(())
}


pub fn indexing_over_slice_not_vec() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // theoretically it omits bound checks
    for v in vec.as_slice() {
        println!("{}", v);
    }
}


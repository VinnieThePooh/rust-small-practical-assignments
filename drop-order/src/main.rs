fn main() {
    println!("Entering nested scope");
    {
        let foo = DropContainer::new();
        println!("{foo:?}");
    }
    println!("Escaped nested scope");
}


#[derive(Debug)]
struct DropContainer {
    buzz: Buzz,
    bar: Bar,
    foo: Foo,
}
impl DropContainer {
    fn new() -> Self {
        Self {
            foo: Foo {val: 42 },
            bar: Bar {name: String::from("Ryan")},
            buzz: Buzz {age: 36}
        }
    }
}

impl Drop for DropContainer {
    fn drop(&mut self) {
        println!("Dropping container");
    }
}

#[derive(Debug)]
struct Foo {
    val: usize
}
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo");
    }
}


#[derive(Debug)]
struct Bar {
    name: String,
}
impl Drop for Bar {
    fn drop(&mut self) {
        println!("Dropping Bar");
        self.name.clear();
    }
}

#[derive(Debug)]
struct Buzz {
    age: u8,
}
impl Drop for Buzz {
    fn drop(&mut self) {
        println!("Dropping Buzz");
        self.age = 0;
    }
}
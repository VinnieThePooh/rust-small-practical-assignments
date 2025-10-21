fn main() {

    let mut bar = Bar;
    bar.consume();
    (&mut bar).consume();
    bar.consume();
    (&mut bar).consume();

    bar.show_name();
    bar.show_name_mut();
}

trait Foo {
    fn consume(self);
}

struct Bar;
impl Bar {
    fn show_name(&self) {
        println!("I am Bar!");
    }

    fn show_name_mut(&mut self) {
        println!("I am mutable Bar!");
    }
}

impl Foo for &Bar {
    fn consume(self) {
        self.show_name();
    }
}

impl Foo for &mut Bar {
    fn consume(self) {
        self.show_name_mut();
    }
}
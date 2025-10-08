use std::borrow::Borrow;

fn main() {
    let my_struct = MyStruct;
    g(my_struct);

    let my_struct = MyStruct;
    g2(&my_struct);

    let my_struct_boxed: Box<dyn MyTrait> = Box::new(MyStruct);
    g::<dyn MyTrait, _>(my_struct_boxed);

    let my_struct_boxed: Box<dyn MyTrait> = Box::new(MyStruct);
    // deref coercion from &Box<T> to &T - looks better
    g2(&my_struct_boxed);
}

trait MyTrait: Send + Sync {
    fn f(&self);
}

struct MyStruct;
impl MyTrait for MyStruct {
    fn f(&self) {
        println!("MyStruct::f called");
    }
}

impl AsRef<MyStruct> for MyStruct {
    fn as_ref(&self) -> &MyStruct {
        self
    }
}

fn g<T: MyTrait + ?Sized, B: Borrow<T>>(x: B) {
    x.borrow().f();
}

fn g2<T: MyTrait + ?Sized, B: AsRef<T>>(x: &B) {
    x.as_ref().f();
}

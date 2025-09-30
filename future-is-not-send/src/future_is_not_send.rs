use std::sync::Mutex;

pub fn is_send<T: Send>(t: T) {}

pub fn future_is_not_send_demo() {
    is_send(foo());
}

async fn foo() {
    bar(&Mutex::new(22)).await
}

async fn bar(x: &Mutex<u32>) {
    // MutexGuard is not Send => parent Future too
    let g = x.lock().unwrap();
    baz().await;;
}

async fn baz() {}


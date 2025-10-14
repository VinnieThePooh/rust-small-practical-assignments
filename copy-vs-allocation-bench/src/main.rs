use std::thread;
use std::time::Instant;

fn main() {
    array_sharing_bench();
    vector_sharing_bench();
}

// new Vec<> instance per iteration
fn vector_sharing_bench() {

    println!("Vector sharing bench:");
    let (sender, receiver) = std::sync::mpsc::channel::<Vec<_>>();


    thread::scope(|s| {
        let instant = Instant::now();
        // producer
        s.spawn(move || {
            println!("\tProducer: {:?}", thread::current().id());
            for _ in 0..10_000 {

                // аллоцируем новый вектор на каждой итерации,
                // let vec: Vec<_> = rand::random_iter::<u8>().take(1024).collect();
                let vec: Vec<u8> = Vec::with_capacity(1024);
                sender.send(vec).unwrap();
            }
        });

        // consumer
        println!("\tConsumer: {:?}", thread::current().id());
        while let Ok(res) = receiver.recv() {
            std::hint::black_box(&res);
            // println!("Got data: {} bytes", res.len())
        }
        println!("Elapsed: {:?}ms", instant.elapsed().as_millis());
    });
}

fn array_sharing_bench() {

    println!("Array copy bench:");

    // аллоцируем только один раз
    let buffer: [u8; 1024] = [0; _];
    let (sender, receiver) = std::sync::mpsc::channel::<[u8; 1024]>();

    thread::scope(|s| {
        let instant = Instant::now();
        // producer
        s.spawn(move || {
            println!("\tProducer: {:?}", thread::current().id());
            for _ in 0..10_000 {
                // rand::fill(&mut buffer);
                // просто копируем один и тот же буфер
                sender.send(buffer).unwrap();
            }
        });

        // consumer
        println!("\tConsumer: {:?}", thread::current().id());
        while let Ok(res) = receiver.recv() {
            std::hint::black_box(&res);
            // println!("Got data: {} bytes", res.len())
        }

        println!("Elapsed: {:?}ms", instant.elapsed().as_millis());
    });
}

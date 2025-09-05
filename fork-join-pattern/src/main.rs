use std::slice::Chunks;
use std::thread;
use std::thread::ScopedJoinHandle;
use std::time::Instant;

fn main() {

    let size = 1000;
    let chunk_size = 250;

    let now = Instant::now();
    let _res = solution_two(size, chunk_size);
    println!("v2 elapsed: {:?}", now.elapsed());

    let now = Instant::now();
    let _res = solution_three(size, chunk_size);
    println!("v3 elapsed: {:?}", now.elapsed());

    let now = Instant::now();
    let _res = solution_four(size, chunk_size);
    println!("v4 elapsed: {:?}", now.elapsed());
}

// not working - it prolongs `scope lifetime - it's prohibited
// NOTE(source code): good lifetime relationship
fn solution_one(size: i32, chunk_size: i32) {
    let nums = Vec::from_iter(0..100);
    let mut results: [Option<ScopedJoinHandle<i32>>; 4] = [const { None }; 4];

    thread::scope(|s| {
        let mut i = 0;
        for chunk in nums.chunks(25) {
            let handle = s.spawn(|| {
                let local = chunk.iter().sum::<i32>();
                println!("Computed result in {}th thread", local);
                return local;
            });
            // not working - it prolongs `scope lifetime
            // results[i] = Some(handle.into());
            i += 1;
        }
        println!("Hello from the main thread!");
    });

    // it is guaranteed that all the threads are completed here
    dbg!(&results.map(|x| x.unwrap().join()));
}

// split_first_mut based - it works!
fn solution_two(size: usize, chunk_size: usize) -> Vec<usize> {

    let nums = Vec::from_iter(0..size);
    let chunks = nums.chunks(chunk_size);

    let (_, Some(len)) = chunks.size_hint() else {
        panic!("No upper bound determined!");
    };
    let mut results = vec![0; size];
    let (mut dest, mut rest) = results.split_first_mut().unwrap();

    thread::scope(|s| {
        for (index, chunk) in chunks.enumerate() {
            s.spawn(|| {
                *dest = chunk.iter().sum();
            });
            if index == len - 1 {
                break;
            }
            (dest, rest) = rest.split_first_mut().unwrap();
        }
    });
    results
}

// variant1 (from Katasonov)
fn solution_three(size: usize, chunk_size: usize) -> Vec<usize> {
    let nums = Vec::from_iter(0..size);

    let chunks = nums.chunks(chunk_size);
    thread::scope(|s| {
        // burst threads
        let threads: Vec<_> = chunks
            .map(|chunk| s.spawn(|| chunk.iter().sum::<usize>()))
            .collect();
        // collect results
        threads.into_iter().map(|it| it.join().unwrap()).collect()
    })
}

// variant2 (from Katasonov)
fn solution_four(size: usize, chunk_size: usize) -> Vec<usize> {

    let nums = Vec::from_iter(0..size);

    let chunks = nums.chunks(chunk_size);
    let (_, Some(len)) = chunks.size_hint() else {
        panic!("No upper bound determined!");
    };
    let mut results = vec![0; len];

    thread::scope(|s| {
        chunks.zip(results.iter_mut()).for_each(|(chunk, res)| {
            s.spawn(|| *res = chunk.iter().sum::<usize>());
        })
    });
    results
}

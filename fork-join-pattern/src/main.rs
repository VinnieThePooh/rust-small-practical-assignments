use std::thread;
use std::thread::ScopedJoinHandle;

fn main() {

    // solution_one();
    solution_two();
}

// not working - it prolongs `scope lifetime
fn solution_one() {
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
fn solution_two() {
    let nums = Vec::from_iter(0..100);
    let mut results: [i32; 4] = [0; 4];
    let (mut dest, mut rest) = results.split_first_mut().unwrap();

    let chunks = nums.chunks(25);
    let (low, upper) = chunks.size_hint();
    let len = upper.unwrap();
    println!("Size hint {:?}", len);

    thread::scope(|s| {
        for (index, chunk) in  chunks.enumerate() {
            s.spawn(|| {
                *dest = chunk.iter().sum::<i32>();                
            });

            if index == len - 1 {
                break;
            }
            (dest, rest) = rest.split_first_mut().unwrap();
        }
    });
    dbg!(results);
}

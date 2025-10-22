use rayon::prelude::*;

fn main() {
    // enumerate_call_demo();
    // break_iteration_demo();
    enumerate_with_slice_demo();
}


fn enumerate_call_demo() {
    let spd:Vec<i32> = (0..10).collect();
    let _ = spd.par_iter().enumerate().skip(4).for_each(|(i, v) | {
        if i == 7 {
            println!("{i}, {v}");
        }
    });
}


fn enumerate_with_slice_demo() {
    let spd:Vec<i32> = (0..10).collect();
    let _ = spd[4..].par_iter().enumerate().for_each(|(i, v) | {
        println!("checking: {i}, {v}");
        if i == 5 {
            println!("Found: {i}, {v}");
        }
    });
}


// no guarantees on what items processing would be cancelled
// depends on scheduler
fn break_iteration_demo() {
    let spd:Vec<i32> = (0..=10).collect();
    let _ = spd.par_iter().enumerate().skip(4).try_for_each(|(i, v) | {
        println!("checking: {i}, {v}");
        if i == 7 {
            println!("Found: {i}, {v}");
            return Err(());
        }

        Ok(())
    });
}
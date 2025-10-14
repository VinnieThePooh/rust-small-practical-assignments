use std::array;

fn main() {
    move_elements_out_of_an_array();
    iterate_over_array();
}


// You can use a slice pattern to move elements out of an array:
fn move_elements_out_of_an_array() {

    let array = ["John".to_string(), "Roa".to_string()];
    let [john, roa] = array;
    move_away(john);
    move_away(roa);

    // not compiled - value used after being moved
    // array.len()

    // not compiled - use of partially moved value
    // dbg!(array);


    fn move_away(str: String) {
        println!("{}", str);
    }
}

fn iterate_over_array() {
    let mut array: [usize; 5] = array::from_fn(|i| i + 1);
    dbg!(&array);

    for i in array {
        println!("Value: {i}");
    }

    println!("Mutating array");
    for i in &mut array {
        *i *= 2;
    }

    for i in &array {
        println!("Value: {i}");
    }    
}
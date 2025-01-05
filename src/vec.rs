//! Borrowing values through vectors.

fn act<'a>(vec: &'a mut Vec<u32>) {                     // <--              + 'a lifetime
    let first_value = add(vec, rand::random());         // <-- + '1 starts  |
    println!("First value: {}", first_value);           //     | '1 ends    |
    let second_value = add(vec, rand::random());        // <-- / '2 starts  |
    // println!("First value: {}", first_value);        //     |            |
    println!("Second value: {}", second_value);         //     |            |
    clean(vec);                                         // <-- - '2 ends    |
    // println!("First value: {}", first_value);        //                  |
    // println!("Second value: {}", second_value);      //                  |
}                                                       // <--              -

fn add(vec: &mut Vec<u32>, value: u32) -> &u32 {
    vec.push(value);
    vec.last().unwrap()
}

fn clean(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}
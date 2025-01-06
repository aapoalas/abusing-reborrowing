//! Borrowing values through vectors.

#[allow(clippy::needless_lifetimes)]
fn act<'a>(vec: &'a mut Vec<u32>) {                     // <--              + 'a lifetime
    let first_value = add(vec, rand::random());         // <-- + '1 starts  |
    println!("First value: {}", first_value);           //     | '1 ends    |
    let second_value = add(vec, rand::random());        // <-- / '2 starts  |
    // println!("First value: {}", first_value);        //     |            |
    println!("Second value: {}", second_value);         //     |            |
    clean(vec);                                         // <-- - '2 ends    |
    // println!("First value: {}", first_value);        //                  |
    // println!("Second value: {}", second_value);      //                  |
    // use_mut_and_ref_together(vec);                   //                  |
}                                                       // <--              -

fn add(vec: &mut Vec<u32>, value: u32) -> &u32 {
    vec.push(value);
    vec.last().unwrap()
}

fn clean(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}

#[allow(unused)]
fn use_mut_and_ref_together(vec: &mut Vec<u32>) {
    add(vec, rand::random());
    let first_value = vec.first().unwrap();
    println!("&'a mut Vec<u32>: {:?}; &'a u32: {}", vec, first_value);
    add(vec, rand::random());
    use_mut_and_ref_together_interprocedurally(vec);
}

#[allow(unused)]
fn use_mut_and_ref_together_interprocedurally(vec: &mut Vec<u32>) {
    add(vec, rand::random());
    #[allow(unused)]
    let first_value = vec.first().unwrap();
    // interprocedural_print_and_add(vec, first_value);
}

#[allow(unused)]
fn interprocedural_print_and_add(vec: &mut Vec<u32>, first_value: &u32) {
    println!("&'a mut Vec<u32>: {:?}; &'a u32: {}", vec, first_value);
    add(vec, rand::random());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}
//! Borrowing values through vectors.

#[allow(clippy::needless_lifetimes)]
fn act<'a>(vec: &'a mut Vec<u32>) {						// <--				+ 'a lifetime
    let first_value = add(vec, rand::random());			// <--	+ '1		|
    println!("First value: {}", first_value);			//		|			|
    let second_value = add(vec, rand::random());        // <--	- '1, + '2	|
    // println!("First value: {}", first_value);		//		|			|
    println!("Second value: {}", second_value);         //		|			|
    clean(vec);											// <--	- '2		|
    // println!("First value: {}", first_value);        //					|
    // println!("Second value: {}", second_value);      //					|
    // use_mut_and_ref_together(vec);					//					|
}														// <--				-

/// Add value to vector and return a reference to it.
fn add(vec: &mut Vec<u32>, value: u32) -> &u32 {
    vec.push(value);
    vec.last().unwrap()
}

/// Clean the vector of unwanted values.
fn clean(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}








/// Boggle the mind by using a mutable reference and a shared reference
/// reborrowed from it at the same time.
#[allow(unused)]
fn use_mut_and_ref_together(vec: &mut Vec<u32>) {
    add(vec, rand::random());
    let first_value = vec.first().unwrap();
    println!("&'a mut Vec<u32>: {:?}; &'a u32: {}", vec, first_value);
    add(vec, rand::random());
    
    // Let's do that again!
    use_mut_and_ref_together_interprocedurally(vec);
}

/// Boggle the mind by attempting to do the exact same as above, but this time
/// just move the last two lines inside a separate function.
#[allow(unused)]
fn use_mut_and_ref_together_interprocedurally(vec: &mut Vec<u32>) {
    add(vec, rand::random());
    #[allow(unused)]
    let first_value = vec.first().unwrap();
    // interprocedural_print_and_add(vec, first_value);
}

/// Perfectly valid code, I guarantee it!
#[allow(unused)]
fn interprocedural_print_and_add(vec: &mut Vec<u32>, first_value: &u32) {
    println!("&'a mut Vec<u32>: {:?}; &'a u32: {}", vec, first_value);
    add(vec, rand::random());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}
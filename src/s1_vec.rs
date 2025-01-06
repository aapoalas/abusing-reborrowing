//! Borrowing values through vectors.

fn act<'a>(vec: &'a mut Vec<u32>) {						// ==>				+ 'a
    let first = add(vec, rand::random());				// ==>	+ '1		| &'a mut
    println!("First value: {}", first);					//		|			|
    let second = add(vec, rand::random());				// <=>	- '1, + '2	| &'a mut
    // println!("First value: {}", first);				//		|			|
    println!("Second value: {}", second);				//		|			|
    clean(vec);											// <==	- '2		| &'a mut
    // println!("First value: {}", first);        		//					|
    // println!("Second value: {}", second);      		//					|
    // act_two(vec);									//					|
}														// <==				-

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
fn act_two(vec: &mut Vec<u32>) {
	// Act 2 preparation
    add(vec, rand::random());
    let first = vec.first().unwrap();
    // Act 3 finale
    println!("&'a mut Vec<u32>: {:?}; &'a u32: {}", vec, first);
    add(vec, rand::random());

    // Let's do that again!
    act_three(vec);
}

/// Boggle the mind by attempting to do the exact same as above, but this time
/// just move the last two lines inside a separate function.
fn act_three(vec: &mut Vec<u32>) {
	// Act 3 preparation
    add(vec, rand::random());
    let first = vec.first().unwrap();
    // Act 3 finale
    // act_three_finale(vec, first);
}

/// Perfectly valid code, I guarantee it!
fn act_three_finale(vec: &mut Vec<u32>, first: &u32) {
    println!("&'a mut Vec<u32>: {:?}; &'a u32: {}", vec, first);
    add(vec, rand::random());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}

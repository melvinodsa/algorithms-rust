use std::io;
fn main() {
    const MAX: usize = 10000;
    println!("Input the array followed by 'e'");

    //variable to store the input array
    let mut arr: [i32; MAX] = [0; MAX];
    let mut counter: usize = 0; //coutner has the length of the array to be considered

    //building the array
    loop {
        //getting the input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.as_str() {
                "e\n" => break,
                _ => {}
            },
            Err(_) => continue,
        };
        //parsing the input
        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input a number or e to end the array");
                continue;
            }
        };
        //adding it to the array
        arr[counter] = number;
        counter += 1;
    }

    //insertion algorithm
    insertion_sort(&mut arr[..counter]);

    //printing the sequence
    print_array(&arr[..counter]);
}

fn insertion_sort(arr: &mut [i32]) {
    //for rust, only usize is allowed as array size.
    //But usize cannot be < 0. To overcome that we use int for updation.
    let mut i_u: usize;
    let mut i: i32;
    let mut key: i32;
    //insertion sort begins
    for j in 1..arr.len() {
        key = arr[j];
        //insert the key into the sequence arr[1..j-1]
        i_u = j - 1;
        i = i_u as i32;
        while i >= 0 && arr[i_u] > key {
            arr[i_u + 1] = arr[i_u];
            i -= 1;
            if i >= 0 {
                i_u = i as usize;
            }
        }
        arr[(i + 1) as usize] = key;
    }
}

fn print_array(arr: &[i32]) {
    print!("The sorted sequence is:\n[");
    for j in 0..arr.len() {
        print!("{}", arr[j]);
        if j != arr.len() - 1 {
            print!(", ");
        }
    }
    print!("]\n");
}

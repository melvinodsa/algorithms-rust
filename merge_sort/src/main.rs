use std::io;

const MAX: usize = 10000;
const MAXI32: i32 = std::i32::MAX;

fn main() {
    println!("Input the array followed by 'e'");

    //variable to store the input array
    let mut arr: [i32; MAX] = [0; MAX];
    let mut counter: usize = 0; //counter has the length of the array to be considered

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

    //merge sort
    print!("The input sequence is:");
    print_array(&arr[..counter]);
    merge_sort(&mut arr[..counter], 0, counter - 1);

    //printing the sequence
    print!("The sorted sequence is:");
    print_array(&arr[..counter]);
}

fn merge_sort(arr: &mut [i32], p: usize, r: usize) {
    //if p >= r then we already have a sorted array
    if p < r {
        let q = (p + r) / 2;
        merge_sort(arr, p, q);
        merge_sort(arr, q + 1, r);
        merge_subroutine(arr, p, q, r);
    }
}

fn merge_subroutine(arr: &mut [i32], p: usize, q: usize, r: usize) {
    let n1 = q - p + 1;
    let n2 = r - q;
    //variable to store the input array
    let mut l_arr: [i32; MAX] = [0; MAX];
    let mut r_arr: [i32; MAX] = [0; MAX];
    for i in 0..n1 {
        l_arr[i] = arr[p + i];
    }
    for i in 0..n2 {
        r_arr[i] = arr[q + i + 1];
    }
    l_arr[n1] = MAXI32;
    r_arr[n2] = MAXI32;
    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in p..r + 1 {
        if l_arr[i] <= r_arr[j] {
            arr[k] = l_arr[i];
            i += 1;
        } else {
            arr[k] = r_arr[j];
            j += 1;
        }
    }
}

fn print_array(arr: &[i32]) {
    if arr.len() > 0 {
        print!("\n[ ");
    }
    for j in 0..arr.len() {
        print!("{}", arr[j]);
        if j != arr.len() - 1 {
            print!(", ");
        }
    }
    if arr.len() > 0 {
        print!(" ]\n");
    }
}

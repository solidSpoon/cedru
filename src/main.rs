use std::env;
use std::str::FromStr;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && (args[1] == "-h" || args[1] == "--help") {
        print_help(args.get(2));
        std::process::exit(0);
    }

    let mut reverse = false;
    let mut unique = false;
    let mut input_arg_index = None;

    for (i, arg) in args.iter().enumerate().skip(1) {
        match arg.as_str() {
            "-r" | "--reverse" => reverse = true,
            "-u" | "--unique" => unique = true,
            _ => {
                if input_arg_index.is_none() {
                    input_arg_index = Some(i);
                } else {
                    eprintln!("Error: Incorrect arguments.");
                    print_usage(&args[0]);
                    std::process::exit(1);
                }
            }
        }
    }

    let input_arg_index = match input_arg_index {
        Some(index) => index,
        None => {
            eprintln!("Error: Missing input.");
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };

    let numbers: Result<Vec<i32>, _> = args[input_arg_index]
        .split(',')
        .map(|s| i32::from_str(s))
        .collect();

    match numbers {
        Ok(mut numbers) => {
            quick_sort(&mut numbers);
            if unique {
                numbers.dedup();
            }
            if reverse {
                numbers.reverse();
            }
            println!("Sorted: {:?}", numbers);
        }
        Err(e) => {
            eprintln!(
                "Error: Invalid input. Please enter comma-separated integers. Error: {:?}",
                e
            );
            std::process::exit(1);
        }
    }

}

fn print_help(topic: Option<&String>) {
    match topic {
        Some(t) if t == "-r" || t == "--reverse" => {
            println!("Option: -r, --reverse");
            println!("Sort the list of comma-separated integers in descending order.");
        }
        Some(t) if t == "-u" || t == "--unique" => {
            println!("Option: -u, --unique");
            println!("Remove duplicate values from the sorted list of comma-separated integers.");
        }
        _ => {
            print_usage(&env::args().nth(0).unwrap_or_else(|| String::from("quicksort")));
        }
    }
}

fn print_usage(program_name: &str) {
    println!("Usage: {} [-r | --reverse] [-u | --unique] <comma-separated numbers>", program_name);
    println!("Sort a list of comma-separated integers using the quick sort algorithm.");
    println!("Options:");
    println!("-r, --reverse    Sort in descending order.");
    println!("-u, --unique     Remove duplicate values from the sorted output.");
    println!("-h, --help       Display this help message.");
}

// A generic quick_sort function that accepts a mutable slice of type T.
// It sorts the slice in-place using the quick sort algorithm.
fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let middle = partition(arr);
    quick_sort(&mut arr[0..middle]);
    quick_sort(&mut arr[middle + 1..])
}

// A generic partition function that accepts a mutable slice of type T.
// It rearranges the elements in the slice using the last element as a pivot,
// and returns the final index of the pivot element.
fn partition<T: Ord + Copy>(nums: &mut [T]) -> usize {
    let mut pivot_index = 0;
    let right = nums.len() - 1;
    for index in 0..nums.len() {
        if nums[index] < nums[right] {
            nums.swap(pivot_index, index);
            pivot_index += 1;
        }
    }
    nums.swap(pivot_index, right);
    return pivot_index;
}

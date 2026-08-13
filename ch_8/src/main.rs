use ch_8::{add_employee_names, convert_to_pig_latin, find_median_mode};
fn main() {
    // creating a new vector
    //let v:Vec<i32> = Vec::new();
    //let v = vec![1,2,3]

    //let mut v = Vec::new();
    //v.push(5);
    //v.push(6);
    //v.push(7);
    //v.push(8);
    //
    ////reading elements of vectors
    //let vec = vec![1, 2, 3, 4, 5];
    //let third: &i32 = &vec[2];
    //println!("The third element is {third}");

    //let third: Option<i32> = vec.get(2);
    //match third {
    //    Some(third) => println!("The third element is {third}"),
    //    None => println!("There is no third element."),
    //}
    let numbers = vec![3, 4, 2, 3, 3, 5];
    let stats = find_median_mode(&numbers);
    if let Some(median) = stats.median {
        println!("{}", median);
    }
    if let Some(mode) = stats.mode {
        println!("{}", mode);
    }

    let word = String::from("apple");
    //println!("{}", convert_to_pig_latin(&word));
    //println!("{}", convert_to_pig_latin("first"));
    add_employee_names();
}

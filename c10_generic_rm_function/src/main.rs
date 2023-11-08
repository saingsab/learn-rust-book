fn leargest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = leargest(&number_list);

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest = leargest(&number_list);

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    println!("The largest number is {}", largest);
}

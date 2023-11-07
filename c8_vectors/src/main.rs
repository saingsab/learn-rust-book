fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(100);
    /*
    When the get method is passed an index that is outside the vector, 
    it returns None without panicking. 
    You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances. 
    */
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."), 
    }

    //  loop to get immutable references to each element in a vector of i32 values and print them.
    for i in &v {
        println!("{i}");
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("List of vectors {:?}", v);

    // Because vector stor only one type, here is the solution to store the deference type of data 
    // by using enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    
    println!("The value of vector: {:#?}", row);
}

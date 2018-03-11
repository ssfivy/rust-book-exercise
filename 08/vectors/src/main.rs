fn main() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);

    let third: &i32 = &v2[2]; // if invalid, causes panic
    let third2: Option<&i32> = v2.get(2); //if invalid, returns None
    let third3 = v2.get(2);

    {
        let first = &v3[2];
        //v3.push(8) // causes error
    }

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}

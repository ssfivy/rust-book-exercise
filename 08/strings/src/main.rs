fn main() {
    let mut sm = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = String::from("some sontents");
    println!("{}", s);

    let s2 = "bar";
    sm.push_str(&s2);

    println!("Hello, {}!", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    // stirng slicing ( bytes!)
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // string iteration - code points
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // string iteration - bytes (for sending down serial line etc)
    for b in "नमस्ते".bytes() {
    println!("{}", b);
    }
}

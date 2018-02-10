fn main() {
    println!("Main function!");
    another_function(five());
}

fn another_function(x : i64) {
    println!("Another function! x = {}", x);
}

fn five() -> i64 {
    let x = 3;
    x + 2
}


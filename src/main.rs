fn fibonacci(n: i128) -> i128 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let mut i:i128 = 0;
    let mut First:i128 = 0;
    let mut Second:i128 = 1;
    let mut Next:i128 = 0;
    while true {
        // println!("index ({}) => {}", i, fibonacci(i));
        if i <= 1 {
            println!("index ({}) => {}", i, i);
        } else {
            Next = First + Second;
            First = Second;
            Second = Next;
            println!("index ({}) => {}", i, Next);
        }
        i = i + 1;
    }
}

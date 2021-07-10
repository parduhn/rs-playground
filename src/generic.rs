pub fn run() {
    // generische Typen
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Die größte Zahl ist {}", result);
    let char_list = vec!['y', 'z', 'a', 'q'];
    let result = largest(&char_list);
    println!("Das größte Zeichen ist {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

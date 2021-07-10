fn main() {
    // generische Typen
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Die größte Zahl ist {}", result);
    let char_list = vec!['y', 'z', 'a', 'q'];
    let result = largest(&char_list);
    println!("Das größte Zeichen ist {}", result);

    // gültigkeiten
    let string1 = String::from("lange Zeichenkette ist lang");
    let string2 = String::from("xyz"); //works
    let result;
    {
        // let string2 = String::from("xyz");//dontwork
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("Die längere Zeichenkette ist {}", result);
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

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

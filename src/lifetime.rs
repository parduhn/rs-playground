pub fn run() {
    let string1 = String::from("lange Zeichenkette ist lang");
    let string2 = String::from("xyz"); //works
    let result;
    {
        // let string2 = String::from("xyz");//dontwork
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("Die längere Zeichenkette ist {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Hello, {}!", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    let x = 4;

    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("others"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("matched y = {:?}", y),
        _ => println!("default case x = {:?}", x),
    }
    println!("at the end: x = {:?} y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        2 | 4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let x = 9;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

   
}

struct Point {
    x: i32,
    y: i32,
}

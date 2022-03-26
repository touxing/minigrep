use gui::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                name: "selectBox".to_string(),
                width: 73,
                height: 32,
                options: vec![
                    String::from("apple"),
                    String::from("banana"),
                    String::from("durian"),
                ],
            }),
            Box::new(Button {
                name: "button".to_string(),
                width: 30,
                height: 30,
                label: String::from("click me"),
            }),
            // 需要为 String 实现 Draw trait
            // 孤儿原则，不能为外部实现实现外部 trait
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button{
  pub width: u32,
  pub height: u32,
  pub label: String,
  pub name: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Button {} draw", self.name);
  }
}

pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
  pub name: String,
}

impl Draw for SelectBox {
  fn draw(&self) {
      println!("SelectBox {} draw", self.name);
  }
}


// 孤儿原则
impl Draw for String {
  fn draw(&self) {
      println!("draw from String");
  }
}
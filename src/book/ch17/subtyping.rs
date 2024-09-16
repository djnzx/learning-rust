pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
    //                       ^
    //                       |
    //                       allows subtyping
    //                       and say, resolution will be made in runtime (means extra call)
    //                       therefore can't be inlined
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// better syntax
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        self.components
            .iter()
            .for_each(|c| c.draw());
    }
}

// impl<T: Draw> Screen2<T> {
//     pub fn run(&self) {
//         self.components.iter().for_each(|c| c.draw());
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "I'm a button with w:{} h:{} label:{}",
            self.width, self.height, self.label
        );
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "I'm a selectBox with w:{} h:{} options:{:?}",
            self.width, self.height, self.options
        );
    }
}

#[test]
fn example() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Yes"), String::from("Maybe"), String::from("No")],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

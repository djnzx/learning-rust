pub struct AveragedCollection {
    // they are private
    list: Vec<i32>,
    average: Option<f64>,
}

pub trait HasAverage {
    fn average(&self) -> Option<f64>;
}

// how to attach trait to the impl block
impl HasAverage for AveragedCollection {
    fn average(&self) -> Option<f64> {
        self.average
    }
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: None,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // private
    fn update_average(&mut self) {
        self.average = match self.list.len() {
            0 => None,
            len => {
                let sum = self.list.iter().sum::<i32>();
                let avg = sum as f64 / len as f64;
                Some(avg)
            }
        }
    }
}

#[test]
fn code1() {
    let mut ac = AveragedCollection::new();

    dbg!(ac.average()); // None

    ac.add(10);
    dbg!(ac.average()); // Some 10

    ac.add(20);
    dbg!(ac.average()); // Some 15

    ac.remove();
    dbg!(ac.average()); // Some 10
}

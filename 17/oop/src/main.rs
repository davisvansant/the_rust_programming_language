fn seventeen_one() {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }
}

fn seventeen_two() {
    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.listpop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
}

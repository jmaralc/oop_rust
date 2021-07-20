// Concerning encapsulation, inheritance, polymorphism
use std::collections::HashSet;

pub struct AveragedCollection {
    // list: Vec<i32>,
    list: HashSet<i32>,
    average: f64,
}

pub trait Average {
    // This is similar to a parent class having an implementation 
    // of a method and an inheriting child class also having the implementation of the method. 
    // Ok, but how to do something more complex referring to the self?
    fn update_average(&mut self) {
        println!("Average trait under construction!!")
    }

}

// impl Average for AveragedCollection {}
impl Average for AveragedCollection{
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

impl AveragedCollection {
    // pub fn new(list: Vec<i32>) -> AveragedCollection{
    pub fn new(list: HashSet<i32>) -> AveragedCollection{
        let average: f64 = 0.0;
        let mut initial_collection = AveragedCollection{
            list,
            average
        };
        initial_collection.update_average();
        initial_collection
    }

    pub fn add(&mut self, value: i32) {
        // self.list.push(value);
        self.list.insert(value);
        self.update_average();
    }

    // pub fn remove(&mut self) -> Option<i32> {
    //     let result = self.list.pop();
    //     match result {
    //         Some(value) => {
    //             self.update_average();
    //             Some(value)
    //         }
    //         None => None,
    //     }
    // }

    pub fn remove(&mut self, element: i32) {
        let result = self.list.remove(&element);
        self.update_average();
        println!("Removed: {}", result);
    }

    // Classic getter
    pub fn average(&self) -> f64 {
        self.average
    }

    // fn update_average(&mut self) {
    //     let total: i32 = self.list.iter().sum();
    //     self.average = total as f64 / self.list.len() as f64;
    // }
}

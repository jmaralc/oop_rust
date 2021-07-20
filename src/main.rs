mod example1;
mod example2;
mod example3;
mod example4;

use std::io;
use std::io::Write;
use std::collections::HashSet;

use crate::example1::*;
use crate::example2::Draw;
use crate::example2::{Button, Screen};


fn execute_example1() {
    println!("######Example 17.1 #########");
    // Let initialize the collection with a simple vector
    // let intial_vector = vec![1, 2, 3];
    let mut intial_vector = HashSet::new();

    intial_vector.insert(1);
    intial_vector.insert(2);
    intial_vector.insert(3);

    let mut average_collection = AveragedCollection::new(intial_vector);


    println!("The average of 1,2,3 is:");
    println!("Average {}",average_collection.average());

    // Let increase the vector
    print!("Insert a new value for the collection:");
    io::stdout().flush().unwrap();

    let mut new_entry = String::new();
    io::stdin().read_line(&mut new_entry).expect("Failed to read line");
    let new_entry: i32 = new_entry.trim().parse().expect("Please try a number!");

    println!("Adding {} to the collection", new_entry);
    average_collection.add(new_entry);

    // Let remove the 
    println!("Then the average is:");
    println!("Average {}",average_collection.average());


    // average_collection.remove();
    average_collection.remove(new_entry);

    println!("Average {}",average_collection.average());
}


fn execute_example2() {
    println!("######Example 17.2 #########");
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            for option in self.options.iter(){
                println!("🔘 {}", option);
            }
        }
    }


    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    screen.run();
/*
This concept—of being concerned only with the messages a value responds to rather than 
the value’s concrete type—is similar to the concept of duck typing in dynamically typed 
languages: if it walks like a duck and quacks like a duck, then it must be a duck! 
In the implementation of run on Screen in Listing 17-5, run doesn’t need to know what 
the concrete type of each component is. It doesn’t check whether a component is an 
instance of a Button or a SelectBox, it just calls the draw method on the component. 
By specifying Box<dyn Draw> as the type of the values in the components vector, 
we’ve defined Screen to need values that we can call the draw method on.
*/
}


fn execute_example3() {
    println!("######Example 17.3 #########");
    use crate::example3::Post;
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    println!("DRAFT state");
    println!("Content: {}", post.content());

    post.request_review();
    println!("PENDING REVIEW state");
    println!("Content: {}", post.content());

    post.approve();
    println!("PUBLISHED state");
    println!("Content: {}", post.content());



    // use crate::example4::Post;
    // let mut post = Post::new();
    // we’ll make it so draft posts don’t have the content method at all
    // println!("DRAFT state");
    // println!("Content: {}", post.content());

    // post.add_text("I ate a salad for lunch today");

    // let post = post.request_review();
    // PendingReviewPost struct doesn’t have a content method defined on it,
    // so attempting to read its content results in a compiler error, as with DraftPost
    // println!("PENDING REVIEW state");
    // println!("Content: {}", post.content());

    // let post = post.approve();
    // println!("DRAFT state");
    // println!("Content: {}", post.content());
}

fn main(){
    loop{
        println!("###############");
        println!("1. Example of 17.1 Characteristics of Object Oriented Languages");
        println!("2. Example of 17.2 Using Trait Objects that allow for values of different types");
        println!("3. Example of 17.3 Implementing an Object Oriented Design Pattern");
        println!("Press any other number to exit");
        print!("Select :");
        io::stdout().flush().unwrap();

        let mut new_entry = String::new();
        io::stdin().read_line(&mut new_entry).expect("Failed to read line");
        let new_entry: i32 = new_entry.trim().parse().expect("Please try a number!");


        match new_entry {
            1 => execute_example1(),
            2 => execute_example2(),
            3 => execute_example3(),
            _ => break
        }
    }
    println!("Bye!")
}
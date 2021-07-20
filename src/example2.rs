// Concerning trait objects

// Rust version of an interface
/*When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t 
know all the types that might be used with the code that is using trait objects, 
so it doesn’t know which method implemented on which type to call. 
Instead, at runtime, Rust uses the pointers inside the trait object to know which
 method to call
 */
pub trait Draw {
    fn draw(&self);
}

/*
A trait object points to both an instance of a type implementing our specified 
trait as well as a table used to look up trait methods on that type at runtime
*/

/*
We create a trait object by specifying some sort of pointer, 
such as a & reference or a Box<T> smart pointer, then the dyn keyword, 
and then specifying the relevant trait.
*/

// First trait object
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("🆗 {}", self.label);
    }
}
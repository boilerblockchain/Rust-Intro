

struct Generic<T>{
    pub val: T,
}


impl<T> Generic<T> 
// This is called a trait bound
where T: std::fmt::Display,
{
    pub fn display(&self) {
        println!("My val is: {}", self.val);
    }
}

#[allow(dead_code)]
struct CustomStruct {
    pub val: usize,
}

struct CustomWorkingStruct {
    pub val:usize,
}

// This is called implementing a trait
impl std::fmt::Display for CustomWorkingStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.val)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let num = Generic::<usize>{val:0};
    num.display();

    let string_t = Generic::<&'static str>{val: "Hello"};
    string_t.display();

    // This is okay!
    let _custom = Generic::<CustomStruct>{val: CustomStruct{val: 0}};
    // This is not okay!
    // _custom.display();

    // This is okay!
    let custom = Generic::<CustomWorkingStruct>{val: CustomWorkingStruct{val:0}};
    custom.display();

    let f = std::fs::File::open("does-not-exist");
    let _f = match f {
        Ok(_inner_file) => (),
        Err(e) => {
            println!("Got an error: {}", e);
        },
    };

    // short-hand form
    let _f = std::fs::File::open("does-not-exist")?;

    Ok(())
}

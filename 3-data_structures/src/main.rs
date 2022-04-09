mod my_mod {
    
// #[allow(dead_code)]
pub struct DS {
    pub val: usize,
}
pub struct PrivateDS {
    val: usize,
}

impl DS {
    // How to declare global constants for a data structure
    const VALUE:usize = 10;

    // Private struct function
    fn local_display() {
        println!("Local display: {}", DS::VALUE);
    }

    // Public struct function
    pub fn pub_display() {
        println!("Public display");
    }

    // Module public struct function
    pub(crate) fn pub_crate_display() {
        // These can call private functions
        // NOTE: Use Self instead of DS
        Self::local_display();
        println!("Public only for the crate - display");
    }
}

impl PrivateDS {
    // -> specifies the return type
    pub fn new(val:usize) -> Self {
        // The last open value is the returned value
        PrivateDS{val}
        // Above is the same as below
        // return PrivateDS{val};
    }

    // Member function that does not modify the structure
    pub fn display(&self) {
        println!("My value is {}", self.val);
    }

    // Member function that modifies the structure
    pub fn increment_and_display(&mut self) {
        self.val += 10;
        // This function can call immutable functions
        self.display();
    }

    // Consuming function
    pub fn consume(self) {
        println!("I have consumed myself");
    } 
}
}

fn main() {
    println!("Hello, world!");
    // Cannot call
    // my_mod::DS::local_display();
    my_mod::DS::pub_crate_display();
    my_mod::DS::pub_display();

    // Adding _ to a variable makes it ignore the unused variable warnings
    let _ds = my_mod::DS{val:0};

    // NOTE: global functions are not also local functions
    // _ds.local_display();

    // Cannot construct PrivateDS as above
    // let priv_ds = my_mod::PrivateDS{val: 0};

    let priv_ds = my_mod::PrivateDS::new(0); // We can construct it like this
    priv_ds.display();
    // Below does not work, since priv_ds is not declared mutable
    // All declarations are by default immutable
    // priv_ds.increment_and_display();

    // This is a move
    let mut mutable_ds = priv_ds;
    // priv_ds.display(); 
    // Above no longer works, since it does not exist as it has moved to mutable_ds

    // Now we can call mutable functions
    mutable_ds.increment_and_display();

    mutable_ds.consume();
    // Below will no longer work, as the object has consumed itself
    // mutable_ds.display();
}

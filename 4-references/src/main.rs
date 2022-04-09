mod my_mod {

pub struct DS {
    pub val: usize,
}

impl DS {
    pub fn display(&self) {
        println!("Displaying self");
    }

    // Static means that the reference is valid for the lifetime of the entire program
    pub fn display_str(&self, string: &'static str) {
        println!("Print string: {}", string);
    }

    pub fn limited_lifetime_str_display<'a>(&self, string: &'_ str) {
        println!("Printing limited lifetime string: {}", string);
    }
}

}

fn main() {
    let ds = my_mod::DS{val:10};
    // Borrow immutably
    let _ds2 = &ds;

    // Cannot borrow mutably since ds is immutable
    // let ds3 = &mut ds;

    let mut ds_mut = ds;
    // ds2 is invalid from here
    // ds2.display();

    let ds3 = &mut ds_mut;
    // References can call functions just like objects
    ds3.display();
    (*ds3).display();

    ds3.display_str("hello");
    let str = String::from("hello");
    // Below does not work, since this string is not valid for the lifetime of the program
    // ds3.display_str(str.as_str());

    // This will work
    ds3.limited_lifetime_str_display(str.as_str());
    // We are telling the borrow checker
}

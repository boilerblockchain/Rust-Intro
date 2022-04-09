mod file_local_module {
    static PRIVATE_VAL:usize = 10;
    pub static PUBLIC_VAL:usize = PRIVATE_VAL*2;
}

// NOTE: All file modules must be declared from the starting point!
mod file_mod;

// Add overview in slides

// NOTE: All folder modules must be declared from the starting point!
mod folder_mod;

fn main() {
    println!("Hello, world!");
    // Below will not compile
    // println!("Local module value: {}", file_local_module::PRIVATE_VAL);
    println!("Public local module value: {}", file_local_module::PUBLIC_VAL);

    // Below will not compile
    // println!("File local module value: {}", file_mod::PRIVATE_VAL);
    println!("File local module value: {}", file_mod::PUBLIC_VAL);

    // TIP: To include everything in a module
    // use file_mod::*; 
    
    use file_mod::PUBLIC_VAL; // To include a specific value from a module
    // Now we can directly use PUBLIC_VALUE
    println!("File local module value: {}", PUBLIC_VAL);

    // To rename imports
    use file_mod::PUBLIC_VAL as PVALUE;
    println!("File local module value: {}", PVALUE);

    // Using folder module
    // Error because two variables have the same name
    // use folder_mod::PUBLIC_VAL; 
    println!("Folder local module value: {}", folder_mod::PUBLIC_VAL);
}

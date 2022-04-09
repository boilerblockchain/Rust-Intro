struct DS {
    pub val: usize,
}

impl std::fmt::Debug for DS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "My bad non-standard implementation: {}", self.val)
    }
}

#[derive(Debug)]
struct DebugDS{
    pub val: usize,
}

#[cfg(test)]
mod test {
    use super::DS;

    #[test]
    fn test_print() {
        let ds1 = DS{val:0};
        println!("Print: {:?}", ds1);
    }
}

fn main() {
    println!("Hello, world!");

    let ds1 = DS{val:0};
    // Does not work since display is not implemented
    // println!("Print: {}", ds1);
    println!("Print: {:?}", ds1);

    let ds2 = DebugDS{val: 0};
    println!("Print custom: {:?}", ds2);

    // Compile only if this is macos
    #[cfg(target_os = "macos")]
    println!("I am running macos");

    #[cfg(not(target_os = "macos"))]
    println!("I am not running expensive macos");
}

pub trait LibraryFunction {
    fn call(&self, func_name: &str, args: Vec<&str>);
}


pub struct LibLibrary;

impl LibraryFunction for LibLibrary {
    fn call(&self, func_name: &str, args: Vec<&str>) {
        match func_name {
            "print" => self.print(args),
            "printf" => self.printf(args),
            _ => println!("Function {} not found.", func_name),
        }
    }
}

impl LibLibrary {
    fn print(&self, args: Vec<&str>) {
        println!("Calling LIB::print with args: {:?}", args);
    }

    fn printf(&self, args: Vec<&str>) {
        println!("Calling LIB::printf with args: {:?}", args);
    }
}
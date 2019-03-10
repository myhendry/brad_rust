// MODULES (mod keyword) L36

mod dcode {
    // private function
    fn read_message() {
        println!("{}", "reading message");
    }

    // public function
    pub fn print_message() {
        // as read message is a private function, it is only visible to functions in this module
        read_message();
        println!("How's it going")
    }

    // module in module
    pub mod water {
        pub fn print_message() {
            println!("{}", "Inside Water")
        }
    }
}

pub fn run() {
    dcode::print_message();
    dcode::water::print_message();
}

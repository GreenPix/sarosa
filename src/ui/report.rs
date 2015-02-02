
pub trait ErrorReporter {

    fn log(&self, msg: String);
}

#[derive(Copy)]
pub struct StdOutErrorReporter;
#[derive(Copy)]
pub struct EmptyErrorReporter;


impl ErrorReporter for StdOutErrorReporter {

    fn log(&self, msg: String) {
        println!("{}", msg);
    }
}

impl ErrorReporter for EmptyErrorReporter {

    fn log(&self, msg: String) {
        // Does nothing
    }
}

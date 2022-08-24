use crossterm::terminal;

/// `Cleanup` is a struct, which gets initiatialized in the starting line of main function,
///
/// and before the main function is completed, this object gets destroyed,
/// and we know that it is destroyed,
/// as we have implemented `Drop` trait for this `Cleanup` struct
pub struct Cleanup {}

impl Cleanup {
    pub fn new() -> Self {
        Self {}
    }
}

impl Cleanup {
    fn perform_cleanup_operations(self: &mut Self) {
        terminal::disable_raw_mode().expect("Unable to disable the raw mode..");
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        self.perform_cleanup_operations();
    }
}

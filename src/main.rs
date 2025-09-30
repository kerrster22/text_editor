use crossterm::terminal;
use std::io;
use std::io::Read;


/* This is simular to a typescript interface. */
struct CleanUp;

/* Here we are implementing our own function for our struct(interface) CleanUp  */
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("could not turn on Raw mode");
    let mut buf = [0; 1]; /*Read one byte at a time*/

    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf[0] != b'q' {}
    panic!();
}

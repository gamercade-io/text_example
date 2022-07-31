// External Gamercade Api Functions
extern "C" {
    fn console_log(text_ptr: i32, len: i32);
    fn random_int_range(min: i32, max: i32) -> i32;
}

// Set up some useful variables
const HELLO_WORLD: &str = "Hello, World!";
static mut FRAME_COUNT: usize = 0;

/// # Safety
/// This function calls external Gamercade Api Functions
unsafe fn log(text: &str) {
    // Casting a pointer to an i32 is giving us the memory address.
    let addr = text.as_ptr() as i32;

    console_log(addr, text.len() as i32)
}

/// # Safety
/// This function calls our unsafe logging function.
#[no_mangle]
pub unsafe extern "C" fn init() {
    log(HELLO_WORLD);
}

/// # Safety
/// This function calls external Gamercade Api Functions.
#[no_mangle]
pub unsafe extern "C" fn update() {
    FRAME_COUNT += 1;

    // Every few seconds, print a new random number
    if FRAME_COUNT > 150 {
        // Get a random number and build some text.
        let num = random_int_range(i32::MIN, i32::MAX);
        let text = format!("Got a random number: {}", num);

        // Output the text.
        log(&text);

        // Reset the counter
        FRAME_COUNT -= 150;
    }
}

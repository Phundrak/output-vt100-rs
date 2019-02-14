#[cfg(windows)]
pub fn init() {
    use winapi::shared::minwindef::DWORD;
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::{DISABLE_NEWLINE_AUTO_RETURN, ENABLE_VIRTUAL_TERMINAL_PROCESSING};

    let console_out = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    let mut state: DWORD = 0;
    assert_ne!(unsafe { GetConsoleMode(console_out, &mut state) }, 0);
    state |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    assert_ne!(unsafe { SetConsoleMode(console_out, state) }, 0);
}
#[cfg(not(windows))]
pub fn init() {
    ;
}

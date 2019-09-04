#[test]
fn init_test() {
  // Note(Lokathor): The purpose of this test is that it forces the OS to link
  // us against the SDL2 shared library, thus verifying that SDL2 is installed
  // to a location it will properly see. It's not a test of the code, it's more
  // of a test of the build environment outside of the code.
  unsafe {
    fermium::SDL_Init(0);
    fermium::SDL_Quit();
  }
}

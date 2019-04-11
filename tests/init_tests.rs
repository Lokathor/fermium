
#[test]
fn init_test() {
  unsafe {
    fermium::SDL_Init(0);
    fermium::SDL_Quit();
  }
}

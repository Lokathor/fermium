use fermium::*;

#[test]
fn init_nothing_and_then_quit() {
  unsafe {
    SDL_Init(SDL_InitFlags(0));
    SDL_Quit();
  }
}

use fermium::*;

fn main() {
  unsafe {
    SDL_Init(SDL_InitFlags(0));
    SDL_Quit();
  }
}

use fermium::{video::*, *};
use gl33::global_commands::load_global_gl_with;

fn main() {
  unsafe {
    assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);

    let win = SDL_CreateWindow(
      b"demo\0".as_ptr().cast(),
      SDL_WINDOWPOS_CENTERED,
      SDL_WINDOWPOS_CENTERED,
      800,
      600,
      (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
    );
    assert!(!win.is_null());

    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
    SDL_GL_SetAttribute(
      SDL_GL_CONTEXT_PROFILE_MASK,
      SDL_GL_CONTEXT_PROFILE_CORE.0 as _,
    );
    SDL_GL_SetAttribute(
      SDL_GL_CONTEXT_FLAGS,
      SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG.0 as _,
    );

    let ctx = SDL_GL_CreateContext(win);
    assert!(!ctx.0.is_null());

    let time_before = std::time::Instant::now();
    load_global_gl_with(|c| {
      let p = SDL_GL_GetProcAddress(c);
      if p.is_null() {
        print!("Failed to find fn `");
        let mut letter = c;
        while *letter != 0 {
          print!("{}", *letter as char);
          letter = letter.add(1);
        }
        println!("`");
      }
      p
    });
    let time_after = std::time::Instant::now();
    let dur = time_after.duration_since(time_before);
    println!("loading took {:?}", dur);

    SDL_Quit();
  }
}

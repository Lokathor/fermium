// all of this code in the run() function works in a no_std environment
// feel free to split this example into main.rs and lib.rs files

//#![no_std]

use fermium::prelude::*;

struct ProgramState {
    done: bool,
    hue: f32,
    hue_shift_speed: f32,
}

// start main.rs
// use fermium_no_std_rendering::run;

fn main() {
  run();
}
// end main.rs

#[no_mangle]
pub extern "C" fn run() {
    let mut ps = ProgramState {
        done: false,
        hue: 0.0,
        hue_shift_speed: 0.001,
    };

    unsafe { assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0) };
    let window = unsafe {
        SDL_CreateWindow(
            b"Fermium no_std Hue Shift Rendering Demo\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            800,
            600,
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        )
    };
    assert!(!window.is_null());

    let default_driver = -1;
    let renderer =
        unsafe { SDL_CreateRenderer(window, default_driver, SDL_RENDERER_ACCELERATED.0) };
    assert!(!renderer.is_null());

    while !ps.done {
        update(&mut ps);
        render(renderer, &ps);
    }
    unsafe { SDL_Quit() };
}

fn update(ps: &mut ProgramState) {
    // handle quitting
    let mut event = SDL_Event::default();
    let pending_events = 0 < unsafe { SDL_PollEvent(&mut event) };
    if pending_events {
        let event_type = unsafe { event.type_ };
        if event_type == SDL_QUIT || event_type == SDL_KEYDOWN {
            ps.done = true
        }
    }

    // handle hue animation
    ps.hue += ps.hue_shift_speed;
    ps.hue %= 1.0;
}

fn render(renderer: *mut SDL_Renderer, ps: &ProgramState) {
    // calculate values for hue to RGB conversion
    let chroma = 1.0;
    let h_prime = ps.hue * 6.0;
    // no no_std abs(), so this variable is defined
    // (h_prime % 2.0 - 1.0).abs()
    let hue_shifted = if 1.0 < h_prime % 2.0 {
        h_prime % 2.0 - 1.0
    } else {
        1.0 - h_prime % 2.0
    };
    let x = chroma * (1.0 - hue_shifted);

    // get RGB components
    let (r, g, b) = if h_prime < 1.0 {
        (chroma, x, 0.0)
    } else if h_prime < 2.0 {
        (x, chroma, 0.0)
    } else if h_prime < 3.0 {
        (0.0, chroma, x)
    } else if h_prime < 4.0 {
        (0.0, x, chroma)
    } else if h_prime < 5.0 {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    // set window color
    unsafe {
        SDL_SetRenderDrawColor(
            renderer,
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            255,
        );
        SDL_RenderClear(renderer);
        SDL_RenderPresent(renderer);
    }
}


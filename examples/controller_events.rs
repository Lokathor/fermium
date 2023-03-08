#![allow(clippy::uninlined_format_args)]

use fermium::prelude::*;

fn print_error() {
  unsafe {
    let mut v = Vec::with_capacity(4096);
    let p = SDL_GetErrorMsg(v.as_mut_ptr(), v.capacity() as _).cast();
    print_ptr(p);
  }
}

unsafe fn print_ptr(mut p: *const u8) {
  while *p != 0 {
    print!("{}", *p as char);
    p = p.add(1);
  }
}

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

    let num_joysticks = SDL_NumJoysticks();
    println!("SDL2 reports num_joysticks: {}", num_joysticks);

    let mut event = SDL_Event::default();
    loop {
      assert_eq!(SDL_WaitEvent(&mut event), 1);
      match event.type_ {
        SDL_QUIT => {
          println!("SDL_QUIT");
          break;
        }
        SDL_KEYDOWN => {
          println!("SDL_KEYDOWN: {:?}", event.key);
        }
        SDL_KEYUP => {
          println!("SDL_KEYUP {:?}", event.key);
        }
        SDL_CONTROLLERAXISMOTION => {
          println!("SDL_CONTROLLERAXISMOTION: {:?}", event.caxis);
        }
        SDL_CONTROLLERBUTTONDOWN => {
          println!("SDL_CONTROLLERBUTTONDOWN: {:?}", event.cbutton);
        }
        SDL_CONTROLLERBUTTONUP => {
          println!("SDL_CONTROLLERBUTTONUP: {:?}", event.cbutton);
        }
        SDL_CONTROLLERDEVICEADDED => {
          println!("SDL_CONTROLLERDEVICEADDED: {:?}", event.cdevice);
          let id = event.cdevice.which;
          println!("Opening joystick {} as a controller...", id);
          let controller = SDL_GameControllerOpen(id);
          if controller.is_null() {
            print!("Error while opening: ");
            print_error();
            println!();
          } else {
            print!("> Name: ");
            let name_p = SDL_GameControllerName(controller).cast();
            print_ptr(name_p);
            println!();
            //
            println!("> Type: {:?}", SDL_GameControllerGetType(controller));
            println!(
              "> PlayerIndex: {:?}",
              SDL_GameControllerGetPlayerIndex(controller)
            );
            println!(
              "> Vendor: {:#X?}",
              SDL_GameControllerGetVendor(controller)
            );
            println!(
              "> Product: {:#X?}",
              SDL_GameControllerGetProduct(controller)
            );
            println!(
              "> ProductVersion: {:?}",
              SDL_GameControllerGetProductVersion(controller)
            );
            //
            print!("> Serial: ");
            let serial_p: *const u8 =
              SDL_GameControllerGetSerial(controller).cast();
            if serial_p.is_null() {
              println!("not available");
            } else {
              print_ptr(serial_p);
              println!();
            }
          }
        }
        SDL_CONTROLLERDEVICEREMOVED => {
          println!("SDL_CONTROLLERDEVICEREMOVED: {:?}", event.cdevice);
          let id = event.cdevice.which;
          println!("Closing ID {}...", id);
          let controller = SDL_GameControllerFromInstanceID(SDL_JoystickID(id));
          if controller.is_null() {
            println!("but it was already closed!");
          } else {
            SDL_GameControllerClose(controller);
          }
        }
        SDL_CONTROLLERDEVICEREMAPPED => {
          println!("SDL_CONTROLLERDEVICEREMAPPED: {:?}", event.cdevice);
        }
        SDL_CONTROLLERTOUCHPADDOWN => {
          println!("SDL_CONTROLLERTOUCHPADDOWN: {:?}", event.ctouchpad);
        }
        SDL_CONTROLLERTOUCHPADMOTION => {
          println!("SDL_CONTROLLERTOUCHPADMOTION: {:?}", event.ctouchpad);
        }
        SDL_CONTROLLERTOUCHPADUP => {
          println!("SDL_CONTROLLERTOUCHPADUP: {:?}", event.ctouchpad);
        }
        SDL_CONTROLLERSENSORUPDATE => {
          println!("SDL_CONTROLLERSENSORUPDATE: {:?}", event.ctouchpad);
        }
        _ => (),
      }
    }

    SDL_Quit();
  }
}

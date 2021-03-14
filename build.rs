use std::{env, path::PathBuf};

fn cmake_build(cfg: &CargoCfg) -> PathBuf {
  let mut cm = cmake::Config::new(cfg.manifest_dir.join("SDL2-2.0.14"));
  cm.static_crt(true);
  cm.target(&cfg.target);
  // FERMIUM_NATIVE_OUT_DIR should be something like
  // `/path/to/your_project/.native_build`
  let shared_output = if let Some(o) = user_env("FERMIUM_NATIVE_OUT_DIR") {
    let path = PathBuf::from(o);
    assert!(
      path.is_absolute(),
      "FERMIUM_NATIVE_OUT_DIR must be an absolute path, got: {}",
      path.display()
    );
    let out = path.join(&cfg.target).join("fermium").join("SDL2-2.0.14");
    cm.out_dir(&out);
    true
  } else {
    false
  };

  let off_unless_shared = if shared_output { "ON" } else { "OFF" };
  if cfg.feat_dynamic_link {
    cm.define("SDL_SHARED", "ON");
    cm.define("SDL_STATIC", off_unless_shared);
  } else {
    cm.define("SDL_SHARED", off_unless_shared);
    cm.define("SDL_STATIC", "ON");
    // windows-gnu targets don't use PIC
    if !cfg.target.contains("windows-gnu") {
      cm.define("SDL_STATIC_PIC", "ON");
    }
  }

  // Allow overriding the CMake generator (e.g. `Ninja` is generally the fastest
  // if set up, but it generally isn't set up, so CMake won't use it — define
  // `FERMIUM_CMAKE_GENERATOR="Ninja"` if you know you it should work for you)
  match (user_env_flag("FERMIUM_USE_NINJA"), have_ninja()) {
    // If nothing is specified, treat it as "auto", e.g. use if present (it's
    // *substantially* faster, bringing the time for building the C code to
    // ~17s — this doesn't count `cmake` configure time, sadly)
    (None, true) | (Some(true), true) => {
      cm.generator("Ninja");
    }
    (Some(true), false) => {
      // We could panic... but lets just let cmake trigger the error, in case it
      // manages to find it or something. Just warn, although they probably
      // won't see it.
      println!(
        "cargo:warning=`FERMIUM_USE_NINJA=1` is specified in the \
         environment, but ninja doesn't seem to be installed",
      );
      cm.generator("Ninja");
    }
    // Not installed, not requested, explicitly disabled...
    _ => {}
  }

  // cm.always_configure(false);
  cm.profile("Release");

  let build_output_path = cm.build();
  println!("build_output_path: {}", build_output_path.display());
  build_output_path
}

fn have_ninja() -> bool {
  have_exe("ninja") || have_exe("ninja-build")
}

fn windows_link(dynamic: bool) {
  if dynamic {
    println!("cargo:rustc-link-lib=SDL2");
  } else {
    println!("cargo:rustc-link-lib=static=SDL2-static");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=imm32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=setupapi");
    println!("cargo:rustc-link-lib=shell32");
  }
}

fn unix_link(cfg: &CargoCfg, static_libs: bool, sdl_config_stdout: &str) {
  for term in sdl_config_stdout.split_whitespace() {
    if term.starts_with("-L") {
      println!("cargo:rustc-link-search=native={}", &term[2..]);
    } else if term.starts_with("-lSDL2") {
      if static_libs {
        println!("cargo:rustc-link-lib=SDL2");
      } else {
        println!("cargo:rustc-link-lib=static=SDL2");
      }
    } else if term.starts_with("-l") {
      // normal link
      if term.ends_with(".framework") {
        // macOS framework link, weirdly through -l. It's a bug that this
        // happens: https://github.com/libsdl-org/SDL/issues/4181
        let name_framework = term.rsplit("/").next().unwrap();
        let name = name_framework.split(".").next().unwrap();
        println!("cargo:rustc-link-lib=framework={}", name);
      } else {
        println!("cargo:rustc-link-lib={}", &term[2..]);
      }
    } else if term.starts_with("-Wl,-framework,") {
      // macOS framework link
      println!("cargo:rustc-link-lib=framework={}", &term[15..]);
    } else if term.starts_with("-Wl,-weak_framework,") {
      // rust doesn't seem to have "weak" framework linking so we just
      // declare a normal framework link, but let the user opt into an
      // unstable feature where we try to force it. FIXME: does this work?

      // TODO(thom): Ideally we should handle GameController/CoreHaptics
      // differently from Metal/QuartzCore here, since the latter is so much
      // more widely supported. TODO: exclusions should be an env param.
      let lib = &term[20..];
      if cfg.feat_nightly_macos_link_args
        && !matches!(lib, "Metal" | "QuartzCore")
      {
        match lib {
          "GameController" => {
            println!("cargo:rustc-cfg=weak_framework_game_controller");
          }
          "CoreHaptics" => {
            println!("cargo:rustc-cfg=weak_framework_core_haptics");
          }
          // "Metal" => {
          //   println!("cargo:rustc-cfg=weak_framework_metal");
          // }
          // "QuartzCore" => {
          //   println!("cargo:rustc-cfg=weak_framework_quartz_core");
          // }
          lib => {
            panic!("Unknown weak_framework: {}", lib);
          }
        }
      } else {
        // Didn't opt in to unstable, or it's commen enough not to be worth
        // it, do a strong link.
        println!("cargo:rustc-link-lib=framework={}", lib);
      }
    } else if false
    /* && term.starts_with("-Wl,-rpath,") */
    {
      // I don't know why this works, but it does seem to?

      // IMPORTANT(thom): depending on what this path is, it will make
      // binaries non-portable! It may even prevent them from working
      // outside of `cargo run`! Remember to come back to this.
      // (in case I forget to search for IMPORTANT: TODO, FIXME)
      println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", &term[11..]);
    } else if term.starts_with("-Wl,--enable-new-dtags") {
      // Nothing
    } else if term.starts_with("-Wl,--no-undefined") {
      // Nothing
    } else if term.starts_with("-pthread") {
      // Nothing special on the Rust side, I'm told.
    } else if term.starts_with("-Wl,-current_version,") {
      // It's a bug we get these: https://github.com/libsdl-org/SDL/issues/4181
    } else if term.starts_with("-Wl,-compatibility_version,") {
      // It's a bug we get these: (see above)
    } else if term.starts_with("-Wl,-undefined,error") {
      // It's a bug we get these: (see above)
    } else {
      panic!("Unknown term: >>{}<<", term);
    }
  }
}

fn link_sdl2_using_config(cfg: &CargoCfg, cmd: &std::path::Path) {
  let link_style_arg: &str =
    if cfg.feat_dynamic_link { "--libs" } else { "--static-libs" };
  let sd2_config_linking = std::process::Command::new(cmd)
    .arg(link_style_arg)
    .output()
    .unwrap_or_else(|_| {
      panic!("Couldn't run `{} {}`.", cmd.display(), link_style_arg)
    });
  let stdout = String::from_utf8_lossy(&sd2_config_linking.stdout);
  let stderr = String::from_utf8_lossy(&sd2_config_linking.stderr);
  println!("`{} {}` stdout: {:?}", cmd.display(), link_style_arg, stdout);
  println!("`{} {}` stderr: {:?}", cmd.display(), link_style_arg, stderr);
  println!(
    "`{} {}` status: {:?}",
    cmd.display(),
    link_style_arg,
    sd2_config_linking.status,
  );
  assert!(sd2_config_linking.status.success());
  assert!(!cfg.is_windows, "sdl2-config on windows? {:?}", &stdout);
  assert!(
    cfg.is_unix,
    "Everything that isn't windows is unix. (target = {:?})",
    cfg.target
  );
  unix_link(cfg, cfg.feat_dynamic_link, stdout.trim());
}

fn sdl2_config_version_check(cmd: &str) -> bool {
  if let Some(true) = user_env_flag("FERMIUM_ALLOW_OLDER") {
    return true;
  }
  let ver = std::process::Command::new(cmd)
    .arg("--version")
    .output()
    .expect("sdl2-config failed");
  let stdout = String::from_utf8_lossy(&ver.stdout);
  let stderr = String::from_utf8_lossy(&ver.stderr);
  println!("`sdl2-config --version` stdout: {:?}", stdout);
  println!("`sdl2-config --version` stderr: {:?}", stderr);
  println!("`sdl2-config --version` status: {:?}", ver.status);
  if !ver.status.success() {
    println!(
      "cargo:warning=`sdl2-config --version` failed with: {:?}",
      ver.status.code()
    );
    return false;
  }
  return if let Some(semver) = hacky_parse_semver(&stdout) {
    let result = semver >= (2, 0, 14);
    println!("determined version: {:?}. good enough? {:?}", semver, result);
    result
  } else {
    println!(
      "cargo:warning=Failed to parse `sdl2-config --version` output: {:?}",
      stdout
    );
    false
  };
  fn hacky_parse_semver(version: &str) -> Option<(u64, u64, u64)> {
    let version = version.trim();
    let vals = version
      .find(|c| c == '-' || c == '+')
      .map(|v| &version[..v])
      .unwrap_or(version);
    let res = vals
      .split('.')
      .map(|v| v.trim().parse::<u64>())
      .collect::<Result<Vec<_>, _>>();
    if let Ok(&[maj, min, patch, ..]) = res.as_deref() {
      Some((maj, min, patch))
    } else {
      None
    }
  }
}

fn main() {
  // Note(Lokathor): I was told by thomcc@github to put this in because it makes
  // the build faster if this crate is used within a workspace.
  //
  // The build script only uses the bundled C source to build the lib, so unless
  // we actually replace the C source, we don't need to re-run the build script.
  // This won't affect our ability to develop the Rust level bindings.
  println!("cargo:rerun-if-changed=build.rs");

  let config = CargoCfg::get();

  // Copy the DLL for loka's binary.
  if config.is_windows {
    println!("cargo:rerun-if-changed=SDL2.dll");
    let dll_from = config.manifest_dir.join("SDL2.dll");
    println!("dll_from:{}", dll_from.display());
    let dll_to = config.out_dir.join("SDL2.dll");
    println!("dll_to:{}", dll_to.display());
    std::fs::copy(dll_from, dll_to)
      .expect("Failed to copy DLL for fermium binary");
  }

  // run the experimental-fast-build feature
  // TODO(thom): set up for x86_64-apple-darwin?
  if config.feat_experimental_fast_build
    && config.target == "x86_64-pc-windows-msvc"
  {
    println!("cargo:rerun-if-changed=SDL2-2.0.14-devel");
    // declare search path. This doesn't need to be absolute, but *shrug*.
    println!(
      "cargo:rustc-link-search={}",
      config
        .manifest_dir
        .join("SDL2-2.0.14-devel")
        .join("x86_64-pc-windows-msvc")
        .display()
    );
    // declare linking (the bundled files only work with dynamic linking)
    assert!(config.feat_dynamic_link);
    println!("cargo:rustc-link-lib=SDL2");
    return;
  }
  if config.feat_cargo_check {
    return;
  }
  if config.feat_use_sdl2_config && !config.feat_use_sdl2_config {
    if let Some(p) = user_env("FERMIUM_SDL2_CONFIG_PATH") {
      // If the path is explicitly specified via the environment, use it without
      // bothering to check the version.
      return link_sdl2_using_config(&config, p.as_ref());
    } else if have_exe("sdl2-config")
      && sdl2_config_version_check("sdl2-config")
    {
      return link_sdl2_using_config(&config, "sdl2-config".as_ref());
    } else if user_env_flag("FERMIUM_NO_CMAKE_FALLBACK").unwrap_or(false) {
      panic!(
        "Failed to locate an up to date `sdl2-config`. Ensure it's at least version 2.0.14, \
         or alternatively set `FERMIUM_ALLOW_OLDER=1` variable in the environment."
      );
    } else {
      // fallback to building with `cmake`.
    }
  }

  let build_output_path = cmake_build(&config);
  println!(
    "cargo:rustc-link-search={}",
    build_output_path.join("lib").display()
  );

  if config.is_windows {
    windows_link(config.feat_dynamic_link);
  } else if config.is_unix {
    let sdl2_cfg_cmd = build_output_path.join("bin").join("sdl2-config");
    link_sdl2_using_config(&config, &sdl2_cfg_cmd);
  } else {
    panic!("Sorry, I only know how to build/link SDL2 on windows and unix.");
  }
}

/// Config from cargo, all in one place. For the most part, it's better to keep
/// user parameters out of this other than features.
struct CargoCfg {
  feat_cargo_check: bool,
  feat_experimental_fast_build: bool,
  feat_dynamic_link: bool,
  feat_nightly_macos_link_args: bool,
  feat_use_sdl2_config: bool,
  out_dir: PathBuf,
  manifest_dir: PathBuf,
  target: String,
  is_windows: bool,
  is_unix: bool,
}

impl CargoCfg {
  fn get() -> Self {
    Self {
      feat_cargo_check: cargo_env_flag("CARGO_FEATURE_CARGO_CHECK"),
      feat_dynamic_link: cargo_env_flag("CARGO_FEATURE_DYNAMIC_LINK"),
      feat_experimental_fast_build: cargo_env_flag(
        "CARGO_FEATURE_EXPERIMENTAL_FAST_BUILD",
      ),
      feat_nightly_macos_link_args: cargo_env_flag(
        "CARGO_FEATURE_NIGHTLY_MACOS_LINK_ARGS",
      ),
      feat_use_sdl2_config: cargo_env_flag("CARGO_FEATURE_USE_SDL2_CONFIG"),
      is_windows: cargo_env_flag("CARGO_CFG_WINDOWS"),
      is_unix: cargo_env_flag("CARGO_CFG_UNIX"),
      out_dir: PathBuf::from(cargo_env("OUT_DIR")),
      manifest_dir: PathBuf::from(
        cargo_opt_env("CARGO_MANIFEST_DIR").unwrap_or_else(|| ".".into()),
      ),
      target: cargo_env("TARGET"),
    }
  }
}

fn have_exe(cmd: &str) -> bool {
  // Note: In theory we should rerun on change for `PATH`, but in practice it
  // would be way to high churn.
  env::split_paths(&env::var_os("PATH").unwrap_or(Default::default()))
    .find(|p| p.join(cmd).exists())
    .is_some()
}

fn user_env(s: &str) -> Option<String> {
  println!("cargo:rerun-if-env-changed={}", s);
  env::var(s).ok()
}

fn user_env_flag(s: &str) -> Option<bool> {
  match user_env(s).map(|s| s.to_lowercase()).as_deref() {
    None | Some("") => None,
    // cmake-alike flags
    Some("0") | Some("off") | Some("no") | Some("false") => Some(false),
    Some(_) => Some(true),
  }
}

/// Returns the value of `s` in the env, or none.
fn cargo_opt_env(s: &str) -> Option<String> {
  let r = env::var(s).ok();
  println!("env[{:?}] = {:?}", s, r);
  r
}

/// Returns the value of `s` in the env, or panics.
#[track_caller]
fn cargo_env(s: &str) -> String {
  cargo_opt_env(s).unwrap_or_else(|| panic!("Environment is missing {:?}", s))
}

/// Returns `true` if `s` is set in the env.
fn cargo_env_flag(s: &str) -> bool {
  let r = env::var_os(s).is_some();
  println!("env[{:?}] = {:?}", s, r);
  r
}

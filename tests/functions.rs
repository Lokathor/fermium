//! This module tests that specific functions exist with specific signatures.
//! It's more of a guard against possible breaks when we do future releases than
//! it is a check of specific functionality.

use fermium::*;

/// Asserts the existence of a function with a given name and type.
///
/// * **Usage:** `assert_fn!(name: type);`
///
/// The assertion is a static check, so if there's a problem it will
/// cause a compilation failure rather than a runtime error.
macro_rules! assert_fn {
  ($f:ident : $t:ty) => {
    #[allow(bad_style)]
    #[allow(unused)]
    const $f: $t = fermium::$f;
  };
}

assert_fn!(SDL_GetPlatform: unsafe extern "C" fn() -> *const c_char);
assert_fn!(SDL_malloc: unsafe extern "C" fn(size: usize) -> *mut c_void);
assert_fn!(
  SDL_calloc: unsafe extern "C" fn(nmemb: usize, size: usize) -> *mut c_void
);
assert_fn!(
  SDL_realloc:
    unsafe extern "C" fn(mem: *mut c_void, size: usize) -> *mut c_void
);
assert_fn!(SDL_free: unsafe extern "C" fn(mem: *mut c_void));
assert_fn!(
  SDL_GetMemoryFunctions:
    unsafe extern "C" fn(
      malloc_func: *mut SDL_malloc_func,
      calloc_func: *mut SDL_calloc_func,
      realloc_func: *mut SDL_realloc_func,
      free_func: *mut SDL_free_func,
    )
);
assert_fn!(
  SDL_SetMemoryFunctions:
    unsafe extern "C" fn(
      malloc_func: SDL_malloc_func,
      calloc_func: SDL_calloc_func,
      realloc_func: SDL_realloc_func,
      free_func: SDL_free_func,
    ) -> c_int
);
assert_fn!(SDL_GetNumAllocations: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_getenv: unsafe extern "C" fn(name: *const c_char) -> *mut c_char
);
assert_fn!(
  SDL_setenv:
    unsafe extern "C" fn(
      name: *const c_char,
      value: *const c_char,
      overwrite: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_qsort:
    unsafe extern "C" fn(
      base: *mut c_void,
      nmemb: usize,
      size: usize,
      compare: Option<
        unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int,
      >,
    )
);
assert_fn!(SDL_abs: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(SDL_isdigit: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(SDL_isspace: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(SDL_isupper: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(SDL_islower: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(SDL_toupper: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(SDL_tolower: unsafe extern "C" fn(x: c_int) -> c_int);
assert_fn!(
  SDL_memset:
    unsafe extern "C" fn(dst: *mut c_void, c: c_int, len: usize) -> *mut c_void
);
assert_fn!(
  SDL_memcpy:
    unsafe extern "C" fn(
      dst: *mut c_void,
      src: *const c_void,
      len: usize,
    ) -> *mut c_void
);
assert_fn!(
  SDL_memmove:
    unsafe extern "C" fn(
      dst: *mut c_void,
      src: *const c_void,
      len: usize,
    ) -> *mut c_void
);
assert_fn!(
  SDL_memcmp:
    unsafe extern "C" fn(
      s1: *const c_void,
      s2: *const c_void,
      len: usize,
    ) -> c_int
);
assert_fn!(SDL_wcslen: unsafe extern "C" fn(wstr: *const wchar_t) -> usize);
assert_fn!(
  SDL_wcslcpy:
    unsafe extern "C" fn(
      dst: *mut wchar_t,
      src: *const wchar_t,
      maxlen: usize,
    ) -> usize
);
assert_fn!(
  SDL_wcslcat:
    unsafe extern "C" fn(
      dst: *mut wchar_t,
      src: *const wchar_t,
      maxlen: usize,
    ) -> usize
);
assert_fn!(
  SDL_wcsdup: unsafe extern "C" fn(wstr: *const wchar_t) -> *mut wchar_t
);
assert_fn!(
  SDL_wcsstr:
    unsafe extern "C" fn(
      haystack: *const wchar_t,
      needle: *const wchar_t,
    ) -> *mut wchar_t
);
assert_fn!(
  SDL_wcscmp:
    unsafe extern "C" fn(str1: *const wchar_t, str2: *const wchar_t) -> c_int
);
assert_fn!(
  SDL_wcsncmp:
    unsafe extern "C" fn(
      str1: *const wchar_t,
      str2: *const wchar_t,
      maxlen: usize,
    ) -> c_int
);
assert_fn!(SDL_strlen: unsafe extern "C" fn(str: *const c_char) -> usize);
assert_fn!(
  SDL_strlcpy:
    unsafe extern "C" fn(
      dst: *mut c_char,
      src: *const c_char,
      maxlen: usize,
    ) -> usize
);
assert_fn!(
  SDL_utf8strlcpy:
    unsafe extern "C" fn(
      dst: *mut c_char,
      src: *const c_char,
      dst_bytes: usize,
    ) -> usize
);
assert_fn!(
  SDL_strlcat:
    unsafe extern "C" fn(
      dst: *mut c_char,
      src: *const c_char,
      maxlen: usize,
    ) -> usize
);
assert_fn!(SDL_strdup: unsafe extern "C" fn(str: *const c_char) -> *mut c_char);
assert_fn!(SDL_strrev: unsafe extern "C" fn(str: *mut c_char) -> *mut c_char);
assert_fn!(SDL_strupr: unsafe extern "C" fn(str: *mut c_char) -> *mut c_char);
assert_fn!(SDL_strlwr: unsafe extern "C" fn(str: *mut c_char) -> *mut c_char);
assert_fn!(
  SDL_strchr: unsafe extern "C" fn(str: *const c_char, c: c_int) -> *mut c_char
);
assert_fn!(
  SDL_strrchr:
    unsafe extern "C" fn(str: *const c_char, c: c_int) -> *mut c_char
);
assert_fn!(
  SDL_strstr:
    unsafe extern "C" fn(
      haystack: *const c_char,
      needle: *const c_char,
    ) -> *mut c_char
);
assert_fn!(
  SDL_strtokr:
    unsafe extern "C" fn(
      s1: *mut c_char,
      s2: *const c_char,
      saveptr: *mut *mut c_char,
    ) -> *mut c_char
);
assert_fn!(SDL_utf8strlen: unsafe extern "C" fn(str: *const c_char) -> usize);
assert_fn!(
  SDL_itoa:
    unsafe extern "C" fn(
      value: c_int,
      str: *mut c_char,
      radix: c_int,
    ) -> *mut c_char
);
assert_fn!(
  SDL_uitoa:
    unsafe extern "C" fn(
      value: c_uint,
      str: *mut c_char,
      radix: c_int,
    ) -> *mut c_char
);
assert_fn!(
  SDL_ltoa:
    unsafe extern "C" fn(
      value: c_long,
      str: *mut c_char,
      radix: c_int,
    ) -> *mut c_char
);
assert_fn!(
  SDL_ultoa:
    unsafe extern "C" fn(
      value: c_ulong,
      str: *mut c_char,
      radix: c_int,
    ) -> *mut c_char
);
assert_fn!(
  SDL_lltoa:
    unsafe extern "C" fn(
      value: Sint64,
      str: *mut c_char,
      radix: c_int,
    ) -> *mut c_char
);
assert_fn!(
  SDL_ulltoa:
    unsafe extern "C" fn(
      value: Uint64,
      str: *mut c_char,
      radix: c_int,
    ) -> *mut c_char
);
assert_fn!(SDL_atoi: unsafe extern "C" fn(str: *const c_char) -> c_int);
assert_fn!(SDL_atof: unsafe extern "C" fn(str: *const c_char) -> f64);
assert_fn!(
  SDL_strtol:
    unsafe extern "C" fn(
      str: *const c_char,
      endp: *mut *mut c_char,
      base: c_int,
    ) -> c_long
);
assert_fn!(
  SDL_strtoul:
    unsafe extern "C" fn(
      str: *const c_char,
      endp: *mut *mut c_char,
      base: c_int,
    ) -> c_ulong
);
assert_fn!(
  SDL_strtoll:
    unsafe extern "C" fn(
      str: *const c_char,
      endp: *mut *mut c_char,
      base: c_int,
    ) -> Sint64
);
assert_fn!(
  SDL_strtoull:
    unsafe extern "C" fn(
      str: *const c_char,
      endp: *mut *mut c_char,
      base: c_int,
    ) -> Uint64
);
assert_fn!(
  SDL_strtod:
    unsafe extern "C" fn(str: *const c_char, endp: *mut *mut c_char) -> f64
);
assert_fn!(
  SDL_strcmp:
    unsafe extern "C" fn(str1: *const c_char, str2: *const c_char) -> c_int
);
assert_fn!(
  SDL_strncmp:
    unsafe extern "C" fn(
      str1: *const c_char,
      str2: *const c_char,
      maxlen: usize,
    ) -> c_int
);
assert_fn!(
  SDL_strcasecmp:
    unsafe extern "C" fn(str1: *const c_char, str2: *const c_char) -> c_int
);
assert_fn!(
  SDL_strncasecmp:
    unsafe extern "C" fn(
      str1: *const c_char,
      str2: *const c_char,
      len: usize,
    ) -> c_int
);
assert_fn!(
  SDL_sscanf:
    unsafe extern "C" fn(text: *const c_char, fmt: *const c_char, ...) -> c_int
);
assert_fn!(
  SDL_snprintf:
    unsafe extern "C" fn(
      text: *mut c_char,
      maxlen: usize,
      fmt: *const c_char,
      ...
    ) -> c_int
);
assert_fn!(SDL_acos: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_acosf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_asin: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_asinf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_atan: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_atanf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_atan2: unsafe extern "C" fn(x: f64, y: f64) -> f64);
assert_fn!(SDL_atan2f: unsafe extern "C" fn(x: f32, y: f32) -> f32);
assert_fn!(SDL_ceil: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_ceilf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_copysign: unsafe extern "C" fn(x: f64, y: f64) -> f64);
assert_fn!(SDL_copysignf: unsafe extern "C" fn(x: f32, y: f32) -> f32);
assert_fn!(SDL_cos: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_cosf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_exp: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_expf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_fabs: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_fabsf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_floor: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_floorf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_fmod: unsafe extern "C" fn(x: f64, y: f64) -> f64);
assert_fn!(SDL_fmodf: unsafe extern "C" fn(x: f32, y: f32) -> f32);
assert_fn!(SDL_log: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_logf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_log10: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_log10f: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_pow: unsafe extern "C" fn(x: f64, y: f64) -> f64);
assert_fn!(SDL_powf: unsafe extern "C" fn(x: f32, y: f32) -> f32);
assert_fn!(SDL_scalbn: unsafe extern "C" fn(x: f64, n: c_int) -> f64);
assert_fn!(SDL_scalbnf: unsafe extern "C" fn(x: f32, n: c_int) -> f32);
assert_fn!(SDL_sin: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_sinf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_sqrt: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_sqrtf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(SDL_tan: unsafe extern "C" fn(x: f64) -> f64);
assert_fn!(SDL_tanf: unsafe extern "C" fn(x: f32) -> f32);
assert_fn!(
  SDL_iconv_open:
    unsafe extern "C" fn(
      tocode: *const c_char,
      fromcode: *const c_char,
    ) -> SDL_iconv_t
);
assert_fn!(SDL_iconv_close: unsafe extern "C" fn(cd: SDL_iconv_t) -> c_int);
assert_fn!(
  SDL_iconv:
    unsafe extern "C" fn(
      cd: SDL_iconv_t,
      inbuf: *mut *const c_char,
      inbytesleft: *mut usize,
      outbuf: *mut *mut c_char,
      outbytesleft: *mut usize,
    ) -> usize
);
assert_fn!(
  SDL_iconv_string:
    unsafe extern "C" fn(
      tocode: *const c_char,
      fromcode: *const c_char,
      inbuf: *const c_char,
      inbytesleft: usize,
    ) -> *mut c_char
);
assert_fn!(
  SDL_main: unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int
);
assert_fn!(SDL_SetMainReady: unsafe extern "C" fn());
assert_fn!(
  SDL_ReportAssertion:
    unsafe extern "C" fn(
      arg1: *mut SDL_AssertData,
      arg2: *const c_char,
      arg3: *const c_char,
      arg4: c_int,
    ) -> SDL_AssertState
);
assert_fn!(
  SDL_SetAssertionHandler:
    unsafe extern "C" fn(handler: SDL_AssertionHandler, userdata: *mut c_void)
);
assert_fn!(
  SDL_GetDefaultAssertionHandler:
    unsafe extern "C" fn() -> SDL_AssertionHandler
);
assert_fn!(
  SDL_GetAssertionHandler:
    unsafe extern "C" fn(puserdata: *mut *mut c_void) -> SDL_AssertionHandler
);
assert_fn!(
  SDL_GetAssertionReport: unsafe extern "C" fn() -> *const SDL_AssertData
);
assert_fn!(SDL_ResetAssertionReport: unsafe extern "C" fn());
assert_fn!(
  SDL_AtomicTryLock: unsafe extern "C" fn(lock: *mut SDL_SpinLock) -> SDL_bool
);
assert_fn!(SDL_AtomicLock: unsafe extern "C" fn(lock: *mut SDL_SpinLock));
assert_fn!(SDL_AtomicUnlock: unsafe extern "C" fn(lock: *mut SDL_SpinLock));
assert_fn!(SDL_MemoryBarrierReleaseFunction: unsafe extern "C" fn());
assert_fn!(SDL_MemoryBarrierAcquireFunction: unsafe extern "C" fn());
assert_fn!(
  SDL_AtomicCAS:
    unsafe extern "C" fn(
      a: *mut SDL_atomic_t,
      oldval: c_int,
      newval: c_int,
    ) -> SDL_bool
);
assert_fn!(
  SDL_AtomicSet: unsafe extern "C" fn(a: *mut SDL_atomic_t, v: c_int) -> c_int
);
assert_fn!(SDL_AtomicGet: unsafe extern "C" fn(a: *mut SDL_atomic_t) -> c_int);
assert_fn!(
  SDL_AtomicAdd: unsafe extern "C" fn(a: *mut SDL_atomic_t, v: c_int) -> c_int
);
assert_fn!(
  SDL_AtomicCASPtr:
    unsafe extern "C" fn(
      a: *mut *mut c_void,
      oldval: *mut c_void,
      newval: *mut c_void,
    ) -> SDL_bool
);
assert_fn!(
  SDL_AtomicSetPtr:
    unsafe extern "C" fn(a: *mut *mut c_void, v: *mut c_void) -> *mut c_void
);
assert_fn!(
  SDL_AtomicGetPtr: unsafe extern "C" fn(a: *mut *mut c_void) -> *mut c_void
);
assert_fn!(
  SDL_SetError: unsafe extern "C" fn(fmt: *const c_char, ...) -> c_int
);
assert_fn!(SDL_GetError: unsafe extern "C" fn() -> *const c_char);
assert_fn!(SDL_ClearError: unsafe extern "C" fn());
assert_fn!(SDL_Error: unsafe extern "C" fn(code: SDL_errorcode) -> c_int);
assert_fn!(SDL_CreateMutex: unsafe extern "C" fn() -> *mut SDL_mutex);
assert_fn!(SDL_LockMutex: unsafe extern "C" fn(mutex: *mut SDL_mutex) -> c_int);
assert_fn!(
  SDL_TryLockMutex: unsafe extern "C" fn(mutex: *mut SDL_mutex) -> c_int
);
assert_fn!(
  SDL_UnlockMutex: unsafe extern "C" fn(mutex: *mut SDL_mutex) -> c_int
);
assert_fn!(SDL_DestroyMutex: unsafe extern "C" fn(mutex: *mut SDL_mutex));
assert_fn!(
  SDL_CreateSemaphore:
    unsafe extern "C" fn(initial_value: Uint32) -> *mut SDL_sem
);
assert_fn!(SDL_DestroySemaphore: unsafe extern "C" fn(sem: *mut SDL_sem));
assert_fn!(SDL_SemWait: unsafe extern "C" fn(sem: *mut SDL_sem) -> c_int);
assert_fn!(SDL_SemTryWait: unsafe extern "C" fn(sem: *mut SDL_sem) -> c_int);
assert_fn!(
  SDL_SemWaitTimeout:
    unsafe extern "C" fn(sem: *mut SDL_sem, ms: Uint32) -> c_int
);
assert_fn!(SDL_SemPost: unsafe extern "C" fn(sem: *mut SDL_sem) -> c_int);
assert_fn!(SDL_SemValue: unsafe extern "C" fn(sem: *mut SDL_sem) -> Uint32);
assert_fn!(SDL_CreateCond: unsafe extern "C" fn() -> *mut SDL_cond);
assert_fn!(SDL_DestroyCond: unsafe extern "C" fn(cond: *mut SDL_cond));
assert_fn!(SDL_CondSignal: unsafe extern "C" fn(cond: *mut SDL_cond) -> c_int);
assert_fn!(
  SDL_CondBroadcast: unsafe extern "C" fn(cond: *mut SDL_cond) -> c_int
);
assert_fn!(
  SDL_CondWait:
    unsafe extern "C" fn(cond: *mut SDL_cond, mutex: *mut SDL_mutex) -> c_int
);
assert_fn!(
  SDL_CondWaitTimeout:
    unsafe extern "C" fn(
      cond: *mut SDL_cond,
      mutex: *mut SDL_mutex,
      ms: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_GetThreadName:
    unsafe extern "C" fn(thread: *mut SDL_Thread) -> *const c_char
);
assert_fn!(SDL_ThreadID: unsafe extern "C" fn() -> SDL_threadID);
assert_fn!(
  SDL_GetThreadID:
    unsafe extern "C" fn(thread: *mut SDL_Thread) -> SDL_threadID
);
assert_fn!(
  SDL_SetThreadPriority:
    unsafe extern "C" fn(priority: SDL_ThreadPriority) -> c_int
);
assert_fn!(
  SDL_WaitThread:
    unsafe extern "C" fn(thread: *mut SDL_Thread, status: *mut c_int)
);
assert_fn!(SDL_DetachThread: unsafe extern "C" fn(thread: *mut SDL_Thread));
assert_fn!(SDL_TLSCreate: unsafe extern "C" fn() -> SDL_TLSID);
assert_fn!(SDL_TLSGet: unsafe extern "C" fn(id: SDL_TLSID) -> *mut c_void);
assert_fn!(
  SDL_RWFromFile:
    unsafe extern "C" fn(
      file: *const c_char,
      mode: *const c_char,
    ) -> *mut SDL_RWops
);
assert_fn!(
  SDL_RWFromMem:
    unsafe extern "C" fn(mem: *mut c_void, size: c_int) -> *mut SDL_RWops
);
assert_fn!(
  SDL_RWFromConstMem:
    unsafe extern "C" fn(mem: *const c_void, size: c_int) -> *mut SDL_RWops
);
assert_fn!(SDL_AllocRW: unsafe extern "C" fn() -> *mut SDL_RWops);
assert_fn!(SDL_FreeRW: unsafe extern "C" fn(area: *mut SDL_RWops));
assert_fn!(SDL_RWsize: unsafe extern "C" fn(context: *mut SDL_RWops) -> Sint64);
assert_fn!(
  SDL_RWseek:
    unsafe extern "C" fn(
      context: *mut SDL_RWops,
      offset: Sint64,
      whence: c_int,
    ) -> Sint64
);
assert_fn!(SDL_RWtell: unsafe extern "C" fn(context: *mut SDL_RWops) -> Sint64);
assert_fn!(
  SDL_RWread:
    unsafe extern "C" fn(
      context: *mut SDL_RWops,
      ptr: *mut c_void,
      size: usize,
      maxnum: usize,
    ) -> usize
);
assert_fn!(
  SDL_RWwrite:
    unsafe extern "C" fn(
      context: *mut SDL_RWops,
      ptr: *const c_void,
      size: usize,
      num: usize,
    ) -> usize
);
assert_fn!(SDL_RWclose: unsafe extern "C" fn(context: *mut SDL_RWops) -> c_int);
assert_fn!(
  SDL_LoadFile_RW:
    unsafe extern "C" fn(
      src: *mut SDL_RWops,
      datasize: *mut usize,
      freesrc: c_int,
    ) -> *mut c_void
);
assert_fn!(
  SDL_LoadFile:
    unsafe extern "C" fn(
      file: *const c_char,
      datasize: *mut usize,
    ) -> *mut c_void
);
assert_fn!(SDL_ReadU8: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint8);
assert_fn!(SDL_ReadLE16: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint16);
assert_fn!(SDL_ReadBE16: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint16);
assert_fn!(SDL_ReadLE32: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint32);
assert_fn!(SDL_ReadBE32: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint32);
assert_fn!(SDL_ReadLE64: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint64);
assert_fn!(SDL_ReadBE64: unsafe extern "C" fn(src: *mut SDL_RWops) -> Uint64);
assert_fn!(
  SDL_WriteU8: unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint8) -> usize
);
assert_fn!(
  SDL_WriteLE16:
    unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint16) -> usize
);
assert_fn!(
  SDL_WriteBE16:
    unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint16) -> usize
);
assert_fn!(
  SDL_WriteLE32:
    unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint32) -> usize
);
assert_fn!(
  SDL_WriteBE32:
    unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint32) -> usize
);
assert_fn!(
  SDL_WriteLE64:
    unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint64) -> usize
);
assert_fn!(
  SDL_WriteBE64:
    unsafe extern "C" fn(dst: *mut SDL_RWops, value: Uint64) -> usize
);
assert_fn!(SDL_GetNumAudioDrivers: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_GetAudioDriver: unsafe extern "C" fn(index: c_int) -> *const c_char
);
assert_fn!(
  SDL_AudioInit: unsafe extern "C" fn(driver_name: *const c_char) -> c_int
);
assert_fn!(SDL_AudioQuit: unsafe extern "C" fn());
assert_fn!(SDL_GetCurrentAudioDriver: unsafe extern "C" fn() -> *const c_char);
assert_fn!(
  SDL_OpenAudio:
    unsafe extern "C" fn(
      desired: *mut SDL_AudioSpec,
      obtained: *mut SDL_AudioSpec,
    ) -> c_int
);
assert_fn!(
  SDL_GetNumAudioDevices: unsafe extern "C" fn(iscapture: c_int) -> c_int
);
assert_fn!(
  SDL_GetAudioDeviceName:
    unsafe extern "C" fn(index: c_int, iscapture: c_int) -> *const c_char
);
assert_fn!(
  SDL_OpenAudioDevice:
    unsafe extern "C" fn(
      device: *const c_char,
      iscapture: c_int,
      desired: *const SDL_AudioSpec,
      obtained: *mut SDL_AudioSpec,
      allowed_changes: c_int,
    ) -> SDL_AudioDeviceID
);
assert_fn!(SDL_GetAudioStatus: unsafe extern "C" fn() -> SDL_AudioStatus);
assert_fn!(
  SDL_GetAudioDeviceStatus:
    unsafe extern "C" fn(dev: SDL_AudioDeviceID) -> SDL_AudioStatus
);
assert_fn!(SDL_PauseAudio: unsafe extern "C" fn(pause_on: c_int));
assert_fn!(
  SDL_PauseAudioDevice:
    unsafe extern "C" fn(dev: SDL_AudioDeviceID, pause_on: c_int)
);
assert_fn!(
  SDL_LoadWAV_RW:
    unsafe extern "C" fn(
      src: *mut SDL_RWops,
      freesrc: c_int,
      spec: *mut SDL_AudioSpec,
      audio_buf: *mut *mut Uint8,
      audio_len: *mut Uint32,
    ) -> *mut SDL_AudioSpec
);
assert_fn!(SDL_FreeWAV: unsafe extern "C" fn(audio_buf: *mut Uint8));
assert_fn!(
  SDL_BuildAudioCVT:
    unsafe extern "C" fn(
      cvt: *mut SDL_AudioCVT,
      src_format: SDL_AudioFormat,
      src_channels: Uint8,
      src_rate: c_int,
      dst_format: SDL_AudioFormat,
      dst_channels: Uint8,
      dst_rate: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_ConvertAudio: unsafe extern "C" fn(cvt: *mut SDL_AudioCVT) -> c_int
);
assert_fn!(
  SDL_NewAudioStream:
    unsafe extern "C" fn(
      src_format: SDL_AudioFormat,
      src_channels: Uint8,
      src_rate: c_int,
      dst_format: SDL_AudioFormat,
      dst_channels: Uint8,
      dst_rate: c_int,
    ) -> *mut SDL_AudioStream
);
assert_fn!(
  SDL_AudioStreamPut:
    unsafe extern "C" fn(
      stream: *mut SDL_AudioStream,
      buf: *const c_void,
      len: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_AudioStreamGet:
    unsafe extern "C" fn(
      stream: *mut SDL_AudioStream,
      buf: *mut c_void,
      len: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_AudioStreamAvailable:
    unsafe extern "C" fn(stream: *mut SDL_AudioStream) -> c_int
);
assert_fn!(
  SDL_AudioStreamFlush:
    unsafe extern "C" fn(stream: *mut SDL_AudioStream) -> c_int
);
assert_fn!(
  SDL_AudioStreamClear: unsafe extern "C" fn(stream: *mut SDL_AudioStream)
);
assert_fn!(
  SDL_FreeAudioStream: unsafe extern "C" fn(stream: *mut SDL_AudioStream)
);
assert_fn!(
  SDL_MixAudio:
    unsafe extern "C" fn(
      dst: *mut Uint8,
      src: *const Uint8,
      len: Uint32,
      volume: c_int,
    )
);
assert_fn!(
  SDL_MixAudioFormat:
    unsafe extern "C" fn(
      dst: *mut Uint8,
      src: *const Uint8,
      format: SDL_AudioFormat,
      len: Uint32,
      volume: c_int,
    )
);
assert_fn!(
  SDL_QueueAudio:
    unsafe extern "C" fn(
      dev: SDL_AudioDeviceID,
      data: *const c_void,
      len: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_DequeueAudio:
    unsafe extern "C" fn(
      dev: SDL_AudioDeviceID,
      data: *mut c_void,
      len: Uint32,
    ) -> Uint32
);
assert_fn!(
  SDL_GetQueuedAudioSize:
    unsafe extern "C" fn(dev: SDL_AudioDeviceID) -> Uint32
);
assert_fn!(SDL_ClearQueuedAudio: unsafe extern "C" fn(dev: SDL_AudioDeviceID));
assert_fn!(SDL_LockAudio: unsafe extern "C" fn());
assert_fn!(SDL_LockAudioDevice: unsafe extern "C" fn(dev: SDL_AudioDeviceID));
assert_fn!(SDL_UnlockAudio: unsafe extern "C" fn());
assert_fn!(SDL_UnlockAudioDevice: unsafe extern "C" fn(dev: SDL_AudioDeviceID));
assert_fn!(SDL_CloseAudio: unsafe extern "C" fn());
assert_fn!(SDL_CloseAudioDevice: unsafe extern "C" fn(dev: SDL_AudioDeviceID));
assert_fn!(
  SDL_SetClipboardText: unsafe extern "C" fn(text: *const c_char) -> c_int
);
assert_fn!(SDL_GetClipboardText: unsafe extern "C" fn() -> *mut c_char);
assert_fn!(SDL_HasClipboardText: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_GetCPUCount: unsafe extern "C" fn() -> c_int);
assert_fn!(SDL_GetCPUCacheLineSize: unsafe extern "C" fn() -> c_int);
assert_fn!(SDL_HasRDTSC: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasAltiVec: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasMMX: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_Has3DNow: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasSSE: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasSSE2: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasSSE3: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasSSE41: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasSSE42: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasAVX: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasAVX2: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasAVX512F: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasARMSIMD: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_HasNEON: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_GetSystemRAM: unsafe extern "C" fn() -> c_int);
assert_fn!(SDL_SIMDGetAlignment: unsafe extern "C" fn() -> usize);
assert_fn!(SDL_SIMDAlloc: unsafe extern "C" fn(len: usize) -> *mut c_void);
assert_fn!(SDL_SIMDFree: unsafe extern "C" fn(ptr: *mut c_void));
assert_fn!(
  SDL_GetPixelFormatName: unsafe extern "C" fn(format: Uint32) -> *const c_char
);
assert_fn!(
  SDL_PixelFormatEnumToMasks:
    unsafe extern "C" fn(
      format: Uint32,
      bpp: *mut c_int,
      Rmask: *mut Uint32,
      Gmask: *mut Uint32,
      Bmask: *mut Uint32,
      Amask: *mut Uint32,
    ) -> SDL_bool
);
assert_fn!(
  SDL_MasksToPixelFormatEnum:
    unsafe extern "C" fn(
      bpp: c_int,
      Rmask: Uint32,
      Gmask: Uint32,
      Bmask: Uint32,
      Amask: Uint32,
    ) -> Uint32
);
assert_fn!(
  SDL_AllocFormat:
    unsafe extern "C" fn(pixel_format: Uint32) -> *mut SDL_PixelFormat
);
assert_fn!(SDL_FreeFormat: unsafe extern "C" fn(format: *mut SDL_PixelFormat));
assert_fn!(
  SDL_AllocPalette: unsafe extern "C" fn(ncolors: c_int) -> *mut SDL_Palette
);
assert_fn!(
  SDL_SetPixelFormatPalette:
    unsafe extern "C" fn(
      format: *mut SDL_PixelFormat,
      palette: *mut SDL_Palette,
    ) -> c_int
);
assert_fn!(
  SDL_SetPaletteColors:
    unsafe extern "C" fn(
      palette: *mut SDL_Palette,
      colors: *const SDL_Color,
      firstcolor: c_int,
      ncolors: c_int,
    ) -> c_int
);
assert_fn!(SDL_FreePalette: unsafe extern "C" fn(palette: *mut SDL_Palette));
assert_fn!(
  SDL_MapRGB:
    unsafe extern "C" fn(
      format: *const SDL_PixelFormat,
      r: Uint8,
      g: Uint8,
      b: Uint8,
    ) -> Uint32
);
assert_fn!(
  SDL_MapRGBA:
    unsafe extern "C" fn(
      format: *const SDL_PixelFormat,
      r: Uint8,
      g: Uint8,
      b: Uint8,
      a: Uint8,
    ) -> Uint32
);
assert_fn!(
  SDL_GetRGB:
    unsafe extern "C" fn(
      pixel: Uint32,
      format: *const SDL_PixelFormat,
      r: *mut Uint8,
      g: *mut Uint8,
      b: *mut Uint8,
    )
);
assert_fn!(
  SDL_GetRGBA:
    unsafe extern "C" fn(
      pixel: Uint32,
      format: *const SDL_PixelFormat,
      r: *mut Uint8,
      g: *mut Uint8,
      b: *mut Uint8,
      a: *mut Uint8,
    )
);
assert_fn!(
  SDL_CalculateGammaRamp: unsafe extern "C" fn(gamma: f32, ramp: *mut Uint16)
);
assert_fn!(
  SDL_HasIntersection:
    unsafe extern "C" fn(A: *const SDL_Rect, B: *const SDL_Rect) -> SDL_bool
);
assert_fn!(
  SDL_IntersectRect:
    unsafe extern "C" fn(
      A: *const SDL_Rect,
      B: *const SDL_Rect,
      result: *mut SDL_Rect,
    ) -> SDL_bool
);
assert_fn!(
  SDL_UnionRect:
    unsafe extern "C" fn(
      A: *const SDL_Rect,
      B: *const SDL_Rect,
      result: *mut SDL_Rect,
    )
);
assert_fn!(
  SDL_EnclosePoints:
    unsafe extern "C" fn(
      points: *const SDL_Point,
      count: c_int,
      clip: *const SDL_Rect,
      result: *mut SDL_Rect,
    ) -> SDL_bool
);
assert_fn!(
  SDL_IntersectRectAndLine:
    unsafe extern "C" fn(
      rect: *const SDL_Rect,
      X1: *mut c_int,
      Y1: *mut c_int,
      X2: *mut c_int,
      Y2: *mut c_int,
    ) -> SDL_bool
);
assert_fn!(
  SDL_ComposeCustomBlendMode:
    unsafe extern "C" fn(
      srcColorFactor: SDL_BlendFactor,
      dstColorFactor: SDL_BlendFactor,
      colorOperation: SDL_BlendOperation,
      srcAlphaFactor: SDL_BlendFactor,
      dstAlphaFactor: SDL_BlendFactor,
      alphaOperation: SDL_BlendOperation,
    ) -> SDL_BlendMode
);
assert_fn!(
  SDL_CreateRGBSurface:
    unsafe extern "C" fn(
      flags: Uint32,
      width: c_int,
      height: c_int,
      depth: c_int,
      Rmask: Uint32,
      Gmask: Uint32,
      Bmask: Uint32,
      Amask: Uint32,
    ) -> *mut SDL_Surface
);
assert_fn!(
  SDL_CreateRGBSurfaceWithFormat:
    unsafe extern "C" fn(
      flags: Uint32,
      width: c_int,
      height: c_int,
      depth: c_int,
      format: Uint32,
    ) -> *mut SDL_Surface
);
assert_fn!(
  SDL_CreateRGBSurfaceFrom:
    unsafe extern "C" fn(
      pixels: *mut c_void,
      width: c_int,
      height: c_int,
      depth: c_int,
      pitch: c_int,
      Rmask: Uint32,
      Gmask: Uint32,
      Bmask: Uint32,
      Amask: Uint32,
    ) -> *mut SDL_Surface
);
assert_fn!(
  SDL_CreateRGBSurfaceWithFormatFrom:
    unsafe extern "C" fn(
      pixels: *mut c_void,
      width: c_int,
      height: c_int,
      depth: c_int,
      pitch: c_int,
      format: Uint32,
    ) -> *mut SDL_Surface
);
assert_fn!(SDL_FreeSurface: unsafe extern "C" fn(surface: *mut SDL_Surface));
assert_fn!(
  SDL_SetSurfacePalette:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      palette: *mut SDL_Palette,
    ) -> c_int
);
assert_fn!(
  SDL_LockSurface: unsafe extern "C" fn(surface: *mut SDL_Surface) -> c_int
);
assert_fn!(SDL_UnlockSurface: unsafe extern "C" fn(surface: *mut SDL_Surface));
assert_fn!(
  SDL_LoadBMP_RW:
    unsafe extern "C" fn(
      src: *mut SDL_RWops,
      freesrc: c_int,
    ) -> *mut SDL_Surface
);
assert_fn!(
  SDL_SaveBMP_RW:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      dst: *mut SDL_RWops,
      freedst: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_SetSurfaceRLE:
    unsafe extern "C" fn(surface: *mut SDL_Surface, flag: c_int) -> c_int
);
assert_fn!(
  SDL_SetColorKey:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      flag: c_int,
      key: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_HasColorKey: unsafe extern "C" fn(surface: *mut SDL_Surface) -> SDL_bool
);
assert_fn!(
  SDL_GetColorKey:
    unsafe extern "C" fn(surface: *mut SDL_Surface, key: *mut Uint32) -> c_int
);
assert_fn!(
  SDL_SetSurfaceColorMod:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      r: Uint8,
      g: Uint8,
      b: Uint8,
    ) -> c_int
);
assert_fn!(
  SDL_GetSurfaceColorMod:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      r: *mut Uint8,
      g: *mut Uint8,
      b: *mut Uint8,
    ) -> c_int
);
assert_fn!(
  SDL_SetSurfaceAlphaMod:
    unsafe extern "C" fn(surface: *mut SDL_Surface, alpha: Uint8) -> c_int
);
assert_fn!(
  SDL_GetSurfaceAlphaMod:
    unsafe extern "C" fn(surface: *mut SDL_Surface, alpha: *mut Uint8) -> c_int
);
assert_fn!(
  SDL_SetSurfaceBlendMode:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      blendMode: SDL_BlendMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetSurfaceBlendMode:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      blendMode: *mut SDL_BlendMode,
    ) -> c_int
);
assert_fn!(
  SDL_SetClipRect:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      rect: *const SDL_Rect,
    ) -> SDL_bool
);
assert_fn!(
  SDL_GetClipRect:
    unsafe extern "C" fn(surface: *mut SDL_Surface, rect: *mut SDL_Rect)
);
assert_fn!(
  SDL_DuplicateSurface:
    unsafe extern "C" fn(surface: *mut SDL_Surface) -> *mut SDL_Surface
);
assert_fn!(
  SDL_ConvertSurface:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      fmt: *const SDL_PixelFormat,
      flags: Uint32,
    ) -> *mut SDL_Surface
);
assert_fn!(
  SDL_ConvertSurfaceFormat:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      pixel_format: Uint32,
      flags: Uint32,
    ) -> *mut SDL_Surface
);
assert_fn!(
  SDL_ConvertPixels:
    unsafe extern "C" fn(
      width: c_int,
      height: c_int,
      src_format: Uint32,
      src: *const c_void,
      src_pitch: c_int,
      dst_format: Uint32,
      dst: *mut c_void,
      dst_pitch: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_FillRect:
    unsafe extern "C" fn(
      dst: *mut SDL_Surface,
      rect: *const SDL_Rect,
      color: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_FillRects:
    unsafe extern "C" fn(
      dst: *mut SDL_Surface,
      rects: *const SDL_Rect,
      count: c_int,
      color: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_UpperBlit:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      srcrect: *const SDL_Rect,
      dst: *mut SDL_Surface,
      dstrect: *mut SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_LowerBlit:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      srcrect: *mut SDL_Rect,
      dst: *mut SDL_Surface,
      dstrect: *mut SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_SoftStretch:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      srcrect: *const SDL_Rect,
      dst: *mut SDL_Surface,
      dstrect: *const SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_UpperBlitScaled:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      srcrect: *const SDL_Rect,
      dst: *mut SDL_Surface,
      dstrect: *mut SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_LowerBlitScaled:
    unsafe extern "C" fn(
      src: *mut SDL_Surface,
      srcrect: *mut SDL_Rect,
      dst: *mut SDL_Surface,
      dstrect: *mut SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_SetYUVConversionMode: unsafe extern "C" fn(mode: SDL_YUV_CONVERSION_MODE)
);
assert_fn!(
  SDL_GetYUVConversionMode: unsafe extern "C" fn() -> SDL_YUV_CONVERSION_MODE
);
assert_fn!(
  SDL_GetYUVConversionModeForResolution:
    unsafe extern "C" fn(
      width: c_int,
      height: c_int,
    ) -> SDL_YUV_CONVERSION_MODE
);
assert_fn!(SDL_GetNumVideoDrivers: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_GetVideoDriver: unsafe extern "C" fn(index: c_int) -> *const c_char
);
assert_fn!(
  SDL_VideoInit: unsafe extern "C" fn(driver_name: *const c_char) -> c_int
);
assert_fn!(SDL_VideoQuit: unsafe extern "C" fn());
assert_fn!(SDL_GetCurrentVideoDriver: unsafe extern "C" fn() -> *const c_char);
assert_fn!(SDL_GetNumVideoDisplays: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_GetDisplayName:
    unsafe extern "C" fn(displayIndex: c_int) -> *const c_char
);
assert_fn!(
  SDL_GetDisplayBounds:
    unsafe extern "C" fn(displayIndex: c_int, rect: *mut SDL_Rect) -> c_int
);
assert_fn!(
  SDL_GetDisplayUsableBounds:
    unsafe extern "C" fn(displayIndex: c_int, rect: *mut SDL_Rect) -> c_int
);
assert_fn!(
  SDL_GetDisplayDPI:
    unsafe extern "C" fn(
      displayIndex: c_int,
      ddpi: *mut f32,
      hdpi: *mut f32,
      vdpi: *mut f32,
    ) -> c_int
);
assert_fn!(
  SDL_GetDisplayOrientation:
    unsafe extern "C" fn(displayIndex: c_int) -> SDL_DisplayOrientation
);
assert_fn!(
  SDL_GetNumDisplayModes: unsafe extern "C" fn(displayIndex: c_int) -> c_int
);
assert_fn!(
  SDL_GetDisplayMode:
    unsafe extern "C" fn(
      displayIndex: c_int,
      modeIndex: c_int,
      mode: *mut SDL_DisplayMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetDesktopDisplayMode:
    unsafe extern "C" fn(
      displayIndex: c_int,
      mode: *mut SDL_DisplayMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetCurrentDisplayMode:
    unsafe extern "C" fn(
      displayIndex: c_int,
      mode: *mut SDL_DisplayMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetClosestDisplayMode:
    unsafe extern "C" fn(
      displayIndex: c_int,
      mode: *const SDL_DisplayMode,
      closest: *mut SDL_DisplayMode,
    ) -> *mut SDL_DisplayMode
);
assert_fn!(
  SDL_GetWindowDisplayIndex:
    unsafe extern "C" fn(window: *mut SDL_Window) -> c_int
);
assert_fn!(
  SDL_SetWindowDisplayMode:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      mode: *const SDL_DisplayMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetWindowDisplayMode:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      mode: *mut SDL_DisplayMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetWindowPixelFormat:
    unsafe extern "C" fn(window: *mut SDL_Window) -> Uint32
);
assert_fn!(
  SDL_CreateWindow:
    unsafe extern "C" fn(
      title: *const c_char,
      x: c_int,
      y: c_int,
      w: c_int,
      h: c_int,
      flags: Uint32,
    ) -> *mut SDL_Window
);
assert_fn!(
  SDL_CreateWindowFrom:
    unsafe extern "C" fn(data: *const c_void) -> *mut SDL_Window
);
assert_fn!(
  SDL_GetWindowID: unsafe extern "C" fn(window: *mut SDL_Window) -> Uint32
);
assert_fn!(
  SDL_GetWindowFromID: unsafe extern "C" fn(id: Uint32) -> *mut SDL_Window
);
assert_fn!(
  SDL_GetWindowFlags: unsafe extern "C" fn(window: *mut SDL_Window) -> Uint32
);
assert_fn!(
  SDL_SetWindowTitle:
    unsafe extern "C" fn(window: *mut SDL_Window, title: *const c_char)
);
assert_fn!(
  SDL_GetWindowTitle:
    unsafe extern "C" fn(window: *mut SDL_Window) -> *const c_char
);
assert_fn!(
  SDL_SetWindowIcon:
    unsafe extern "C" fn(window: *mut SDL_Window, icon: *mut SDL_Surface)
);
assert_fn!(
  SDL_SetWindowData:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      name: *const c_char,
      userdata: *mut c_void,
    ) -> *mut c_void
);
assert_fn!(
  SDL_GetWindowData:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      name: *const c_char,
    ) -> *mut c_void
);
assert_fn!(
  SDL_SetWindowPosition:
    unsafe extern "C" fn(window: *mut SDL_Window, x: c_int, y: c_int)
);
assert_fn!(
  SDL_GetWindowPosition:
    unsafe extern "C" fn(window: *mut SDL_Window, x: *mut c_int, y: *mut c_int)
);
assert_fn!(
  SDL_SetWindowSize:
    unsafe extern "C" fn(window: *mut SDL_Window, w: c_int, h: c_int)
);
assert_fn!(
  SDL_GetWindowSize:
    unsafe extern "C" fn(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int)
);
assert_fn!(
  SDL_GetWindowBordersSize:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      top: *mut c_int,
      left: *mut c_int,
      bottom: *mut c_int,
      right: *mut c_int,
    ) -> c_int
);
assert_fn!(
  SDL_SetWindowMinimumSize:
    unsafe extern "C" fn(window: *mut SDL_Window, min_w: c_int, min_h: c_int)
);
assert_fn!(
  SDL_GetWindowMinimumSize:
    unsafe extern "C" fn(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int)
);
assert_fn!(
  SDL_SetWindowMaximumSize:
    unsafe extern "C" fn(window: *mut SDL_Window, max_w: c_int, max_h: c_int)
);
assert_fn!(
  SDL_GetWindowMaximumSize:
    unsafe extern "C" fn(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int)
);
assert_fn!(
  SDL_SetWindowBordered:
    unsafe extern "C" fn(window: *mut SDL_Window, bordered: SDL_bool)
);
assert_fn!(
  SDL_SetWindowResizable:
    unsafe extern "C" fn(window: *mut SDL_Window, resizable: SDL_bool)
);
assert_fn!(SDL_ShowWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_HideWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_RaiseWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_MaximizeWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_MinimizeWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_RestoreWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(
  SDL_SetWindowFullscreen:
    unsafe extern "C" fn(window: *mut SDL_Window, flags: Uint32) -> c_int
);
assert_fn!(
  SDL_GetWindowSurface:
    unsafe extern "C" fn(window: *mut SDL_Window) -> *mut SDL_Surface
);
assert_fn!(
  SDL_UpdateWindowSurface:
    unsafe extern "C" fn(window: *mut SDL_Window) -> c_int
);
assert_fn!(
  SDL_UpdateWindowSurfaceRects:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      rects: *const SDL_Rect,
      numrects: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_SetWindowGrab:
    unsafe extern "C" fn(window: *mut SDL_Window, grabbed: SDL_bool)
);
assert_fn!(
  SDL_GetWindowGrab: unsafe extern "C" fn(window: *mut SDL_Window) -> SDL_bool
);
assert_fn!(SDL_GetGrabbedWindow: unsafe extern "C" fn() -> *mut SDL_Window);
assert_fn!(
  SDL_SetWindowBrightness:
    unsafe extern "C" fn(window: *mut SDL_Window, brightness: f32) -> c_int
);
assert_fn!(
  SDL_GetWindowBrightness: unsafe extern "C" fn(window: *mut SDL_Window) -> f32
);
assert_fn!(
  SDL_SetWindowOpacity:
    unsafe extern "C" fn(window: *mut SDL_Window, opacity: f32) -> c_int
);
assert_fn!(
  SDL_GetWindowOpacity:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      out_opacity: *mut f32,
    ) -> c_int
);
assert_fn!(
  SDL_SetWindowModalFor:
    unsafe extern "C" fn(
      modal_window: *mut SDL_Window,
      parent_window: *mut SDL_Window,
    ) -> c_int
);
assert_fn!(
  SDL_SetWindowInputFocus:
    unsafe extern "C" fn(window: *mut SDL_Window) -> c_int
);
assert_fn!(
  SDL_SetWindowGammaRamp:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      red: *const Uint16,
      green: *const Uint16,
      blue: *const Uint16,
    ) -> c_int
);
assert_fn!(
  SDL_GetWindowGammaRamp:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      red: *mut Uint16,
      green: *mut Uint16,
      blue: *mut Uint16,
    ) -> c_int
);
assert_fn!(
  SDL_SetWindowHitTest:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      callback: SDL_HitTest,
      callback_data: *mut c_void,
    ) -> c_int
);
assert_fn!(SDL_DestroyWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_IsScreenSaverEnabled: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_EnableScreenSaver: unsafe extern "C" fn());
assert_fn!(SDL_DisableScreenSaver: unsafe extern "C" fn());
assert_fn!(
  SDL_GL_LoadLibrary: unsafe extern "C" fn(path: *const c_char) -> c_int
);
assert_fn!(
  SDL_GL_GetProcAddress:
    unsafe extern "C" fn(proc_: *const c_char) -> *mut c_void
);
assert_fn!(SDL_GL_UnloadLibrary: unsafe extern "C" fn());
assert_fn!(
  SDL_GL_ExtensionSupported:
    unsafe extern "C" fn(extension: *const c_char) -> SDL_bool
);
assert_fn!(SDL_GL_ResetAttributes: unsafe extern "C" fn());
assert_fn!(
  SDL_GL_SetAttribute:
    unsafe extern "C" fn(attr: SDL_GLattr, value: c_int) -> c_int
);
assert_fn!(
  SDL_GL_GetAttribute:
    unsafe extern "C" fn(attr: SDL_GLattr, value: *mut c_int) -> c_int
);
assert_fn!(
  SDL_GL_CreateContext:
    unsafe extern "C" fn(window: *mut SDL_Window) -> SDL_GLContext
);
assert_fn!(
  SDL_GL_MakeCurrent:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      context: SDL_GLContext,
    ) -> c_int
);
assert_fn!(SDL_GL_GetCurrentWindow: unsafe extern "C" fn() -> *mut SDL_Window);
assert_fn!(SDL_GL_GetCurrentContext: unsafe extern "C" fn() -> SDL_GLContext);
assert_fn!(
  SDL_GL_GetDrawableSize:
    unsafe extern "C" fn(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int)
);
assert_fn!(
  SDL_GL_SetSwapInterval: unsafe extern "C" fn(interval: c_int) -> c_int
);
assert_fn!(SDL_GL_GetSwapInterval: unsafe extern "C" fn() -> c_int);
assert_fn!(SDL_GL_SwapWindow: unsafe extern "C" fn(window: *mut SDL_Window));
assert_fn!(SDL_GL_DeleteContext: unsafe extern "C" fn(context: SDL_GLContext));
assert_fn!(SDL_GetKeyboardFocus: unsafe extern "C" fn() -> *mut SDL_Window);
assert_fn!(
  SDL_GetKeyboardState:
    unsafe extern "C" fn(numkeys: *mut c_int) -> *const Uint8
);
assert_fn!(SDL_GetModState: unsafe extern "C" fn() -> SDL_Keymod);
assert_fn!(SDL_SetModState: unsafe extern "C" fn(modstate: SDL_Keymod));
assert_fn!(
  SDL_GetKeyFromScancode:
    unsafe extern "C" fn(scancode: SDL_Scancode) -> SDL_Keycode
);
assert_fn!(
  SDL_GetScancodeFromKey:
    unsafe extern "C" fn(key: SDL_Keycode) -> SDL_Scancode
);
assert_fn!(
  SDL_GetScancodeName:
    unsafe extern "C" fn(scancode: SDL_Scancode) -> *const c_char
);
assert_fn!(
  SDL_GetScancodeFromName:
    unsafe extern "C" fn(name: *const c_char) -> SDL_Scancode
);
assert_fn!(
  SDL_GetKeyName: unsafe extern "C" fn(key: SDL_Keycode) -> *const c_char
);
assert_fn!(
  SDL_GetKeyFromName: unsafe extern "C" fn(name: *const c_char) -> SDL_Keycode
);
assert_fn!(SDL_StartTextInput: unsafe extern "C" fn());
assert_fn!(SDL_IsTextInputActive: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_StopTextInput: unsafe extern "C" fn());
assert_fn!(SDL_SetTextInputRect: unsafe extern "C" fn(rect: *mut SDL_Rect));
assert_fn!(SDL_HasScreenKeyboardSupport: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(
  SDL_IsScreenKeyboardShown:
    unsafe extern "C" fn(window: *mut SDL_Window) -> SDL_bool
);
assert_fn!(SDL_GetMouseFocus: unsafe extern "C" fn() -> *mut SDL_Window);
assert_fn!(
  SDL_GetMouseState:
    unsafe extern "C" fn(x: *mut c_int, y: *mut c_int) -> Uint32
);
assert_fn!(
  SDL_GetGlobalMouseState:
    unsafe extern "C" fn(x: *mut c_int, y: *mut c_int) -> Uint32
);
assert_fn!(
  SDL_GetRelativeMouseState:
    unsafe extern "C" fn(x: *mut c_int, y: *mut c_int) -> Uint32
);
assert_fn!(
  SDL_WarpMouseInWindow:
    unsafe extern "C" fn(window: *mut SDL_Window, x: c_int, y: c_int)
);
assert_fn!(
  SDL_WarpMouseGlobal: unsafe extern "C" fn(x: c_int, y: c_int) -> c_int
);
assert_fn!(
  SDL_SetRelativeMouseMode: unsafe extern "C" fn(enabled: SDL_bool) -> c_int
);
assert_fn!(SDL_CaptureMouse: unsafe extern "C" fn(enabled: SDL_bool) -> c_int);
assert_fn!(SDL_GetRelativeMouseMode: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(
  SDL_CreateCursor:
    unsafe extern "C" fn(
      data: *const Uint8,
      mask: *const Uint8,
      w: c_int,
      h: c_int,
      hot_x: c_int,
      hot_y: c_int,
    ) -> *mut SDL_Cursor
);
assert_fn!(
  SDL_CreateColorCursor:
    unsafe extern "C" fn(
      surface: *mut SDL_Surface,
      hot_x: c_int,
      hot_y: c_int,
    ) -> *mut SDL_Cursor
);
assert_fn!(
  SDL_CreateSystemCursor:
    unsafe extern "C" fn(id: SDL_SystemCursor) -> *mut SDL_Cursor
);
assert_fn!(SDL_SetCursor: unsafe extern "C" fn(cursor: *mut SDL_Cursor));
assert_fn!(SDL_GetCursor: unsafe extern "C" fn() -> *mut SDL_Cursor);
assert_fn!(SDL_GetDefaultCursor: unsafe extern "C" fn() -> *mut SDL_Cursor);
assert_fn!(SDL_FreeCursor: unsafe extern "C" fn(cursor: *mut SDL_Cursor));
assert_fn!(SDL_ShowCursor: unsafe extern "C" fn(toggle: c_int) -> c_int);
assert_fn!(SDL_LockJoysticks: unsafe extern "C" fn());
assert_fn!(SDL_UnlockJoysticks: unsafe extern "C" fn());
assert_fn!(SDL_NumJoysticks: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_JoystickNameForIndex:
    unsafe extern "C" fn(device_index: c_int) -> *const c_char
);
assert_fn!(
  SDL_JoystickGetDevicePlayerIndex:
    unsafe extern "C" fn(device_index: c_int) -> c_int
);
assert_fn!(
  SDL_JoystickGetDeviceGUID:
    unsafe extern "C" fn(device_index: c_int) -> SDL_JoystickGUID
);
assert_fn!(
  SDL_JoystickGetDeviceVendor:
    unsafe extern "C" fn(device_index: c_int) -> Uint16
);
assert_fn!(
  SDL_JoystickGetDeviceProduct:
    unsafe extern "C" fn(device_index: c_int) -> Uint16
);
assert_fn!(
  SDL_JoystickGetDeviceProductVersion:
    unsafe extern "C" fn(device_index: c_int) -> Uint16
);
assert_fn!(
  SDL_JoystickGetDeviceType:
    unsafe extern "C" fn(device_index: c_int) -> SDL_JoystickType
);
assert_fn!(
  SDL_JoystickGetDeviceInstanceID:
    unsafe extern "C" fn(device_index: c_int) -> SDL_JoystickID
);
assert_fn!(
  SDL_JoystickOpen:
    unsafe extern "C" fn(device_index: c_int) -> *mut SDL_Joystick
);
assert_fn!(
  SDL_JoystickFromInstanceID:
    unsafe extern "C" fn(instance_id: SDL_JoystickID) -> *mut SDL_Joystick
);
assert_fn!(
  SDL_JoystickFromPlayerIndex:
    unsafe extern "C" fn(player_index: c_int) -> *mut SDL_Joystick
);
assert_fn!(
  SDL_JoystickName:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> *const c_char
);
assert_fn!(
  SDL_JoystickGetPlayerIndex:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> c_int
);
assert_fn!(
  SDL_JoystickSetPlayerIndex:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick, player_index: c_int)
);
assert_fn!(
  SDL_JoystickGetGUID:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> SDL_JoystickGUID
);
assert_fn!(
  SDL_JoystickGetVendor:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> Uint16
);
assert_fn!(
  SDL_JoystickGetProduct:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> Uint16
);
assert_fn!(
  SDL_JoystickGetProductVersion:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> Uint16
);
assert_fn!(
  SDL_JoystickGetType:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> SDL_JoystickType
);
assert_fn!(
  SDL_JoystickGetGUIDString:
    unsafe extern "C" fn(
      guid: SDL_JoystickGUID,
      pszGUID: *mut c_char,
      cbGUID: c_int,
    )
);
assert_fn!(
  SDL_JoystickGetGUIDFromString:
    unsafe extern "C" fn(pchGUID: *const c_char) -> SDL_JoystickGUID
);
assert_fn!(
  SDL_JoystickGetAttached:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> SDL_bool
);
assert_fn!(
  SDL_JoystickInstanceID:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> SDL_JoystickID
);
assert_fn!(
  SDL_JoystickNumAxes:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> c_int
);
assert_fn!(
  SDL_JoystickNumBalls:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> c_int
);
assert_fn!(
  SDL_JoystickNumHats:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> c_int
);
assert_fn!(
  SDL_JoystickNumButtons:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> c_int
);
assert_fn!(SDL_JoystickUpdate: unsafe extern "C" fn());
assert_fn!(SDL_JoystickEventState: unsafe extern "C" fn(state: c_int) -> c_int);
assert_fn!(
  SDL_JoystickGetAxis:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick, axis: c_int) -> Sint16
);
assert_fn!(
  SDL_JoystickGetAxisInitialState:
    unsafe extern "C" fn(
      joystick: *mut SDL_Joystick,
      axis: c_int,
      state: *mut Sint16,
    ) -> SDL_bool
);
assert_fn!(
  SDL_JoystickGetHat:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick, hat: c_int) -> Uint8
);
assert_fn!(
  SDL_JoystickGetBall:
    unsafe extern "C" fn(
      joystick: *mut SDL_Joystick,
      ball: c_int,
      dx: *mut c_int,
      dy: *mut c_int,
    ) -> c_int
);
assert_fn!(
  SDL_JoystickGetButton:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick, button: c_int) -> Uint8
);
assert_fn!(
  SDL_JoystickRumble:
    unsafe extern "C" fn(
      joystick: *mut SDL_Joystick,
      low_frequency_rumble: Uint16,
      high_frequency_rumble: Uint16,
      duration_ms: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_JoystickClose: unsafe extern "C" fn(joystick: *mut SDL_Joystick)
);
assert_fn!(
  SDL_JoystickCurrentPowerLevel:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> SDL_JoystickPowerLevel
);
assert_fn!(
  SDL_GameControllerAddMappingsFromRW:
    unsafe extern "C" fn(rw: *mut SDL_RWops, freerw: c_int) -> c_int
);
assert_fn!(
  SDL_GameControllerAddMapping:
    unsafe extern "C" fn(mappingString: *const c_char) -> c_int
);
assert_fn!(SDL_GameControllerNumMappings: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_GameControllerMappingForIndex:
    unsafe extern "C" fn(mapping_index: c_int) -> *mut c_char
);
assert_fn!(
  SDL_GameControllerMappingForGUID:
    unsafe extern "C" fn(guid: SDL_JoystickGUID) -> *mut c_char
);
assert_fn!(
  SDL_GameControllerMapping:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
    ) -> *mut c_char
);
assert_fn!(
  SDL_IsGameController: unsafe extern "C" fn(joystick_index: c_int) -> SDL_bool
);
assert_fn!(
  SDL_GameControllerNameForIndex:
    unsafe extern "C" fn(joystick_index: c_int) -> *const c_char
);
assert_fn!(
  SDL_GameControllerTypeForIndex:
    unsafe extern "C" fn(joystick_index: c_int) -> SDL_GameControllerType
);
assert_fn!(
  SDL_GameControllerMappingForDeviceIndex:
    unsafe extern "C" fn(joystick_index: c_int) -> *mut c_char
);
assert_fn!(
  SDL_GameControllerOpen:
    unsafe extern "C" fn(joystick_index: c_int) -> *mut SDL_GameController
);
assert_fn!(
  SDL_GameControllerFromInstanceID:
    unsafe extern "C" fn(joyid: SDL_JoystickID) -> *mut SDL_GameController
);
assert_fn!(
  SDL_GameControllerFromPlayerIndex:
    unsafe extern "C" fn(player_index: c_int) -> *mut SDL_GameController
);
assert_fn!(
  SDL_GameControllerName:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
    ) -> *const c_char
);
assert_fn!(
  SDL_GameControllerGetType:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
    ) -> SDL_GameControllerType
);
assert_fn!(
  SDL_GameControllerGetPlayerIndex:
    unsafe extern "C" fn(gamecontroller: *mut SDL_GameController) -> c_int
);
assert_fn!(
  SDL_GameControllerSetPlayerIndex:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
      player_index: c_int,
    )
);
assert_fn!(
  SDL_GameControllerGetVendor:
    unsafe extern "C" fn(gamecontroller: *mut SDL_GameController) -> Uint16
);
assert_fn!(
  SDL_GameControllerGetProduct:
    unsafe extern "C" fn(gamecontroller: *mut SDL_GameController) -> Uint16
);
assert_fn!(
  SDL_GameControllerGetProductVersion:
    unsafe extern "C" fn(gamecontroller: *mut SDL_GameController) -> Uint16
);
assert_fn!(
  SDL_GameControllerGetAttached:
    unsafe extern "C" fn(gamecontroller: *mut SDL_GameController) -> SDL_bool
);
assert_fn!(
  SDL_GameControllerGetJoystick:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
    ) -> *mut SDL_Joystick
);
assert_fn!(
  SDL_GameControllerEventState: unsafe extern "C" fn(state: c_int) -> c_int
);
assert_fn!(SDL_GameControllerUpdate: unsafe extern "C" fn());
assert_fn!(
  SDL_GameControllerGetAxisFromString:
    unsafe extern "C" fn(pchString: *const c_char) -> SDL_GameControllerAxis
);
assert_fn!(
  SDL_GameControllerGetStringForAxis:
    unsafe extern "C" fn(axis: SDL_GameControllerAxis) -> *const c_char
);
assert_fn!(
  SDL_GameControllerGetBindForAxis:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
      axis: SDL_GameControllerAxis,
    ) -> SDL_GameControllerButtonBind
);
assert_fn!(
  SDL_GameControllerGetAxis:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
      axis: SDL_GameControllerAxis,
    ) -> Sint16
);
assert_fn!(
  SDL_GameControllerGetButtonFromString:
    unsafe extern "C" fn(pchString: *const c_char) -> SDL_GameControllerButton
);
assert_fn!(
  SDL_GameControllerGetStringForButton:
    unsafe extern "C" fn(button: SDL_GameControllerButton) -> *const c_char
);
assert_fn!(
  SDL_GameControllerGetBindForButton:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
      button: SDL_GameControllerButton,
    ) -> SDL_GameControllerButtonBind
);
assert_fn!(
  SDL_GameControllerGetButton:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
      button: SDL_GameControllerButton,
    ) -> Uint8
);
assert_fn!(
  SDL_GameControllerRumble:
    unsafe extern "C" fn(
      gamecontroller: *mut SDL_GameController,
      low_frequency_rumble: Uint16,
      high_frequency_rumble: Uint16,
      duration_ms: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_GameControllerClose:
    unsafe extern "C" fn(gamecontroller: *mut SDL_GameController)
);
assert_fn!(SDL_GetNumTouchDevices: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_GetTouchDevice: unsafe extern "C" fn(index: c_int) -> SDL_TouchID
);
assert_fn!(
  SDL_GetTouchDeviceType:
    unsafe extern "C" fn(touchID: SDL_TouchID) -> SDL_TouchDeviceType
);
assert_fn!(
  SDL_GetNumTouchFingers: unsafe extern "C" fn(touchID: SDL_TouchID) -> c_int
);
assert_fn!(
  SDL_GetTouchFinger:
    unsafe extern "C" fn(touchID: SDL_TouchID, index: c_int) -> *mut SDL_Finger
);
assert_fn!(
  SDL_RecordGesture: unsafe extern "C" fn(touchId: SDL_TouchID) -> c_int
);
assert_fn!(
  SDL_SaveAllDollarTemplates:
    unsafe extern "C" fn(dst: *mut SDL_RWops) -> c_int
);
assert_fn!(
  SDL_SaveDollarTemplate:
    unsafe extern "C" fn(
      gestureId: SDL_GestureID,
      dst: *mut SDL_RWops,
    ) -> c_int
);
assert_fn!(
  SDL_LoadDollarTemplates:
    unsafe extern "C" fn(touchId: SDL_TouchID, src: *mut SDL_RWops) -> c_int
);
assert_fn!(SDL_PumpEvents: unsafe extern "C" fn());
assert_fn!(
  SDL_PeepEvents:
    unsafe extern "C" fn(
      events: *mut SDL_Event,
      numevents: c_int,
      action: SDL_eventaction,
      minType: Uint32,
      maxType: Uint32,
    ) -> c_int
);
assert_fn!(SDL_HasEvent: unsafe extern "C" fn(type_: Uint32) -> SDL_bool);
assert_fn!(
  SDL_HasEvents:
    unsafe extern "C" fn(minType: Uint32, maxType: Uint32) -> SDL_bool
);
assert_fn!(SDL_FlushEvent: unsafe extern "C" fn(type_: Uint32));
assert_fn!(
  SDL_FlushEvents: unsafe extern "C" fn(minType: Uint32, maxType: Uint32)
);
assert_fn!(SDL_PollEvent: unsafe extern "C" fn(event: *mut SDL_Event) -> c_int);
assert_fn!(SDL_WaitEvent: unsafe extern "C" fn(event: *mut SDL_Event) -> c_int);
assert_fn!(
  SDL_WaitEventTimeout:
    unsafe extern "C" fn(event: *mut SDL_Event, timeout: c_int) -> c_int
);
assert_fn!(SDL_PushEvent: unsafe extern "C" fn(event: *mut SDL_Event) -> c_int);
assert_fn!(
  SDL_SetEventFilter:
    unsafe extern "C" fn(filter: SDL_EventFilter, userdata: *mut c_void)
);
assert_fn!(
  SDL_GetEventFilter:
    unsafe extern "C" fn(
      filter: *mut SDL_EventFilter,
      userdata: *mut *mut c_void,
    ) -> SDL_bool
);
assert_fn!(
  SDL_AddEventWatch:
    unsafe extern "C" fn(filter: SDL_EventFilter, userdata: *mut c_void)
);
assert_fn!(
  SDL_DelEventWatch:
    unsafe extern "C" fn(filter: SDL_EventFilter, userdata: *mut c_void)
);
assert_fn!(
  SDL_FilterEvents:
    unsafe extern "C" fn(filter: SDL_EventFilter, userdata: *mut c_void)
);
assert_fn!(
  SDL_EventState: unsafe extern "C" fn(type_: Uint32, state: c_int) -> Uint8
);
assert_fn!(
  SDL_RegisterEvents: unsafe extern "C" fn(numevents: c_int) -> Uint32
);
assert_fn!(SDL_GetBasePath: unsafe extern "C" fn() -> *mut c_char);
assert_fn!(
  SDL_GetPrefPath:
    unsafe extern "C" fn(org: *const c_char, app: *const c_char) -> *mut c_char
);
assert_fn!(SDL_NumHaptics: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_HapticName: unsafe extern "C" fn(device_index: c_int) -> *const c_char
);
assert_fn!(
  SDL_HapticOpen: unsafe extern "C" fn(device_index: c_int) -> *mut SDL_Haptic
);
assert_fn!(
  SDL_HapticOpened: unsafe extern "C" fn(device_index: c_int) -> c_int
);
assert_fn!(
  SDL_HapticIndex: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(SDL_MouseIsHaptic: unsafe extern "C" fn() -> c_int);
assert_fn!(SDL_HapticOpenFromMouse: unsafe extern "C" fn() -> *mut SDL_Haptic);
assert_fn!(
  SDL_JoystickIsHaptic:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> c_int
);
assert_fn!(
  SDL_HapticOpenFromJoystick:
    unsafe extern "C" fn(joystick: *mut SDL_Joystick) -> *mut SDL_Haptic
);
assert_fn!(SDL_HapticClose: unsafe extern "C" fn(haptic: *mut SDL_Haptic));
assert_fn!(
  SDL_HapticNumEffects: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticNumEffectsPlaying:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticQuery: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_uint
);
assert_fn!(
  SDL_HapticNumAxes: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticEffectSupported:
    unsafe extern "C" fn(
      haptic: *mut SDL_Haptic,
      effect: *mut SDL_HapticEffect,
    ) -> c_int
);
assert_fn!(
  SDL_HapticNewEffect:
    unsafe extern "C" fn(
      haptic: *mut SDL_Haptic,
      effect: *mut SDL_HapticEffect,
    ) -> c_int
);
assert_fn!(
  SDL_HapticUpdateEffect:
    unsafe extern "C" fn(
      haptic: *mut SDL_Haptic,
      effect: c_int,
      data: *mut SDL_HapticEffect,
    ) -> c_int
);
assert_fn!(
  SDL_HapticRunEffect:
    unsafe extern "C" fn(
      haptic: *mut SDL_Haptic,
      effect: c_int,
      iterations: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_HapticStopEffect:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic, effect: c_int) -> c_int
);
assert_fn!(
  SDL_HapticDestroyEffect:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic, effect: c_int)
);
assert_fn!(
  SDL_HapticGetEffectStatus:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic, effect: c_int) -> c_int
);
assert_fn!(
  SDL_HapticSetGain:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic, gain: c_int) -> c_int
);
assert_fn!(
  SDL_HapticSetAutocenter:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic, autocenter: c_int) -> c_int
);
assert_fn!(
  SDL_HapticPause: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticUnpause: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticStopAll: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticRumbleSupported:
    unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticRumbleInit: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_HapticRumblePlay:
    unsafe extern "C" fn(
      haptic: *mut SDL_Haptic,
      strength: f32,
      length: Uint32,
    ) -> c_int
);
assert_fn!(
  SDL_HapticRumbleStop: unsafe extern "C" fn(haptic: *mut SDL_Haptic) -> c_int
);
assert_fn!(
  SDL_SetHintWithPriority:
    unsafe extern "C" fn(
      name: *const c_char,
      value: *const c_char,
      priority: SDL_HintPriority,
    ) -> SDL_bool
);
assert_fn!(
  SDL_SetHint:
    unsafe extern "C" fn(name: *const c_char, value: *const c_char) -> SDL_bool
);
assert_fn!(
  SDL_GetHint: unsafe extern "C" fn(name: *const c_char) -> *const c_char
);
assert_fn!(
  SDL_GetHintBoolean:
    unsafe extern "C" fn(
      name: *const c_char,
      default_value: SDL_bool,
    ) -> SDL_bool
);
assert_fn!(
  SDL_AddHintCallback:
    unsafe extern "C" fn(
      name: *const c_char,
      callback: SDL_HintCallback,
      userdata: *mut c_void,
    )
);
assert_fn!(
  SDL_DelHintCallback:
    unsafe extern "C" fn(
      name: *const c_char,
      callback: SDL_HintCallback,
      userdata: *mut c_void,
    )
);
assert_fn!(SDL_ClearHints: unsafe extern "C" fn());
assert_fn!(
  SDL_LoadObject: unsafe extern "C" fn(sofile: *const c_char) -> *mut c_void
);
assert_fn!(
  SDL_LoadFunction:
    unsafe extern "C" fn(
      handle: *mut c_void,
      name: *const c_char,
    ) -> *mut c_void
);
assert_fn!(SDL_UnloadObject: unsafe extern "C" fn(handle: *mut c_void));
assert_fn!(
  SDL_LogSetAllPriority: unsafe extern "C" fn(priority: SDL_LogPriority)
);
assert_fn!(
  SDL_LogSetPriority:
    unsafe extern "C" fn(category: c_int, priority: SDL_LogPriority)
);
assert_fn!(
  SDL_LogGetPriority: unsafe extern "C" fn(category: c_int) -> SDL_LogPriority
);
assert_fn!(SDL_LogResetPriorities: unsafe extern "C" fn());
assert_fn!(SDL_Log: unsafe extern "C" fn(fmt: *const c_char, ...));
assert_fn!(
  SDL_LogVerbose:
    unsafe extern "C" fn(category: c_int, fmt: *const c_char, ...)
);
assert_fn!(
  SDL_LogDebug: unsafe extern "C" fn(category: c_int, fmt: *const c_char, ...)
);
assert_fn!(
  SDL_LogInfo: unsafe extern "C" fn(category: c_int, fmt: *const c_char, ...)
);
assert_fn!(
  SDL_LogWarn: unsafe extern "C" fn(category: c_int, fmt: *const c_char, ...)
);
assert_fn!(
  SDL_LogError: unsafe extern "C" fn(category: c_int, fmt: *const c_char, ...)
);
assert_fn!(
  SDL_LogCritical:
    unsafe extern "C" fn(category: c_int, fmt: *const c_char, ...)
);
assert_fn!(
  SDL_LogMessage:
    unsafe extern "C" fn(
      category: c_int,
      priority: SDL_LogPriority,
      fmt: *const c_char,
      ...
    )
);
assert_fn!(
  SDL_LogGetOutputFunction:
    unsafe extern "C" fn(
      callback: *mut SDL_LogOutputFunction,
      userdata: *mut *mut c_void,
    )
);
assert_fn!(
  SDL_LogSetOutputFunction:
    unsafe extern "C" fn(
      callback: SDL_LogOutputFunction,
      userdata: *mut c_void,
    )
);
assert_fn!(
  SDL_ShowMessageBox:
    unsafe extern "C" fn(
      messageboxdata: *const SDL_MessageBoxData,
      buttonid: *mut c_int,
    ) -> c_int
);
assert_fn!(
  SDL_ShowSimpleMessageBox:
    unsafe extern "C" fn(
      flags: Uint32,
      title: *const c_char,
      message: *const c_char,
      window: *mut SDL_Window,
    ) -> c_int
);
assert_fn!(
  SDL_Metal_CreateView:
    unsafe extern "C" fn(window: *mut SDL_Window) -> SDL_MetalView
);
assert_fn!(SDL_Metal_DestroyView: unsafe extern "C" fn(view: SDL_MetalView));
assert_fn!(
  SDL_GetPowerInfo:
    unsafe extern "C" fn(secs: *mut c_int, pct: *mut c_int) -> SDL_PowerState
);
assert_fn!(SDL_GetNumRenderDrivers: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_GetRenderDriverInfo:
    unsafe extern "C" fn(index: c_int, info: *mut SDL_RendererInfo) -> c_int
);
assert_fn!(
  SDL_CreateWindowAndRenderer:
    unsafe extern "C" fn(
      width: c_int,
      height: c_int,
      window_flags: Uint32,
      window: *mut *mut SDL_Window,
      renderer: *mut *mut SDL_Renderer,
    ) -> c_int
);
assert_fn!(
  SDL_CreateRenderer:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      index: c_int,
      flags: Uint32,
    ) -> *mut SDL_Renderer
);
assert_fn!(
  SDL_CreateSoftwareRenderer:
    unsafe extern "C" fn(surface: *mut SDL_Surface) -> *mut SDL_Renderer
);
assert_fn!(
  SDL_GetRenderer:
    unsafe extern "C" fn(window: *mut SDL_Window) -> *mut SDL_Renderer
);
assert_fn!(
  SDL_GetRendererInfo:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      info: *mut SDL_RendererInfo,
    ) -> c_int
);
assert_fn!(
  SDL_GetRendererOutputSize:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      w: *mut c_int,
      h: *mut c_int,
    ) -> c_int
);
assert_fn!(
  SDL_CreateTexture:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      format: Uint32,
      access: c_int,
      w: c_int,
      h: c_int,
    ) -> *mut SDL_Texture
);
assert_fn!(
  SDL_CreateTextureFromSurface:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      surface: *mut SDL_Surface,
    ) -> *mut SDL_Texture
);
assert_fn!(
  SDL_QueryTexture:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      format: *mut Uint32,
      access: *mut c_int,
      w: *mut c_int,
      h: *mut c_int,
    ) -> c_int
);
assert_fn!(
  SDL_SetTextureColorMod:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      r: Uint8,
      g: Uint8,
      b: Uint8,
    ) -> c_int
);
assert_fn!(
  SDL_GetTextureColorMod:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      r: *mut Uint8,
      g: *mut Uint8,
      b: *mut Uint8,
    ) -> c_int
);
assert_fn!(
  SDL_SetTextureAlphaMod:
    unsafe extern "C" fn(texture: *mut SDL_Texture, alpha: Uint8) -> c_int
);
assert_fn!(
  SDL_GetTextureAlphaMod:
    unsafe extern "C" fn(texture: *mut SDL_Texture, alpha: *mut Uint8) -> c_int
);
assert_fn!(
  SDL_SetTextureBlendMode:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      blendMode: SDL_BlendMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetTextureBlendMode:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      blendMode: *mut SDL_BlendMode,
    ) -> c_int
);
assert_fn!(
  SDL_SetTextureScaleMode:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      scaleMode: SDL_ScaleMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetTextureScaleMode:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      scaleMode: *mut SDL_ScaleMode,
    ) -> c_int
);
assert_fn!(
  SDL_UpdateTexture:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      rect: *const SDL_Rect,
      pixels: *const c_void,
      pitch: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_UpdateYUVTexture:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      rect: *const SDL_Rect,
      Yplane: *const Uint8,
      Ypitch: c_int,
      Uplane: *const Uint8,
      Upitch: c_int,
      Vplane: *const Uint8,
      Vpitch: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_LockTexture:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      rect: *const SDL_Rect,
      pixels: *mut *mut c_void,
      pitch: *mut c_int,
    ) -> c_int
);
assert_fn!(
  SDL_LockTextureToSurface:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      rect: *const SDL_Rect,
      surface: *mut *mut SDL_Surface,
    ) -> c_int
);
assert_fn!(SDL_UnlockTexture: unsafe extern "C" fn(texture: *mut SDL_Texture));
assert_fn!(
  SDL_RenderTargetSupported:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> SDL_bool
);
assert_fn!(
  SDL_SetRenderTarget:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      texture: *mut SDL_Texture,
    ) -> c_int
);
assert_fn!(
  SDL_GetRenderTarget:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> *mut SDL_Texture
);
assert_fn!(
  SDL_RenderSetLogicalSize:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      w: c_int,
      h: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderGetLogicalSize:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      w: *mut c_int,
      h: *mut c_int,
    )
);
assert_fn!(
  SDL_RenderSetIntegerScale:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      enable: SDL_bool,
    ) -> c_int
);
assert_fn!(
  SDL_RenderGetIntegerScale:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> SDL_bool
);
assert_fn!(
  SDL_RenderSetViewport:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderGetViewport:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect)
);
assert_fn!(
  SDL_RenderSetClipRect:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderGetClipRect:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer, rect: *mut SDL_Rect)
);
assert_fn!(
  SDL_RenderIsClipEnabled:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> SDL_bool
);
assert_fn!(
  SDL_RenderSetScale:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      scaleX: f32,
      scaleY: f32,
    ) -> c_int
);
assert_fn!(
  SDL_RenderGetScale:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      scaleX: *mut f32,
      scaleY: *mut f32,
    )
);
assert_fn!(
  SDL_SetRenderDrawColor:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      r: Uint8,
      g: Uint8,
      b: Uint8,
      a: Uint8,
    ) -> c_int
);
assert_fn!(
  SDL_GetRenderDrawColor:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      r: *mut Uint8,
      g: *mut Uint8,
      b: *mut Uint8,
      a: *mut Uint8,
    ) -> c_int
);
assert_fn!(
  SDL_SetRenderDrawBlendMode:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      blendMode: SDL_BlendMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetRenderDrawBlendMode:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      blendMode: *mut SDL_BlendMode,
    ) -> c_int
);
assert_fn!(
  SDL_RenderClear: unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> c_int
);
assert_fn!(
  SDL_RenderDrawPoint:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      x: c_int,
      y: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawPoints:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      points: *const SDL_Point,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawLine:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      x1: c_int,
      y1: c_int,
      x2: c_int,
      y2: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawLines:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      points: *const SDL_Point,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawRect:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawRects:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rects: *const SDL_Rect,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderFillRect:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderFillRects:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rects: *const SDL_Rect,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderCopy:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      texture: *mut SDL_Texture,
      srcrect: *const SDL_Rect,
      dstrect: *const SDL_Rect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderCopyEx:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      texture: *mut SDL_Texture,
      srcrect: *const SDL_Rect,
      dstrect: *const SDL_Rect,
      angle: f64,
      center: *const SDL_Point,
      flip: SDL_RendererFlip,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawPointF:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer, x: f32, y: f32) -> c_int
);
assert_fn!(
  SDL_RenderDrawPointsF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      points: *const SDL_FPoint,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawLineF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      x1: f32,
      y1: f32,
      x2: f32,
      y2: f32,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawLinesF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      points: *const SDL_FPoint,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawRectF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_FRect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderDrawRectsF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rects: *const SDL_FRect,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderFillRectF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_FRect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderFillRectsF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rects: *const SDL_FRect,
      count: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderCopyF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      texture: *mut SDL_Texture,
      srcrect: *const SDL_Rect,
      dstrect: *const SDL_FRect,
    ) -> c_int
);
assert_fn!(
  SDL_RenderCopyExF:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      texture: *mut SDL_Texture,
      srcrect: *const SDL_Rect,
      dstrect: *const SDL_FRect,
      angle: f64,
      center: *const SDL_FPoint,
      flip: SDL_RendererFlip,
    ) -> c_int
);
assert_fn!(
  SDL_RenderReadPixels:
    unsafe extern "C" fn(
      renderer: *mut SDL_Renderer,
      rect: *const SDL_Rect,
      format: Uint32,
      pixels: *mut c_void,
      pitch: c_int,
    ) -> c_int
);
assert_fn!(
  SDL_RenderPresent: unsafe extern "C" fn(renderer: *mut SDL_Renderer)
);
assert_fn!(SDL_DestroyTexture: unsafe extern "C" fn(texture: *mut SDL_Texture));
assert_fn!(
  SDL_DestroyRenderer: unsafe extern "C" fn(renderer: *mut SDL_Renderer)
);
assert_fn!(
  SDL_RenderFlush: unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> c_int
);
assert_fn!(
  SDL_GL_BindTexture:
    unsafe extern "C" fn(
      texture: *mut SDL_Texture,
      texw: *mut f32,
      texh: *mut f32,
    ) -> c_int
);
assert_fn!(
  SDL_GL_UnbindTexture:
    unsafe extern "C" fn(texture: *mut SDL_Texture) -> c_int
);
assert_fn!(
  SDL_RenderGetMetalLayer:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> *mut c_void
);
assert_fn!(
  SDL_RenderGetMetalCommandEncoder:
    unsafe extern "C" fn(renderer: *mut SDL_Renderer) -> *mut c_void
);
assert_fn!(SDL_NumSensors: unsafe extern "C" fn() -> c_int);
assert_fn!(
  SDL_SensorGetDeviceName:
    unsafe extern "C" fn(device_index: c_int) -> *const c_char
);
assert_fn!(
  SDL_SensorGetDeviceType:
    unsafe extern "C" fn(device_index: c_int) -> SDL_SensorType
);
assert_fn!(
  SDL_SensorGetDeviceNonPortableType:
    unsafe extern "C" fn(device_index: c_int) -> c_int
);
assert_fn!(
  SDL_SensorGetDeviceInstanceID:
    unsafe extern "C" fn(device_index: c_int) -> SDL_SensorID
);
assert_fn!(
  SDL_SensorOpen: unsafe extern "C" fn(device_index: c_int) -> *mut SDL_Sensor
);
assert_fn!(
  SDL_SensorFromInstanceID:
    unsafe extern "C" fn(instance_id: SDL_SensorID) -> *mut SDL_Sensor
);
assert_fn!(
  SDL_SensorGetName:
    unsafe extern "C" fn(sensor: *mut SDL_Sensor) -> *const c_char
);
assert_fn!(
  SDL_SensorGetType:
    unsafe extern "C" fn(sensor: *mut SDL_Sensor) -> SDL_SensorType
);
assert_fn!(
  SDL_SensorGetNonPortableType:
    unsafe extern "C" fn(sensor: *mut SDL_Sensor) -> c_int
);
assert_fn!(
  SDL_SensorGetInstanceID:
    unsafe extern "C" fn(sensor: *mut SDL_Sensor) -> SDL_SensorID
);
assert_fn!(
  SDL_SensorGetData:
    unsafe extern "C" fn(
      sensor: *mut SDL_Sensor,
      data: *mut f32,
      num_values: c_int,
    ) -> c_int
);
assert_fn!(SDL_SensorClose: unsafe extern "C" fn(sensor: *mut SDL_Sensor));
assert_fn!(SDL_SensorUpdate: unsafe extern "C" fn());
assert_fn!(
  SDL_CreateShapedWindow:
    unsafe extern "C" fn(
      title: *const c_char,
      x: c_uint,
      y: c_uint,
      w: c_uint,
      h: c_uint,
      flags: Uint32,
    ) -> *mut SDL_Window
);
assert_fn!(
  SDL_IsShapedWindow:
    unsafe extern "C" fn(window: *const SDL_Window) -> SDL_bool
);
assert_fn!(
  SDL_SetWindowShape:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      shape: *mut SDL_Surface,
      shape_mode: *mut SDL_WindowShapeMode,
    ) -> c_int
);
assert_fn!(
  SDL_GetShapedWindowMode:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      shape_mode: *mut SDL_WindowShapeMode,
    ) -> c_int
);
assert_fn!(SDL_IsTablet: unsafe extern "C" fn() -> SDL_bool);
assert_fn!(SDL_OnApplicationWillTerminate: unsafe extern "C" fn());
assert_fn!(SDL_OnApplicationDidReceiveMemoryWarning: unsafe extern "C" fn());
assert_fn!(SDL_OnApplicationWillResignActive: unsafe extern "C" fn());
assert_fn!(SDL_OnApplicationDidEnterBackground: unsafe extern "C" fn());
assert_fn!(SDL_OnApplicationWillEnterForeground: unsafe extern "C" fn());
assert_fn!(SDL_OnApplicationDidBecomeActive: unsafe extern "C" fn());
assert_fn!(SDL_GetTicks: unsafe extern "C" fn() -> Uint32);
assert_fn!(SDL_GetPerformanceCounter: unsafe extern "C" fn() -> Uint64);
assert_fn!(SDL_GetPerformanceFrequency: unsafe extern "C" fn() -> Uint64);
assert_fn!(SDL_Delay: unsafe extern "C" fn(ms: Uint32));
assert_fn!(
  SDL_AddTimer:
    unsafe extern "C" fn(
      interval: Uint32,
      callback: SDL_TimerCallback,
      param: *mut c_void,
    ) -> SDL_TimerID
);
assert_fn!(SDL_RemoveTimer: unsafe extern "C" fn(id: SDL_TimerID) -> SDL_bool);
assert_fn!(SDL_GetVersion: unsafe extern "C" fn(ver: *mut SDL_version));
assert_fn!(SDL_GetRevision: unsafe extern "C" fn() -> *const c_char);
assert_fn!(SDL_GetRevisionNumber: unsafe extern "C" fn() -> c_int);
assert_fn!(SDL_Init: unsafe extern "C" fn(flags: Uint32) -> c_int);
assert_fn!(SDL_InitSubSystem: unsafe extern "C" fn(flags: Uint32) -> c_int);
assert_fn!(SDL_QuitSubSystem: unsafe extern "C" fn(flags: Uint32));
assert_fn!(SDL_WasInit: unsafe extern "C" fn(flags: Uint32) -> Uint32);
assert_fn!(SDL_Quit: unsafe extern "C" fn());
assert_fn!(
  SDL_Vulkan_LoadLibrary: unsafe extern "C" fn(path: *const c_char) -> c_int
);
assert_fn!(
  SDL_Vulkan_GetVkGetInstanceProcAddr: unsafe extern "C" fn() -> *mut c_void
);
assert_fn!(SDL_Vulkan_UnloadLibrary: unsafe extern "C" fn());
assert_fn!(
  SDL_Vulkan_GetInstanceExtensions:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      pCount: *mut c_uint,
      pNames: *mut *const c_char,
    ) -> SDL_bool
);
assert_fn!(
  SDL_Vulkan_CreateSurface:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      instance: VkInstance,
      surface: *mut VkSurfaceKHR,
    ) -> SDL_bool
);
assert_fn!(
  SDL_Vulkan_GetDrawableSize:
    unsafe extern "C" fn(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int)
);
assert_fn!(
  SDL_GetWindowWMInfo:
    unsafe extern "C" fn(
      window: *mut SDL_Window,
      info: *mut SDL_SysWMinfo,
    ) -> SDL_bool
);

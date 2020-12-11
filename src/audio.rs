//! Audio playback.

use crate::{c_char, c_int, c_void, rwops::*, stdinc::*};

/// Audio format flags.
///
/// Use the appropriate const functions to query the bits.
///
/// ```txt
/// ++-----------------------sample is signed if set
/// ||
/// ||       ++----------sample is big-endian if set
/// ||       ||
/// ||       ||          ++---sample is float if set
/// ||       ||          ||
/// ||       ||          || +---sample bit size---+
/// ||       ||          || |                     |
/// 15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00
/// ```
/// (Unspecified bits are always zero.)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_AudioFormat(pub u16);

/// Pits per sample. eg: i16 = 16, f32 = 32.
pub const fn SDL_AUDIO_BITSIZE(af: SDL_AudioFormat) -> u16 {
  af.0 & 0xFF
}
/// If the sample type is a floating type.
pub const fn SDL_AUDIO_ISFLOAT(af: SDL_AudioFormat) -> bool {
  af.0 & (1 << 8) != 0
}
/// If the samples are big-endian.
pub const fn SDL_AUDIO_ISBIGENDIAN(af: SDL_AudioFormat) -> bool {
  af.0 & (1 << 12) != 0
}
/// If the samples are signed values.
pub const fn SDL_AUDIO_ISSIGNED(af: SDL_AudioFormat) -> bool {
  af.0 & (1 << 15) != 0
}
/// If the samples are an int type.
pub const fn SDL_AUDIO_ISINT(af: SDL_AudioFormat) -> bool {
  !SDL_AUDIO_ISFLOAT(af)
}
/// If the samples are little-endian.
pub const fn SDL_AUDIO_ISLITTLEENDIAN(af: SDL_AudioFormat) -> bool {
  !SDL_AUDIO_ISBIGENDIAN(af)
}
/// If the samples are unsigned values.
pub const fn SDL_AUDIO_ISUNSIGNED(af: SDL_AudioFormat) -> bool {
  !SDL_AUDIO_ISSIGNED(af)
}
/// Unsigned 8-bit samples
pub const AUDIO_U8: SDL_AudioFormat = SDL_AudioFormat(0x0008);
/// Signed 8-bit samples
pub const AUDIO_S8: SDL_AudioFormat = SDL_AudioFormat(0x8008);
/// Unsigned 16-bit samples
pub const AUDIO_U16LSB: SDL_AudioFormat = SDL_AudioFormat(0x0010);
/// Signed 16-bit samples
pub const AUDIO_S16LSB: SDL_AudioFormat = SDL_AudioFormat(0x8010);
/// As [`AUDIO_S16LSB`], but big-endian byte order
pub const AUDIO_U16MSB: SDL_AudioFormat = SDL_AudioFormat(0x1010);
/// As [`AUDIO_U16MSB`], but big-endian byte order
pub const AUDIO_S16MSB: SDL_AudioFormat = SDL_AudioFormat(0x9010);
/// Alias for [`AUDIO_U16LSB`]
pub const AUDIO_U16: SDL_AudioFormat = AUDIO_U16LSB;
/// Alias for [`AUDIO_S16LSB`]
pub const AUDIO_S16: SDL_AudioFormat = AUDIO_S16LSB;
/// 32-bit integer samples
pub const AUDIO_S32LSB: SDL_AudioFormat = SDL_AudioFormat(0x8020);
/// As [`AUDIO_S32MSB`], but big-endian byte order
pub const AUDIO_S32MSB: SDL_AudioFormat = SDL_AudioFormat(0x9020);
/// Alias for [`AUDIO_S32LSB`];
pub const AUDIO_S32: SDL_AudioFormat = AUDIO_S32LSB;
/// 32-bit floating point samples
pub const AUDIO_F32LSB: SDL_AudioFormat = SDL_AudioFormat(0x8120);
/// As [`AUDIO_F32LSB`], but big-endian byte order
pub const AUDIO_F32MSB: SDL_AudioFormat = SDL_AudioFormat(0x9120);
/// Alias for [`AUDIO_F32LSB`]
pub const AUDIO_F32: SDL_AudioFormat = AUDIO_F32LSB;
/// Native-endian u16 samples
pub const AUDIO_U16SYS: SDL_AudioFormat =
  if cfg!(target_endian = "little") { AUDIO_U16LSB } else { AUDIO_U16MSB };
/// Native-endian i16 samples
pub const AUDIO_S16SYS: SDL_AudioFormat =
  if cfg!(target_endian = "little") { AUDIO_S16LSB } else { AUDIO_S16MSB };
/// Native-endian i32 samples
pub const AUDIO_S32SYS: SDL_AudioFormat =
  if cfg!(target_endian = "little") { AUDIO_S32LSB } else { AUDIO_S32MSB };
/// Native-endian f32 samples
pub const AUDIO_F32SYS: SDL_AudioFormat =
  if cfg!(target_endian = "little") { AUDIO_F32LSB } else { AUDIO_F32MSB };

/// This function is called when the audio device needs more data.
///
/// * `userdata` An application-specific parameter saved in the `SDL_AudioSpec`
///   structure
/// * `stream` A pointer to the audio data buffer.
/// * `len`    The length of that buffer in bytes.
///
/// The buffer is uninitialized when the callback is called.
///
/// The callback *must* initialize the entire buffer.
///
/// Once the callback returns, the buffer will no longer be valid.
///
/// See [`SDL_AudioSpec`] for default sample ordering.
///
/// You can choose to avoid callbacks and use SDL_QueueAudio() instead, if you
/// like. Just open your audio device with a NULL callback.
pub type SDL_AudioCallback = Option<
  unsafe extern "C" fn(userdata: *mut c_void, stream: *mut Uint8, len: c_int),
>;

/// The calculated values in this structure are calculated by SDL_OpenAudio().
///
/// For multi-channel audio, the default SDL channel mapping is:
/// ```txt
/// 2:  FL FR                       (stereo)
/// 3:  FL FR LFE                   (2.1 surround)
/// 4:  FL FR BL BR                 (quad)
/// 5:  FL FR FC BL BR              (quad + center)
/// 6:  FL FR FC LFE SL SR          (5.1 surround - last two can also be BL BR)
/// 7:  FL FR FC LFE BC SL SR       (6.1 surround)
/// 8:  FL FR FC LFE BL BR SL SR    (7.1 surround)
/// ```
#[repr(C)]
pub struct SDL_AudioSpec {
  /// DSP frequency -- samples per second
  pub freq: c_int,
  /// Audio data format
  pub format: SDL_AudioFormat,
  /// Number of channels: 1 mono, 2 stereo, etc
  pub channels: Uint8,
  /// Audio buffer silence value (calculated)
  pub silence: Uint8,
  /// Audio buffer size in sample FRAMES (total samples divided by channel
  /// count)
  pub samples: Uint16,
  /// Necessary for some compile environments
  pub padding: Uint16,
  /// Audio buffer size in bytes (calculated)
  pub size: Uint32,
  /// Callback that feeds the audio device (NULL to use SDL_QueueAudio()).
  pub callback: SDL_AudioCallback,
  /// Userdata passed to callback (ignored for NULL callbacks).
  pub userdata: *mut c_void,
}

/// Filter function used by [`SDL_AudioCVT`]
pub type SDL_AudioFilter =
  Option<unsafe extern "C" fn(cvt: *mut SDL_AudioCVT, format: SDL_AudioFormat)>;

/// A structure to hold a set of audio conversion filters and buffers.
///
/// Note that various parts of the conversion pipeline can take advantage
/// of SIMD operations (like SSE2, for example). SDL_AudioCVT doesn't require
/// you to pass it aligned data, but can possibly run much faster if you
/// set both its (buf) field to a pointer that is aligned to 16 bytes, and its
/// (len) field to something that's a multiple of 16, if possible.
#[repr(C)]
pub struct SDL_AudioCVT {
  /// Set to 1 if conversion possible
  pub needed: c_int,
  /// Source audio format
  pub src_format: SDL_AudioFormat,
  /// Target audio format
  pub dst_format: SDL_AudioFormat,
  /// Rate conversion increment
  pub rate_incr: f64,
  /// Buffer to hold entire audio data
  pub buf: *mut Uint8,
  /// Length of original audio buffer
  pub len: c_int,
  /// Length of converted audio buffer
  pub len_cvt: c_int,
  /// buffer must be len*len_mult big
  pub len_mult: c_int,
  /// Given len, final size is len*len_ratio
  pub len_ratio: f64,
  /// NULL-terminated list of filter functions
  pub filters: [SDL_AudioFilter; 10],
  /// Current audio conversion function
  pub filter_index: c_int,
}

/// You can only use 9 slots in the `SDL_AudioCVT.filters` field. The 10th is
/// reserved as a null terminator.
pub const SDL_AUDIOCVT_MAX_FILTERS: u32 = 9;

/// Identifies an audio device.
///
/// A successful call to [`SDL_OpenAudio`] is always device id 1, and legacy SDL
/// audio APIs assume you want this device ID. [`SDL_OpenAudioDevice`] calls
/// always returns devices >= 2 on success. The legacy calls are good both for
/// backwards compatibility and when you don't care about multiple devices,
/// specific devices, or capture devices.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_AudioDeviceID(pub u32);

/// The status of an audio device.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SDL_AudioStatus(pub u32);
#[allow(missing_docs)]
pub const SDL_AUDIO_STOPPED: SDL_AudioStatus = SDL_AudioStatus(0);
#[allow(missing_docs)]
pub const SDL_AUDIO_PLAYING: SDL_AudioStatus = SDL_AudioStatus(1);
#[allow(missing_docs)]
pub const SDL_AUDIO_PAUSED: SDL_AudioStatus = SDL_AudioStatus(2);

/// SDL_AudioStream is a new audio conversion interface.
///
/// The benefits vs SDL_AudioCVT:
/// * it can handle re-sampling data in chunks without generating artifacts,
///   when it doesn't have the complete buffer available.
/// * it can handle incoming data in any variable size.
/// * You push data as you have it, and pull it when you need it.
#[allow(unused)]
#[repr(transparent)]
pub struct SDL_AudioStream(c_void);

/// Maximum volume value, for use with [`SDL_MixAudio`].
pub const SDL_MIX_MAXVOLUME: c_int = 128;

extern "C" {
  /// The number of audio drivers. See [`SDL_GetAudioDriver`]
  pub fn SDL_GetNumAudioDrivers() -> c_int;

  /// The name of an audio driver. See [`SDL_GetAudioDriver`]
  pub fn SDL_GetAudioDriver(index: c_int) -> *const c_char;

  // skipped because they're mostly internal use: SDL_AudioInit / SDL_AudioQuit

  /// The name of the current audio driver (or null if no driver is
  /// initialized).
  pub fn SDL_GetCurrentAudioDriver() -> *const c_char;

  /// This function opens the audio device with the desired parameters, and
  /// returns 0 if successful, placing the actual hardware parameters in the
  /// structure pointed to by `obtained`. If `obtained` is NULL, the audio
  /// data passed to the callback function will be guaranteed to be in the
  /// requested format, and will be automatically converted to the hardware
  /// audio format if necessary.  This function returns -1 if it failed
  /// to open the audio device, or couldn't set up the audio thread.
  ///
  /// When filling in the desired audio spec structure,
  /// - `desired->freq` should be the desired audio frequency in samples-per-
  ///   second.
  /// - `desired->format` should be the desired audio format.
  /// - `desired->samples` is the desired size of the audio buffer, in samples.
  ///   This number should be a power of two, and may be adjusted by the audio
  ///   driver to a value more suitable for the hardware.  Good values seem to
  ///   range between 512 and 8096 inclusive, depending on the application and
  ///   CPU speed.  Smaller values yield faster response time, but can lead to
  ///   underflow if the application is doing heavy processing and cannot fill
  ///   the audio buffer in time.  A stereo sample consists of both right and
  ///   left channels in LR ordering. Note that the number of samples is
  ///   directly related to time by the following formula: `ms =
  ///   (samples*1000)/freq`
  /// - `desired->size` is the size in bytes of the audio buffer, and is
  ///   calculated by SDL_OpenAudio().
  /// - `desired->silence` is the value used to set the buffer to silence, and
  ///   is calculated by SDL_OpenAudio().
  /// - `desired->callback` should be set to a function that will be called when
  ///   the audio device is ready for more data.  It is passed a pointer to the
  ///   audio buffer, and the length in bytes of the audio buffer. This function
  ///   usually runs in a separate thread, and so you should protect data
  ///   structures that it accesses by calling SDL_LockAudio() and
  ///   SDL_UnlockAudio() in your code. Alternately, you may pass a NULL pointer
  ///   here, and call SDL_QueueAudio() with some frequency, to queue more audio
  ///   samples to be played (or for capture devices, call SDL_DequeueAudio()
  ///   with some frequency, to obtain audio samples).
  /// - `desired->userdata` is passed as the first parameter to your callback
  ///   function. If you passed a NULL callback, this value is ignored.
  ///
  /// The audio device starts out playing silence when it's opened, and should
  /// be enabled for playing by calling [`SDL_PauseAudio(0)`](SDL_PauseAudio)
  /// when you are ready  for your audio callback function to be called.
  /// Since the audio driver  may modify the requested size of the audio
  /// buffer, you should allocate  any local mixing buffers after you open the
  /// audio device.
  pub fn SDL_OpenAudio(
    desired: *mut SDL_AudioSpec, obtained: *mut SDL_AudioSpec,
  ) -> c_int;

  /// Get the number of available devices exposed by the current driver.
  ///
  /// Only valid after a successfully initializing the audio subsystem.
  /// Returns -1 if an explicit list of devices can't be determined; this is
  /// not an error. For example, if SDL is set up to talk to a remote audio
  /// server, it can't list every one available on the Internet, but it will
  /// still allow a specific host to be specified to [`SDL_OpenAudioDevice`].
  ///
  /// In many common cases, when this function returns a value <= 0, it can
  /// still successfully open the default device (NULL for first argument of
  /// [`SDL_OpenAudioDevice`]).
  pub fn SDL_GetNumAudioDevices(iscapture: c_int) -> c_int;

  /// Get the human-readable name of a specific audio device.
  ///
  /// Must be a value between 0 and (number of audio devices-1).
  /// Only valid after a successfully initializing the audio subsystem.
  /// The values returned by this function reflect the latest call to
  /// [`SDL_GetNumAudioDevices`]; Call it again to re-detect available
  /// hardware.
  ///
  /// The string returned by this function is UTF-8 encoded, read-only, and
  /// managed internally. You are not to free it. If you need to keep the
  /// string for any length of time, you should make your own copy of it, as it
  /// will be invalid next time any of several other SDL functions is called.
  pub fn SDL_GetAudioDeviceName(
    index: c_int, iscapture: c_int,
  ) -> *const c_char;

  /// Open a specific audio device.
  ///
  /// Passing in a device name of NULL requests
  /// the most reasonable default (and is equivalent to calling
  /// [`SDL_OpenAudio`]).
  ///
  /// The device name is a UTF-8 string reported by SDL_GetAudioDeviceName(),
  /// but some drivers allow arbitrary and driver-specific strings, such as a
  /// hostname/IP address for a remote audio server, or a filename in the
  /// disk audio driver.
  ///
  /// **Returns:** 0 on error, a valid device ID that is >= 2 on success.
  ///
  /// [`SDL_OpenAudio`], unlike this function, always acts on device ID 1.
  pub fn SDL_OpenAudioDevice(
    device: *const c_char, iscapture: c_int, desired: *const SDL_AudioSpec,
    obtained: *mut SDL_AudioSpec, allowed_changes: c_int,
  ) -> SDL_AudioDeviceID;

  /// Gets the current audio status (stopped / playing / paused).
  pub fn SDL_GetAudioStatus() -> SDL_AudioStatus;

  /// Gets the current audio status (stopped / playing / paused).
  pub fn SDL_GetAudioDeviceStatus(dev: SDL_AudioDeviceID) -> SDL_AudioStatus;

  /// Pause and unpause the audio callback processing.
  ///
  /// Should be called with a parameter of 0 after opening the audio
  /// device to start playing sound.  This is so you can safely initialize
  /// data for your callback function after opening the audio device.
  /// Silence will be written to the audio device during the pause.
  pub fn SDL_PauseAudio(pause_on: c_int);

  /// Pause and unpause the audio callback processing.
  ///
  /// Should be called with a parameter of 0 after opening the audio
  /// device to start playing sound.  This is so you can safely initialize
  /// data for your callback function after opening the audio device.
  /// Silence will be written to the audio device during the pause.
  pub fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: c_int);

  /// Load the audio data of a WAVE file into memory
  ///
  /// Loading a WAVE file requires `src`, `spec`, `audio_buf`, and `audio_len`
  /// to be valid pointers. The entire data portion of the file is then loaded
  /// into memory and decoded if necessary.
  ///
  /// If `freesrc` is non-zero, the data source gets automatically closed and
  /// freed before the function returns.
  ///
  /// Supported are RIFF WAVE files with the formats PCM (8, 16, 24, and 32
  /// bits), IEEE Float (32 bits), Microsoft ADPCM and IMA ADPCM (4 bits), and
  /// A-law and µ-law (8 bits). Other formats are currently unsupported and
  /// cause an error.
  ///
  /// If this function succeeds, the pointer returned by it is equal to `spec`
  /// and the pointer to the audio data allocated by the function is written to
  /// `audio_buf` and its length in bytes to `audio_len`. The [`SDL_AudioSpec`]
  /// members `freq`, `channels`, and `format` are set to the values of the
  /// audio data in the buffer. The `samples` member is set to a sane default
  /// and all others are set to zero.
  ///
  /// It's necessary to use [`SDL_FreeWAV`] to free the audio data returned in
  /// `audio_buf` when it is no longer used.
  ///
  /// Because of the under-specification of the Waveform format, there are many
  /// problematic files in the wild that cause issues with strict decoders. To
  /// provide compatibility with these files, this decoder is lenient in regards
  /// to the truncation of the file, the fact chunk, and the size of the RIFF
  /// chunk. The hints SDL_HINT_WAVE_RIFF_CHUNK_SIZE, SDL_HINT_WAVE_TRUNCATION,
  /// and SDL_HINT_WAVE_FACT_CHUNK can be used to tune the behavior of the
  /// loading process.
  ///
  /// Any file that is invalid (due to truncation, corruption, or wrong values
  /// in the headers), too big, or unsupported causes an error. Additionally,
  /// any critical I/O error from the data source will terminate the loading
  /// process with an error. The function returns NULL on error and in all
  /// cases (with the exception of `src` being NULL), an appropriate error
  /// message will be set.
  ///
  /// It is required that the data source supports seeking.
  ///
  /// Example:
  /// ```txt
  /// SDL_LoadWAV_RW(SDL_RWFromFile("sample.wav", "rb"), 1, ...);
  /// ```
  ///
  /// * `src` The data source with the WAVE data
  /// * `freesrc` A integer value that makes the function close the data source
  ///   if non-zero
  /// * `spec` A pointer filled with the audio format of the audio data
  /// * `audio_buf` A pointer filled with the audio data allocated by the
  ///   function
  /// * `audio_len` A pointer filled with the length of the audio data buffer in
  ///   bytes
  ///
  /// **Return:** NULL on error, or non-NULL on success.
  pub fn SDL_LoadWAV_RW(
    src: *mut SDL_RWops, freesrc: c_int, spec: *mut SDL_AudioSpec,
    audio_buf: *mut *mut Uint8, audio_len: *mut Uint32,
  ) -> *mut SDL_AudioSpec;

  /// This function frees data previously allocated with [`SDL_LoadWAV_RW`]
  pub fn SDL_FreeWAV(audio_buf: *mut Uint8);

  /// Initializes an [`SDL_AudioCVT`].
  ///
  /// This function takes a source format and rate and a destination format
  /// and rate, and initializes the `cvt` structure with information needed
  /// by [`SDL_ConvertAudio`] to convert a buffer of audio data from one format
  /// to the other. An unsupported format causes an error and -1 will be
  /// returned.
  ///
  /// **Return:** 0 if no conversion is needed, 1 if the audio filter is set up,
  /// or -1 on error.
  pub fn SDL_BuildAudioCVT(
    cvt: *mut SDL_AudioCVT, src_format: SDL_AudioFormat, src_channels: Uint8,
    src_rate: c_int, dst_format: SDL_AudioFormat, dst_channels: Uint8,
    dst_rate: c_int,
  ) -> c_int;

  /// Performs an in-place audio conversion.
  ///
  /// Once you have initialized the `cvt` structure using [`SDL_BuildAudioCVT`],
  /// created an audio buffer `cvt->buf`, and filled it with `cvt->len` bytes of
  /// audio data in the source format, this function will convert it in-place
  /// to the desired format.
  ///
  /// The data conversion may expand the size of the audio data, so the buffer
  /// `cvt->buf` should be allocated after the `cvt` structure is initialized by
  /// [`SDL_BuildAudioCVT`], and should be `cvt->len * cvt->len_mult` bytes
  /// long.
  ///
  /// **Return:** 0 on success, or -1 if `cvt->buf` is NULL.
  pub fn SDL_ConvertAudio(cvt: *mut SDL_AudioCVT) -> c_int;

  /// Create a new audio stream
  ///
  /// * `src_format` The format of the source audio
  /// * `src_channels` The number of channels of the source audio
  /// * `src_rate` The sampling rate of the source audio
  /// * `dst_format` The format of the desired audio output
  /// * `dst_channels` The number of channels of the desired audio output
  /// * `dst_rate` The sampling rate of the desired audio output
  ///
  /// **Return:** 0 on success, or -1 on error.
  pub fn SDL_NewAudioStream(
    src_format: SDL_AudioFormat, src_channels: Uint8, src_rate: c_int,
    dst_format: SDL_AudioFormat, dst_channels: Uint8, dst_rate: c_int,
  ) -> *mut SDL_AudioStream;

  /// Add data to be converted/re-sampled to the stream
  ///
  /// * `stream` The stream the audio data is being added to
  /// * `buf` A pointer to the audio data to add
  /// * `len` The number of bytes to write to the stream
  ///
  /// **Return:** 0 on success, or -1 on error.
  pub fn SDL_AudioStreamPut(
    stream: *mut SDL_AudioStream, buf: *const c_void, len: c_int,
  ) -> c_int;

  /// Get converted/re-sampled data from the stream
  ///
  /// * `stream` The stream the audio is being requested from
  /// * `buf` A buffer to fill with audio data
  /// * `len` The maximum number of bytes to fill
  ///
  /// **Return:** The number of bytes read from the stream, or -1 on error
  pub fn SDL_AudioStreamGet(
    stream: *mut SDL_AudioStream, buf: *mut c_void, len: c_int,
  ) -> c_int;

  /// Get the number of converted/re-sampled bytes available.
  ///
  /// The stream may be buffering data behind the scenes until it has enough to
  /// resample correctly, so this number might be lower than what you expect, or
  /// even be zero. Add more data or flush the stream if you need the data now.
  pub fn SDL_AudioStreamAvailable(stream: *mut SDL_AudioStream) -> c_int;

  /// Flush the stream.
  ///
  /// This tell the stream that you're done sending data, and anything being
  /// buffered should be converted/re-sampled and made available immediately.
  ///
  /// It is legal to add more data to a stream after flushing, but there will
  /// be audio gaps in the output. Generally this is intended to signal the
  /// end of input, so the complete output becomes available.
  pub fn SDL_AudioStreamFlush(stream: *mut SDL_AudioStream) -> c_int;

  /// Clear any pending data in the stream without converting it
  pub fn SDL_AudioStreamClear(stream: *mut SDL_AudioStream);

  /// Free an audio stream
  pub fn SDL_FreeAudioStream(stream: *mut SDL_AudioStream);

  /// In-place mixes two audio sources.
  ///
  /// This takes two audio buffers of the playing audio format and mixes
  /// them, performing addition, volume adjustment, and overflow clipping.
  /// The volume ranges from 0 - 128, and should be set to [`SDL_MIX_MAXVOLUME`]
  /// for full audio volume.  Note this does not change hardware volume.
  ///
  /// This is provided for convenience, you can mix your own audio data.
  pub fn SDL_MixAudio(
    dst: *mut Uint8, src: *const Uint8, len: Uint32, volume: c_int,
  );

  /// Mix according to a format.
  ///
  /// This works like [`SDL_MixAudio`], but you specify the audio format instead
  /// of using the format of audio device 1. Thus it can be used when no audio
  /// device is open at all.
  pub fn SDL_MixAudioFormat(
    dst: *mut Uint8, src: *const Uint8, format: SDL_AudioFormat, len: Uint32,
    volume: c_int,
  );

  /// Queue more audio on non-callback devices.
  ///
  /// (If you are looking to retrieve queued audio from a non-callback capture
  /// device, you want [`SDL_DequeueAudio`] instead. This will return -1 to
  /// signify an error if you use it with capture devices.)
  ///
  /// SDL offers two ways to feed audio to the device: you can either supply a
  /// callback that SDL triggers with some frequency to obtain more audio
  /// (pull method), or you can supply no callback, and then SDL will expect
  /// you to supply data at regular intervals (push method) with this function.
  ///
  /// There are no limits on the amount of data you can queue, short of
  /// exhaustion of address space. Queued data will drain to the device as
  /// necessary without further intervention from you. If the device needs
  /// audio but there is not enough queued, it will play silence to make up
  /// the difference. This means you will have skips in your audio playback
  /// if you aren't routinely queueing sufficient data.
  ///
  /// This function copies the supplied data, so you are safe to free it when
  /// the function returns. This function is thread-safe, but queueing to the
  /// same device from two threads at once does not promise which buffer will
  /// be queued first.
  ///
  /// You may not queue audio on a device that is using an application-supplied
  /// callback; doing so returns an error. You have to use the audio callback
  /// or queue audio with this function, but not both.
  ///
  /// You should **not** call [`SDL_LockAudio`] on the device before queueing;
  /// SDL handles locking internally for this function.
  ///
  /// * `dev` The device ID to which we will queue audio.
  /// * `data` The data to queue to the device for later playback.
  /// * `len` The number of bytes (not samples!) to which (data) points.
  ///
  /// **Return:** 0 on success, or -1 on error.
  pub fn SDL_QueueAudio(
    dev: SDL_AudioDeviceID, data: *const c_void, len: Uint32,
  ) -> c_int;

  /// Dequeue more audio on non-callback devices.
  ///
  /// (If you are looking to queue audio for output on a non-callback playback
  /// device, you want [``SDL_QueueAudio`] instead. This will always return 0
  /// if you use it with playback devices.)
  ///
  /// SDL offers two ways to retrieve audio from a capture device: you can
  /// either supply a callback that SDL triggers with some frequency as the
  /// device records more audio data, (push method), or you can supply no
  /// callback, and then SDL will expect you to retrieve data at regular
  /// intervals (pull method) with this function.
  ///
  /// There are no limits on the amount of data you can queue, short of
  /// exhaustion of address space. Data from the device will keep queuing as
  /// necessary without further intervention from you. This means you will
  /// eventually run out of memory if you aren't routinely dequeueing data.
  ///
  /// Capture devices will not queue data when paused; if you are expecting
  /// to not need captured audio for some length of time, use
  /// SDL_PauseAudioDevice() to stop the capture device from queueing more
  /// data. This can be useful during, say, level loading times. When
  /// un-paused, capture devices will start queueing data from that point,
  /// having flushed any capturable data available while paused.
  ///
  /// This function is thread-safe, but dequeueing from the same device from
  /// two threads at once does not promise which thread will dequeued data
  /// first.
  ///
  /// You may not dequeue audio from a device that is using an
  /// application-supplied callback; doing so returns an error. You have to use
  /// the audio callback, or dequeue audio with this function, but not both.
  ///
  /// You should not call SDL_LockAudio() on the device before queueing; SDL
  /// handles locking internally for this function.
  ///
  /// * `dev` The device ID from which we will dequeue audio.
  /// * `data` A pointer into where audio data should be copied.
  /// * `len` The number of bytes (not samples!) to which (data) points.
  ///
  /// **Return:** number of bytes dequeued, which could be less than requested.
  pub fn SDL_DequeueAudio(
    dev: SDL_AudioDeviceID, data: *mut c_void, len: Uint32,
  ) -> Uint32;

  /// Get the number of bytes of still-queued audio.
  ///
  /// * For playback device:
  ///  * This is the number of bytes that have been queued for playback with
  ///    SDL_QueueAudio(), but have not yet been sent to the hardware. This
  ///    number may shrink at any time, so this only informs of pending data.
  ///  * Once we've sent it to the hardware, this function can not decide the
  ///    exact byte boundary of what has been played. It's possible that we just
  ///    gave the hardware several kilobytes right before you called this
  ///    function, but it hasn't played any of it yet, or maybe half of it, etc.
  ///
  /// * For capture devices:
  ///   * This is the number of bytes that have been captured by the device and
  ///     are waiting for you to dequeue. This number may grow at any time, so
  ///     this only informs of the lower-bound of available data.
  ///
  /// You may not queue audio on a device that is using an application-supplied
  /// callback; calling this function on such a device always returns 0. You
  /// have to queue audio with SDL_QueueAudio()/SDL_DequeueAudio(), or use the
  /// audio callback, but not both.
  ///
  /// You should not call SDL_LockAudio() on the device before querying; SDL
  /// handles locking internally for this function.
  ///
  /// * `dev` The device ID of which we will query queued audio size.
  ///
  /// **Return:** Number of bytes (not samples!) of queued audio.
  pub fn SDL_GetQueuedAudioSize(dev: SDL_AudioDeviceID) -> Uint32;

  /// Drop any queued audio data.
  ///
  /// For playback devices, this is any queued data
  /// still waiting to be submitted to the hardware. For capture devices, this
  /// is any data that was queued by the device that hasn't yet been dequeued by
  /// the application.
  ///
  /// Immediately after this call, [`SDL_GetQueuedAudioSize`] will return 0. For
  /// playback devices, the hardware will start playing silence if more audio
  /// isn't queued. Un-paused capture devices will start filling the queue again
  /// as soon as they have more data available (which, depending on the state
  /// of the hardware and the thread, could be before this function call
  /// returns!).
  ///
  /// This will not prevent playback of queued audio that's already been sent
  /// to the hardware, as we can not undo that, so expect there to be some
  /// fraction of a second of audio that might still be heard. This can be
  /// useful if you want to, say, drop any pending music during a level change
  /// in your game.
  ///
  /// You may not queue audio on a device that is using an application-supplied
  /// callback; calling this function on such a device is always a no-op.
  /// You have to queue audio with [`SDL_QueueAudio`]/[`SDL_DequeueAudio`], or
  /// use the audio callback, but not both.
  ///
  /// You should not call SDL_LockAudio() on the device before clearing the
  /// queue; SDL handles locking internally for this function.
  ///
  /// This function always succeeds and thus returns void.
  ///
  /// * `dev` The device ID of which to clear the audio queue.
  pub fn SDL_ClearQueuedAudio(dev: SDL_AudioDeviceID);

  /// Locks the default audio device.
  ///
  /// The lock manipulated by this function protects the audio callback
  /// function. During an [`SDL_LockAudio`]/[`SDL_UnlockAudio`] pair, you can be
  /// guaranteed that the callback function is not running.
  ///
  /// Do not call these from the callback function or you will cause deadlock.
  pub fn SDL_LockAudio();

  /// Locks the given audio device.
  ///
  /// See [`SDL_LockAudio`]
  pub fn SDL_LockAudioDevice(dev: SDL_AudioDeviceID);

  /// Unlocks the default audio device.
  ///
  /// See [`SDL_LockAudio`]
  pub fn SDL_UnlockAudio();

  /// Unlocks the given audio device.
  ///
  /// See [`SDL_LockAudio`]
  pub fn SDL_UnlockAudioDevice(dev: SDL_AudioDeviceID);

  /// This function shuts down audio processing and closes the default audio
  /// device.
  pub fn SDL_CloseAudio();

  /// This function shuts down audio processing and closes the given audio
  /// device.
  pub fn SDL_CloseAudioDevice(dev: SDL_AudioDeviceID);
}

/// Loads a WAV file.
///
/// Works like [`SDL_LoadWAV_RW`], but automatically creates and then frees the
/// intermediate [`SDL_RWops`] for you.
pub unsafe fn SDL_LoadWAV(
  file: *const c_char, spec: *mut SDL_AudioSpec, audio_buf: *mut *mut Uint8,
  audio_len: *mut Uint32,
) -> *mut SDL_AudioSpec {
  SDL_LoadWAV_RW(
    SDL_RWFromFile(file, b"rb\0".as_ptr().cast()),
    1,
    spec,
    audio_buf,
    audio_len,
  )
}

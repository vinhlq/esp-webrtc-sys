#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

pub type esp_webrtc_handle_t = *mut c_void;
pub type esp_peer_handle_t = *mut c_void;

#[repr(C)]
pub enum esp_webrtc_custom_data_via_t {
    ESP_WEBRTC_CUSTOM_DATA_VIA_NONE,
    ESP_WEBRTC_CUSTOM_DATA_VIA_SIGNALING,
    ESP_WEBRTC_CUSTOM_DATA_VIA_DATA_CHANNEL,
}

#[repr(C)]
pub struct esp_peer_ice_server_cfg_t {
    pub stun_url: *mut core::ffi::c_char,
    pub user: *mut core::ffi::c_char,
    pub psw: *mut core::ffi::c_char,
}

#[repr(C)]
pub enum esp_peer_ice_trans_policy_t {
    ESP_PEER_ICE_TRANS_POLICY_ALL = 0,
    ESP_PEER_ICE_TRANS_POLICY_RELAY = 1,
}

#[repr(C)]
pub struct esp_peer_audio_stream_info_t {
    pub codec: i32,
    pub sample_rate: u32,
    pub channel: u8,
}

#[repr(C)]
pub struct esp_peer_video_stream_info_t {
    pub codec: i32,
    pub width: i32,
    pub height: i32,
    pub fps: i32,
}

#[repr(C)]
pub enum esp_peer_media_dir_t {
    ESP_PEER_MEDIA_DIR_NONE = 0,
    ESP_PEER_MEDIA_DIR_SENDONLY = 1,
    ESP_PEER_MEDIA_DIR_RECVONLY = 2,
    ESP_PEER_MEDIA_DIR_SENDRECV = 3,
}

#[repr(C)]
pub struct esp_webrtc_peer_cfg_t {
    pub server_lists: *mut esp_peer_ice_server_cfg_t,
    pub server_num: u8,
    pub ice_trans_policy: esp_peer_ice_trans_policy_t,
    pub audio_info: esp_peer_audio_stream_info_t,
    pub video_info: esp_peer_video_stream_info_t,
    pub audio_dir: esp_peer_media_dir_t,
    pub video_dir: esp_peer_media_dir_t,
    pub enable_data_channel: bool,
    pub manual_ch_create: bool,
    pub video_over_data_channel: bool,
    pub no_auto_reconnect: bool,
    pub extra_cfg: *mut c_void,
    pub extra_size: i32,
    pub ctx: *mut c_void,

    pub on_custom_data: Option<unsafe extern "C" fn(via: esp_webrtc_custom_data_via_t, data: *mut u8, size: i32, ctx: *mut c_void) -> i32>,
    pub on_channel_open: Option<unsafe extern "C" fn(ch: *mut c_void, ctx: *mut c_void) -> i32>,
    pub on_data: Option<unsafe extern "C" fn(frame: *mut c_void, ctx: *mut c_void) -> i32>,
    pub on_channel_close: Option<unsafe extern "C" fn(ch: *mut c_void, ctx: *mut c_void) -> i32>,
    pub on_video_send: Option<unsafe extern "C" fn(frame: *mut c_void, ctx: *mut c_void) -> i32>,
}

#[repr(C)]
pub struct esp_webrtc_signaling_cfg_t {
    pub signal_url: *mut core::ffi::c_char,
    pub extra_cfg: *mut c_void,
    pub extra_size: i32,
    pub ctx: *mut c_void,
}

#[repr(C)]
pub struct esp_webrtc_cfg_t {
    pub signaling_impl: *const c_void,
    pub signaling_cfg: esp_webrtc_signaling_cfg_t,
    pub peer_impl: *const c_void,
    pub peer_cfg: esp_webrtc_peer_cfg_t,
}

#[repr(C)]
pub enum esp_peer_signaling_whip_auth_type_t {
    ESP_PEER_SIGNALING_WHIP_AUTH_TYPE_BEARER = 0,
    ESP_PEER_SIGNALING_WHIP_AUTH_TYPE_BASIC = 1,
}

#[repr(C)]
pub struct esp_peer_audio_frame_t {
    pub pts: u32,
    pub data: *mut u8,
    pub size: i32,
}

#[repr(C)]
pub struct esp_peer_signaling_whip_cfg_t {
    pub auth_type: esp_peer_signaling_whip_auth_type_t,
    pub token: *mut core::ffi::c_char,
}

#[repr(C)]
pub enum esp_webrtc_event_type_t {
    ESP_WEBRTC_EVENT_NONE = 0,
    ESP_WEBRTC_EVENT_CONNECTING = 1,
    ESP_WEBRTC_EVENT_PAIRED = 2,
    ESP_WEBRTC_EVENT_CONNECTED = 3,
    ESP_WEBRTC_EVENT_CONNECT_FAILED = 4,
    ESP_WEBRTC_EVENT_DISCONNECTED = 5,
    ESP_WEBRTC_EVENT_DATA_CHANNEL_CONNECTED = 6,
    ESP_WEBRTC_EVENT_DATA_CHANNEL_DISCONNECTED = 7,
    ESP_WEBRTC_EVENT_DATA_CHANNEL_OPENED = 8,
    ESP_WEBRTC_EVENT_DATA_CHANNEL_CLOSED = 9,
}

#[repr(C)]
pub struct esp_webrtc_event_t {
    pub type_: esp_webrtc_event_type_t,
    pub body: *mut core::ffi::c_char,
}

pub type esp_webrtc_event_handler_t = Option<unsafe extern "C" fn(event: *mut esp_webrtc_event_t, ctx: *mut c_void) -> i32>;

extern "C" {
    // esp_webrtc_defaults.h
    pub fn esp_signaling_get_whip_impl() -> *const c_void;
    pub fn esp_peer_get_default_impl() -> *const c_void;

    // esp_webrtc.h
    pub fn esp_webrtc_open(cfg: *mut esp_webrtc_cfg_t, rtc_handle: *mut esp_webrtc_handle_t) -> i32;
    pub fn esp_webrtc_set_no_auto_capture(rtc_handle: esp_webrtc_handle_t, no_auto_capture: bool) -> i32;
    pub fn esp_webrtc_set_event_handler(rtc_handle: esp_webrtc_handle_t, handler: esp_webrtc_event_handler_t, ctx: *mut c_void) -> i32;
    pub fn esp_webrtc_enable_peer_connection(rtc_handle: esp_webrtc_handle_t, enable: bool) -> i32;
    pub fn esp_webrtc_start(rtc_handle: esp_webrtc_handle_t) -> i32;
    pub fn esp_webrtc_send_custom_data(rtc_handle: esp_webrtc_handle_t, via: esp_webrtc_custom_data_via_t, data: *mut u8, size: i32) -> i32;
    pub fn esp_webrtc_get_peer_connection(rtc_handle: esp_webrtc_handle_t, peer_handle: *mut esp_peer_handle_t) -> i32;
    pub fn esp_webrtc_query(rtc_handle: esp_webrtc_handle_t) -> i32;
    pub fn esp_webrtc_stop(rtc_handle: esp_webrtc_handle_t) -> i32;
    pub fn esp_webrtc_close(rtc_handle: esp_webrtc_handle_t) -> i32;
    pub fn esp_peer_send_audio(peer_handle: esp_peer_handle_t, info: *mut esp_peer_audio_frame_t) -> i32;
}

pub const ESP_PEER_AUDIO_CODEC_NONE: i32 = 0;
pub const ESP_PEER_AUDIO_CODEC_G711A: i32 = 1;
pub const ESP_PEER_AUDIO_CODEC_G711U: i32 = 2;
pub const ESP_PEER_AUDIO_CODEC_OPUS: i32 = 3;

pub type esp_peer_signaling_handle_t = *mut c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum esp_peer_signaling_msg_type_t {
    ESP_PEER_SIGNALING_MSG_NONE = 0,
    ESP_PEER_SIGNALING_MSG_SDP = 1,
    ESP_PEER_SIGNALING_MSG_CANDIDATE = 2,
    ESP_PEER_SIGNALING_MSG_BYE = 3,
    ESP_PEER_SIGNALING_MSG_CUSTOMIZED = 4,
}

#[repr(C)]
pub struct esp_peer_signaling_msg_t {
    pub type_: esp_peer_signaling_msg_type_t,
    pub data: *mut u8,
    pub size: i32,
}

#[repr(C)]
pub struct esp_peer_signaling_ice_info_t {
    pub server_info: esp_peer_ice_server_cfg_t,
    pub is_initiator: bool,
}

#[repr(C)]
pub struct esp_peer_signaling_cfg_t {
    pub on_ice_info: Option<unsafe extern "C" fn(info: *mut esp_peer_signaling_ice_info_t, ctx: *mut c_void) -> i32>,
    pub on_connected: Option<unsafe extern "C" fn(ctx: *mut c_void) -> i32>,
    pub on_msg: Option<unsafe extern "C" fn(msg: *mut esp_peer_signaling_msg_t, ctx: *mut c_void) -> i32>,
    pub on_close: Option<unsafe extern "C" fn(ctx: *mut c_void) -> i32>,
    pub signal_url: *mut core::ffi::c_char,
    pub extra_cfg: *mut c_void,
    pub extra_size: i32,
    pub ctx: *mut c_void,
}

#[repr(C)]
pub struct esp_peer_signaling_impl_t {
    pub start: Option<unsafe extern "C" fn(cfg: *mut esp_peer_signaling_cfg_t, sig: *mut esp_peer_signaling_handle_t) -> i32>,
    pub send_msg: Option<unsafe extern "C" fn(sig: esp_peer_signaling_handle_t, msg: *mut esp_peer_signaling_msg_t) -> i32>,
    pub stop: Option<unsafe extern "C" fn(sig: esp_peer_signaling_handle_t) -> i32>,
}

// ===========================================================================
// Layout verification
// ===========================================================================
//
// Every type above is transcribed by hand from the vendored `esp-webrtc-solution` headers, and
// `build.rs` does not run bindgen. Nothing in the build would notice if a vendor update reordered a
// field: the struct would still compile, and C would read a callback pointer out of what Rust wrote
// as an integer. That failure is silent and looks like a crash somewhere unrelated.
//
// These assertions are the substitute for bindgen. They are `const`, so a mismatch fails the build
// rather than the device, and they need no C toolchain to check.
//
// The expected values were taken from the vendored headers compiled for the real target with
// `xtensa-esp32s3-elf-gcc`, and independently reproduced with host `gcc -m32`; both agreed on every
// size and offset below. Note the agreement is not general — x86-32 aligns 64-bit members to 4
// bytes where Xtensa uses 8 — it holds here only because none of these structs contain a 64-bit
// member. If one ever gains a `u64`/`f64`, re-derive these numbers with the target compiler.
//
// All offsets are for a 32-bit target (4-byte pointers), which every supported chip is.
const _: () = {
  use core::mem::{align_of, offset_of, size_of};

  macro_rules! assert_layout {
    ($t:ty, $size:expr) => {
      assert!(size_of::<$t>() == $size);
      assert!(align_of::<$t>() == 4);
    };
  }

  assert_layout!(esp_peer_ice_server_cfg_t, 12);
  assert!(offset_of!(esp_peer_ice_server_cfg_t, stun_url) == 0);
  assert!(offset_of!(esp_peer_ice_server_cfg_t, user) == 4);
  assert!(offset_of!(esp_peer_ice_server_cfg_t, psw) == 8);

  assert_layout!(esp_peer_audio_stream_info_t, 12);
  assert_layout!(esp_peer_video_stream_info_t, 16);

  // The one the review called tricky: nested structs, an enum, and a run of four `bool`s that must
  // pack into bytes 48..52 rather than being padded apart.
  assert_layout!(esp_webrtc_peer_cfg_t, 84);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, server_lists) == 0);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, server_num) == 4);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, ice_trans_policy) == 8);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, audio_info) == 12);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, video_info) == 24);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, audio_dir) == 40);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, video_dir) == 44);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, enable_data_channel) == 48);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, manual_ch_create) == 49);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, video_over_data_channel) == 50);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, no_auto_reconnect) == 51);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, extra_cfg) == 52);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, extra_size) == 56);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, ctx) == 60);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, on_custom_data) == 64);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, on_channel_open) == 68);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, on_data) == 72);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, on_channel_close) == 76);
  assert!(offset_of!(esp_webrtc_peer_cfg_t, on_video_send) == 80);

  assert_layout!(esp_webrtc_signaling_cfg_t, 16);
  assert_layout!(esp_webrtc_cfg_t, 108);
  assert!(offset_of!(esp_webrtc_cfg_t, signaling_impl) == 0);
  assert!(offset_of!(esp_webrtc_cfg_t, signaling_cfg) == 4);
  assert!(offset_of!(esp_webrtc_cfg_t, peer_impl) == 20);
  assert!(offset_of!(esp_webrtc_cfg_t, peer_cfg) == 24);

  assert_layout!(esp_peer_audio_frame_t, 12);
  assert_layout!(esp_peer_signaling_whip_cfg_t, 8);

  assert_layout!(esp_webrtc_event_t, 8);
  assert!(offset_of!(esp_webrtc_event_t, type_) == 0);
  assert!(offset_of!(esp_webrtc_event_t, body) == 4);

  assert_layout!(esp_peer_signaling_msg_t, 12);
  assert_layout!(esp_peer_signaling_ice_info_t, 16);
  assert_layout!(esp_peer_signaling_cfg_t, 32);

  // C enums cross this boundary as plain ints; these must stay 4 bytes.
  assert!(size_of::<esp_peer_ice_trans_policy_t>() == 4);
  assert!(size_of::<esp_peer_media_dir_t>() == 4);
  assert!(size_of::<esp_webrtc_event_type_t>() == 4);
  assert!(size_of::<esp_peer_signaling_msg_type_t>() == 4);
};

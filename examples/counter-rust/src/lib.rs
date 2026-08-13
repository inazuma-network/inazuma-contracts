//! Inazuma example contract: a counter.
//!
//!   args ""        -> increment by 1
//!   args "get"     -> return the counter without changing it
//!   args "add:<n>" -> add n
//!
//! Build: cargo build --release --target wasm32-unknown-unknown
#![no_std]
#![allow(clippy::missing_safety_doc)]

use core::panic::PanicInfo;

extern "C" {
    fn inz_input_len() -> i32;
    fn inz_input(ptr: *mut u8, len: i32) -> i32;
    fn inz_read(k: *const u8, k_len: i32, v: *mut u8, v_len: i32) -> i32;
    fn inz_write(k: *const u8, k_len: i32, v: *const u8, v_len: i32) -> i32;
    fn inz_return(ptr: *const u8, len: i32);
    fn inz_log(ptr: *const u8, len: i32);
}

const KEY: &[u8] = b"count";
static mut INPUT: [u8; 64] = [0; 64];
static mut VALUE: [u8; 32] = [0; 32];

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

/// Reads the stored counter, or 0 when unset.
unsafe fn load() -> u64 {
    let n = inz_read(KEY.as_ptr(), KEY.len() as i32, VALUE.as_mut_ptr(), VALUE.len() as i32);
    if n <= 0 {
        return 0;
    }
    parse(&VALUE[..n as usize])
}

unsafe fn store(value: u64) {
    let mut buf = [0u8; 20];
    let text = format_u64(value, &mut buf);
    inz_write(KEY.as_ptr(), KEY.len() as i32, text.as_ptr(), text.len() as i32);
    inz_return(text.as_ptr(), text.len() as i32);
}

fn parse(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0u64, |acc, b| match b {
        b'0'..=b'9' => acc.saturating_mul(10).saturating_add((b - b'0') as u64),
        _ => acc,
    })
}

fn format_u64(mut value: u64, buf: &mut [u8; 20]) -> &[u8] {
    if value == 0 {
        buf[0] = b'0';
        return &buf[..1];
    }
    let mut i = buf.len();
    while value > 0 {
        i -= 1;
        buf[i] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    let len = buf.len() - i;
    buf.copy_within(i.., 0);
    &buf[..len]
}

#[no_mangle]
pub unsafe extern "C" fn call() {
    let len = inz_input_len().clamp(0, INPUT.len() as i32);
    if len > 0 {
        inz_input(INPUT.as_mut_ptr(), len);
    }
    let args = &INPUT[..len as usize];
    let current = load();

    // "get" -> read-only
    if args == b"get" {
        let mut buf = [0u8; 20];
        let text = format_u64(current, &mut buf);
        inz_return(text.as_ptr(), text.len() as i32);
        return;
    }

    // "add:<n>" -> add n, otherwise increment by one
    let delta = if args.len() > 4 && &args[..4] == b"add:" { parse(&args[4..]) } else { 1 };

    let msg = b"counter bumped";
    inz_log(msg.as_ptr(), msg.len() as i32);
    store(current.saturating_add(delta));
}

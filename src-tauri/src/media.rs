use tracing::{error, info};

use crate::protocol;
use crate::state::AppState;

const COOLDOWN_MS: i64 = 500;

pub fn simulate_toggle(state: &AppState) {
    // Set cooldown to prevent feedback loop
    let until = protocol::now_ms() + COOLDOWN_MS;
    state.set_cooldown(until);

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(50));

        #[cfg(target_os = "macos")]
        simulate_macos();

        #[cfg(target_os = "windows")]
        simulate_windows();
    });
}

#[cfg(target_os = "macos")]
fn simulate_macos() {
    use objc2_app_kit::{NSEvent, NSEventModifierFlags, NSEventType};
    use objc2_core_graphics::{CGEvent, CGEventTapLocation};
    use objc2_foundation::NSPoint;

    const NX_KEYTYPE_PLAY: isize = 16;

    fn post_media_key(key_code: isize, key_down: bool) {
        let flags = NSEventModifierFlags::from_bits_retain(if key_down { 0xa00 } else { 0xb00 });
        let data1: isize = (key_code << 16) | ((if key_down { 0xa } else { 0xb }) << 8);

        let event = NSEvent::otherEventWithType_location_modifierFlags_timestamp_windowNumber_context_subtype_data1_data2(
            NSEventType::SystemDefined,
            NSPoint::new(0.0, 0.0),
            flags,
            0.0,
            0,
            None,
            8, // NX_SUBTYPE_AUX_CONTROL_BUTTONS
            data1,
            -1,
        );

        if let Some(ev) = event {
            if let Some(cg_event) = ev.CGEvent() {
                CGEvent::post(CGEventTapLocation(0), Some(&cg_event));
                info!("Posted media key event");
            } else {
                error!("Failed to get CGEvent from NSEvent");
            }
        } else {
            error!("Failed to create NSEvent");
        }
    }

    info!("Posting media key down...");
    post_media_key(NX_KEYTYPE_PLAY, true);
    std::thread::sleep(std::time::Duration::from_millis(100));
    info!("Posting media key up...");
    post_media_key(NX_KEYTYPE_PLAY, false);
    info!("Simulated media play/pause (macOS)");
}

#[cfg(target_os = "windows")]
fn simulate_windows() {
    use std::ptr::null_mut;

    // VK_MEDIA_PLAY_PAUSE = 0xB3
    const VK_MEDIA_PLAY_PAUSE: u16 = 0xB3;
    const KEYEVENTF_EXTENDEDKEY: u32 = 0x0001;
    const KEYEVENTF_KEYUP: u32 = 0x0002;
    const INPUT_KEYBOARD: u32 = 1;

    #[repr(C)]
    struct KeyboardInput {
        r#type: u32,
        ki: KeybdInput,
    }

    #[repr(C)]
    struct KeybdInput {
        wVk: u16,
        wScan: u16,
        dwFlags: u32,
        time: u32,
        dwExtraInfo: usize,
        _padding: [u8; 8],
    }

    #[link(name = "user32")]
    extern "system" {
        fn SendInput(cInputs: u32, pInputs: *const KeyboardInput, cbSize: i32) -> u32;
    }

    let mut inputs = [
        KeyboardInput {
            r#type: INPUT_KEYBOARD,
            ki: KeybdInput {
                wVk: VK_MEDIA_PLAY_PAUSE,
                wScan: 0,
                dwFlags: KEYEVENTF_EXTENDEDKEY,
                time: 0,
                dwExtraInfo: 0,
                _padding: [0; 8],
            },
        },
        KeyboardInput {
            r#type: INPUT_KEYBOARD,
            ki: KeybdInput {
                wVk: VK_MEDIA_PLAY_PAUSE,
                wScan: 0,
                dwFlags: KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
                _padding: [0; 8],
            },
        },
    ];

    unsafe {
        let sent = SendInput(
            2,
            inputs.as_ptr(),
            std::mem::size_of::<KeyboardInput>() as i32,
        );
        if sent == 2 {
            info!("Simulated media play/pause (Windows)");
        } else {
            error!("SendInput failed, sent {} of 2 inputs", sent);
        }
    }
}


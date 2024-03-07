// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use windows::{
    core::{h, s},
    Security::{
        Credentials::{KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialStatus},
        Cryptography::CryptographicBuffer,
    },
    Win32::{
        Foundation::HWND,
        UI::{
            Input::KeyboardAndMouse::{
                keybd_event, GetAsyncKeyState, SetFocus, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP,
                VK_MENU,
            },
            WindowsAndMessaging::{FindWindowA, SetForegroundWindow},
        },
    },
};

fn set_focus(window: HWND) {
    let mut pressed = false;
    // SAFETY:
    // Many calls to windows unsafe API
    unsafe {
        // Simulate holding down Alt key to bypass windows limitations
        //  https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate#return-value
        //  The most significant bit indicates if the key is currently being pressed. This means the
        //  value will be negative if the key is pressed.
        if GetAsyncKeyState(VK_MENU.0 as i32) >= 0 {
            pressed = true;
            keybd_event(VK_MENU.0 as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
        }
        SetForegroundWindow(window);
        SetFocus(window);
        if pressed {
            keybd_event(
                VK_MENU.0 as u8,
                0,
                KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
                0,
            );
        }
    }
}

fn focus_security_prompt() -> Result<(), ()> {
    fn try_find_and_set_focus(class_name: windows::core::PCSTR) -> Result<(), ()> {
        // SAFETY:
        // Call to unsafe windows API
        let hwnd = unsafe { FindWindowA(class_name, None) };
        if hwnd.0 != 0 {
            set_focus(hwnd);
            return Ok(());
        }
        Err(())
    }
    let class_name = s!("Credential Dialog Xaml Host");
    // Try 30 times, waiting 50 ms (1.5 seconds total)
    for _ in 0..30 {
        if try_find_and_set_focus(class_name).is_ok() {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    Err(())
}

pub fn derive_key_from_biometrics(challenge: &str) -> Result<String, String> {
    let challenge_bytes = challenge.as_bytes();
    let service_name = h!("parsec-v3");

    let result = KeyCredentialManager::RequestCreateAsync(
        service_name,
        KeyCredentialCreationOption::FailIfExists,
    )
    .map_err(|x| x.message())?
    .get()
    .map_err(|x| x.message())?;

    let result = match result.Status().map_err(|x| x.message())? {
        KeyCredentialStatus::CredentialAlreadyExists => {
            KeyCredentialManager::OpenAsync(service_name)
                .map_err(|x| x.message())?
                .get()
                .map_err(|x| x.message())?
        }
        KeyCredentialStatus::Success => result,
        _ => return Err("Failed to create key credential".to_string()),
    };

    let challenge_buffer =
        CryptographicBuffer::CreateFromByteArray(challenge_bytes).map_err(|x| x.message())?;
    let async_operation = result
        .Credential()
        .map_err(|x| x.message())?
        .RequestSignAsync(&challenge_buffer)
        .map_err(|x| x.message())?;
    focus_security_prompt().ok();
    let signature = async_operation.get().map_err(|x| x.message())?;

    if signature.Status().map_err(|x| x.message())? != KeyCredentialStatus::Success {
        return Err("Failed to sign data".to_string());
    }

    let signature_buffer = signature.Result().map_err(|x| x.message())?;
    let mut signature_value =
        windows::core::Array::<u8>::with_len(signature_buffer.Length().unwrap() as usize);
    CryptographicBuffer::CopyToByteArray(&signature_buffer, &mut signature_value)
        .map_err(|x| x.message())?;
    let signature_b64 = base64_engine.encode(&*signature_value);
    Ok(signature_b64)
}

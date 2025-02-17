// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::anyhow::{anyhow, Result};
use libparsec_types::{DeviceID, SecretKey};
use sha2::{Digest, Sha256};

use windows::{
    core::{s, HSTRING},
    Security::{
        Credentials::{
            KeyCredential, KeyCredentialCreationOption, KeyCredentialManager, KeyCredentialStatus,
        },
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
        // TODO: This Alt key hack has been recently removed from bitwarden:
        //   https://github.com/bitwarden/clients/pull/12255
        // We should investigate if it's still necessary, or if the approach can be improved.

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

pub fn is_biometrics_available() -> bool {
    let result = KeyCredentialManager::IsSupportedAsync().and_then(|x| x.get());
    match result {
        Ok(x) => x,
        Err(e) => {
            log::warn!("Unexpected error while accessing the KeyCredentialManager API: {e}");
            false
        }
    }
}

pub fn get_key_credential(service: &str) -> Result<KeyCredential> {
    let service_name = HSTRING::from(service);

    // TODO: The approach here is to create a new key credential if it doesn't exist.
    // This allows us to expose only a single `derive_key_from_biometrics` API for
    // both loading and saving.
    // However, this means we cannot detect when an existing key credential has been
    // removed from the system (e.g when the pin code has been removed and recreated).
    // Another approach would be to retrieve the corresponding public key when saving
    // a new device using `KeyCredential::RetrievePublicKey()`. This way, this information
    // could be stored in the device key file and use it to detect when the key credential
    // has been removed or changed.
    let result = KeyCredentialManager::RequestCreateAsync(
        &service_name,
        KeyCredentialCreationOption::FailIfExists,
    )?
    .get()?;

    let result = match result.Status()? {
        KeyCredentialStatus::CredentialAlreadyExists => {
            KeyCredentialManager::OpenAsync(&service_name)?.get()?
        }
        KeyCredentialStatus::Success => result,
        unknown => return Err(anyhow!("Failed to create key credential: {unknown:?}")),
    };
    let credential = result.Credential()?;
    Ok(credential)
}

// TODO: this function is synchronous but will block the thread while waiting for the
// user to enter this pin code. Either this function should be async or we make sure
// the caller uses `tokio::task::spawn_blocking`
pub fn derive_key_from_biometrics(service: &str, device_id: DeviceID) -> Result<SecretKey> {
    // TODO: Is this the formatting we want for the challenge?
    // Is the device ID the right thing to use here?
    let challenge = format!("BIOMETRICS_CHALLENGE:{}", device_id.hex());
    let challenge_bytes = challenge.as_bytes();
    let challenge_buffer = CryptographicBuffer::CreateFromByteArray(challenge_bytes)?;

    let credential = get_key_credential(service)?;
    let async_operation = credential.RequestSignAsync(&challenge_buffer)?;
    focus_security_prompt().ok();
    let signature = async_operation.get()?;
    if signature.Status()? != KeyCredentialStatus::Success {
        return Err(anyhow!("Failed to sign data"));
    }

    let signature_buffer = signature.Result()?;
    let mut signature_value =
        windows::core::Array::<u8>::with_len(signature_buffer.Length()? as usize);
    CryptographicBuffer::CopyToByteArray(&signature_buffer, &mut signature_value)?;
    let raw_key: [u8; SecretKey::SIZE] = Sha256::digest(&*signature_value).into();
    let key = SecretKey::try_from(raw_key)?;
    Ok(key)
}

use std::{
    ffi::CString,
    ptr::{null, null_mut},
};

use winapi::{
    shared::{
        minwindef::{DWORD, HKEY, MAX_PATH},
        windef::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
    },
    um::{
        libloaderapi::GetModuleFileNameA,
        wincon::{AttachConsole, FreeConsole, ATTACH_PARENT_PROCESS},
        winnt::{KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ},
        winreg::{RegCloseKey, RegCreateKeyExA, RegDeleteTreeA, RegSetValueExA, HKEY_CURRENT_USER},
        winuser::SetProcessDpiAwarenessContext,
        winuser::SetProcessDPIAware,
    },
};

fn get_binary_path() -> String {
    let mut buffer = vec![0u8; MAX_PATH];
    unsafe {
        GetModuleFileNameA(
            null_mut(),
            buffer.as_mut_ptr() as *mut i8,
            MAX_PATH as DWORD,
        );
        CString::from_vec_unchecked(buffer)
            .into_string()
            .unwrap_or_else(|_| "".to_string())
            .trim_end_matches(char::from(0))
            .to_string()
    }
}

pub fn unregister_rightclick() -> bool {
    let str_registry_path_1 =
        CString::new("Software\\Classes\\Directory\\Background\\shell\\Neovide").unwrap();
    let str_registry_path_2 = CString::new("Software\\Classes\\*\\shell\\Neovide").unwrap();
    unsafe {
        let s1 = RegDeleteTreeA(HKEY_CURRENT_USER, str_registry_path_1.as_ptr());
        let s2 = RegDeleteTreeA(HKEY_CURRENT_USER, str_registry_path_2.as_ptr());
        s1 == 0 && s2 == 0
    }
}

pub fn register_rightclick_directory() -> bool {
    let neovide_path = get_binary_path();
    let mut registry_key: HKEY = null_mut();
    let str_registry_path =
        CString::new("Software\\Classes\\Directory\\Background\\shell\\Neovide").unwrap();
    let str_registry_command_path =
        CString::new("Software\\Classes\\Directory\\Background\\shell\\Neovide\\command").unwrap();
    let str_icon = CString::new("Icon").unwrap();
    let str_command = CString::new(format!("{} \"%V\"", neovide_path).as_bytes()).unwrap();
    let str_description = CString::new("Open with Neovide").unwrap();
    let str_neovide_path = CString::new(neovide_path.as_bytes()).unwrap();
    unsafe {
        if RegCreateKeyExA(
            HKEY_CURRENT_USER,
            str_registry_path.as_ptr(),
            0,
            null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            null_mut(),
            &mut registry_key,
            null_mut(),
        ) != 0
        {
            RegCloseKey(registry_key);
            return false;
        }
        let registry_values = [
            (
                null(),
                REG_SZ,
                str_description.as_ptr() as *const u8,
                str_description.to_bytes().len() + 1,
            ),
            (
                str_icon.as_ptr(),
                REG_SZ,
                str_neovide_path.as_ptr() as *const u8,
                str_neovide_path.to_bytes().len() + 1,
            ),
        ];
        for &(key, keytype, value_ptr, size_in_bytes) in &registry_values {
            RegSetValueExA(
                registry_key,
                key,
                0,
                keytype,
                value_ptr,
                size_in_bytes as u32,
            );
        }
        RegCloseKey(registry_key);

        if RegCreateKeyExA(
            HKEY_CURRENT_USER,
            str_registry_command_path.as_ptr(),
            0,
            null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            null_mut(),
            &mut registry_key,
            null_mut(),
        ) != 0
        {
            return false;
        }
        let registry_values = [(
            null(),
            REG_SZ,
            str_command.as_ptr() as *const u8,
            str_command.to_bytes().len() + 1,
        )];
        for &(key, keytype, value_ptr, size_in_bytes) in &registry_values {
            RegSetValueExA(
                registry_key,
                key,
                0,
                keytype,
                value_ptr,
                size_in_bytes as u32,
            );
        }
        RegCloseKey(registry_key);
    }
    true
}

pub fn register_rightclick_file() -> bool {
    let neovide_path = get_binary_path();
    let mut registry_key: HKEY = null_mut();
    let str_registry_path = CString::new("Software\\Classes\\*\\shell\\Neovide").unwrap();
    let str_registry_command_path =
        CString::new("Software\\Classes\\*\\shell\\Neovide\\command").unwrap();
    let str_icon = CString::new("Icon").unwrap();
    let str_command = CString::new(format!("{} \"%1\"", neovide_path).as_bytes()).unwrap();
    let str_description = CString::new("Open with Neovide").unwrap();
    let str_neovide_path = CString::new(neovide_path.as_bytes()).unwrap();
    unsafe {
        if RegCreateKeyExA(
            HKEY_CURRENT_USER,
            str_registry_path.as_ptr(),
            0,
            null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            null_mut(),
            &mut registry_key,
            null_mut(),
        ) != 0
        {
            RegCloseKey(registry_key);
            return false;
        }
        let registry_values = [
            (
                null(),
                REG_SZ,
                str_description.as_ptr() as *const u8,
                str_description.to_bytes().len() + 1,
            ),
            (
                str_icon.as_ptr(),
                REG_SZ,
                str_neovide_path.as_ptr() as *const u8,
                str_neovide_path.to_bytes().len() + 1,
            ),
        ];
        for &(key, keytype, value_ptr, size_in_bytes) in &registry_values {
            RegSetValueExA(
                registry_key,
                key,
                0,
                keytype,
                value_ptr,
                size_in_bytes as u32,
            );
        }
        RegCloseKey(registry_key);

        if RegCreateKeyExA(
            HKEY_CURRENT_USER,
            str_registry_command_path.as_ptr(),
            0,
            null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            null_mut(),
            &mut registry_key,
            null_mut(),
        ) != 0
        {
            return false;
        }
        let registry_values = [(
            null(),
            REG_SZ,
            str_command.as_ptr() as *const u8,
            str_command.to_bytes().len() + 1,
        )];
        for &(key, keytype, value_ptr, size_in_bytes) in &registry_values {
            RegSetValueExA(
                registry_key,
                key,
                0,
                keytype,
                value_ptr,
                size_in_bytes as u32,
            );
        }
        RegCloseKey(registry_key);
    }
    true
}

#[cfg(feature = "legacy-win-compat")]
pub fn windows_fix_dpi() {
    unsafe {
        SetProcessDPIAware();
    }
}

#[cfg(not(feature = "legacy-win-compat"))]
pub fn windows_fix_dpi() {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
}

pub fn windows_attach_to_console() {
    // Attach to parent console tip found here: https://github.com/rust-lang/rust/issues/67159#issuecomment-987882771
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

pub fn windows_detach_from_console() {
    unsafe {
        FreeConsole();
    }
}

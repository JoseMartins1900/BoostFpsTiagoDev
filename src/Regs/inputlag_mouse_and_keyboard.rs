use std::fmt::Display;
use std::io::Result;
use winreg::{enums::*, RegKey, RegValue};

pub fn inputlag_mouse_keyboard() -> Result<()> {
    // Abre a chave HKEY_CURRENT_USER
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let inputlag_mouse =
        hkcu.open_subkey_with_flags("Control Panel\\Accessibility\\MouseKeys", KEY_SET_VALUE)?;
    inputlag_mouse.set_value("Flags", &0u32)?;

    let inputlag_keyboard = hkcu.open_subkey_with_flags(
        "Control Panel\\Accessibility\\Keyboard Response",
        KEY_SET_VALUE,
    )?;
    inputlag_keyboard.set_value("Flags", &0u32)?;

    let sticky_keys =
        hkcu.open_subkey_with_flags("Control Panel\\Accessibility\\StickyKeys", KEY_SET_VALUE)?;
    sticky_keys.set_value("Flags", &0u32)?;

    let (mouse_key, _disp) = hkcu.create_subkey("Control Panel\\Mouse")?;
    mouse_key.set_value("MouseSensitivity", &"10")?; // se for uma string no regedit é desta forma.

    // Define o valor binário SmoothMouseXCurve
    let bytes: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0xCC, 0x0C, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x80, 0x99, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x66, 0x26, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x33, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let smooth_mouse_x_curve = RegValue {
        vtype: REG_BINARY,
        bytes: bytes,
    };

    mouse_key.set_raw_value("SmoothMouseXCurve", &smooth_mouse_x_curve)?;

    // Define o valor binário SmoothMouseYCurve
    let bytes: Vec<u8> = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xA8, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let smooth_mouse_y_curve = RegValue {
        vtype: REG_BINARY,
        bytes: bytes,
    };

    mouse_key.set_raw_value("SmoothMouseYCurve", &smooth_mouse_y_curve)?; // se for um binario no regedit é desta forma.

    let hkuser = RegKey::predef(HKEY_USERS);

    let default_user_key =
        hkuser.open_subkey_with_flags(".DEFAULT\\Control Panel\\Mouse", KEY_SET_VALUE)?;
    default_user_key.set_value("MouseSpeed", &"0")?;
    default_user_key.set_value("MouseThreshold1", &"0")?;
    default_user_key.set_value("MouseThreshold2", &"0")?;

    Ok(())
}

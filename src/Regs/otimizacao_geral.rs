use winreg::{RegKey, enums::*};
use std::io::Result;

pub(crate) fn modify_registry() -> Result<()> {

    //  //////   //////   //////  Abre a chave HKEY_LOCAL_MACHINE   //////   //////   //////   ////// 
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Altera a chave SYSTEM\CurrentControlSet\Services\Spooler
    let spooler = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Services\\Spooler", KEY_SET_VALUE)?;
    spooler.set_value("Start", &4u32)?;

    // Altera a chave prioridade gpu
    let prioridade_gpu = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile\\Tasks\\Games", KEY_SET_VALUE)?;
    prioridade_gpu.set_value("GPU Priority", &8u32)?;
    prioridade_gpu.set_value("Priority", &6u32)?;
    prioridade_gpu.set_value("Scheduling Category", &"High")?;
    prioridade_gpu.set_value("SFIO Priority", &"High")?;

    // Altera a chave power_throttling
    let power_throttling = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling", KEY_SET_VALUE)?;
    power_throttling.set_value("PowerThrottlingOff", &1u32)?;

    // Altera a chave hibernate_enabled_default
    let hibernate_enabled_default = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Power", KEY_SET_VALUE)?;
    hibernate_enabled_default.set_value("HibernateEnabledDefault", &0u32)?;

    // Altera a chave DriverSearching
    let driver_searching = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\DriverSearching", KEY_SET_VALUE)?;
    driver_searching.set_value("SearchOrderConfig", &0u32)?;

    // Altera a chave HiberbootEnabled
    let hiberboot_enabled = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power", KEY_SET_VALUE)?;
    hiberboot_enabled.set_value("HiberbootEnabled", &0u32)?;

    // Altera a chave SYSTEM\CurrentControlSet\Services\PrintNotify
    let print_notify = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Services\\PrintNotify", KEY_SET_VALUE)?;
    print_notify.set_value("Start", &4u32)?; // se for um dword no regedit é desta forma.


    // Altera a chave SYSTEM\CurrentControlSet\Services\PrintWorkflowUserSvc
    let print_workflow = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Services\\PrintWorkflowUserSvc", KEY_SET_VALUE)?;
    print_workflow.set_value("Start", &4u32)?;


    // Configuração de GameDVR (como exemplo)
    let game_dvr = hklm.open_subkey_with_flags("SOFTWARE\\Microsoft\\PolicyManager\\default\\ApplicationManagement\\AllowGameDVR", KEY_SET_VALUE)?;
    game_dvr.set_value("value", &0u32)?;



    //////   //////   //////   //////  Abre a chave HKEY_CURRENT_USER    //////   //////   ////// 
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Configuração de SettingSync
    let setting_sync = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\SettingSync", KEY_SET_VALUE)?;
    setting_sync.set_value("SyncPolicy", &5u32)?;
    
    // Configuração de Personalize (como exemplo)
    let personalize = hkcu.open_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", KEY_SET_VALUE)?;
    personalize.set_value("EnableTransparency", &0u32)?;

    // Configuração da Prioridade da GPU





    Ok(())
}
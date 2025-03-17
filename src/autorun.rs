use windows_registry::{
    LOCAL_MACHINE,
    Result as WindowsRegistryResult
};
use windows_result::Error as WindowsError;

use crate::constants::{
    APP_NAME,
    ADMIN_AL_REGKEY,
    ADMIN_TASK_MANAGER_OVERRIDE_REGKEY,
    TASK_MANAGER_OVERRIDE_ENABLED_VALUE,
    TASK_MANAGER_OVERRIDE_DISABLED_VALUE
};

pub fn register() -> Result<(), WindowsError> {
    let app_path = std::env::current_exe().unwrap();
    
    LOCAL_MACHINE.create(ADMIN_AL_REGKEY)?.set_string(
        &APP_NAME,
        &app_path.to_str().unwrap()
    )
}

pub fn enable() -> WindowsRegistryResult<()> {
    if let Ok(key) = LOCAL_MACHINE.create(ADMIN_TASK_MANAGER_OVERRIDE_REGKEY) {
        key.set_bytes(
            &APP_NAME, 
            windows_registry::Type::Bytes, 
            &TASK_MANAGER_OVERRIDE_ENABLED_VALUE
        )?;
    }

    Ok(())
}

pub fn disable() -> WindowsRegistryResult<()> {
    if let Ok(key) = LOCAL_MACHINE.create(ADMIN_TASK_MANAGER_OVERRIDE_REGKEY) {
        key.set_bytes(
            &APP_NAME, 
            windows_registry::Type::Bytes, 
            &TASK_MANAGER_OVERRIDE_DISABLED_VALUE
        )?;
    }

    Ok(())    
}

pub fn is_enabled() -> WindowsRegistryResult<bool> {
    let value = LOCAL_MACHINE
        .open(ADMIN_TASK_MANAGER_OVERRIDE_REGKEY)?
        .get_value(&APP_NAME)?;

    if value[0] == 2 {
        Ok(true)
    } else {
        Ok(false)
    }
}
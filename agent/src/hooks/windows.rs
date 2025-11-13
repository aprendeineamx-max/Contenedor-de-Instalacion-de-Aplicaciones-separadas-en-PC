use crate::runtime::{HookPlan, PathRedirect};
use anyhow::{Context, Result};
use detour::static_detour;
use once_cell::sync::OnceCell;
use std::{
    ffi::OsString,
    os::windows::ffi::OsStringExt,
    path::{Path, PathBuf},
    sync::RwLock,
};
use tracing::{info, warn};
use widestring::U16CString;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HANDLE,
        Security::SECURITY_ATTRIBUTES,
        Storage::FileSystem::{CreateFileW, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_MODE},
        System::SystemServices::GENERIC_ACCESS_RIGHTS,
    },
};

static_detour! {
    static CreateFileHook: unsafe extern "system" fn(
        PCWSTR,
        GENERIC_ACCESS_RIGHTS,
        FILE_SHARE_MODE,
        *const SECURITY_ATTRIBUTES,
        u32,
        FILE_FLAGS_AND_ATTRIBUTES,
        HANDLE
    ) -> HANDLE;
}

static PLAN: OnceCell<RwLock<PlanContext>> = OnceCell::new();

pub struct DetoursHookManager;

impl DetoursHookManager {
    pub fn new() -> Self {
        Self
    }

    pub fn apply(&self, plan: &HookPlan) -> Result<()> {
        {
            let ctx = PLAN.get_or_init(|| RwLock::new(PlanContext::default()));
            let mut guard = ctx.write().expect("lock poisoned");
            *guard = PlanContext::from(plan);
        }

        unsafe {
            if !CreateFileHook.is_enabled() {
                CreateFileHook
                    .initialize(CreateFileW, create_file_redirect)
                    .context("No se pudo inicializar el hook CreateFileW")?;
                CreateFileHook
                    .enable()
                    .context("No se pudo habilitar el hook CreateFileW")?;
                info!("Hook de CreateFileW activado mediante Detours");
            }
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
struct PlanContext {
    redirects: Vec<PathRedirect>,
}

impl From<&HookPlan> for PlanContext {
    fn from(plan: &HookPlan) -> Self {
        Self {
            redirects: plan.redirects.clone(),
        }
    }
}

impl PlanContext {
    fn rewrite(&self, input: &Path) -> Option<PathBuf> {
        for redirect in &self.redirects {
            if input.starts_with(&redirect.original) {
                if let Ok(remainder) = input.strip_prefix(&redirect.original) {
                    return Some(redirect.redirected.join(remainder));
                }
            }
        }
        None
    }
}

unsafe extern "system" fn create_file_redirect(
    file_name: PCWSTR,
    desired_access: GENERIC_ACCESS_RIGHTS,
    share_mode: FILE_SHARE_MODE,
    security_attributes: *const SECURITY_ATTRIBUTES,
    creation_disposition: u32,
    flags: FILE_FLAGS_AND_ATTRIBUTES,
    template_file: HANDLE,
) -> HANDLE {
    let path = pcwstr_to_path(file_name);
    if path.as_os_str().is_empty() {
        return CreateFileHook.call(
            file_name,
            desired_access,
            share_mode,
            security_attributes,
            creation_disposition,
            flags,
            template_file,
        );
    }

    let maybe_redirect = PLAN
        .get()
        .and_then(|cell| cell.read().ok())
        .and_then(|ctx| ctx.rewrite(&path));

    if let Some(redirected) = maybe_redirect {
        if let Ok(wide) = U16CString::from_os_str(redirected.as_os_str()) {
            let new_ptr = PCWSTR(wide.as_ptr());
            return CreateFileHook.call(
                new_ptr,
                desired_access,
                share_mode,
                security_attributes,
                creation_disposition,
                flags,
                template_file,
            );
        } else {
            warn!("No se pudo convertir la ruta redirigida {:?}", redirected);
        }
    }

    CreateFileHook.call(
        file_name,
        desired_access,
        share_mode,
        security_attributes,
        creation_disposition,
        flags,
        template_file,
    )
}

fn pcwstr_to_path(value: PCWSTR) -> PathBuf {
    if value.is_null() {
        return PathBuf::new();
    }
    unsafe {
        let mut len = 0;
        let mut ptr = value.0;
        while *ptr != 0 {
            len += 1;
            ptr = ptr.add(1);
        }
        let slice = std::slice::from_raw_parts(value.0, len);
        OsString::from_wide(slice).into()
    }
}

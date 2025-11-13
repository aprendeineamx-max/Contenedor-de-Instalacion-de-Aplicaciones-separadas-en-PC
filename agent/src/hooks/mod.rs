use crate::runtime::HookPlan;
use anyhow::Result;

#[cfg(all(target_os = "windows", feature = "native-hooks"))]
mod windows;

#[cfg(all(target_os = "windows", feature = "native-hooks"))]
use windows::DetoursHookManager as PlatformHookManager;

#[cfg(not(all(target_os = "windows", feature = "native-hooks")))]
struct PlatformHookManager;

#[cfg(not(all(target_os = "windows", feature = "native-hooks")))]
impl PlatformHookManager {
    fn new() -> Self {
        Self
    }

    fn apply(&self, plan: &HookPlan) -> Result<()> {
        tracing::debug!(
            "Hooks nativos deshabilitados; omitiendo plan ({} alias)",
            plan.redirects.len()
        );
        Ok(())
    }
}

pub struct NativeHookPipeline {
    inner: PlatformHookManager,
}

impl NativeHookPipeline {
    pub fn new() -> Self {
        Self {
            inner: PlatformHookManager::new(),
        }
    }

    pub fn apply(&self, plan: &HookPlan) -> Result<()> {
        self.inner.apply(plan)
    }
}

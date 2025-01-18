use core::ops::Deref;
use snafu::GenerateImplicitData;

pub struct StaticLocationRef(&'static core::panic::Location<'static>);

impl Deref for StaticLocationRef {
    type Target = core::panic::Location<'static>;
    fn deref(&self) -> &core::panic::Location<'static> {
        self.0
    }
}

impl core::fmt::Debug for StaticLocationRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}:{}:{}", self.file(), self.line(), self.column())
    }
}

impl GenerateImplicitData for StaticLocationRef {
    #[track_caller]
    fn generate() -> Self {
        Self(core::panic::Location::caller())
    }
}

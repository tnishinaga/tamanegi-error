use snafu::{GenerateImplicitData, Snafu};
use tamanegi_error::{location::StaticLocationRef, TamanegiError};

#[derive(Snafu, TamanegiError)]
pub struct ErrorSubA {
    #[snafu(implicit)]
    location: StaticLocationRef,
}

impl ErrorSubA {
    pub fn new() -> Self {
        Self {
            location: StaticLocationRef::generate(),
        }
    }
}

#[derive(Snafu, TamanegiError)]
#[snafu(context(false))]
struct MyError {
    #[snafu(source)]
    source: ErrorSubA,
    #[snafu(implicit)]
    location: StaticLocationRef,
}

fn err() -> Result<(), MyError> {
    let err: Result<(), ErrorSubA> = Err(ErrorSubA::new());
    let _ = err?;
    Ok(())
}

fn main() {
    if let Err(e) = err() {
        println!("{:?}", e);
    }
}

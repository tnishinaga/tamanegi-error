use snafu::Snafu;
use tamanegi_error::location::StaticLocationRef;
use tamanegi_error_impl::TamanegiError;

#[test]
fn test_error_enum() {
    #[derive(Debug, Snafu)]
    pub struct ErrorSubA {
        #[snafu(implicit)]
        location: StaticLocationRef,
    }

    #[derive(Snafu, TamanegiError)]
    enum MyError {
        #[snafu(context(false))]
        SubA {
            source: ErrorSubA,
            #[snafu(implicit)]
            location: StaticLocationRef,
        },
    }
}

#[test]
fn test_error_struct() {
    #[derive(Debug, Snafu)]
    pub struct ErrorSubA {
        #[snafu(implicit)]
        location: StaticLocationRef,
    }

    #[derive(Snafu, TamanegiError)]
    #[allow(dead_code)]
    struct MyError {
        a: ErrorSubA,
        #[snafu(implicit)]
        location: StaticLocationRef,
    }
}

#[test]
fn test_non_tamanegi_leaf() {
    use snafu::Snafu;
    use tamanegi_error::{location::StaticLocationRef, TamanegiError};

    #[derive(Debug)]
    pub struct ErrorSubA {
        _location: StaticLocationRef,
    }

    impl core::fmt::Display for ErrorSubA {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl core::error::Error for ErrorSubA {}

    #[derive(Snafu, TamanegiError)]
    enum MyError {
        #[snafu(context(false))]
        SubA {
            source: ErrorSubA,
            #[snafu(implicit)]
            location: StaticLocationRef,
        },
    }
}

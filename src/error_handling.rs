use std::io;

pub trait OkOrIoOther<T> {
    fn ok_or_ioerror<E>(self, error: E) -> io::Result<T>
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>;
}

impl<T> OkOrIoOther<T> for Option<T> {
    #[inline]
    fn ok_or_ioerror<E>(self, error: E) -> io::Result<T>
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self.ok_or_else(|| io::Error::other(error))
    }
}

pub trait UnwrapQuit<T, E> {
    fn unwrap_or_quit(self, err_code: i32, err: &str) -> T
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>;
}
impl<T, E> UnwrapQuit<T, E> for Result<T,E> 
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>> {
    #[inline]
    fn unwrap_or_quit(self, err_code: i32, err: &str) -> T {
        self.unwrap_or_else(
            |_| {
                quit_with_error(err_code, err);
                unreachable!();
            }
        )
    }
}
impl<T,E> UnwrapQuit<T, E> for Option<T> {
    #[inline]
    fn unwrap_or_quit(self, err_code: i32, err: &str) -> T
    {
        self.unwrap_or_else(
            || {
                quit_with_error(err_code, err);
                unreachable!();
            }
        )

    }
}

#[inline]
pub fn quit_with_error(code: i32, err: &str) {
    eprintln!("Error: {}", err);
    std::process::exit(code);
}

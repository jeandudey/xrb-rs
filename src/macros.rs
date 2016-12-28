#[macro_export]
macro_rules! try_future {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(e) => return futures::done(Err(e)).boxed(),
        }
    }
}

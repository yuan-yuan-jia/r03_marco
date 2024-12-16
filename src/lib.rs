#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr),+ $(,)?) => {
        <[_]>::into_vec(Box::new([$($x),*]))
    }
}

#[macro_export]
macro_rules! my_try {
    ($expr: expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}

#[macro_export]
macro_rules! my_ready {
    ($expr: expr) => {
        match $expr {
            std::task::Poll::Ready(t) => std::task::Poll::Ready(t),
            std::task::Poll::Pending => std::task::Poll::Pending,

        }
    };
}
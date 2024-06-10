macro_rules! query_guard {
    ($query:expr) => {
        if let Ok(result) = $query {
            result
        } else {
            return;
        }
    };
    ($($query:expr),+) => {
        ($(query_guard!($query),)+)
    }
}

pub(crate) use query_guard;

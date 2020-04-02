pub enum FrameworkError {
    BalanceError,
}

pub type Result<T> = core::result::Result<T, FrameworkError>;


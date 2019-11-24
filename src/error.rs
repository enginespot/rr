#[derive(Debug, Fail)]
pub enum RRError {
    #[fail(
        display = "An error type which cannot be instantiated. Used as a placeholder for associated error types if something cannot fail."
    )]
    NoError,
    #[fail(display = "{}", _0)]
    IoError(#[cause] std::io::Error),
    #[fail(display = "Schedule Error")]
    ScheduleError,
    #[fail(display = "Subscribe On Error")]
    SubscribeOnError,
    #[fail(display = "Thread Panic")]
    ThreadPanic,
    #[fail(display = "Recv Error")]
    RecvError,
    #[fail(display = "Subscription Error")]
    SubscriptionError,
}

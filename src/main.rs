use std::{env::args, process::ExitCode, str::FromStr, thread::sleep, time::Duration};

trait ParseOr<T> {
    fn parse_or(self, default: T) -> T
    where
        T: FromStr;
}

impl<T> ParseOr<T> for Option<String> {
    fn parse_or(self, default: T) -> T
    where
        T: FromStr,
    {
        self.and_then(|arg| arg.parse().ok()).unwrap_or(default)
    }
}

fn main() -> ExitCode {
    let time = args().nth(1).parse_or(1);
    let error = args().nth(2).parse_or(1);
    sleep(Duration::from_secs(time));
    ExitCode::from(error)
}

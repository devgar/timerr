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

#[cfg(test)]
mod tests {
    use super::ParseOr;
    #[test]
    fn test_parse_or() {
        assert_eq!(Some("123".to_string()).parse_or(0), 123);
        assert_eq!(Some("abc".to_string()).parse_or(0), 0);
        assert_eq!(None.parse_or(42), 42);
    }
}
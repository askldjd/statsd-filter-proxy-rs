use std::str;

pub fn filter<'a>(block_list: &'a Vec<String>, buf: &'a [u8]) -> Vec<&'a str> {
    let statsd_str = unsafe { str::from_utf8_unchecked(&buf) };

    statsd_str.split("\n")
        .filter(|line| {
            !block_list.iter().any(|prefix| line.starts_with(prefix))
        })
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_not_block_multi_metric() {
        let block_list = vec![String::from("notfoo"), String::from("otherfoo")];
        let statsd_str_bytes = "foo:1|c\nfoo:2|c\nfoo:3|c".as_bytes();
        let result = filter(&block_list, &statsd_str_bytes).join("\n");
        assert_eq!("foo:1|c\nfoo:2|c\nfoo:3|c", result);
    }


    #[test]
    fn test_should_not_block_single_metric() {
        let block_list = vec![String::from("notfoo"), String::from("otherfoo")];
        let statsd_str_bytes = "foo:1|c".as_bytes();
        let result = filter(&block_list, &statsd_str_bytes).join("\n");
        assert_eq!("foo:1|c", result);
    }

    #[test]
    fn test_should_block_completely_single_metric() {
        let block_list = vec![String::from("foo"), String::from("otherfoo")];
        let statsd_str_bytes = "foo:1|c".as_bytes();
        let result = filter(&block_list, &statsd_str_bytes).join("\n");
        assert_eq!("", result);
    }

    #[test]
    fn test_should_block_completely_multi_metric() {
        let block_list = vec![String::from("foo"), String::from("otherfoo")];
        let statsd_str_bytes = "foo:1|c\nfoo:2|c\nfoo:3|c".as_bytes();
        let result = filter(&block_list, &statsd_str_bytes).join("\n");
        assert_eq!("", result);
    }

    #[test]
    fn test_should_block_partially_multi_metric() {
        let block_list = vec![String::from("foo"), String::from("otherfoo")];
        let statsd_str_bytes = "notfoo:1|c\nfoo:2|c\nnotfoo:3|c".as_bytes();
        let result = filter(&block_list, &statsd_str_bytes).join("\n");
        assert_eq!("notfoo:1|c\nnotfoo:3|c", result);
    }
}

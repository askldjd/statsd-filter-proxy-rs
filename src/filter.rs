use std::str;

pub fn should_be_blocked(block_list: &Vec<String>, buf: &[u8]) -> bool {
    let statsd_str = unsafe { str::from_utf8_unchecked(&buf) };

    for prefix in block_list.iter() {
        if statsd_str.starts_with(prefix) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_block() {
        let block_list = vec![String::from("foo"), String::from("otherfoo")];
        let statsd_str_bytes = "foo:1|c".as_bytes();
        assert_eq!(true, should_be_blocked(&block_list, &statsd_str_bytes));
    }

    #[test]
    fn test_should_not_block() {
        let block_list = vec![String::from("notfoo"), String::from("otherfoo")];
        let statsd_str_bytes = "foo:1|c".as_bytes();
        assert_eq!(false, should_be_blocked(&block_list, &statsd_str_bytes));
    }
}

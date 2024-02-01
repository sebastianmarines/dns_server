pub fn fqdn_to_vec(fqdn: &str) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    for part in fqdn.split('.') {
        buf.push(part.len() as u8);
        for c in part.chars() {
            buf.push(c as u8);
        }
    }
    buf.push(0);
    return buf;
}

pub fn vec_to_fqdn(buf: &[u8]) -> (String, usize) {
    let mut fqdn = String::new();
    let mut i = 0;
    loop {
        let len = buf[i] as usize;
        if len == 0 {
            break;
        }
        if fqdn.len() > 0 {
            fqdn.push('.');
        }
        fqdn.push_str(std::str::from_utf8(&buf[i + 1..i + 1 + len]).unwrap());
        i += len + 1;
    }
    return (fqdn, i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fqdn_to_vec() {
        let fqdn = "facebook.com";
        let buf = fqdn_to_vec(fqdn);
        assert_eq!(
            buf,
            vec![8, 102, 97, 99, 101, 98, 111, 111, 107, 3, 99, 111, 109, 0]
        );
    }

    #[test]
    fn test_vec_to_fqdn() {
        let buf = vec![8, 102, 97, 99, 101, 98, 111, 111, 107, 3, 99, 111, 109, 0];
        let (fqdn, _) = vec_to_fqdn(&buf);
        assert_eq!(fqdn, "facebook.com");
    }
}

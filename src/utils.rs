use crate::dns_header::DNSHeader;
use crate::dns_header::Flags;
use crate::dns_question::DNSQuestion;
use base64::{engine::general_purpose, Engine as _};
use std::net::UdpSocket;

#[non_exhaustive]
pub struct RecordType;

impl RecordType {
    pub const A: u16 = 1;
    pub const NS: u16 = 2;
    pub const CNAME: u16 = 5;
}

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

pub fn vec_to_fqdn(buf: &[u8], p: usize) -> (String, usize) {
    let mut fqdn = String::new();
    let mut i = p;
    loop {
        let len = buf[i] as usize;
        // Check if the first two bits are set, which means that the next 14 bits are an offset
        if len & 0xc0 == 0xc0 {
            let offset = (((len as u16) & 0x3f) << 8) | buf[i + 1] as u16;
            let (new_fqdn, _) = vec_to_fqdn(buf, offset as usize);
            fqdn.push_str(&new_fqdn);
            // Since this is an offset, it doesn't end with a null byte, so we just increment the index by 1
            i += 1;
            break;
        }
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

pub fn base64_to_vec(encoded_string: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(encoded_string.as_bytes())
}

// Recursive query to a nameserver
pub fn query_ns(fqdn: &str) -> Result<Vec<u8>, String> {
    let root_nameservers = vec![
        "198.41.0.4",    // a.root-servers.net
        "170.247.170.2", // b.root-servers.net
    ];

    // Create header
    let header = DNSHeader {
        id: 0,
        flags: Flags::QUERY | Flags::RECURSION_DESIRED,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };

    let question = DNSQuestion {
        qname: fqdn.parse().unwrap(),
        qtype: RecordType::A,
        qclass: 1,
    };

    let header_buf = header.build(None);
    let question_buf = question.build();
    let mut buf: Vec<u8> = Vec::new();
    buf.extend(header_buf);
    buf.extend(question_buf);
    for ns in root_nameservers {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");
        socket
            .connect(format!("{}:53", ns))
            .expect("Could not connect to nameserver");
        socket.send(&buf).expect("Could not send data");
        let mut response_buf = [0; 1024];
        let (_amt, _src) = socket
            .recv_from(&mut response_buf)
            .expect("Could not receive data");
        return Ok(response_buf.to_vec());
    }
    return Ok(vec![0]);
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
        let (fqdn, _) = vec_to_fqdn(&buf, 0);
        assert_eq!(fqdn, "facebook.com");
    }

    #[test]
    fn test_query_ns() {
        let response = query_ns("facebook.com").unwrap();
        let response = crate::dns_response::parse_response(&response);
        assert_eq!(response.answers.len(), 0);
        assert_ne!(response.authorities.len(), 0);
        assert_ne!(response.additionals.len(), 0);
    }
}

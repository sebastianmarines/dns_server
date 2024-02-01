pub struct DNSQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl DNSQuestion {
    pub fn build(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for part in self.qname.split('.') {
            buf.push(part.len() as u8);
            for c in part.chars() {
                buf.push(c as u8);
            }
        }
        buf.push(0);
        buf.push((self.qtype >> 8) as u8);
        buf.push(self.qtype as u8);
        buf.push((self.qclass >> 8) as u8);
        buf.push(self.qclass as u8);
        return buf;
    }
}

pub fn parse_questions(buf: &[u8], qdcount: u16) -> (Vec<DNSQuestion>, usize) {
    let mut questions: Vec<DNSQuestion> = Vec::new();
    let mut i = 0;
    for _ in 0..qdcount {
        let mut qname = String::new();
        loop {
            let len = buf[i] as usize;
            if len == 0 {
                i += 1;
                break;
            }
            if qname.len() > 0 {
                qname.push('.');
            }
            qname.push_str(std::str::from_utf8(&buf[i + 1..i + 1 + len]).unwrap());
            i += len + 1;
        }
        let qtype = ((buf[i] as u16) << 8) | buf[i + 1] as u16;
        let qclass = ((buf[i + 2] as u16) << 8) | buf[i + 3] as u16;
        i += 4;
        questions.push(DNSQuestion {
            qname: qname,
            qtype: qtype,
            qclass: qclass,
        });
    }
    return (questions, i);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_questions() {
        // 0866616365626f6f6b03636f6d0000010001
        let buf = vec![
            8, 102, 97, 99, 101, 98, 111, 111, 107, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        let (questions, _) = parse_questions(&buf, 1);
        assert_eq!(questions.len(), 1);
        assert_eq!(questions[0].qname, "facebook.com");
        assert_eq!(questions[0].qtype, 1);
        assert_eq!(questions[0].qclass, 1);
    }

    #[test]
    fn test_build_question() {
        let question = DNSQuestion {
            qname: String::from("facebook.com"),
            qtype: 1,
            qclass: 1,
        };
        let buf = question.build();
        assert_eq!(
            buf,
            vec![8, 102, 97, 99, 101, 98, 111, 111, 107, 3, 99, 111, 109, 0, 0, 1, 0, 1]
        );
    }
}

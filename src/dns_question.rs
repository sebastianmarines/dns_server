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

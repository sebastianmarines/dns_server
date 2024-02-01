use std::net::UdpSocket;

struct DNSHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

struct DNSQuestion {
    qname: String,
    qtype: u16,
    qclass: u16,
}

struct DNSResourceRecord {
    name: String,
    rtype: u16,
    rclass: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>,
}

impl DNSHeader {
    fn build(&self, ancount: Option<u16>) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        buf.push((self.id >> 8) as u8);
        buf.push(self.id as u8);
        buf.push((self.flags >> 8) as u8);
        buf.push(self.flags as u8);
        buf.push((self.qdcount >> 8) as u8);
        buf.push(self.qdcount as u8);
        buf.push((ancount.unwrap_or(self.ancount) >> 8) as u8);
        buf.push(ancount.unwrap_or(self.ancount) as u8);
        buf.push((self.nscount >> 8) as u8);
        buf.push(self.nscount as u8);
        buf.push((self.arcount >> 8) as u8);
        buf.push(self.arcount as u8);
        return buf;
    }
}

impl DNSQuestion {
    fn build(&self) -> Vec<u8> {
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

impl DNSResourceRecord {
    fn build(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        for part in self.name.split('.') {
            buf.push(part.len() as u8);
            for c in part.chars() {
                buf.push(c as u8);
            }
        }
        buf.push(0);
        buf.push((self.rtype >> 8) as u8);
        buf.push(self.rtype as u8);
        buf.push((self.rclass >> 8) as u8);
        buf.push(self.rclass as u8);
        buf.push((self.ttl >> 24) as u8);
        buf.push((self.ttl >> 16) as u8);
        buf.push((self.ttl >> 8) as u8);
        buf.push(self.ttl as u8);
        buf.push((self.rdlength >> 8) as u8);
        buf.push(self.rdlength as u8);
        buf.extend(self.rdata.clone());
        return buf;
    }
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:5003").expect("Could not bind socket");
    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf).expect("Could not receive data");

        // Print hex values
        for i in &buf[..amt] {
            print!("{:02x} ", i);
        }
        println!();

        // Parse the DNS header
        let mut header = DNSHeader {
            id: ((buf[0] as u16) << 8) | buf[1] as u16,
            flags: ((buf[2] as u16) << 8) | buf[3] as u16,
            qdcount: ((buf[4] as u16) << 8) | buf[5] as u16,
            ancount: ((buf[6] as u16) << 8) | buf[7] as u16,
            nscount: ((buf[8] as u16) << 8) | buf[9] as u16,
            arcount: ((buf[10] as u16) << 8) | buf[11] as u16,
        };

        // Parse the DNS questions
        let mut questions: Vec<DNSQuestion> = Vec::new();

        let mut i = 12;

        for _ in 0..header.qdcount {
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

        // Create fake record
        let record = DNSResourceRecord {
            name: questions[0].qname.clone(),
            rtype: 1,
            rclass: 1,
            ttl: 60,
            rdlength: 4,
            rdata: vec![127, 0, 0, 1],
        };

        header.ancount = 1;
        header.flags |= 0x8000;
        header.arcount = 0;

        let header_buf = header.build(None);
        let question_buf = questions[0].build();
        let record_buf = record.build();
        let mut response_buf: Vec<u8> = Vec::new();
        response_buf.extend(header_buf);
        response_buf.extend(question_buf);
        response_buf.extend(record_buf);

        for i in &response_buf {
            print!("{:02x} ", i);
        }
        println!();

        socket
            .send_to(&response_buf, src)
            .expect("Could not send data");
    }
}

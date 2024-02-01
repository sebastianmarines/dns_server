pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DNSHeader {
    pub fn build(&self, ancount: Option<u16>) -> Vec<u8> {
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

pub fn parse_header(buf: &[u8]) -> DNSHeader {
    return DNSHeader {
        id: ((buf[0] as u16) << 8) | buf[1] as u16,
        flags: ((buf[2] as u16) << 8) | buf[3] as u16,
        qdcount: ((buf[4] as u16) << 8) | buf[5] as u16,
        ancount: ((buf[6] as u16) << 8) | buf[7] as u16,
        nscount: ((buf[8] as u16) << 8) | buf[9] as u16,
        arcount: ((buf[10] as u16) << 8) | buf[11] as u16,
    };
}

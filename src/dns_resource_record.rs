pub struct DNSResourceRecord {
    pub name: String,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl DNSResourceRecord {
    pub fn build(&self) -> Vec<u8> {
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

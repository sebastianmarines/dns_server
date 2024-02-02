use crate::utils::{fqdn_to_vec, vec_to_fqdn};

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
        let mut buf = fqdn_to_vec(&self.name);
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

pub fn parse_records(buf: &[u8], offset: usize, n: u16) -> (Vec<DNSResourceRecord>, usize) {
    let mut records: Vec<DNSResourceRecord> = Vec::new();
    let mut i = offset;
    for _ in 0..n {
        let (name, new_i) = vec_to_fqdn(&buf, i);
        i = new_i + 1;
        let rtype = ((buf[i] as u16) << 8) | buf[i + 1] as u16;
        i += 2;
        let rclass = ((buf[i] as u16) << 8) | buf[i + 1] as u16;
        i += 2;
        let ttl = ((buf[i] as u32) << 24)
            | ((buf[i + 1] as u32) << 16)
            | ((buf[i + 2] as u32) << 8)
            | buf[i + 3] as u32;
        i += 4;
        let rdlength = ((buf[i] as u16) << 8) | buf[i + 1] as u16;
        i += 2;
        let rdata = buf[i..i + rdlength as usize].to_vec();
        i += rdlength as usize;
        records.push(DNSResourceRecord {
            name,
            rtype,
            rclass,
            ttl,
            rdlength,
            rdata,
        });
    }
    return (records, i);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::base64_to_vec;

    #[test]
    fn test_build_resource_record() {
        let record = DNSResourceRecord {
            name: String::from("facebook.com"),
            rtype: 1,
            rclass: 1,
            ttl: 60,
            rdlength: 4,
            rdata: vec![127, 0, 0, 1],
        };
        let buf = record.build();
        assert_eq!(
            buf,
            vec![
                8, 102, 97, 99, 101, 98, 111, 111, 107, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 0,
                60, 0, 4, 127, 0, 0, 1,
            ]
        );
    }

    #[test]
    fn test_parse_records() {
        let buf = vec![
            8, 102, 97, 99, 101, 98, 111, 111, 107, 3, 99, 111, 109, 0, 0, 1, 0, 1, 0, 0, 0, 60, 0,
            4, 127, 0, 0, 1,
        ];
        let (records, _) = parse_records(&buf, 0, 1);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "facebook.com");
        assert_eq!(records[0].rtype, 1);
        assert_eq!(records[0].rclass, 1);
        // assert_eq!(records[0].ttl, 0);
        assert_eq!(records[0].rdlength, 4);
        assert_eq!(records[0].rdata, vec![127, 0, 0, 1]);
    }

    #[test]
    fn test_parse_compressed() {
        let buf = base64_to_vec("OOaBgAABAAEAAAAACGZhY2Vib29rA2NvbQAAAQABwAwAAQABAAAAJwAEnfAZIw==")
            .unwrap();
        let (records, _) = parse_records(&buf, 0x1E, 1);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name, "facebook.com");
        assert_eq!(records[0].rtype, 1);
        assert_eq!(records[0].rclass, 1);
        assert_eq!(records[0].ttl, 39);
    }
}

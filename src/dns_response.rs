use crate::dns_header::{parse_header, DNSHeader};
use crate::dns_question::{parse_questions, DNSQuestion};
use crate::dns_resource_record::{parse_records, DNSResourceRecord};

pub struct DNSResponse {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSResourceRecord>,
    pub authorities: Vec<DNSResourceRecord>,
    pub additionals: Vec<DNSResourceRecord>,
}

pub fn parse_response(buf: &[u8]) -> DNSResponse {
    let header = parse_header(&buf[..12]);
    let (questions, offset) = parse_questions(&buf[12..], header.qdcount);
    let (answers, offset) = parse_records(&buf[offset..], header.ancount);
    let (authorities, offset) = parse_records(&buf[offset..], header.nscount);
    let (additionals, _) = parse_records(&buf[offset..], header.arcount);

    return DNSResponse {
        header,
        questions,
        answers,
        authorities,
        additionals,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::base64_to_vec;

    #[test]
    fn test_parse() {
        let buf = base64_to_vec("OOYBIAABAAAAAAAACGZhY2Vib29rA2NvbQAAAQAB").unwrap();
        let response = parse_response(&buf);
        assert_eq!(response.questions.len(), 1);
        assert_eq!(response.questions[0].qname, "facebook.com");
        assert_eq!(response.questions[0].qtype, 1);
        assert_eq!(response.questions[0].qclass, 1);
        assert_eq!(response.header.ancount, 0);
        assert_eq!(response.header.nscount, 0);
        assert_eq!(response.header.arcount, 0);
        assert_eq!(response.answers.len(), 0);
        assert_eq!(response.authorities.len(), 0);
        assert_eq!(response.additionals.len(), 0);
    }

    #[test]
    fn test_parse_compressed() {
        let buf = base64_to_vec("OOaBgAABAAEAAAAACGZhY2Vib29rA2NvbQAAAQABwAwAIAABAAEAAAAnAASd8Bkj")
            .unwrap();
        let response = parse_response(&buf);
        assert_eq!(response.questions.len(), 1);
        assert_eq!(response.questions[0].qname, "facebook.com");
        assert_eq!(response.questions[0].qtype, 1);
        assert_eq!(response.questions[0].qclass, 1);
        assert_eq!(response.header.ancount, 1);
        assert_eq!(response.header.nscount, 0);
        assert_eq!(response.header.arcount, 0);
        assert_eq!(response.answers.len(), 0);
        assert_eq!(response.authorities.len(), 0);
        assert_eq!(response.additionals.len(), 0);
    }
}

mod dns_header;
mod dns_question;
mod dns_resource_record;
mod utils;
use dns_resource_record::DNSResourceRecord;
use std::net::UdpSocket;
use utils::RecordType;

fn main() {
    let address = String::from("127.0.0.1");
    let port = 5003;

    let socket =
        UdpSocket::bind(format!("{}:{}", address, port)).expect("Could not bind to address");
    loop {
        let mut buf = [0; 1024];
        let (_amt, src) = socket.recv_from(&mut buf).expect("Could not receive data");

        let mut header = dns_header::parse_header(&buf[..12]);

        // Parse the DNS questions
        let (questions, _) = dns_question::parse_questions(&buf[12..], header.qdcount);

        // Create fake records
        let mut records: Vec<dns_resource_record::DNSResourceRecord> = Vec::new();
        for question in &questions {
            let (response_type, response_length, response_data) = match question.qtype {
                RecordType::A => (1, 4, vec![127, 0, 0, 1]),
                RecordType::CNAME => {
                    let data = utils::fqdn_to_vec("ns1.example.com");
                    (2, data.len() as u16, data)
                }
                RecordType::NS => {
                    let data = utils::fqdn_to_vec("cname.example.com");
                    (5, data.len() as u16, data)
                }
                _ => continue,
            };

            records.push(DNSResourceRecord {
                name: question.qname.clone(),
                rtype: response_type,
                rclass: 1,
                ttl: 60,
                rdlength: response_length,
                rdata: response_data,
            });
        }

        header.ancount = 1;

        // Set the response bit
        header.flags |= 0x8000;
        header.arcount = 0;

        let header_buf = header.build(None);
        let question_buf = questions[0].build();
        let mut response_buf: Vec<u8> = Vec::new();
        response_buf.extend(header_buf);
        response_buf.extend(question_buf);

        for record in &records {
            let record_buf = record.build();
            response_buf.extend(record_buf);
        }

        socket
            .send_to(&response_buf, src)
            .expect("Could not send data");
    }
}

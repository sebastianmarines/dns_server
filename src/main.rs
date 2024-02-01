mod dns_header;
mod dns_question;
mod dns_resource_record;
use dns_resource_record::DNSResourceRecord;
use std::net::UdpSocket;

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
            if question.qtype == 1 && question.qclass == 1 {
                records.push(DNSResourceRecord {
                    name: question.qname.clone(),
                    rtype: 1,
                    rclass: 1,
                    ttl: 60,
                    rdlength: 4,
                    rdata: vec![127, 0, 0, 1],
                });
            }
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

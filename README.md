# Rust DNS Server

This is a simple DNS server implementation written in Rust designed for educational purposes. It demonstrates the basic structure of DNS packets and how a DNS server might parse and respond to requests.

## Features

- Parse incoming DNS queries.
- Build DNS response packets with hardcoded values.
- Listen for UDP DNS requests on a specified address and port.
- Respond to DNS requests with an example A record.

## Project Structure

The project is organized into the following modules:

- `dns_header`: Handles the DNS header structure and methods.
- `dns_question`: Processes DNS question section.
- `dns_resource_record`: Creates DNS resource records for the response.
- `utils`: (Optional) Contains shared utility functions, if needed.

## Getting Started

To get started with this project:

1. Install Rust if you haven't already done so.
2. Clone this repository.
3. Navigate to the project directory.
4. Run `cargo build` to compile the program.
5. Execute `cargo run` to start the DNS server.

## Configuration

The server is currently set to listen on `127.0.0.1:5003`. To change this, modify the `bind` address in `main.rs`.

## Testing

To test the DNS server, use a DNS query tool like `dig` or `nslookup` and direct the queries to the local address and the port the server is listening on.

Example using `dig` command:

```bash
dig @127.0.0.1 -p 5003 example.com
```

# RustyCrate: A Deliberately Vulnerable Rust Web Application

**RustyCrate** is a fictional e-commerce platform for managing and selling virtual goods. This application has been deliberately created with numerous security vulnerabilities for educational purposes. Inspired by projects like OWASP Juice Shop and WebGoat, **RustyCrate** is designed to help developers and security researchers understand and identify common vulnerabilities in Rust web applications.

## Project Overview

RustyCrate covers vulnerabilities across multiple categories, including but not limited to:
- **SQL Injection**
- **Cross-Site Scripting (XSS)**
- **JWT Mismanagement**
- **Path Traversal**
- **File Handling Vulnerabilities**
- **Cookie Security Issues**
- **Improper Error Handling**
- **Cryptographic Weaknesses**
- **PCI Data Exposure**
- **Server-Side Request Forgery (SSRF)**
- **Open Redirects**
- **Unsafe Reflection**

## Key Features
- **Modular Structure**: The application is broken into multiple files, each containing specific vulnerabilities.
- **Educational Purpose**: The goal of RustyCrate is to provide a hands-on experience for developers to practice identifying and mitigating security vulnerabilities in a real application.
- **Actix Web Framework**: Built using the [Actix](https://actix.rs/) web framework, making it a great way to explore vulnerabilities in a modern Rust web stack.

## Vulnerabilities List

- **SQL Injection**
- **Second-Order SQL Injection**
- **Stored and Reflected XSS**
- **Command Injection**
- **Path Traversal (Absolute and Relative)**
- **Arbitrary File Write**
- **JWT Signature Skipping and Hardcoded Secrets**
- **Insecure Cookie Attributes (SameSite, HttpOnly)**
- **Sensitive Data Exposure in Logs, URLs, and Files**
- **Improper Error Handling**
- **Parameter Tampering**
- **PCI Data Exposure in Error Messages**
- **Server-Side Request Forgery (SSRF)**
- **Open Redirects**
- **Log Forging**
- **Unsafe Reflection**
  
## Setup Instructions

### Prerequisites
- Rust (install from [here](https://www.rust-lang.org/tools/install))
- PostgreSQL (for database-related vulnerabilities)
  
### Steps to Run Locally
1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/rustycrate.git
   cd rustycrate
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Set Up PostgreSQL**
   - Ensure that PostgreSQL is installed and running.
   - Create a new database:
     ```bash
     createdb rustycrate_db
     ```
   - Update the connection string in `db.rs` to point to your PostgreSQL instance.

4. **Run the Application**
   ```bash
   cargo run
   ```

   The application should now be running on `localhost:8080`.

### Testing Vulnerabilities
Visit different endpoints in the application to trigger vulnerabilities:
- **SQL Injection**: `/find_product?name=<input>`
- **XSS (Reflected)**: `/reflected_xss?input=<script>alert('XSS')</script>`
- **Command Injection**: `/execute?cmd=<input>`
- **Path Traversal**: `/read_file?path=../../etc/passwd`
- **Open Redirect**: `/redirect?url=http://malicious-site.com`

### Important Disclaimer

**RustyCrate** is meant for educational and research purposes only. Do **not** deploy this application in a production environment, as it is intentionally insecure. The application contains vulnerabilities that could compromise the security of any system on which it is run.

## Contributing

Feel free to contribute to RustyCrate by submitting issues or pull requests. For major changes, please open an issue to discuss your proposal.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For questions or support, reach out via GitHub issues or email me at Sean.Carroll@checkmarx.com

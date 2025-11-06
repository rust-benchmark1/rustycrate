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


# Vulnerabilities Overview
This project contains some vulnerabilities, but there is no main.rs or lib.rs, so the functions are never called and we cannot confirm the presence of user input.

## /src/auth.rs
**Example 1** - CWE-321: Use of Hard-coded Cryptographic Key (Not supported)

Not expected to be detected, the function is never called.
- **Source:** Line 12
- **Sink:** Line 22  

**Example 2** - CWE-321: Use of Hard-coded Cryptographic Key (Not supported)

Not expected to be detected, the function is never called.
- **Source:** Line 12
- **Sink:** Line 27 

**Example 3** - CWE-613: Insufficient Session Expiration (Not supported)

Not expected to be detected, the function is never called.
- **Source:** Line 16
- **Sink:** Line 22 

**Example 4** - CWE-345: Insufficient Verification of Data Authenticity (Not supported)

Not expected to be detected, the function is never called.
- **Source:** Line 31
- **Sink:** Line 34

---

## /src/command.rs
**Example 1** - CWE-78: OS Command Injection (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 4
- **Sink:** Line 5  

---

## /src/db.rs
**Example 1** - CWE-89: SQL Injection (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 5
- **Sink:** Line 7  

**Example 2** - CWE-89: SQL Injection (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 12
- **Sink:** Line 13

**Example 3** - CWE-89: SQL Injection (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 23
- **Sink:** Line 25

**Example 4** - CWE-78: OS Command Injection (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 34
- **Sink:** Line 35

---

## /src/file_handler.rs
**Example 1** - CWE-22: Path Traversal (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 6
- **Sink:** Line 7  

**Example 2** - CWE-22: Path Traversal (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 13
- **Sink:** Line 14 

**Example 3** - CWE-22: Path Traversal (Supported)

Not expected to be detected, the function is never called.
- **Source:** Line 13
- **Sink:** Line 16  


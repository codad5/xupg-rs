
---

# **xupg-rs**

**xupg-rs** is a unified CLI tool for managing and updating versions of **PHP, MySQL, Node.js**, and more. Whether you use standalone tools or environments like **XAMPP, Laragon**, or **Node.js**, xupg-rs helps you stay organized by simplifying version management.

---

## **Features**

- List installed and available tool versions (locally and online).
- Install specific versions of tools to custom paths.
- Manage PHP versions within XAMPP seamlessly.
- Support for multiple environments: XAMPP, Laragon, Node.js, etc.

---

## **Installation**

### Option 1: Install via `cargo` from Crates.io  
(Requires Rust and Cargo installed on your machine.)  

```bash
cargo install xupg-rs
```

Verify the installation:

```bash
xupg --version
```

---

### Option 2: Build and Install from Source

If you want to build the project from the source:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/codad5/xupg-rs.git
   cd xupg-rs
   ```

2. **Build and install the project with Cargo:**

   ```bash
   cargo install --path .
   ```

   This will build the binary and place it in your Cargo bin directory (`~/.cargo/bin` by default).

3. **Verify the installation:**

   ```bash
   xupg --version
   ```

   If Cargo’s bin directory isn’t in your `PATH`, you may need to add it:  

   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

---

## **Usage**

Here are some common commands for managing tools using `xupg-rs`.

### 1. **List Available Versions**

```bash
xupg list -p
```

- `-p, --php`: List all available PHP versions.
- `-o, --online`: Fetch online versions for download.

Example:

```bash
xupg list -p -o
```

This lists online-available PHP versions.

```bash
xupg list -p
```
This lists locally-installed PHP versions.

---

### 2. **Install a Specific Tool Version**

```bash
xupg install -p <version> -pa <path>
```

- `-p, --php <version>`: Install a specific PHP version.
- `-pa, --path <path>`: (Optional) Specify installation path.

Example:

```bash
xupg install -p 8.1.0
```

This installs PHP version 8.1.0 to the default path.

---

### 3. **Set PHP Version for XAMPP**

```bash
xupg xampp php -s <version> -p <path>
```

- `-s, --set <version>`: Set a specific PHP version for XAMPP.
- `-p, --path [path]`: (Optional) Specify the XAMPP path.

Example:

```bash
xupg xampp php -s 8.0.0 -p c:/zampp/
```

This sets PHP version 8.0.0 as the default for XAMPP installed at `c:/zampp/`.

---

### 4. **Install and Set PHP for XAMPP Automatically**

To install PHP 7.4.0 directly into XAMPP’s PHP directory:

```bash
xupg xampp php -s 7.4.0
```
This would install PHP 7.4.0 and set it as the default PHP version for XAMPP.
- On Windows, the default XAMPP path is `C:\xampp`.
- On Linux, the default XAMPP path is `/opt/lampp`.


---

---

## **Dependencies**

- **`cached`**: Caches data for improved performance.
- **`colored`**: Adds color to terminal output.
- **`dirs-next`**: Manages directories and paths.
- **`fli`**: A framework for building CLI applications.
- **`indicatif`**: Displays progress bars during installation.
- **`reqwest`**: Handles HTTP requests (e.g., fetching online versions).
- **`serde`** and **`serde_json`**: Parse JSON data.
- **`tokio`**: Supports asynchronous operations.
- **`zip`**: Manages compressed files during downloads.

---

## **Contributing**

1. **Fork the repository** on GitHub.
2. **Create a new branch**:  
   ```bash
   git checkout -b feature-branch
   ```
3. **Make your changes and commit**:  
   ```bash
   git commit -m "Add new feature"
   ```
4. **Push your branch**:  
   ```bash
   git push origin feature-branch
   ```
5. **Open a Pull Request** on GitHub.

---

## **License**

This project is licensed under the **MIT License**. See the `LICENSE` file for more information.

---

## **Contact**

- **GitHub**: [codad5](https://github.com/codad5)  
- **Portfolio**: [https://codad5.me](https://codad5.me)  
- **Twitter**: [@codad5_](https://twitter.com/codad5_)

---

## **Future Improvements**

- Add **Docker support** for managing tools in containers.
- Enhance **Windows support** for Laragon integration.
- Promote the tool on **`crates.io`** for community feedback and adoption.

---

With **xupg-rs**, managing versions across environments becomes straightforward and efficient. Install it today and simplify your workflow! 🚀

---

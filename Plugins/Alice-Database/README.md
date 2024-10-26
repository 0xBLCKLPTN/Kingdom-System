<div align="center">
  <h1> Alice DBMS</h1>
  <img src="https://github.com/0xBLCKLPTN/Kingdom-System/blob/alice_database-dev/Docs/illustrations/alice_db_logo.png" alt="Alice DBMS Logo"/>
  
  <!-- Badges with better aesthetics -->
  <p>
    <a href="https://bazel.build" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/Bazel-Supported-brightgreen?style=flat-square" alt="Bazel Supported"/>
    </a>
    <a href="https://nixos.org/nix/" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/Nix-Supported-blue?style=flat-square" alt="Nix Supported"/>
    </a>
    <a href="https://en.wikipedia.org/wiki/Cross-platform" style="text-decoration: none;">
      <img src="https://img.shields.io/badge/Cross%20Platform-Yes-orange?style=flat-square" alt="Cross Platform"/>
    </a>
  </p>
</div>

## 🌟 Features

- **JSON Data Handling**: Efficiently store and manipulate JSON documents.
- **Structured Management**: Organize documents into collections for better management.
- **Cross-Platform**: Compatible across multiple operating systems.
- **Extensible**: Easily extend the functionality to meet your specific use cases.
- **ROBUST**: Leveraging Serde for powerful serialization and deserialization.

## 📦 Installation

To set up Alice DBMS, ensure you have Rust and Cargo installed. You can then add it to your project by including it in your `Cargo.toml`.

```toml
[dependencies]
AliceDBMS = "^1.2.7"
```

Then, run:

```bash
cargo build
```

## 🚀 Getting Started

Here's a brief example of how to use the Alice DBMS module:

```rust
use AliceDBMS::prelude::*;

fn main() -> std::io::Result<()> {
    let mut instance_manager = InstanceManager::new(&get_root_path());
    cli(&mut instance_manager);
    Ok(())
}
```

## 🔍 Testing

Alice DBMS comes with a suite of tests to ensure functionality. You can run the tests using:

```bash
cargo test
```

### Example Tests

These tests cover the creation, updating, and deletion of documents and collections. For example:

```rust
#[test]
fn test_create_and_add_document() {
    // Test implementation here...
}
```

## 🤝 Contributing

Contributions are welcome! Here’s how you can help:

1. Open an issue if you have a bug to report or a suggestion.
2. Fork the repository.
3. Create a new branch for your feature or bug fix.
4. Submit a pull request.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 📧 Contact

For questions or feedback, please reach out to [blcklptn@icloud.com](mailto:blcklptn@icloud.com).

---

<div align="center">
  <p>Happy coding with Alice DBMS! 🚀</p>
</div>
```

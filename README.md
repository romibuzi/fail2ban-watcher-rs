fail2ban-watcher-rs
=====

Display the most banned countries and ips from a [fail2ban](https://www.fail2ban.org) database.

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/7426cee063f0441282c9ab7cf9d9001b)](https://app.codacy.com/gh/romibuzi/fail2ban-watcher-rs/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

### Demo

![demo](demo.gif)

## Requirements

Rust : https://www.rust-lang.org/tools/install

### Build

```bash
cargo build --release
```

### Run

```bash
.target/release/fail2ban-watcher-rs --help
Usage: fail2ban-watcher-rs [OPTIONS]

Options:
  -f, --fail2ban-db-path <FAIL2BAN_DB_PATH>
          fail2ban database to analyze [default: /var/lib/fail2ban/fail2ban.sqlite3]
  -d, --display-limit <DISPLAY_LIMIT>
          number of elements to display [default: 10]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Tests

```bash
cargo test
```

### Cross compilation

```bash
cargo install cross
TARGET=x86_64-unknown-linux-gnu # https://doc.rust-lang.org/stable/rustc/platform-support.html
cross build --release --target=$TARGET

# then run the binary
./target/$TARGET/release/fail2ban-watcher-rs
```

### Credits

This project includes IP2Location LITE data available from http://www.ip2location.com

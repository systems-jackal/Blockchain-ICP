# 🌐 Blockchain ICP Monorepo

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![ICP](https://img.shields.io/badge/ICP-0.32.0-blue)](https://internetcomputer.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![GitHub last commit](https://img.shields.io/github/last-commit/YOUR_USERNAME/blockchain_icp)](https://github.com/YOUR_USERNAME/blockchain_icp)

A collection of Internet Computer Protocol (ICP) projects built in Rust. This monorepo contains all my ICP canisters, shared libraries, and development tools.

## 📁 Projects

| Project | Description | Status | Canisters |
| :--- | :--- | :--- | :--- |
| [triple_compass](./triple_compass/) | Triple-redundant storage with majority voting | 🟢 Active | 3 |
| *Coming soon* | Fintech application | ⏳ Planned | TBD |
| *Coming soon* | DAO governance protocol | ⏳ Planned | TBD |

### Project Details

#### 🧭 Triple Compass
A fault-tolerant storage system implementing the "triple compass principle" — three independent canisters that store the same data. If one fails or is corrupted, the other two maintain the truth.

**Features:**
- Three independent canisters (compass_a, compass_b, compass_c)
- CRUD operations (put, get, get_all)
- Query with filters and pagination
- Owner-based access control via Internet Identity
- TTL (time-to-live) for auto-expiring data
- Batch operations support

**Canister IDS (Local Deployment):**
- compass_a: `ufxgi-4p777-77774-qaadq-cai`
- compass_b: `vizcg-th777-77774-qaaea-cai`
- compass_c: `vpyes-67777-77774-qaaeq-cai`

## 🚀 Quick Start

### Prerequisites

| Requirement | Version | Installation |
| :--- | :--- | :--- |
| Rust | 1.70+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` |
| dfx (ICP SDK) | 0.32+ | `sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"` |
| WebAssembly target | - | `rustup target add wasm32-unknown-unknown` |
| candid-extractor | latest | `cargo install candid-extractor` |

### Clone and Deploy

```bash
# Clone the monorepo
git clone https://github.com/systems-jackal/triple-compass.git
cd blockchain_icp

# Start local ICP replica
dfx start --background

# Deploy a specific project
cd triple_compass
dfx deploy

# Or deploy all projects (when scripts are set up)
# ./scripts/deploy_all.sh
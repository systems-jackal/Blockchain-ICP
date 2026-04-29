### TRICOMPASS IAM
The problem is not the *intent* of KYC — verifying identity is legitimate and necessary. The problem is the *architecture*: every verification creates a new copy of sensitive identity data, stored in a silo controlled by a single institution, protected by credentials that can be phished, leaked, or brute-forced.

When that institution is breached, users have no recourse. They did not choose to trust that institution with their data forever. They simply had no alternative.

### The Structural Failures of Centralized IAM

Identity and Access Management (IAM) in the centralized model inherits three compounding weaknesses:

**1. Honeypot Concentration**
Centralizing identity data creates high-value targets. The more users an institution onboards through KYC, the more attractive its database becomes to attackers. Scale, in the centralized model, directly increases risk.

**2. Credential Dependency**
Access to identity data is controlled by credentials — admin passwords, API keys, service tokens. Credentials can be stolen, reused across systems, or leaked in plaintext. Once a credential is compromised, every identity record behind it is exposed.

**3. Trust Without Verification**
Users must trust that institutions are storing their identity data securely, retaining only what is necessary, and not sharing it without consent. This trust is unverifiable. There is no on-chain audit trail. There is no cryptographic proof of access. There is only policy — and policy can be violated silently.

The result is a system where **identity belongs to institutions, not to users** — and where every new KYC enrollment is another record waiting to be breached.

---

## How: Decentralized Truth Through Majority Consensus

**Triple Compass** shifts identity storage away from the centralized honeypot model. Instead of one institution holding one database behind one credential, it deploys **three independent canisters** on the Internet Computer Protocol (ICP) — each holding an identical copy of the same data.

No single canister is authoritative. No single point of compromise controls the truth.

### The Consensus Model

| Compass A | Compass B | Compass C | Verdict          |
| :-------- | :-------- | :-------- | :--------------- |
| North     | North     | North     | ✅ Trust: North  |
| North     | South     | North     | ✅ Trust: North  |
| East      | West      | North     | ⚠️ No consensus  |

If one canister is corrupted, manipulated, or goes offline, the remaining two preserve the truth. The system heals itself — no incident response, no manual reconciliation, no delayed disclosure.

### How Internet Identity Replaces the Credential Model

The credential — password, API key, session token — is the single most exploited element in centralized IAM. Internet Identity removes it from the equation entirely.

- Users authenticate via **passkeys** bound to their hardware device (Face ID, Touch ID, FIDO2 security keys)
- No password is created, stored, or transmitted — on either side of the connection
- Each application receives a **unique, anonymous principal ID** per user, preventing cross-platform identity correlation
- Private keys never leave the user's device — there is nothing on the server side to steal

| Authentication Model  | Credential Stored Server-Side? | Phishable? | Exposed in a Credential Breach? |
| :-------------------- | :----------------------------- | :--------- | :------------------------------ |
| Password + Hash       | ✅ Yes                         | ✅ Yes     | ✅ Yes                          |
| Password + MFA        | ✅ Yes                         | ✅ Partial | ✅ Yes                          |
| OAuth / SSO           | ✅ Yes (at provider)           | ✅ Yes     | ✅ Yes                          |
| **Internet Identity** | ❌ No                          | ❌ No      | ❌ No                           |

### What This Means for KYC Architecture

In a Triple Compass model, identity verification does not create a centralized honeypot. Verified identity state is distributed across three independent canisters, access is controlled by cryptographic principal IDs rather than passwords, and all write operations are authenticated on-chain — not at the application layer where they can be bypassed.

Users retain cryptographic proof of who accessed their identity data and when. Institutions can verify identity without holding it indefinitely.

---

## What: System Reference

### Deployed Canisters

| Canister    | Canister ID                    |
| :---------- | :----------------------------- |
| `compass_a` | `ufxgi-4p777-77774-qaadq-cai`  |
| `compass_b` | `vizcg-th777-77774-qaaea-cai`  |
| `compass_c` | `vpyes-67777-77774-qaaeq-cai`  |

### API Surface

| Function          | Call Type | Description                          |
| :---------------- | :-------- | :----------------------------------- |
| `put(key, value)` | Update    | Write a key-value pair to storage    |
| `get(key)`        | Query     | Retrieve a value by key              |
| `get_all()`       | Query     | Retrieve all stored key-value pairs  |

### Identity & Access Control

All mutations are gated by on-chain identity verification via `ic_cdk::caller()`. No write operation executes without a verified, non-anonymous principal.

#### Access Control Patterns

| Pattern            | Implementation                            | Defends Against                        |
| :----------------- | :---------------------------------------- | :------------------------------------- |
| Owner-only writes  | `assert(caller() == owner)`               | Unauthorized mutations                 |
| Role-based access  | Allowlist check against caller principal  | Privilege escalation                   |
| Reject anonymous   | `caller() == Principal::anonymous()`      | Unauthenticated write attempts         |
| Rate limiting      | Per-caller frequency tracking             | Abuse, spam, and replay flooding       |
| Public reads       | Open `get()` calls                        | Transparency and auditability          |

#### Reference Implementation:-(Rust)

**Owner-only mutation:-**
```rust
use ic_cdk::caller;

#[update]
fn put_owner_only(key: String, value: String) -> Result<(), String> {
    const OWNER: &str = "your-principal-id-here";

    if caller().to_text() != OWNER {
        return Err("Unauthorized: caller is not the canister owner.".to_string());
    }

    Ok(())
}
```

**Reject anonymous callers:**
```rust
#[update]
fn put_authenticated_only(key: String, value: String) -> Result<(), String> {
    if caller() == Principal::anonymous() {
        return Err("Authentication required: anonymous calls are not permitted.".to_string());
    }

    Ok(())
}
```

### Threat Model

| Threat Vector               | Centralized KYC / IAM                    | Triple Compass on ICP                                      |
| :-------------------------- | :---------------------------------------- | :--------------------------------------------------------- |
| Honeypot concentration      | Every enrolled user is at risk on breach  | No central store — data distributed across 3 canisters     |
| Credential theft            | Full access on single credential leak     | No credentials exist — passkey auth only                   |
| Unauthorized writes         | Application-layer enforcement (bypassable)| Enforced on-chain via `ic_cdk::caller()`                   |
| Single point of failure     | Total data loss on breach                 | Majority consensus preserves truth across 3 nodes          |
| Replay attacks              | Requires application-level nonces         | ICP enforces timestamps and nonces at the protocol level   |
| Frontend compromise         | Can override backend logic                | Backend verified on-chain — frontend is untrusted by design|
| Silent data corruption      | Requires manual audits and restores       | Auto-detected and healed via majority vote                 |
| Unverifiable access history | Dependent on institution's internal logs  | On-chain audit trail — cryptographically verifiable        |

---

*Built on the Internet Computer Protocol. Authenticated without passwords. Resilient without backups.*
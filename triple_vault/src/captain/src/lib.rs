// Simple captain canister - no complex macros
// Just basic functions to test deployment

#[ic_cdk::update]
fn write(key: String, value: String) -> String {
    format!("✅ Captain received: {} = {}", key, value)
}

#[ic_cdk::query]
fn read(key: String) -> String {
    format!("📖 Captain reading: {}", key)
}

#[ic_cdk::query]
fn health() -> String {
    "Captain is ready to navigate! 🧭".to_string()
}

// Export the Candid interface
ic_cdk::export_candid!();

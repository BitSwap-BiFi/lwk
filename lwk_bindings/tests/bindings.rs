#[cfg(feature = "foreign_bindings")]
uniffi::build_foreign_language_testcases!(
    "tests/bindings/custom_persister.py",
    "tests/bindings/list_transactions.py",
    "tests/bindings/list_transactions.kts"
);

---
name: TEST_FUNCTION_FOR_RUST_OVERRIDES
hash: 0xABCDEF01
return_type: Any
params:
  - name: p0
    type: int
    description: "Original parameter 0, an integer."
  - name: p1_some_struct_ptr
    type: Player*
    description: "Pointer to some player struct."
  - name: p2_any_type
    type: Any
    description: "Parameter that is 'Any' type."
  - name: p3_invalid_type_str
    type: const char*
    description: "Parameter whose Rust override type will be invalid."
  - name: p4_with_transform
    type: float
    description: "Parameter for testing input transformation."
  - name: p5_optional_val
    type: BOOL*
    description: "Optional output boolean."
category: test_category
---

This is a test native function to verify Rust-specific overrides from `native_configs.toml`.
It includes various parameter types and configurations to test renaming, type overriding,
prologue/epilogue code injection, and unsafe marking. 
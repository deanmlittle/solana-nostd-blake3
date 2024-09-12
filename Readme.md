# Solana NoStd blake3

A more efficient implementation of Blake3 for SVM.

# Installation

```cargo add solana-nostd-blake3```

# Features

- Adds `hash_ref` which takes in any type that implements `<AsRef<[u8]>>`
- No `Hash` struct. Returns `[u8;32]` directly.
- Makes use of MaybeUninit to skip zero allocations
- Adds `hash_into` to let you hash directly into a mutable buffer.

# Performance

| library        | function          | CU cost |
|----------------|-------------------|---------|
| nostd-blake3   | hashv(&[b"test"]) | 100     |
| nostd-blake3   | hash(b"test")     | 105     |
| nostd-blake3   | hash_ref("test")  | 105     |
| solana-program | hashv(&[b"test"]) | 120     |
| solana-program | hash(b"test")     | 123     |
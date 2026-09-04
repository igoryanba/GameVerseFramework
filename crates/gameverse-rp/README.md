# gameverse-rp

Server-authoritative domain rules and the PostgreSQL schema for the first closed-alpha vertical slice.

The crate deliberately does not accept plaintext passwords or refresh tokens. The server persistence adapter must hash passwords with Argon2id and store only hashed refresh tokens before calling this domain. Money, inventory changes, purchases, and job payouts are validated by the server and keyed for idempotency.

`migrations/0001_closed_alpha.sql` is the source of truth for the initial PostgreSQL schema. Runtime database wiring is kept outside this crate so gameplay rules remain independently testable.

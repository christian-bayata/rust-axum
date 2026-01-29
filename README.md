```sh
# Terminal 1 - to run the server.
cargo watch -q -c -w src/ -w .cargo/ -x run

# Terminal 2 - to run the quick_dev.
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

# Start the Postgres server docker image:
docker run --rm --name pg -p 5432:5432 -e POSTGRES_PASSWORD=welcome postgres:15

# (Optional) To have a psql terminal on pg.
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql


Serialization: Convert Rust → JSON (or another format)
Deserialization: Convert JSON → Rust

todo!() -> A built-in Rust macro that means "I haven’t finished this part yet."

/**
 * Explanation of Arc<Mutex<Vec<Option<Ticket>>>>
    Vec<Option<Ticket>>
        A list of tickets (some slots may be empty)
        Option<Ticket> allows:
            Some(ticket) → exists
            None → deleted slot

    Mutex<Vec<...>>
        Ensures only one thread can mutate at a time
        Prevents race conditions
        Equivalent to:
        “Only one request can write at a time”
        This avoids reindexing issues.

    Arc<Mutex<...>>
        Arc = Atomic Reference Count
        Allows multiple handlers/services to share the same state
        Thread-safe
*/

/*
    .lock().unwrap()
    lock() can fail only if another thread panicked while holding the lock
    unwrap() says: “If that happens, crash the program.”
    clone() needed because:
        You store it and
        Return it
*/

/*
    store.iter().filter_map(|t| t.clone()).collect() in Rust === store.filter(t => t !== null) in JS

    The goal is: Convert Vec<Option<Ticket>>
        ⟶ into Vec<Ticket>
        ⟶ skipping all None values
*/

# Tracing in Rust
    Tracing is a modern, structured, and async-aware way to observe what your program is doing at runtime — especially useful for servers, async code, and distributed systems.

'static means: “This value lives for the entire lifetime of the program”

The model/store flow chart:
APP LAYER
   ↓
MODEL LAYER  (business logic boundary)
   ↓
STORE LAYER  (database boundary)
   ↓
DATABASE

- store = low-level database access (SQLx, Postgres, pools, connections)

- model = business logic layer

- app/web = API layer

This separation is intentional and very professional architecture.
```

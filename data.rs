/* Start auto-reloading server: */
// cargo watch -q -c -w src/ -x run 
// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

// Serialization: Convert Rust → JSON (or another format)
// Deserialization: Convert JSON → Rust 

//todo!() -> A built-in Rust macro that means "I haven’t finished this part yet."

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
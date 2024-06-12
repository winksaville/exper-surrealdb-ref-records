# Experiment with SurrealDB ref records

Learn how to reference other records with in SurrealDB.

>Note: Used surrealdb v1.5.2. Also, there are commented out lines where I'm
trying different techniques and found that I couldn't use bind with "tables".

Creates a `location_tbl` with one entry and a `person_tbl` with references
from the wo records in `person_tbl` to the one record in the `location_tbl`.

The last `dbg!` prints persons which is all the records in person_tbl,
i.e. 2 records Big Dude and Little Dude.
```rust
wink@3900x 24-06-12T20:08:52.416Z:~/prgs/SurrealDB/exper-surrealdb-ref-records (main)
$ cargo run
   Compiling exper-surrealdb-ref-records v0.1.0 (/home/wink/prgs/SurrealDB/exper-surrealdb-ref-records)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.46s
     Running `target/debug/exper-surrealdb-ref-records`
[src/main.rs:33:5] &location_123_main_st_response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    290.216µs,
                ),
            },
            Err(
                Db(
                    CreateStatement {
                        value: "'location_tbl'",
                    },
                ),
            ),
        ),
    },
    live_queries: {},
}
[src/main.rs:38:5] &location_123_main_st_response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    664.449µs,
                ),
            },
            Ok(
                Object(
                    Object(
                        {
                            "address": Strand(
                                Strand(
                                    "123 Main St",
                                ),
                            ),
                            "id": Thing(
                                Thing {
                                    tb: "location_tbl",
                                    id: String(
                                        "uji2jtwefjxtmq0e9nyw",
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ),
    },
    live_queries: {},
}
[src/main.rs:41:5] &location_123_main_st_result = Some(
    Location {
        id: Thing {
            tb: "location_tbl",
            id: String(
                "uji2jtwefjxtmq0e9nyw",
            ),
        },
        address: "123 Main St",
    },
)
[src/main.rs:45:5] &location_123_main_st = Location {
    id: Thing {
        tb: "location_tbl",
        id: String(
            "uji2jtwefjxtmq0e9nyw",
        ),
    },
    address: "123 Main St",
}
[src/main.rs:54:5] &person_big_dude_response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    282.631µs,
                ),
            },
            Ok(
                Object(
                    Object(
                        {
                            "id": Thing(
                                Thing {
                                    tb: "person_tbl",
                                    id: String(
                                        "6mgxpmjdf1stalgss4h1",
                                    ),
                                },
                            ),
                            "location_rid": Thing(
                                Thing {
                                    tb: "location_tbl",
                                    id: String(
                                        "uji2jtwefjxtmq0e9nyw",
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Big Dude",
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    },
    live_queries: {},
}
[src/main.rs:56:5] &person_big_dude_result = Some(
    Person {
        id: Thing {
            tb: "person_tbl",
            id: String(
                "6mgxpmjdf1stalgss4h1",
            ),
        },
        name: "Big Dude",
        location_rid: Thing {
            tb: "location_tbl",
            id: String(
                "uji2jtwefjxtmq0e9nyw",
            ),
        },
    },
)
[src/main.rs:60:5] &person_big_dude = Person {
    id: Thing {
        tb: "person_tbl",
        id: String(
            "6mgxpmjdf1stalgss4h1",
        ),
    },
    name: "Big Dude",
    location_rid: Thing {
        tb: "location_tbl",
        id: String(
            "uji2jtwefjxtmq0e9nyw",
        ),
    },
}
[src/main.rs:69:5] &person_little_dude_response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    244.009µs,
                ),
            },
            Ok(
                Object(
                    Object(
                        {
                            "id": Thing(
                                Thing {
                                    tb: "person_tbl",
                                    id: String(
                                        "b3kd1iz4rdz0sd6s6qtj",
                                    ),
                                },
                            ),
                            "location_rid": Thing(
                                Thing {
                                    tb: "location_tbl",
                                    id: String(
                                        "uji2jtwefjxtmq0e9nyw",
                                    ),
                                },
                            ),
                            "name": Strand(
                                Strand(
                                    "Little Dude",
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    },
    live_queries: {},
}
[src/main.rs:71:5] &person_little_dude_result = Some(
    Person {
        id: Thing {
            tb: "person_tbl",
            id: String(
                "b3kd1iz4rdz0sd6s6qtj",
            ),
        },
        name: "Little Dude",
        location_rid: Thing {
            tb: "location_tbl",
            id: String(
                "uji2jtwefjxtmq0e9nyw",
            ),
        },
    },
)
[src/main.rs:75:5] &person_little_dude = Person {
    id: Thing {
        tb: "person_tbl",
        id: String(
            "b3kd1iz4rdz0sd6s6qtj",
        ),
    },
    name: "Little Dude",
    location_rid: Thing {
        tb: "location_tbl",
        id: String(
            "uji2jtwefjxtmq0e9nyw",
        ),
    },
}
[src/main.rs:80:5] &location_123_main_st_response = Some(
    Location {
        id: Thing {
            tb: "location_tbl",
            id: String(
                "uji2jtwefjxtmq0e9nyw",
            ),
        },
        address: "123 Main St",
    },
)
[src/main.rs:91:5] &persons_response = Response {
    client: Surreal {
        router: OnceLock(
            Router {
                sender: Sender,
                last_id: 0,
                features: {
                    LiveQueries,
                    Backup,
                },
            },
        ),
        engine: PhantomData<surrealdb::api::engine::any::Any>,
    },
    results: {
        0: (
            Stats {
                execution_time: Some(
                    135.625µs,
                ),
            },
            Ok(
                Array(
                    Array(
                        [
                            Object(
                                Object(
                                    {
                                        "id": Thing(
                                            Thing {
                                                tb: "person_tbl",
                                                id: String(
                                                    "6mgxpmjdf1stalgss4h1",
                                                ),
                                            },
                                        ),
                                        "location_rid": Thing(
                                            Thing {
                                                tb: "location_tbl",
                                                id: String(
                                                    "uji2jtwefjxtmq0e9nyw",
                                                ),
                                            },
                                        ),
                                        "name": Strand(
                                            Strand(
                                                "Big Dude",
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            Object(
                                Object(
                                    {
                                        "id": Thing(
                                            Thing {
                                                tb: "person_tbl",
                                                id: String(
                                                    "b3kd1iz4rdz0sd6s6qtj",
                                                ),
                                            },
                                        ),
                                        "location_rid": Thing(
                                            Thing {
                                                tb: "location_tbl",
                                                id: String(
                                                    "uji2jtwefjxtmq0e9nyw",
                                                ),
                                            },
                                        ),
                                        "name": Strand(
                                            Strand(
                                                "Little Dude",
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ],
                    ),
                ),
            ),
        ),
    },
    live_queries: {},
}
[src/main.rs:94:5] &persons = [
    Person {
        id: Thing {
            tb: "person_tbl",
            id: String(
                "6mgxpmjdf1stalgss4h1",
            ),
        },
        name: "Big Dude",
        location_rid: Thing {
            tb: "location_tbl",
            id: String(
                "uji2jtwefjxtmq0e9nyw",
            ),
        },
    },
    Person {
        id: Thing {
            tb: "person_tbl",
            id: String(
                "b3kd1iz4rdz0sd6s6qtj",
            ),
        },
        name: "Little Dude",
        location_rid: Thing {
            tb: "location_tbl",
            id: String(
                "uji2jtwefjxtmq0e9nyw",
            ),
        },
    },
]
wink@3900x 24-06-12T20:09:01.836Z:~/prgs/SurrealDB/exper-surrealdb-ref-records (main)
$
``` 


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

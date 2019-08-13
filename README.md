# simple_blockchain_rust
Creating simple blockchain system with Rust.

Now you can
- add messages to the registry - just run the program with `cargo run`
- check the registry - you can manually change one of the files (blocks) except the last one and run the program with `cargo run check` command which will check and find that corrupt file

I'm using Blake2b hashing for this project.

## How to add messages to the registry
The initial state
```
mem
└── 0.txt
```
Run the script with `cargo run`
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/simple_blockchain_rust`
Write message to save: This is message 1
Saved.
Write message to save: And this is message 2
Saved.
```
Now the state of mem is - each file is a block and is checking it's previous block with hash values.
```
mem
├── 0.txt
├── 1.txt
└── 2.txt
```
And this is what we have inside 1.txt
```
$ cat mem/1.txt
{"message":"This is message 1","prev_hash":"9327a492264ecac0806b031b780241d86cabe38348fe49c4c5a610ee584cfbaaefd3fdffd1b1b54c9ee225820433a7f902c688b2e123181a56c73b9cbf9cd13f"}
```

## Checking the registry
We can check the registry with `cargo run check` command (the last block is the tail and is not being tested by any other block, so that can be changed easily):
```
$ cargo run check
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/simple_blockchain_rust check`
2 -> tail
1 -> true
0 -> true
```

Not if we change anything in the 1.txt, here is what we would get:
```
// Change 1.txt content to {"message":"This is message 11","prev_hash":"9327a492264ecac0806b031b780241d86cabe38348fe49c4c5a610ee584cfbaaefd3fdffd1b1b54c9ee225820433a7f902c688b2e123181a56c73b9cbf9cd13f"}

cargo run check
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/simple_blockchain_rust check`
2 -> tail
1 -> false
```

So we see that the 1.txt file is corrupted!
And if we fix the file, it will pass the checking (change `...message 11` to `...message 1`).
```
$ cargo run check
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/simple_blockchain_rust check`
2 -> tail
1 -> true
0 -> true
```

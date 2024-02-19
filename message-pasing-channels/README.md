# **Message Passing Channels in Rust**

### Description

Implementations of message-passing channels in Rust:

- `mpsc_channel`: Utilizes an MPSC (multiple producer, single consumer) channel from the `std::sync::mpsc` module.
- `mpmc_channel`: Employs an MPMC (multiple producer, multiple consumer) channel from the `crossbeam` crate.

Both implementations demonstrate how to send and receive messages between multiple threads, calculate factorials concurrently, and store the results in a shared data structure.

### Dependencies

Both projects share the following dependencies:

- `slog`: For logging purposes.
- `slog-term`: Provides a terminal logger.
- `slog-async`: For building `Drain` for the `slog`.
- `num`: Facilitates working with large numbers using the `BigUint` type.

### Building and Running

1. Clone the repository:
    ```Bash
    git clone https://github.com/your-username/message-passing-channels.git
    ```
2. Navigate to the project directory:
    ```Bash
    cd message-passing-channels
    ```
3. Build the project:
    ```Bash
    cargo build --all
    ```
4. Run the desired project:
    ```Bash**
    cargo run --bin mpsc_channel
    ```
    ```Bash**
    cargo run --bin mpmc_channel
    ```


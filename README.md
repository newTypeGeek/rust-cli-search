# rust-cli-search

Rust Command Line Interface (CLI) to search text in a file

This repository is created by following [Chapter 12 - An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), and [Chapter 13.3 Improving Our I/O Project](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html)

# Getting Started
1. Install rust by following the offical website https://www.rust-lang.org/tools/install
2. Clone this repository to your local machine
    ```
    git clone git@github.com:newTypeGeek/rust-cli-search.git
    ```

3. Goto the project directory 
   ```
   cd /path/to/rust-cli-search
   ```

4. Build and run the program now :)
   ```
   cargo run -- to poem.txt
   ```
   It is expected to see this output
   ```
    Are you nobody, too?
    How dreary to be somebody!
   ```

# The CLI 
Here is a brief explaination on how to use this CLI
```
cargo run -- <text-you-want-to-search> <filepath>
```

You can also configure the tool to be **case insensitive**. This can be done by setting environment variable `IGNORE_CASE=1`. For example

```
IGNORE_CASE=1 cargo run -- to poem.txt
```
The output in this case is
```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
where we succesfully search lines containing `to` as well as `To`.



# Unit Tests
The unit tests are located in `src/lib.rs`. To run the unit tests, run this command
```
cargo test
```
It is expected all unit-tests are passed
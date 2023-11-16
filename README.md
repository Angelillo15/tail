```
  ______      _ __   _   __            __                 
 /_  __/___ _(_) /  / | / /___  ____  / /____  __________ 
  / / / __ `/ / /  /  |/ / __ \/ __ \/ //_/ / / / ___/ _ \
 / / / /_/ / / /  / /|  / /_/ / /_/ / ,< / /_/ / /  /  __/
/_/  \__,_/_/_/  /_/ |_/\____/\____/_/|_|\__,_/_/   \___/    

NookPanel Store Backend                
```

# Tail
This is the store backend that integrates the payments to the NookPanel store.
It is written in Rust and uses the [Actix](https://actix.rs/) framework.

## Building
```shell
cargo build --release
```

## Running
### Linux
```shell
./tail
```
### Windows
```shell
.\tail.exe
```

## Development
I recommend using [cargo-watch](https://github.com/watchexec/cargo-watch)
With [cargo-binstall](https://github.com/ryankurte/cargo-binstall):

```shell
cargo binstall cargo-watch
```

From source:

```shell
cargo install cargo-watch
```

Then run:

```shell
cargo watch -x run
```
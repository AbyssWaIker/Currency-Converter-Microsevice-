# What is this?

Microservice powered by rust actix server that does exactly one thing - converts money between currencies
(Technically two since it also provides list of convertable currencies, but that details)

# How does it work?

It uses curl to make requests to [OpenExchangeRates.org](https://openexchangerates.org/). Saves rates in toml file. Then uses this values to calculate new currency when actix catches any requests to specified in config address adn port (by default it's localhost:3000). 

# What is required?

## Dependecies
First things first. To even compile any rust project you need [rust toolchain](https://www.rust-lang.org/learn/get-started) (Whatever instruction I put here won't be as up-to-date and fitting your use case as official ones). 

Also the *curl* crate require openssl development libraries

Install it through your package manager or grab [binaries](https://wiki.openssl.org/index.php/Binaries)

### Examples of installation through package manager

### Ubuntu/Mint/Zorin
>sudo apt-get install libssl-dev
### Fedora/Red Hat/Cent OS
>sudo dnf install openssl-devel
### Arch/Manjaro/Endeavor OS
>sudo pacman -S openssl
### Mac OS X
>brew install openssl

## Configuration
1. Handle dependencies
2. >git clone https://github.com/AbyssWaIker/microservice-converter
3. >cd microservice-converter
4. >cp config.example.toml config.toml
5. Sign Up to [OpenExchangeRates.org](https://openexchangerates.org/) and get a (free) api key.
6. Place it inside config.toml file
7. (Optional) Change host and port from default localhost:3000 if required
8. >cargo build && cargo run

# How to use it?
- To convert money send a post request to localhost:3000/convert with body like
  - >{ "from":"USD", "to":"UAH", "sum":1.0 }
- To get list of currencies in correct format open localhost:3000 (or send a get request there)

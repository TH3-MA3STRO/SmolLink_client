# SmolLink (Client)

*the commandline client for the url shortener*


## Description

Lengthy URLs can be difficult to share and manage. Introducing **SmolLink**, a command-line interface for rapidly shortening URLs, optimizing them for easy sharing and readability.
Built with `rust` for performance and reliability.


## Tools Required

To use the client, you'll need:
- [Rust](https://www.rust-lang.org/tools/install)
- A terminal or command line interface

## Installation

### Prerequisites
Ensure you have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://www.rust-lang.org/tools/install) installed.

### Steps
1. Clone the repository from [GitHub](https://github.com/TH3-MA3STRO/SmolLink_client).
   ```bash
   git clone https://github.com/TH3-MA3STRO/SmolLink_client.git
   ```
3. Navigate to the `SmolLink_client` directory.


## Usage

### Adding the url
1. Edit the ```main.rs``` file and add the server url in **line 34**.
2. Compile the project by running:
   ```bash
   cargo build --release
   ```


   
### Usage
1. Navigate to `target/release` directory

2. Run the `smollink` file by:
   <pre><code>./smollink <i>link</i> -s <i>CUSTOM_SHORTHAND</i>  
   </code></pre>
   
   *NOTE: the -s is optional in case you want a custom shorthand*


### OPTIONS
``` bash
Usage: smollink [OPTIONS] [LINK]

Arguments:
  [LINK]  Link to shorten

Options:
  -s, --shorthand <SHORTHAND>  Assign a shorthand
  -h, --help                   Print help
  -V, --version                Print version
```



How to make CLI tools composing different output formats using [clap](https://crates.io/crates/clap).

Demo:

        $ ./target/debug/rust-clap-output-formats --help
        Usage: rust-clap-output-formats <COMMAND>

        Commands:
          debug  Debug output by default
          text   Text output by default
          api    Unformatted JSON output by default
          json   Pretty formatted JSON output by default
          yaml   YAML output by default
          table  Table output by default
          help   Print this message or the help of the given subcommand(s)

        Options:
          -h, --help  Print help

        $ ./target/debug/rust-clap-output-formats debug --help
        Debug output by default

        Usage: rust-clap-output-formats debug [OPTIONS]

        Options:
              --debug  Display as internal debug representation
              --text   Display as text
              --api    Display as unformatted JSON
          -h, --help   Print help

        $ ./target/debug/rust-clap-output-formats table --help
        Table output by default

        Usage: rust-clap-output-formats table [OPTIONS]

        Options:
              --table  Display as table
              --yaml   Display as YAML
              --json   Display as pretty formatted JSON
              --api    Display as unformatted JSON
              --text   Display as text
              --debug  Display as internal debug representation
          -h, --help   Print help

        $ ./target/debug/rust-clap-output-formats json
        {
          "name": "Hello",
          "value": "world"
        }

        $ ./target/debug/rust-clap-output-formats table
        +-------+-------+
        | Name  | Value |
        +===============+
        | Hello | world |
        +-------+-------+

        $ ./target/debug/rust-clap-output-formats table --json
        {
          "name": "Hello",
          "value": "world"
        }

        $ ./target/debug/rust-clap-output-formats table --yaml
        name: Hello
        value: world


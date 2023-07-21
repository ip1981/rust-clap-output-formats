use clap::Parser;
use comfy_table::{ContentArrangement, Row, Table};
use std::fmt;

trait DisplayType<X> {
    fn next_fmt(&self, x: &X) -> Option<String>;
    fn fmt(&self, x: &X) -> String;
    fn display(&self, x: &X) -> String {
        self.next_fmt(x).unwrap_or_else(|| self.fmt(x))
    }
}

trait ToTable {
    fn to_table(&self) -> Table;
}

#[derive(Parser)]
struct NoDisplay {}

impl<X> DisplayType<X> for NoDisplay {
    fn next_fmt(&self, _: &X) -> Option<String> {
        None
    }
    fn fmt(&self, _: &X) -> String {
        String::new()
    }
}

#[derive(Parser)]
struct TextDisplay<T: clap::Args> {
    /// Display as text
    #[clap(long, group = "fmt_type")]
    text: bool,
    #[clap(flatten)]
    next: T,
}

impl<X, T> DisplayType<X> for TextDisplay<T>
where
    X: fmt::Display,
    T: DisplayType<X> + clap::Args,
{
    fn next_fmt(&self, x: &X) -> Option<String> {
        self.text
            .then(|| self.fmt(x))
            .or_else(|| self.next.next_fmt(x))
    }
    fn fmt(&self, x: &X) -> String {
        format!("{x}")
    }
}

#[derive(Parser)]
struct DebugDisplay<T: clap::Args> {
    /// Display as internal debug representation
    #[clap(long, group = "fmt_type")]
    debug: bool,
    #[clap(flatten)]
    next: T,
}

impl<X, T> DisplayType<X> for DebugDisplay<T>
where
    X: fmt::Debug,
    T: DisplayType<X> + clap::Args,
{
    fn next_fmt(&self, x: &X) -> Option<String> {
        self.debug
            .then(|| self.fmt(x))
            .or_else(|| self.next.next_fmt(x))
    }
    fn fmt(&self, x: &X) -> String {
        format!("{x:?}")
    }
}

#[derive(Parser)]
struct ApiDisplay<T: clap::Args> {
    /// Display as unformatted JSON
    #[clap(long, group = "fmt_type")]
    api: bool,
    #[clap(flatten)]
    next: T,
}

impl<X, T> DisplayType<X> for ApiDisplay<T>
where
    X: serde::Serialize,
    T: DisplayType<X> + clap::Args,
{
    fn next_fmt(&self, x: &X) -> Option<String> {
        self.api
            .then(|| self.fmt(x))
            .or_else(|| self.next.next_fmt(x))
    }
    fn fmt(&self, x: &X) -> String {
        serde_json::to_string(x).expect("Cannot serialize item to JSON")
    }
}

#[derive(Parser)]
struct JsonDisplay<T: clap::Args> {
    /// Display as pretty formatted JSON
    #[clap(long, group = "fmt_type")]
    json: bool,
    #[clap(flatten)]
    next: T,
}

impl<X, T> DisplayType<X> for JsonDisplay<T>
where
    X: serde::Serialize,
    T: DisplayType<X> + clap::Args,
{
    fn next_fmt(&self, x: &X) -> Option<String> {
        self.json
            .then(|| self.fmt(x))
            .or_else(|| self.next.next_fmt(x))
    }
    fn fmt(&self, x: &X) -> String {
        serde_json::to_string_pretty(x).expect("Cannot serialize item to JSON")
    }
}

#[derive(Parser)]
struct YamlDisplay<T: clap::Args> {
    /// Display as YAML
    #[clap(long, group = "fmt_type")]
    yaml: bool,
    #[clap(flatten)]
    next: T,
}

impl<X, T> DisplayType<X> for YamlDisplay<T>
where
    X: serde::Serialize,
    T: DisplayType<X> + clap::Args,
{
    fn next_fmt(&self, x: &X) -> Option<String> {
        self.yaml
            .then(|| self.fmt(x))
            .or_else(|| self.next.next_fmt(x))
    }
    fn fmt(&self, x: &X) -> String {
        serde_yaml::to_string(x).expect("Cannot serialize item to YAML")
    }
}

#[derive(Parser)]
struct TableDisplay<T: clap::Args> {
    /// Display as table
    #[clap(long, group = "fmt_type", alias = "tabular")]
    table: bool,
    #[clap(flatten)]
    next: T,
}

impl<X, T> DisplayType<X> for TableDisplay<T>
where
    X: ToTable,
    T: DisplayType<X> + clap::Args,
{
    fn next_fmt(&self, x: &X) -> Option<String> {
        self.table
            .then(|| self.fmt(x))
            .or_else(|| self.next.next_fmt(x))
    }
    fn fmt(&self, x: &X) -> String {
        x.to_table().to_string()
    }
}

#[derive(Parser)]
enum App {
    /// Debug output by default
    Debug {
        #[clap(flatten)]
        output: DebugDisplay<TextDisplay<ApiDisplay<NoDisplay>>>,
    },
    /// Text output by default
    Text {
        #[clap(flatten)]
        output: TextDisplay<DebugDisplay<ApiDisplay<NoDisplay>>>,
    },
    /// Unformatted JSON output by default
    Api {
        #[clap(flatten)]
        output: ApiDisplay<TextDisplay<DebugDisplay<NoDisplay>>>,
    },
    /// Pretty formatted JSON output by default
    Json {
        #[clap(flatten)]
        output: JsonDisplay<ApiDisplay<TextDisplay<DebugDisplay<NoDisplay>>>>,
    },
    /// YAML output by default
    Yaml {
        #[clap(flatten)]
        output: YamlDisplay<JsonDisplay<ApiDisplay<TextDisplay<DebugDisplay<NoDisplay>>>>>,
    },
    /// Table output by default
    Table {
        #[clap(flatten)]
        output: TableDisplay<
            YamlDisplay<JsonDisplay<ApiDisplay<TextDisplay<DebugDisplay<NoDisplay>>>>>,
        >,
    },
}

#[derive(Debug, serde::Serialize)]
struct Foo {
    name: String,
    value: String,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl ToTable for Foo {
    fn to_table(&self) -> Table {
        let mut table = Table::new();

        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(Row::from(vec!["Name", "Value"]))
            .add_row(Row::from(vec![&self.name, &self.value]));

        table
    }
}

fn main() {
    let app = App::parse();

    let foo = Foo {
        name: "Hello".to_string(),
        value: "world".to_string(),
    };

    match app {
        App::Debug { output } => println!("{}", output.display(&foo)),
        App::Text { output } => println!("{}", output.display(&foo)),
        App::Api { output } => println!("{}", output.display(&foo)),
        App::Json { output } => println!("{}", output.display(&foo)),
        App::Yaml { output } => println!("{}", output.display(&foo)),
        App::Table { output } => println!("{}", output.display(&foo)),
    }
}

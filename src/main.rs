use comrak::{parse_document, Arena, Options};

fn main() {
    let content = {
        let filename = std::env::args().skip(1).next().expect("no filename given");
        std::fs::read_to_string(&filename).unwrap()
    };

    let arena = Arena::new();
    let root = parse_document(&arena, &content, &Options::default());

    println!("<body style=\"background-color: #121212; color: #e0e0e0;\">");
    println!("{}", parse_block(&root));
}

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::nodes::{AstNode, NodeCode, NodeCodeBlock, NodeValue};
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};

use syntect::parsing::{SyntaxSet, SyntaxSetBuilder};
use syntect::highlighting::ThemeSet;

use std::sync::LazyLock;

static ADAPTER: LazyLock<SyntectAdapter> = LazyLock::new(|| {
    let mut set = SyntaxSetBuilder::new();
    set.add_plain_text_syntax();
    set.add_from_folder("syntax/", true).unwrap();

    let mut theme_set = ThemeSet::load_defaults();

    theme_set.add_from_folder("themes").unwrap();

    SyntectAdapterBuilder::new()
        .theme_set(theme_set)
        .theme("gruvbox")
        .syntax_set(set.build())
        .build()
});

fn parse_block<'a>(node: &'a AstNode<'a>) -> String {
    match node.data.borrow().value {
        NodeValue::Heading(_) => format!("\n<h2>{}</h2>\n", to_string(node)),
        NodeValue::Text(ref text) => String::from(text),
        NodeValue::LineBreak => String::from("<br>\n"),
        NodeValue::Paragraph => format!("{}\n", to_string(node)),
        NodeValue::BlockQuote => {
            format!("\n<div class=\"block quote\">{}</div>\n", to_string(node))
        }
        NodeValue::Strong => format!("<b>{}</b>", to_string(node)),
        NodeValue::Emph => format!("<i>{}</i>", to_string(node)),
        NodeValue::Strikethrough => format!("<s>{}</s>", to_string(node)),
        NodeValue::HtmlInline(ref html) => String::from(html),

        NodeValue::Code(NodeCode { ref literal, .. }) => format!("<c>{literal}</c>"),
        NodeValue::CodeBlock(NodeCodeBlock { ref literal, .. }) => {
            let mut out = Vec::new();

            ADAPTER
                .write_highlighted(&mut out, Some("shard"), literal)
                .unwrap();

            format!(
                "<code><pre>\n{}</pre></code>\n",
                String::from_utf8(out).unwrap()
            )
        }

        _ => to_string(node),
    }
}

fn to_string<'a>(node: &'a AstNode<'a>) -> String {
    node.children().map(parse_block).collect::<String>()
}

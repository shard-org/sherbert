use comrak::{parse_document, Arena, Options};

fn main() {
    let content = {
        let filename = std::env::args().nth(1).expect("no filename given");
        std::fs::read_to_string(&filename).unwrap()
    };

    let arena = Arena::new();
    let root = parse_document(&arena, &content, &Options::default());

    println!(
"<!DOCTYPE html>
<body style=\"background-color: #1d2021; color: #ebdbb2\">
");
    println!("{}", parse_block(root));

}

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::nodes::{AstNode, NodeCode, NodeCodeBlock, NodeValue};
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};

use syntect::highlighting::{ThemeSet, Theme, ThemeSettings, ThemeItem, ScopeSelector, ScopeSelectors, StyleModifier, Color, FontStyle};
use syntect::parsing::{SyntaxSetBuilder, Scope, ScopeStack};

use std::sync::LazyLock;
use std::collections::BTreeMap;

static ADAPTER: LazyLock<SyntectAdapter> = LazyLock::new(|| {
    let mut set = SyntaxSetBuilder::new();
    set.add_plain_text_syntax();
    set.add_from_folder("syntax/", true).unwrap();

    SyntectAdapterBuilder::new()
        .theme_set(ThemeSet {
            themes: {
                let mut themes = BTreeMap::new();
                themes.insert("gruvbox".to_string(), THEME.clone());
                themes
            }
        })
        .theme("gruvbox")
        .syntax_set(set.build())
        .build()
});

static THEME: LazyLock<Theme> = LazyLock::new(|| {
    let color = |hex: u32| Color {
        r: (hex >> 16) as u8,
        g: (hex >> 8) as u8,
        b: hex as u8,
        a: 0xff,
    };

    let item = |path: &str, hex: u32, style: Option<FontStyle>| ThemeItem {
        scope: ScopeSelectors { selectors: vec![ScopeSelector{ path: ScopeStack::from_vec(vec![Scope::new(path).unwrap()]), excludes: Vec::new()}]},
        style: StyleModifier {
            foreground: Some(color(hex)),
            background: None,
            font_style: style,
        },
    };

    Theme {
        name:     Some(String::from("gruvbox")),
        author:   None,
        settings: ThemeSettings {
            foreground: Some(color(0xebdbb2)),
            ..Default::default()
        },
        scopes:   vec![
            item("comment",           0x928375, None),

            item("literal.string",    0xb8bb27, None),
            item("literal.float",     0xd3869b, None),
            item("literal.integer",   0xd3869b, None),

            item("keyword.ret",       0xcc241e, Some(FontStyle::BOLD)),
            item("keyword.other",     0xb8bb26, Some(FontStyle::BOLD)),
            item("keyword.attribute", 0x8ec07c, Some(FontStyle::ITALIC)),

            item("tag.inner",         0x689d6b, Some(FontStyle::BOLD)),
            item("tag.outer",         0x689d6b, Some(FontStyle::BOLD)),

            item("preprocess.macro",  0x8ec07c, Some(FontStyle::BOLD)),

            item("entity.type",       0xfe8019, Some(FontStyle::BOLD)),
            item("entity.type.lesser",0xfe8019, Some(FontStyle::ITALIC)),
            item("entity.function",   0xfb4934, None),

            item("builtin.call",      0xfabd2f, Some(FontStyle::BOLD | FontStyle::ITALIC)),
            item("builtin.call",      0xfabd2f, Some(FontStyle::BOLD)),

            item("op.mutation",       0xfb4934, None),
            item("op.declaration",    0xfb4934, None),
            item("op.arithmetic",     0x83a598, None),
            item("op.bitwise",        0x83a598, None),
            item("op.assignment",     0x83a598, None),
            item("op.logic",          0x83a598, None),
            item("op.other",          0x83a598, None),
            item("op.type",           0xfe8019, Some(FontStyle::BOLD)),
            item("op.brackets",       0xfe8019, None),
        ],
    }
});

fn parse_block<'a>(node: &'a AstNode<'a>) -> String {
    match node.data.borrow().value {
        NodeValue::Heading(_)     => format!("\n<h2>{}</h2>\n", to_string(node)),
        NodeValue::Text(ref text) => String::from(text),
        NodeValue::LineBreak      => String::from("<br>\n"),
        NodeValue::Paragraph      => format!("{}\n", to_string(node)),
        NodeValue::BlockQuote     => format!("\n<div class=\"block quote\">{}</div>\n", to_string(node)),
        NodeValue::Strong         => format!("<b>{}</b>", to_string(node)),
        NodeValue::Emph           => format!("<i>{}</i>", to_string(node)),
        NodeValue::Strikethrough  => format!("<s>{}</s>", to_string(node)),
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
    node.children().map(parse_block).collect()
}

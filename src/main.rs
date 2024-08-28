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
<body style=\"background-color: #{:x}; color: #{:x}\">
", COLOUR[0], COLOUR[2]);
    println!("{}", parse_block(root));

}

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::nodes::{AstNode, NodeCode, NodeCodeBlock, NodeValue};
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};

use syntect::highlighting::{ThemeSet, Theme, ThemeSettings, ThemeItem, ScopeSelector, ScopeSelectors, StyleModifier, Color, FontStyle};
use syntect::parsing::{SyntaxSetBuilder, Scope, ScopeStack};

use std::sync::LazyLock;
use std::collections::BTreeMap;

// gruvbox
const COLOUR: [u32; 13] = [
    0x1d2021, // background
    0x928375, // comments
    0xebdbb2, // foreground
    0xebdbb2, // separators
    0xfe8019, // types
    0xfb4934, // functions, mutation, declaration
    0xb8bb26, // keywords
    0x83a598, // operators
    0x689d6b, // tags
    0x8ec07c, // macros
    0xfabd2f, // builtins
    0xb8bb27, // strings
    0xd3869b, // numbers
];

// // nord
// const COLOUR: [u32; 13] = [
//     0x2E3440, // background
//     0x4C566A, // comments
//     0xD8DEE9, // foreground
//     0xECEFF4, // separators
//     0x8FBCBB, // types
//     0x88C0D0, // functions, mutation, declaration
//     0x81A1C1, // keywords
//     0x81A1C1, // operators
//     0x5E81AC, // tags
//     0x5E81AC, // macros
//     0xD08770, // builtins
//     0xA3BE8C, // strings
//     0xB48EAD, // numbers
// ];


static ADAPTER: LazyLock<SyntectAdapter> = LazyLock::new(|| {
    let color = |hex: u32| Color {
        r: (hex >> 16) as u8,
        g: (hex >> 8) as u8,
        b: hex as u8,
        a: 0xff,
    };

    let item = |path: &str, hex: u32, style: Option<FontStyle>| ThemeItem {
        scope: ScopeSelectors { selectors: vec![ScopeSelector { path: ScopeStack::from_vec(vec![Scope::new(path).unwrap()]), excludes: Vec::new()}]},
        style: StyleModifier {
            foreground: Some(color(hex)),
            background: None,
            font_style: style,
        },
    };

    let theme = Theme {
        name:     Some(String::from("theme")),
        author:   None,
        settings: ThemeSettings {
            foreground: Some(color(COLOUR[2])),
            ..Default::default()
        },
        scopes:   vec![
            item("comment",            COLOUR[1],  None),

            item("literal.string",     COLOUR[11], None),
            item("literal.float",      COLOUR[12], None),
            item("literal.integer",    COLOUR[12], None),

            item("keyword.ret",        COLOUR[6],  Some(FontStyle::BOLD)),
            item("keyword.other",      COLOUR[6],  Some(FontStyle::BOLD)),
            item("keyword.attribute",  COLOUR[6],  Some(FontStyle::ITALIC)),

            item("tag.inner",          COLOUR[8],  Some(FontStyle::BOLD)),
            item("tag.outer",          COLOUR[8],  Some(FontStyle::BOLD)),

            item("preprocess.macro",   COLOUR[9],  Some(FontStyle::BOLD)),

            item("entity.type",        COLOUR[4],  Some(FontStyle::BOLD)),
            item("entity.type.lesser", COLOUR[4],  Some(FontStyle::ITALIC)),
            item("entity.function",    COLOUR[5],  None),

            item("builtin.call",       COLOUR[10], Some(FontStyle::BOLD | FontStyle::ITALIC)),
            item("builtin.outer",      COLOUR[10], Some(FontStyle::BOLD)),

            item("op.mutation",        COLOUR[5],  None),
            item("op.declaration",     COLOUR[5],  None),
            item("op.arithmetic",      COLOUR[7],  None),
            item("op.bitwise",         COLOUR[7],  None),
            item("op.assignment",      COLOUR[7],  None),
            item("op.logic",           COLOUR[7],  None),
            item("op.other",           COLOUR[7],  None),
            item("op.type",            COLOUR[4],  Some(FontStyle::BOLD)),
            item("op.ref",             COLOUR[4],  None),
            item("op.brackets",        COLOUR[4],  None),
            item("op.semicolon",       COLOUR[3],  None),

            item("syntax.separator",   COLOUR[3],  None),
        ],
    };

    let mut set = SyntaxSetBuilder::new();
    set.add_plain_text_syntax();
    set.add_from_folder("syntax/", true).unwrap();

    SyntectAdapterBuilder::new()
        .theme_set(ThemeSet {
            themes: {
                let mut themes = BTreeMap::new();
                themes.insert("theme".to_string(), theme);
                themes
            }
        })
        .theme("theme")
        .syntax_set(set.build())
        .build()
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

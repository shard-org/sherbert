use comrak::{parse_document, Arena, Options};
use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::nodes::{AstNode, NodeCode, NodeCodeBlock, NodeValue, NodeHtmlBlock, NodeHeading};
use comrak::plugins::syntect::{SyntectAdapter, SyntectAdapterBuilder};

use syntect::highlighting::{ThemeSet, Theme, ThemeSettings, ThemeItem, ScopeSelector, ScopeSelectors, StyleModifier, Color, FontStyle};
use syntect::parsing::{SyntaxSetBuilder, Scope, ScopeStack};

use regex::{Regex, Captures};

use std::collections::BTreeMap;
use std::sync::LazyLock;

use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

static INDIR:  LazyLock<PathBuf> = LazyLock::new(|| Path::new("files").to_path_buf());
static OUTDIR: LazyLock<PathBuf> = LazyLock::new(|| Path::new("site").to_path_buf());

static COLOUR: LazyLock<[u32; 14]> = LazyLock::new(||  
    match std::env::var("COLOUR").unwrap_or(String::from("nord")).as_str() {
        "nord" => [
            0x1f2029, // darker background
            0x2b313b, // background
            0x4C566A, // comments
            0xD8DEE9, // foreground
            0xECEFF4, // separators
            0x8FBCBB, // types
            0x88C0D0, // functions, mutation, declaration
            0x81A1C1, // keywords
            0x81A1C1, // operators
            0x5E81AC, // tags
            0x5E81AC, // macros
            0xD08770, // builtins
            0xA3BE8C, // strings
            0xB48EAD, // numbers
        ],
        "gruvbox" => [
            0x181a1b, // darker background
            0x282828, // background
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
        ],
        _ => panic!("Invalid colour scheme"),
    });

fn main() {
    let into_html = |path: &Path| {
        let arena = Arena::new();
        let root = parse_document(&arena,
            &std::fs::read_to_string(path).unwrap(),
            &Options::default()
        );

        format!("<!DOCTYPE html>\n<meta charset=\"utf8\">\n<link href=\"theme.css\" rel=\"stylesheet\"/>\n{}", parse_block(root))
    };

    if OUTDIR.exists() 
    { fs::remove_dir_all(&*OUTDIR).unwrap(); }

    fs::create_dir(&*OUTDIR).unwrap();

    for entry in walkdir::WalkDir::new(&*INDIR) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() { continue; }

        let dest = OUTDIR.join(entry.file_name());

        if path.extension().is_some_and(|ext| ext == "md") {
            fs::write(dest.with_extension(""), into_html(path)).unwrap();
            continue;
        }

        fs::copy(path, dest).unwrap();
    }

    fs::File::create(OUTDIR.join("theme.css")).unwrap()
        .write_all(format!(
            ":root {{ --text: #{:06x};--hint: #{:06x};--neg: #{:06x};--neg-acc: #{:06x};--pos: #{:06x};--pos-acc: #{:06x};--base: #{:06x};--base-acc: #{:06x};}}",
            COLOUR[3], COLOUR[2], COLOUR[12], COLOUR[13], COLOUR[6], COLOUR[6], COLOUR[1], COLOUR[0]
        ).as_bytes()).unwrap();
}

fn parse_block<'a>(node: &'a AstNode<'a>) -> String {
    let to_string = |node: &'a AstNode<'a>| {
        println!("{:?}", node.data.borrow().value);
        node.children().map(parse_block).collect()
    };

    println!("{:?}", node.data.borrow().value);

    match node.data.borrow().value {
        NodeValue::Heading(NodeHeading { level, ..})     => format!("\n<h{level}>{}</h{level}>\n", to_string(node)),
        NodeValue::Text(ref text) => String::from(text),
        NodeValue::LineBreak      => String::from("<br>\n"),
        NodeValue::SoftBreak      => String::from(" \n"),
        NodeValue::Paragraph      => format!("<p>{}</p>\n", to_string(node)),
        NodeValue::BlockQuote     => format!("\n<div class=\"block quote\">{}</div>\n", to_string(node)),
        NodeValue::Strong         => format!("<b>{}</b>", to_string(node)),
        NodeValue::Emph           => format!("<i>{}</i>", to_string(node)),
        NodeValue::Strikethrough  => format!("<s>{}</s>", to_string(node)),
        NodeValue::HtmlInline(ref html) => String::from(html),
        NodeValue::HtmlBlock(NodeHtmlBlock { ref literal, .. }) => {
            static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"<include "(.*)">"#).unwrap());
            format!("{}\n", RE.replace_all(literal, |c: &Captures| 
                fs::read_to_string(INDIR.join(&c[1])).unwrap()))
        },
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
            foreground: Some(color(COLOUR[3])),
            ..Default::default( )
        },
        scopes:   vec![
            item("comment",            COLOUR[2],  None),

            item("literal.string",     COLOUR[12], None),
            item("literal.float",      COLOUR[13], None),
            item("literal.integer",    COLOUR[13], None),

            item("keyword.ret",        COLOUR[7],  Some(FontStyle::BOLD)),
            item("keyword.other",      COLOUR[7],  Some(FontStyle::BOLD)),
            item("keyword.attribute",  COLOUR[7],  Some(FontStyle::ITALIC)),

            item("tag.inner",          COLOUR[9],  Some(FontStyle::BOLD)),
            item("tag.outer",          COLOUR[9],  Some(FontStyle::BOLD)),

            item("preprocess.macro",   COLOUR[10], Some(FontStyle::BOLD)),

            item("entity.type",        COLOUR[5],  Some(FontStyle::BOLD)),
            item("entity.type.lesser", COLOUR[5],  Some(FontStyle::ITALIC)),
            item("entity.function",    COLOUR[6],  None),

            item("builtin.call",       COLOUR[11], Some(FontStyle::BOLD | FontStyle::ITALIC)),
            item("builtin.outer",      COLOUR[11], Some(FontStyle::BOLD)),

            item("op.mutation",        COLOUR[6],  None),
            item("op.declaration",     COLOUR[6],  None),
            item("op.arithmetic",      COLOUR[8],  None),
            item("op.bitwise",         COLOUR[8],  None),
            item("op.assignment",      COLOUR[8],  None),
            item("op.logic",           COLOUR[8],  None),
            item("op.other",           COLOUR[8],  None),
            item("op.type",            COLOUR[5],  Some(FontStyle::BOLD)),
            item("op.ref",             COLOUR[5],  None),
            item("op.brackets",        COLOUR[5],  None),
            item("op.semicolon",       COLOUR[4],  None),

            item("syntax.separator",   COLOUR[4],  None),
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

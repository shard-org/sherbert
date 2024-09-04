#![allow(forbidden_lint_groups)]
#![forbid(clippy::complexity, clippy::suspicious, clippy::correctness, clippy::perf, clippy::nursery)] 
#![allow(clippy::style, clippy::restriction, clippy::match_bool, clippy::too_many_lines, clippy::single_match_else, clippy::ignored_unit_patterns, clippy::module_name_repetitions, clippy::needless_for_each, clippy::derive_partial_eq_without_eq, clippy::missing_const_for_fn, clippy::cognitive_complexity, clippy::option_if_let_else, clippy::option_map_unit_fn, clippy::type_complexity)]

use foxhole::{Method::Get, Response, http};
use foxhole::resolve::{Url, Query, HeaderMap, ArgMap};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};

struct Files(HashMap<PathBuf, Arc<Vec<u8>>>);
impl foxhole::TypeCacheKey for Files {
    type Value = Arc<RwLock<Self>>;
}

fn main() {
    let addr = std::env::args().nth(1)
        .unwrap_or_else(|| String::from("127.0.0.1:8080"));

    fn get_file(Query(files): Query<Files>, Url(url): Url, HeaderMap(headers): HeaderMap, ArgMap(args): ArgMap) -> Option<Response> {
        let get = |files: &Arc<RwLock<Files>>, filename: &str| -> std::io::Result<Arc<Vec<u8>>> {
            let path = match Path::new(filename).to_path_buf() {
                p if p.is_dir() => p.join("index.html"),
                p if !p.to_str().unwrap().contains(".") => p.with_extension("html"),
                p => p,
            };

            if let Some(contents) = files.read().unwrap().0.get(&path) {
                return Ok(contents.clone());
            }

            let contents = Arc::new(std::fs::read(&path)?);
            files.write().unwrap().0
                .insert(path, contents.clone());
            Ok(contents)
        };

        let theme = headers.get("referer")
            .and_then(|u| u.to_str().ok())
            .and_then(|u| u.split_once("?t=").map(|s| s.1))
            .map(|u| u.split_once("&").map_or(u, |s| s.0));

        let url = match url {
            url if theme.is_some() && !args.contains_key("t") => {
                let url = url.strip_suffix(".html").unwrap_or(url);
                return http::Response::builder()
                    .status(301)
                    .header("Location", &format!("{url}?t={}", theme.unwrap()))
                    .body(Vec::new())
                    .ok();
            },
            "/theme.css" => {
                const DEFAULT_THEME: &str = env!("DEFAULT_THEME");

                let file = get(&files, &format!("site/style/themes/{}.css", theme.unwrap_or(DEFAULT_THEME)));
                return file.inspect_err(|e| println!("{e}; for: {url:?}"))
                    .map(|f| Response::new((*f).clone())).ok()
            },
            url => String::from(url),
        };

        let file = get(files, &format!("site{url}"));
        file.inspect_err(|e| println!("{e}; for: {url:?}"))
            .map(|f| Response::new((*f).clone())).ok()
    }

    let router = foxhole::Router::new()
        .add_route("/*", Get(get_file))
        .add_route("/",  Get(get_file))
        .fallback(|| std::fs::read("files/not_found.html").map(|chunk| {
            http::Response::builder()
                .status(404)
                .body(chunk)
                .unwrap()
        }).ok());

    let mut cache = foxhole::TypeCache::new();
    cache.insert::<Files>(Arc::new(RwLock::new(Files(HashMap::new()))));

    foxhole::App::builder(router)
        .cache(cache)
        .run::<foxhole::Http1>(addr);
}

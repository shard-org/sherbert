#![allow(forbidden_lint_groups)]
#![forbid(clippy::complexity, clippy::suspicious, clippy::correctness, clippy::perf, clippy::nursery)] 
#![allow(clippy::style, clippy::restriction, clippy::match_bool, clippy::too_many_lines, clippy::single_match_else, clippy::ignored_unit_patterns, clippy::module_name_repetitions, clippy::needless_for_each, clippy::derive_partial_eq_without_eq, clippy::missing_const_for_fn, clippy::cognitive_complexity, clippy::option_if_let_else, clippy::option_map_unit_fn, clippy::type_complexity)]

use foxhole::TypeCacheKey;
use foxhole::{Method::Get, Response, http};
use foxhole::resolve::{Url, Query};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};


struct CacheLen;
impl TypeCacheKey for CacheLen {
    type Value = u16;
}

struct Files(HashMap<String, Arc<Vec<u8>>>);
impl TypeCacheKey for Files {
    type Value = Arc<RwLock<Self>>;
}

fn main() {
    let addr = std::env::args().nth(1)
        .unwrap_or_else(|| String::from("127.0.0.1:8080"));

    fn get_file(Query(cache): Query<CacheLen>, Query(files): Query<Files>, Url(url): Url) -> Option<Response> {
        let get = |files: &Arc<RwLock<Files>>, filename: &str| -> std::io::Result<Arc<Vec<u8>>> {
            if let Some(contents) = files.read().unwrap().0.get(filename) {
                return Ok(contents.clone());
            }

            let contents = Arc::new(std::fs::read(filename)?);
            files.write().unwrap().0
                .insert(String::from(filename), contents.clone());
            Ok(contents)
        };

        let url = if url == "/" { "/index" } else { &url };
        let file = get(files, &format!("site{url}"));

        file.map_err(|e| {println!("{e}; for: {url:?}"); e})
            .map(|f| http::Response::builder()
                .header("cache-control", &format!("max-age={cache}, public"))
                .body((*f).clone())
                .unwrap()
            ).ok()
    }

    let router = foxhole::Router::new()
        .add_route("/*", Get(get_file))
        .add_route("/",  Get(get_file))
        .fallback(|| std::fs::read("files/not_found").map(|chunk| {
            let mut res = Response::new(chunk);
            *res.status_mut() = 404.try_into().unwrap();
            res
        }).ok());

    let mut cache = foxhole::TypeCache::new();
    cache.insert::<CacheLen>(std::env::var("CACHELEN").map_or(360, |s| s.parse::<u16>().unwrap()));
    cache.insert::<Files>(Arc::new(RwLock::new(Files(HashMap::new()))));

    foxhole::App::builder(router)
        .cache(cache)
        .run::<foxhole::Http1>(addr);
}

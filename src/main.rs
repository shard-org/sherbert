#![deny(clippy::complexity, clippy::suspicious, clippy::correctness, clippy::perf, clippy::nursery)] 
#![allow(clippy::style, clippy::restriction, clippy::match_bool, clippy::too_many_lines, clippy::single_match_else, clippy::ignored_unit_patterns, clippy::module_name_repetitions, clippy::needless_for_each, clippy::derive_partial_eq_without_eq, clippy::missing_const_for_fn, clippy::cognitive_complexity, clippy::option_if_let_else, clippy::option_map_unit_fn, clippy::type_complexity)]

use foxhole::{Method::Get, Response, http};
use foxhole::resolve::{Url, Query, HeaderMap, ArgMap};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::PathBuf;

struct Files(HashMap<PathBuf, http::Response<Vec<u8>>>);
impl foxhole::TypeCacheKey for Files {
	type Value = Arc<RwLock<Self>>;
}

fn main() {
	let addr = std::env::args().nth(1)
		.unwrap_or_else(|| String::from("127.0.0.1:8080"));

	const DEFAULT_THEME: &str = env!("DEFAULT_THEME");
	fn get_theme<'a>(HeaderMap(ref headers): &'a HeaderMap<'a>) -> Option<&'a str> {
		headers.get("referer")
			.and_then(|u| u.to_str().ok())
			.and_then(|u| u.split_once("?t=").map(|s| s.1))
			.map(|u| u.split_once("&").map_or(u, |s| s.0))
	}

	fn get_file(Query(files): Query<Files>, Url(url): Url, headers: HeaderMap, ArgMap(args): ArgMap) -> Response {
		let get = |files: &Arc<RwLock<Files>>, filename: String| -> http::Response<Vec<u8>> {
			let path = match PathBuf::from(filename) {
				p if p.is_dir() => p.join("index.html"),
				p if !p.to_str().unwrap().contains(".") => p.with_extension("html"),
				p => p,
			};

			if let Some(response) = files.read().unwrap().0.get(&path) {
				return response.clone();
			}

			match std::fs::read(&path) {
				Ok(contents) => {
					let response = http::Response::builder().status(200)
						.header("Content-Type", mime_guess::from_path(&path)
							.first_or_octet_stream().essence_str())
						.body(contents.clone())
						.unwrap();

					files.write().unwrap().0.insert(path, response.clone());
					response
				},
				Err(e)   => {
					println!("{e}; for: {url:?}");
					http::Response::builder().status(301)
						.header("Location", &format!("/404{}", 
							get_theme(&headers).map_or(String::new(), |t| format!("?t={t}"))))
						.body(Vec::new()).unwrap()
				},
			}
		};

		match (url, get_theme(&headers)) {
			(url, Some(theme)) if !args.contains_key("t") =>
				http::Response::builder().status(307)
					.header("Location", &format!("{}?t={theme}", 
						url.strip_suffix(".html").unwrap_or(url)))
					.body(Vec::new()).unwrap(),
			("/theme.css", theme) => get(&files, format!("site/style/themes/{}.css", theme.unwrap_or(DEFAULT_THEME))),
			(url, _) => get(files, format!("site{url}")),
		}
	}

	let router = foxhole::Router::new()
		.add_route("/*", Get(get_file))
		.add_route("/",  Get(get_file));

	let mut cache = foxhole::TypeCache::new();
	cache.insert::<Files>(Arc::new(RwLock::new(Files(HashMap::new()))));

	foxhole::App::builder(router)
		.cache(cache)
		.run::<foxhole::Http1>(addr);
}

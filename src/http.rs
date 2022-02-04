use std::{collections::HashMap, borrow::Cow};

use ic_cdk::{api, export::candid};
use candid::CandidType;
use ic_certified_map::{RbTree, Hash, AsHashTree};
use include_dir::{include_dir, Dir, File};
use percent_encoding::percent_decode_str;
use url::Url;
use once_cell::sync::{Lazy, OnceCell};
use serde_cbor::Serializer;
use serde::Serialize;

static FRONTEND: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/frontend/public");
static HASHES: Dir<'static> = include_dir!("$OUT_DIR/hashes");
static HASH_TREE: OnceCell<RbTree<String, Hash>> = OnceCell::new();

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: HashMap<&'static str, Cow<'static, str>>,
    body: &'static [u8],
}

static URL: Lazy<Url> = Lazy::new(|| {
    if cfg!(mainnet) {
        Url::parse(&format!("https://{}.ic0.app", api::id()))
    } else {
        Url::parse(&format!("http://{}.localhost:8000", api::id()))
    }.unwrap()
});

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let url = URL.join(&req.url).unwrap();
    let path = percent_decode_str(url.path()).decode_utf8().unwrap();
    let path = if path.starts_with('/') {
        &path[1..]
    } else {
        &path
    };
    let guess = mime_guess::from_path(path).first_or_text_plain();
    let (mime, witness, body) = if let Some(file) = FRONTEND.get_file(&*path) {
        (format!("{guess}").into(), witness(&file.path().to_string_lossy()), file.contents())
    } else {
        ("text/html;charset=UTF-8".into(), witness("index.html"), FRONTEND.get_file("index.html").unwrap().contents())
    };
    const REPLICA: &str = if cfg!(mainnet) { "https://ic0.app" } else { "localhost:8000" };
    let cert = base64::encode(api::data_certificate().unwrap());
    let mut headers = HashMap::from([
        ("Content-Type", mime),
        ("Content-Security-Policy", format!("default-src 'self'; font-src * ; style-src-elem 'self' https://fonts.googleapis.com ; img-src blob: *; media-src blob: *; frame-ancestors 'none'; frame-src 'none'; connect-src 'self' {REPLICA}; script-src 'self' 'unsafe-eval'").into()),
        ("X-Frame-Options", "DENY".into()),
        ("Referrer-Policy", "same-origin".into()),
        ("Permissions-Policy", "publickey-credentials-get=(self), sync-xhr=(self), clipboard-write=(self)".into()),
        ("IC-Certificate", format!("certificate=:{cert}:, tree=:{witness}:").into())
    ]);
    if cfg!(mainnet) {
        headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".into());
    }
    HttpResponse {
        status_code: 200,
        headers,
        body,
    }
}

pub fn bake_hashes() {
    let mut hashes = RbTree::new();
    walk_files(&HASHES, |file| {
        hashes.insert(file.path().to_string_lossy().into_owned(), Hash::try_from(file.contents()).unwrap());
    });
    api::set_certified_data(&ic_certified_map::labeled_hash(b"http_assets", &hashes.root_hash()));
    HASH_TREE.set(hashes).unwrap();
}

fn witness(name: &str) -> String {
    let witness = HASH_TREE.get().unwrap().witness(name.as_bytes());
    let tree = ic_certified_map::labeled(b"http_assets", witness);
    let mut data = vec![];
    let mut serializer = Serializer::new(&mut data);
    serializer.self_describe().unwrap();
    tree.serialize(&mut serializer).unwrap();
    base64::encode(data)
}

fn walk_files<'a>(tree: &'a Dir<'a>, mut func: impl FnMut(&'a File<'a>)) {
    let mut stack = vec![tree];
    while let Some(dir) = stack.pop() {
        for file in dir.files() {
            func(file);
        }
        stack.extend(dir.dirs());
    }
}

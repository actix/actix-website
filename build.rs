extern crate skeptic;

use std::{env, fs};


fn main() {
    let f = env::var("OUT_DIR").unwrap() + "/skeptic-tests.rs";
    let _ = fs::remove_file(f);
    // generates doc tests.
    skeptic::generate_doc_tests(
        &["content/docs/application.md",
          "content/docs/autoreload.md",
          "content/docs/databases.md",
          "content/docs/errors.md",
          "content/docs/extractors.md",
          "content/docs/getting-started.md",
          "content/docs/handlers.md",
          "content/docs/http2.md",
          "content/docs/installation.md",
          "content/docs/middleware.md",
          "content/docs/request.md",
          "content/docs/response.md",
          "content/docs/sentry.md",
          "content/docs/server.md",
          "content/docs/static-files.md",
          "content/docs/testing.md",
          "content/docs/url-dispatch.md",
          "content/docs/websockets.md",
          "content/docs/whatis.md",
        ]);
}

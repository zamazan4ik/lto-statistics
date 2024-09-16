# lto-statistics
A project to gather statistics about LTO enablement across software

Notes:

* GitHub Code Search API is not the same as available on GitHub.com - they return different results
* GitHub Code Search API has ridiculous throttling limits - 10 requests per minute
* GitHub Code Search (both on the website and via API) cannot return more than 1000 total search results. To avoid this, you need to slice your search query with some modifiers like a Cargo.toml file size, repository data modification or smth like that. That's what I did in this repo
* Application urls for "most important Rust applications" were collected with `sed -r -e 's/\[/\n[/g' unparsed_applications.md  | sed -r -e 's/.*(\[.*\]\(.*\)).*/\n\1/g' | grep -oE '\(.*\)' | grep "https://github.com/" | grep -v ".yml" | grep -v "badge" | grep -v ".svg" | grep -v "img.shields.io" | tr -d '(' | tr -d ')'` UNIX spell for [Awesome-Rust](https://github.com/rust-unofficial/awesome-rust) README file + a bit of manuall data processing in VSCode

TODO:
  - Gather Rust repositories (awesome-rust as a starting point). What is about the GitHub API - https://docs.github.com/en/rest/repos/contents?apiVersion=2022-11-28#get-repository-content ?
  - Check in the root Cargo.toml for LTO
  - Save simple statistics: repository address, LTO state, LTO kind (Thin, Fat, true)
  - How to make a difference between library and binary crates? We need some heuristics here like `main.rs` existance in a repo
  - Can we use GitHub search API instead? No, we cannot - https://github.com/orgs/community/discussions/9868 . Can we mitigate it with Search API?

Issues with LTO:

* https://github.com/kakoune-lsp/kakoune-lsp/issues/131
* ThinLTO and Windows: https://github.com/rust-lang/rust/pull/122790 + https://github.com/rust-lang/rust/issues/109114
//! doc-link-checker - validates file-relative links in markdown documentation.
//!
//! Scans `.md`, `.mdx`, extension-less files, and `///` doc comments in `.rs` files.
//! Extracts all non-HTTP links, verifies each target exists on the filesystem, and —
//! when a fragment (`#section`) is present - verifies the heading anchor exists in
//! the target file.
//!
//! # Examples
//!
//! ```bash
//! # Check all docs recursively, warn on broken links (default)
//! doc-link-checker --dir docs --recursive
//!
//! # Exit 1 on any broken link
//! doc-link-checker --dir docs --recursive --error
//!
//! # Check a single file
//! doc-link-checker --file README.md
//!
//! # Resolve all links relative to a base directory
//! doc-link-checker --dir docs --recursive --absolute /project/root
//! ```

use clap::Parser;
use pulldown_cmark::{Event, LinkType, Options, Parser as MdParser, Tag, TagEnd};
use std::collections::{HashMap, HashSet};
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// ── CLI ──────────────────────────────────────────────────────────────────────

/// Checks that all file-relative links in markdown documentation exist on disk.
#[derive(Parser, Debug)]
#[command(name = "doc-link-checker")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Top directory of documentation to scan
    #[arg(short = 'd', long)]
    dir: Option<PathBuf>,

    /// Scan supplied directories recursively
    #[arg(short = 'r', long)]
    recursive: bool,

    /// Single file to include in the scan
    #[arg(short = 'f', long)]
    file: Option<PathBuf>,

    /// Comma-separated file extensions to include (empty string matches extension-less files)
    #[arg(short = 'e', long, default_value = "md,mdx")]
    extensions: String,

    /// Absolute mode: resolve all links relative to this path instead of each file's directory
    #[arg(long, value_name = "PATH")]
    absolute: Option<PathBuf>,

    /// Emit warnings but do not fail (default behaviour)
    #[arg(short = 'w', long, default_value_t = true)]
    warn: bool,

    /// Finish with exit code 1 on any broken link (overrides --warn)
    #[arg(long)]
    error: bool,

    /// Stop at the first broken link
    #[arg(long)]
    fail_fast: bool,

    /// Exclude this file or directory from the scan (repeatable)
    #[arg(long)]
    exclude: Vec<PathBuf>,

    /// Only print the summary line, suppress individual link lines
    #[arg(long)]
    quiet: bool,
}

// ── Config (validated) ───────────────────────────────────────────────────────

#[derive(Debug)]
enum OutputMode {
    Warn,
    Error,
}

#[derive(Debug)]
struct Config {
    dir: Option<PathBuf>,
    recursive: bool,
    extra_file: Option<PathBuf>,
    extensions: HashSet<String>,
    absolute_base: Option<PathBuf>,
    output_mode: OutputMode,
    fail_fast: bool,
    exclude: Vec<PathBuf>,
    quiet: bool,
}

fn build_config(cli: Cli) -> Result<Config, String> {
    if cli.dir.is_none() && cli.file.is_none() {
        return Err("at least one of --dir or --file is required".to_owned());
    }

    let extensions: HashSet<String> = cli
        .extensions
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect();

    let output_mode = if cli.error {
        OutputMode::Error
    } else {
        OutputMode::Warn
    };

    Ok(Config {
        dir: cli.dir,
        recursive: cli.recursive,
        extra_file: cli.file,
        extensions,
        absolute_base: cli.absolute,
        output_mode,
        fail_fast: cli.fail_fast,
        exclude: cli.exclude,
        quiet: cli.quiet,
    })
}

// ── Domain types ─────────────────────────────────────────────────────────────

struct ExtractedLink {
    raw_url: String,
    line: usize,
    source_file: PathBuf,
}

struct BrokenLink {
    link: ExtractedLink,
    /// Human-readable reason: "file not found" or "heading '#foo' not found in bar.md"
    reason: String,
}

// ── File collection ──────────────────────────────────────────────────────────

fn is_excluded(path: &Path, exclude: &[PathBuf]) -> bool {
    for ex in exclude {
        let excluded = std::fs::canonicalize(ex).unwrap_or_else(|_| ex.to_owned());
        let candidate = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_owned());
        if candidate.starts_with(&excluded) || candidate == excluded {
            return true;
        }
    }
    false
}

fn extension_matches(path: &Path, extensions: &HashSet<String>) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    extensions.contains(ext)
}

fn collect_files(config: &Config) -> Vec<PathBuf> {
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut result: Vec<PathBuf> = Vec::new();

    let mut add = |path: PathBuf| {
        if seen.insert(path.clone()) {
            result.push(path);
        }
    };

    if let Some(ref file) = config.extra_file
        && !is_excluded(file, &config.exclude)
        && extension_matches(file, &config.extensions)
    {
        add(file.clone());
    }

    if let Some(ref dir) = config.dir {
        let mut stack = vec![dir.clone()];
        while let Some(current) = stack.pop() {
            let entries = match std::fs::read_dir(&current) {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("warning: cannot read directory {}: {err}", current.display());
                    continue;
                }
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if is_excluded(&path, &config.exclude) {
                    continue;
                }
                if path.is_dir() {
                    if config.recursive {
                        stack.push(path);
                    }
                } else if extension_matches(&path, &config.extensions) {
                    add(path);
                }
            }
        }
    }

    result
}

// ── GFM heading slug ──────────────────────────────────────────────────────────

/// Converts heading text to a GitHub-Flavored Markdown anchor slug.
///
/// Algorithm (GitHub-compatible):
/// - Lowercase the entire string
/// - For each character:
///   - Space → `-`
///   - `_` → `_` (kept as-is)
///   - `-` → `-` (kept as-is)
///   - Alphanumeric (including Unicode letters like ü, ö, ä) → kept as-is
///   - Anything else (`/`, parentheses, commas, `!`, `.`, etc.) → dropped
/// - Multiple consecutive `-` are NOT collapsed (GitHub preserves them)
fn gfm_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('-'),
            '_' | '-' => Some(c),
            c if c.is_alphanumeric() => Some(c),
            _ => None,
        })
        .collect()
}

/// Collects all heading anchors from a markdown source string.
///
/// Handles duplicate headings by appending `-1`, `-2`, … (GFM behaviour).
fn collect_markdown_headings(source: &str) -> HashSet<String> {
    let parser = MdParser::new_ext(source, Options::empty());

    let mut anchors: HashSet<String> = HashSet::new();
    let mut slug_counts: HashMap<String, usize> = HashMap::new();
    let mut in_heading = false;
    let mut heading_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                if in_heading {
                    let base_slug = gfm_slug(&heading_text);
                    let count = slug_counts.entry(base_slug.clone()).or_insert(0);
                    let slug = if *count == 0 {
                        base_slug.clone()
                    } else {
                        format!("{}-{}", base_slug, count)
                    };
                    *count += 1;
                    anchors.insert(slug);
                }
                in_heading = false;
                heading_text.clear();
            }
            Event::Text(text) if in_heading => {
                heading_text.push_str(&text);
            }
            Event::Code(text) if in_heading => {
                // Inline code inside headings contributes its text to the slug
                heading_text.push_str(&text);
            }
            _ => {}
        }
    }

    anchors
}

/// Returns the set of heading anchors for a file, caching results across calls.
///
/// Only parses markdown files (`.md`, `.mdx`, extension-less). Returns empty
/// set for other file types.
fn headings_for_file<'a>(path: &Path, cache: &'a mut HashMap<PathBuf, HashSet<String>>) -> &'a HashSet<String> {
    // We always insert so the borrow checker is satisfied with a single lookup.
    cache.entry(path.to_owned()).or_insert_with(|| {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        // Only attempt heading extraction for markdown files
        if !matches!(ext, "md" | "mdx" | "") {
            return HashSet::new();
        }
        match std::fs::read_to_string(path) {
            Ok(source) => collect_markdown_headings(&source),
            Err(_) => HashSet::new(), // silently skip unreadable or non-UTF-8 files
        }
    })
}

// ── Link extraction ───────────────────────────────────────────────────────────

/// Extracts all markdown links from a standard markdown file.
fn extract_links_from_markdown(file: &Path) -> Result<Vec<ExtractedLink>, io::Error> {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::InvalidData => return Ok(vec![]),
        Err(e) => return Err(e),
    };
    let parser = MdParser::new_ext(&source, Options::empty()).into_offset_iter();
    let mut links: Vec<ExtractedLink> = Vec::new();

    for (event, range) in parser {
        let url = match &event {
            Event::Start(Tag::Link { link_type, dest_url, .. })
                if !matches!(link_type, LinkType::Autolink) =>
            {
                Some(dest_url.as_ref().to_owned())
            }
            Event::Start(Tag::Image { dest_url, .. }) => Some(dest_url.as_ref().to_owned()),
            _ => None,
        };
        if let Some(raw_url) = url {
            let line = source[..range.start]
                .bytes()
                .filter(|&b| b == b'\n')
                .count()
                + 1;
            links.push(ExtractedLink {
                raw_url,
                line,
                source_file: file.to_owned(),
            });
        }
    }

    Ok(links)
}

/// Extracts markdown links from `///` and `//!` doc comment lines in a Rust source file.
///
/// Skips links inside fenced code blocks (` ``` ` fences within doc comments).
fn extract_links_from_rust(file: &Path) -> Result<Vec<ExtractedLink>, io::Error> {
    let source = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::InvalidData => return Ok(vec![]),
        Err(e) => return Err(e),
    };
    let mut links: Vec<ExtractedLink> = Vec::new();

    // Collect doc comment lines with their original line numbers, tracking
    // fenced code block state across lines.
    let mut in_fence = false;
    let mut doc_lines: Vec<(usize, String)> = Vec::new();

    for (zero_idx, line) in source.lines().enumerate() {
        let line_number = zero_idx + 1;
        let trimmed = line.trim_start();

        // Process both `///` (item docs) and `//!` (module/inner docs).
        let content = if let Some(rest) = trimmed.strip_prefix("///") {
            rest
        } else if let Some(rest) = trimmed.strip_prefix("//!") {
            rest
        } else {
            continue;
        };

        // Strip one optional leading space after the comment marker
        let content = content.strip_prefix(' ').unwrap_or(content);

        // Detect fence toggle: a line whose trimmed content starts with ```
        if content.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }

        if !in_fence {
            doc_lines.push((line_number, content.to_owned()));
        }
    }

    if doc_lines.is_empty() {
        return Ok(links);
    }

    // Reconstruct a synthetic markdown document from the doc lines,
    // preserving relative line numbers for accurate reporting.
    // We keep a mapping from synthetic line index → original line number.
    let mut line_map: Vec<usize> = Vec::with_capacity(doc_lines.len());
    let mut synthetic = String::new();
    for (original_line, content) in &doc_lines {
        line_map.push(*original_line);
        synthetic.push_str(content);
        synthetic.push('\n');
    }

    let parser = MdParser::new_ext(&synthetic, Options::empty()).into_offset_iter();
    for (event, range) in parser {
        let url = match &event {
            Event::Start(Tag::Link { link_type, dest_url, .. })
                if !matches!(link_type, LinkType::Autolink) =>
            {
                Some(dest_url.as_ref().to_owned())
            }
            Event::Start(Tag::Image { dest_url, .. }) => Some(dest_url.as_ref().to_owned()),
            _ => None,
        };
        if let Some(raw_url) = url {
            // Map synthetic byte offset back to an original line number
            let synthetic_line_idx = synthetic[..range.start]
                .bytes()
                .filter(|&b| b == b'\n')
                .count();
            let original_line = line_map
                .get(synthetic_line_idx)
                .copied()
                .unwrap_or(synthetic_line_idx + 1);
            links.push(ExtractedLink {
                raw_url,
                line: original_line,
                source_file: file.to_owned(),
            });
        }
    }

    Ok(links)
}

fn extract_links(file: &Path) -> Result<Vec<ExtractedLink>, io::Error> {
    let ext = file.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext == "rs" {
        extract_links_from_rust(file)
    } else {
        extract_links_from_markdown(file)
    }
}

// ── Link resolution ───────────────────────────────────────────────────────────

struct ResolvedLink {
    /// The file path to check for existence
    file_path: PathBuf,
    /// The fragment (without `#`), if present
    fragment: Option<String>,
}

/// Returns `true` if the URL looks like a Rust intra-doc link rather than a file path.
///
/// Intra-doc links reference Rust items using path syntax: `Self::method`,
/// `crate::module::Type`, `std::fmt::Display`, etc. They contain `::` or start
/// with a known Rust path segment (`crate`, `super`, `self`, `Self`), and never
/// contain a `/` (which would indicate a file path).
fn is_rust_intra_doc_link(url: &str) -> bool {
    // File paths always use `/` as a separator; Rust paths use `::`.
    // If the URL contains `/` it is a file path, not an intra-doc link.
    if url.contains('/') {
        return false;
    }
    // Explicit Rust path syntax
    if url.contains("::") {
        return true;
    }
    // Known root path segments that can only appear in Rust intra-doc links
    let root = url.split([':', '<', '!']).next().unwrap_or(url);
    matches!(root, "crate" | "super" | "self" | "Self")
}

/// Returns `Some(resolved)` if this link should be checked, `None` to skip.
fn resolve_link(link: &ExtractedLink, config: &Config) -> Option<ResolvedLink> {
    let url = &link.raw_url;

    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("ftp://")
        || url.starts_with("mailto:")
        || url.starts_with('#')
        || url.is_empty()
        || is_rust_intra_doc_link(url)
    {
        return None;
    }

    // Split path from fragment
    let (path_part, fragment) = match url.split_once('#') {
        Some((p, f)) => (p, Some(f.to_owned())),
        None => (url.as_str(), None),
    };

    if path_part.is_empty() {
        // Fragment-only link like `[text](#heading)` — anchor in same file.
        // We'd need the source file's headings to validate; skip for now.
        return None;
    }

    let file_path = if let Some(ref base) = config.absolute_base {
        base.join(path_part)
    } else {
        let parent = link.source_file.parent().unwrap_or(Path::new("."));
        parent.join(path_part)
    };

    Some(ResolvedLink { file_path, fragment })
}

// ── Link checking ─────────────────────────────────────────────────────────────

fn check_links(files: &[PathBuf], config: &Config) -> Vec<BrokenLink> {
    let mut broken: Vec<BrokenLink> = Vec::new();
    let mut heading_cache: HashMap<PathBuf, HashSet<String>> = HashMap::new();

    'outer: for file in files {
        let links = match extract_links(file) {
            Ok(l) => l,
            Err(err) => {
                eprintln!("warning: cannot read {}: {err}", file.display());
                continue;
            }
        };

        for link in links {
            let Some(resolved) = resolve_link(&link, config) else {
                continue;
            };

            if !resolved.file_path.exists() {
                broken.push(BrokenLink {
                    reason: "file not found".to_owned(),
                    link,
                });
                if config.fail_fast {
                    break 'outer;
                }
                continue;
            }

            // File exists — now validate the fragment if present
            if let Some(ref fragment) = resolved.fragment {
                let anchors = headings_for_file(&resolved.file_path, &mut heading_cache);
                if !anchors.is_empty() && !anchors.contains(fragment.as_str()) {
                    broken.push(BrokenLink {
                        reason: format!(
                            "heading '#{fragment}' not found in {}",
                            resolved.file_path.display()
                        ),
                        link,
                    });
                    if config.fail_fast {
                        break 'outer;
                    }
                }
            }
        }
    }

    broken
}

// ── Output ────────────────────────────────────────────────────────────────────

fn print_result(broken: &[BrokenLink], config: &Config) {
    let failure_label = match config.output_mode {
        OutputMode::Warn => "WARN",
        OutputMode::Error => "ERROR",
    };

    if !config.quiet {
        for b in broken {
            println!(
                "{}: {}:{} {} ({})",
                failure_label,
                b.link.source_file.display(),
                b.link.line,
                b.link.raw_url,
                b.reason,
            );
        }
    }

    if broken.is_empty() {
        println!("All links OK");
    } else {
        let noun = if broken.len() == 1 { "link" } else { "links" };
        println!("{} broken {} found", broken.len(), noun);
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = match build_config(cli) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::from(1);
        }
    };

    let files = collect_files(&config);
    let broken = check_links(&files, &config);
    print_result(&broken, &config);

    if broken.is_empty() || matches!(config.output_mode, OutputMode::Warn) {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfm_slug_basic() {
        assert_eq!(gfm_slug("My Heading"), "my-heading");
    }

    #[test]
    fn gfm_slug_strips_punctuation() {
        assert_eq!(gfm_slug("Hello, World!"), "hello-world");
    }

    #[test]
    fn gfm_slug_preserves_hyphens() {
        assert_eq!(gfm_slug("step-by-step guide"), "step-by-step-guide");
    }

    #[test]
    fn gfm_slug_spaces_become_hyphens() {
        // GitHub converts each space to a hyphen without collapsing multiples.
        assert_eq!(gfm_slug("  multiple   spaces  "), "--multiple---spaces--");
    }

    #[test]
    fn gfm_slug_slash_removed() {
        // GitHub removes `/`; surrounding spaces become hyphens independently.
        assert_eq!(gfm_slug("Cron / Crontab"), "cron--crontab");
        assert_eq!(gfm_slug("GNU / GNU/Linux"), "gnu--gnulinux");
    }

    #[test]
    fn gfm_slug_underscore_preserved() {
        assert_eq!(gfm_slug("pg_dump"), "pg_dump");
    }

    #[test]
    fn gfm_slug_umlauts_preserved() {
        // GitHub keeps Unicode letters like ü, ö, ä.
        assert_eq!(gfm_slug("Gesundheitsprüfung"), "gesundheitsprüfung");
    }

    #[test]
    fn gfm_slug_parens_removed() {
        // Parentheses are dropped.
        assert_eq!(gfm_slug("Health Check (Gesundheitsprüfung)"), "health-check-gesundheitsprüfung");
    }

    #[test]
    fn gfm_slug_real_headings() {
        // Verify the exact anchors from the bug report.
        assert_eq!(gfm_slug("Cron / Crontab"), "cron--crontab");
        assert_eq!(gfm_slug("journald / journalctl"), "journald--journalctl");
        assert_eq!(gfm_slug("pg_dump"), "pg_dump");
        assert_eq!(gfm_slug("Health Check (Gesundheitsprüfung)"), "health-check-gesundheitsprüfung");
        assert_eq!(gfm_slug("SSH-Key (SSH-Schlüsselpaar)"), "ssh-key-ssh-schlüsselpaar");
        assert_eq!(gfm_slug("GNU / GNU/Linux"), "gnu--gnulinux");
        assert_eq!(gfm_slug("Lock-in / Vendor Lock-in"), "lock-in--vendor-lock-in");
        assert_eq!(gfm_slug("UID / GID (User ID / Group ID)"), "uid--gid-user-id--group-id");
    }

    #[test]
    fn collect_headings_simple() {
        let source = "# Hello World\n\nSome text.\n\n## Sub Section\n";
        let anchors = collect_markdown_headings(source);
        assert!(anchors.contains("hello-world"));
        assert!(anchors.contains("sub-section"));
    }

    #[test]
    fn collect_headings_duplicate() {
        let source = "# Foo\n\n# Foo\n\n# Foo\n";
        let anchors = collect_markdown_headings(source);
        assert!(anchors.contains("foo"));
        assert!(anchors.contains("foo-1"));
        assert!(anchors.contains("foo-2"));
    }

    #[test]
    fn collect_headings_inline_code() {
        let source = "## The `run()` method\n";
        let anchors = collect_markdown_headings(source);
        assert!(anchors.contains("the-run-method"));
    }

    #[test]
    fn rust_extract_skips_fenced_code() {
        // Links inside ``` fences should be ignored; links outside should be found.
        let source = "\
/// See [README](README.md) for details.\n\
///\n\
/// ```ignore\n\
/// // [fake](not-a-link.md)\n\
/// ```\n\
///\n\
/// Also see [CHANGELOG](CHANGELOG.md).\n\
fn foo() {}\n";
        let links = extract_links_from_rust_str(source);
        let urls: Vec<_> = links.iter().map(|l| l.raw_url.as_str()).collect();
        assert!(urls.contains(&"README.md"), "should find README.md");
        assert!(urls.contains(&"CHANGELOG.md"), "should find CHANGELOG.md");
        assert!(!urls.contains(&"not-a-link.md"), "should skip fenced link");
    }

    #[test]
    fn rust_extract_module_docs() {
        // //! lines should also be scanned for links.
        let source = "\
//! This module. See [CONTRIBUTING](CONTRIBUTING.md).\n\
//!\n\
//! ```ignore\n\
//! // [fake](not-a-link.md)\n\
//! ```\n\
pub mod foo {}\n";
        let links = extract_links_from_rust_str(source);
        let urls: Vec<_> = links.iter().map(|l| l.raw_url.as_str()).collect();
        assert!(urls.contains(&"CONTRIBUTING.md"), "should find CONTRIBUTING.md");
        assert!(!urls.contains(&"not-a-link.md"), "should skip fenced link");
    }

    // Helper used only in tests: extract from a string rather than a file path.
    fn extract_links_from_rust_str(source: &str) -> Vec<ExtractedLink> {
        let mut in_fence = false;
        let mut doc_lines: Vec<(usize, String)> = Vec::new();

        for (zero_idx, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            let content = if let Some(rest) = trimmed.strip_prefix("///") {
                rest
            } else if let Some(rest) = trimmed.strip_prefix("//!") {
                rest
            } else {
                continue;
            };
            let content = content.strip_prefix(' ').unwrap_or(content);
            if content.trim_start().starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if !in_fence {
                doc_lines.push((zero_idx + 1, content.to_owned()));
            }
        }

        if doc_lines.is_empty() {
            return vec![];
        }

        let mut line_map: Vec<usize> = Vec::new();
        let mut synthetic = String::new();
        for (orig, content) in &doc_lines {
            line_map.push(*orig);
            synthetic.push_str(content);
            synthetic.push('\n');
        }

        let parser = MdParser::new_ext(&synthetic, Options::empty()).into_offset_iter();
        let mut links = Vec::new();
        for (event, range) in parser {
            let url = match &event {
                Event::Start(Tag::Link { link_type, dest_url, .. })
                    if !matches!(link_type, LinkType::Autolink) =>
                {
                    Some(dest_url.as_ref().to_owned())
                }
                Event::Start(Tag::Image { dest_url, .. }) => Some(dest_url.as_ref().to_owned()),
                _ => None,
            };
            if let Some(raw_url) = url {
                let synthetic_line_idx =
                    synthetic[..range.start].bytes().filter(|&b| b == b'\n').count();
                let original_line = line_map.get(synthetic_line_idx).copied().unwrap_or(1);
                links.push(ExtractedLink {
                    raw_url,
                    line: original_line,
                    source_file: PathBuf::from("<test>"),
                });
            }
        }
        links
    }
}

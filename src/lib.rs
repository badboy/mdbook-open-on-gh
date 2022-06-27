use std::path::{Path, PathBuf};

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::{Result, Error};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct OpenOn;

impl Preprocessor for OpenOn {
    fn name(&self) -> &str {
        "open-on-gh"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let book_root = &ctx.root;
        let src_root = book_root.join(&ctx.config.book.src);
        let git_root = find_git(book_root).unwrap();
        log::debug!("Book root: {}", book_root.display());
        log::debug!("Src root: {}", src_root.display());
        log::debug!("Git root: {}", git_root.display());

        let repository_url = match ctx.config.get("output.html.git-repository-url") {
            None => return Ok(book),
            Some(url) => url,
        };
        let repository_url = match repository_url {
            toml::Value::String(s) => s,
            _ => return Ok(book),
        };
        log::debug!("Repository URL: {}", repository_url);

        if repository_url.find("github.com").is_none() {
            return Ok(book);
        }

        let branch = match ctx.config.get("output.html.git-branch") {
            None => "main",
            Some(toml::Value::String(b)) => b,
            _ => return Ok(book),
        };
        log::debug!("Git Branch: {}", branch);

        let open_on_text = match ctx.config.get("output.html.open-on-text") {
            None => "Found a bug? [Edit this page on GitHub.]",
            Some(toml::Value::String(b)) => b,
            _ => return Ok(book),
        };
        log::debug!("Footer text: {}", open_on_text);

        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(
                    open_on(
                        &git_root,
                        &src_root,
                        &repository_url,
                        &branch,
                        &open_on_text,
                        chapter,
                    )
                    .map(|md| {
                        chapter.content = md;
                    }),
                );
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }
}

fn parse_footer_text(text: &str) -> Option<(&str, &str, &str)> {
    let link_start = text.find('[')?;
    let link_end = text[link_start + 1..].find(']')?;
    let pre = &text[0..link_start];
    let link_text = &text[link_start + 1..][..link_end];
    let post = &text[link_start + 1 + link_end + 1..];

    if link_text.is_empty() {
        return None;
    }
    Some((pre, link_text, post))
}

fn open_on(
    git_root: &Path,
    src_root: &Path,
    base_url: &str,
    branch: &str,
    open_on_text: &str,
    chapter: &mut Chapter,
) -> Result<String> {
    let content = &chapter.content;

    let footer_start = "<footer id=\"open-on-gh\">";
    if content.contains(footer_start) {
        return Ok(content.into());
    }

    let path = match chapter.path.as_ref() {
        None => return Ok("".into()),
        Some(path) => path,
    };
    let path = match src_root.join(&path).canonicalize() {
        Ok(path) => path,
        Err(_) => return Ok(content.into()),
    };
    let relpath = path.strip_prefix(git_root).unwrap();
    log::trace!("Chapter path: {}", path.display());
    log::trace!("Relative path: {}", relpath.display());

    let url = format!("{}/edit/{}/{}", base_url, branch, relpath.display());
    log::trace!("URL: {}", url);

    let (pre, link_text, post) = match parse_footer_text(&open_on_text) {
        Some(parsed) => parsed,
        None => Err(Error::msg("can't parse footer text. Missing `[link text]`?"))?
    };

    let link = format!("<a href=\"{}\">{}</a>", url, link_text);
    let content = format!(
        "{}\n{}{}{}{}</footer>",
        content, footer_start, pre, link, post
    );

    Ok(content)
}

fn find_git(path: &Path) -> Option<PathBuf> {
    let mut current_path = path;
    let mut git_dir = current_path.join(".git");
    let root = Path::new("/");

    while !git_dir.exists() {
        current_path = match current_path.parent() {
            Some(p) => p,
            None => return None,
        };

        if current_path == root {
            return None;
        }

        git_dir = current_path.join(".git");
    }

    git_dir.parent().map(|p| p.to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_footer_text() {
        assert_eq!(
            Some(("pre ", "link", " post")),
            parse_footer_text("pre [link] post")
        );
        assert_eq!(Some(("", "link", "")), parse_footer_text("[link]"));
        assert_eq!(Some(("pre ", "link", "")), parse_footer_text("pre [link]"));
        assert_eq!(
            Some(("", "link", " post")),
            parse_footer_text("[link] post")
        );
    }

    #[test]
    fn missing() {
        assert_eq!(None, parse_footer_text("none"));
        assert_eq!(None, parse_footer_text("["));
        assert_eq!(None, parse_footer_text("[]"));
    }
}

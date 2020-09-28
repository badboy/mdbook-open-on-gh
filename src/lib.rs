use std::path::{Path, PathBuf};

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
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
            return Ok(book)
        }

        let branch = match ctx.config.get("output.html.git-branch") {
            None => "master",
            Some(toml::Value::String(b)) => b,
            _ => return Ok(book),
        };
        log::debug!("Git Branch: {}", branch);

        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(open_on(&git_root, &src_root, &repository_url, &branch, chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }
}

fn open_on(git_root: &Path, src_root: &Path, base_url: &str, branch: &str, chapter: &mut Chapter) -> Result<String> {
    let content = &chapter.content;

    let footer_start = "<footer id=\"open-on-gh\">";
    if content.contains(footer_start) {
        return Ok(content.into())
    }

    let path = match chapter.path.as_ref() {
        None => {
            return Ok("".into())
        },
        Some(path) => path
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

    let link = format!("<a href=\"{}\">Edit this file on GitHub.</a>", url);
    let content = format!("{}\n{}Found a bug? {}</footer>", content, footer_start, link);

    Ok(content)
}

fn find_git(path: &Path) -> Option<PathBuf> {
    let mut current_path = path;
    let mut git_dir = current_path.join(".git");
    let root = Path::new("/");

    while !git_dir.exists() {
        current_path = match current_path.parent() {
            Some(p) => p,
            None => return None
        };

        if current_path == root {
            return None;
        }

        git_dir = current_path.join(".git");
    }

    git_dir.parent().map(|p| p.to_owned())
}

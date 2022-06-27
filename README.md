# mdbook-open-on-gh

A preprocessor for [mdbook][] to add a open-on-github link on every page.

[mdbook]: https://github.com/rust-lang/mdBook

It adds an "Edit this file on GitHub" link on the bottom of every page, linking directly to the source file.
It uses the configured `git-repository-url` as the base.

## Installation

If you want to use only this preprocessor, install the tool:

```
cargo install mdbook-open-on-gh
```

Add it as a preprocessor to your `book.toml`:

```
[preprocessor.open-on-gh]
command = "mdbook-open-on-gh"
renderer = ["html"]
```

Add a repository URL to use as a base in your `book.toml`:

```toml
[output.html]
git-repository-url = "https://github.com/mozilla/glean"
```

By default it assumes the repository has a `main` branch.
You can configure another branch using the `git-branch` option:

```toml
[output.html]
git-branch = "trunk"
```

To style the footer add a custom CSS file for your HTML output:

```toml
[output.html]
additional-css = ["open-in.css"]
```

And in `open-in.css` style the `<footer>` element or directly the CSS element id `open-on-gh`:

```css
footer {
  font-size: 0.8em;
  text-align: center;
  border-top: 1px solid black;
  padding: 5px 0;
}
```

This code block shrinks the text size, center-aligns it under the rest of the content
and adds a small horizontal bar above the text to separate it from the page content.


Finally, build your book as normal:

```
mdbook path/to/book
```

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2020 Jan-Erik Rediger <janerik@fnordig.de>

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

## Configuration

`mdbook-open-on-gh` is configured using additional options under `[output.html]`:


```toml
[output.html]
# Required: Your repository URL used in the link.
git-repository-url = "https://github.com/$user/$project"

# Your git branch. Defaults to `main`
git-branch = "main"

# The text to use in the footer.
# The link text is marked by `[]`
open-on-text = "Found a bug? [Edit this page on GitHub.]"
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
Copyright (c) 2020-2022 Jan-Erik Rediger <janerik@fnordig.de>

use std::{env, error::Error, fs, path::Path};

const BASE_URL: &str = "https://beautyxt.app";
const LAYOUT: &str = include_str!("layout.html");

struct Page {
    output: &'static str,
    url: &'static str,
    title: &'static str,
    description: &'static str,
    content: &'static str,
}

const PAGES: &[Page] = &[
    Page {
        output: "index.html",
        url: "/",
        title: "BeauTyXT | Plain Text & Markdown Editor for Android",
        description: "A plain-text and Markdown reader and editor for Android. Material 3 Expressive, native rendering, and files that stay with you.",
        content: include_str!("pages/home.html"),
    },
    Page {
        output: "contact/index.html",
        url: "/contact/",
        title: "Contact | BeauTyXT",
        description: "Join the BeauTyXT community, report an issue, or contact the developer.",
        content: include_str!("pages/contact.html"),
    },
    Page {
        output: "website-privacy-policy/index.html",
        url: "/website-privacy-policy/",
        title: "Website privacy | BeauTyXT",
        description: "How the BeauTyXT website handles privacy, hosting, and external links.",
        content: include_str!("pages/privacy.html"),
    },
    Page {
        output: "404.html",
        url: "/404.html",
        title: "Page not found | BeauTyXT",
        description: "This page could not be found. Return to the BeauTyXT home page.",
        content: include_str!("pages/404.html"),
    },
];

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn render(page: &Page) -> String {
    // Templates and page bodies are trusted, checked-in source. Only metadata
    // is escaped; this generator never accepts visitor content.
    LAYOUT
        .replace("{{title}}", &escape_html(page.title))
        .replace("{{description}}", &escape_html(page.description))
        .replace("{{canonical}}", &format!("{BASE_URL}{}", page.url))
        .replace(
            "{{home_current}}",
            if page.url == "/" {
                " aria-current=\"page\""
            } else {
                ""
            },
        )
        .replace(
            "{{contact_current}}",
            if page.url == "/contact/" {
                " aria-current=\"page\""
            } else {
                ""
            },
        )
        .replace(
            "{{robots}}",
            if page.output == "404.html" {
                "<meta name=\"robots\" content=\"noindex\">"
            } else {
                ""
            },
        )
        .replace("{{content}}", page.content)
}

fn outputs() -> Vec<(String, String)> {
    let mut files: Vec<_> = PAGES
        .iter()
        .map(|page| (page.output.to_owned(), render(page)))
        .collect();
    let urls = PAGES
        .iter()
        .filter(|p| p.output != "404.html")
        .map(|p| format!("  <url><loc>{BASE_URL}{}</loc></url>\n", p.url))
        .collect::<String>();
    files.push(("sitemap.xml".to_owned(), format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{urls}</urlset>\n"
    )));
    files.push(("main.css".to_owned(), include_str!("main.css").to_owned()));
    files.push(("LICENSE".to_owned(), include_str!("../LICENSE").to_owned()));
    files
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let check = match args.as_slice() {
        [] => false,
        [arg] if arg == "--check" => true,
        _ => return Err("Usage: cargo run -- [--check]".into()),
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs");
    let mut stale = Vec::new();
    for (name, content) in outputs() {
        if content.contains("{{") {
            return Err(format!("Unresolved template token in {name}").into());
        }
        let path = root.join(&name);
        if check {
            if fs::read_to_string(&path).as_deref().ok() != Some(content.as_str()) {
                stale.push(name);
            }
        } else {
            fs::create_dir_all(path.parent().ok_or("Missing output directory")?)?;
            fs::write(path, content)?;
        }
    }
    if !stale.is_empty() {
        return Err(format!(
            "Generated files are stale: {}. Run cargo run.",
            stale.join(", ")
        )
        .into());
    }
    println!(
        "{}",
        if check {
            "Generated site is up to date."
        } else {
            "Generated site in docs/."
        }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_cannot_escape_its_attribute() {
        assert_eq!(
            escape_html("<a href=\"x&y\">'"),
            "&lt;a href=&quot;x&amp;y&quot;&gt;&#39;"
        );
    }

    #[test]
    fn every_page_is_resolved_and_uses_its_own_canonical_url() {
        for page in PAGES {
            let rendered = render(page);
            assert!(
                !rendered.contains("{{"),
                "{} has unresolved placeholders",
                page.output
            );
            assert!(rendered.contains(&format!(
                "rel=\"canonical\" href=\"{BASE_URL}{}\"",
                page.url
            )));
            assert_eq!(
                rendered.matches("<h1").count(),
                1,
                "{} must have one main heading",
                page.output
            );
        }
    }
}

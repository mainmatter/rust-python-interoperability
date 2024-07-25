use std::{collections::BTreeSet, num::NonZeroUsize};

use pyo3::{prelude::*, types::PySet};
use scraper::Html;

#[pyfunction]
/// Given a starting URL (`start_from`), discover all the URLs *on the same domain*
/// that can be reached by following links from the starting URL.
///
/// The discovered URLs should be inserted into the `site_map` set provided as an argument.
///
/// # Constraints
///
/// ## GIL
///
/// You should, as much as possible, avoid holding the GIL.
/// Try to scope the GIL to the smallest possible block of code—e.g. when touching `site_map`.
///
/// ## Threads
///
/// The program should use as many threads as there are available cores on the machine.
///
/// ## Invalid URLs
///
/// If a URL is invalid (e.g. it's malformed or it returns a 404 status code), ignore it.
///
/// ## External URLs
///
/// Do not follow links to external websites. Restrict your search to the domain of the
/// starting URL.
///
/// ## Anchors and Query Parameters
///
/// Ignore anchors and query parameters when comparing URLs.
/// E.g. `http://example.com` and `http://example.com#section` should be considered the same URL,
/// and normalizing them to `http://example.com` is the expected approach.
///
/// # Tooling
///
/// We recommend using the following crates to help you with this exercise:
///
/// - `ureq` for making HTTP requests (https://crates.io/crates/ureq)
/// - `scraper` for parsing HTML and extracting links (https://crates.io/crates/scraper)
/// - `url` for parsing URLs (https://crates.io/crates/url)
/// - `std`'s `sync` and `thread` modules for synchronization primitives.
///
/// Feel free to pull in any other crates you think might be useful.
/// If your approach is channel-based, you might want to use the `crossbeam` crate too.
fn site_map<'py>(start_from: String, site_map: Bound<'py, PySet>) {
    let n_cpus =
        std::thread::available_parallelism().unwrap_or_else(|_| NonZeroUsize::new(2).unwrap());
    let (work_queue_sender, work_queue_receiver) = crossbeam::channel::unbounded();
    let (result_queue_sender, result_queue_receiver) = std::sync::mpsc::channel();
    for _ in 0..(n_cpus.get() - 1) {
        let result_queue_sender = result_queue_sender.clone();
        let work_queue_receiver = work_queue_receiver.clone();
        std::thread::spawn(move || scrape_worker(result_queue_sender, work_queue_receiver));
    }
    orchestrator(
        start_from,
        site_map,
        result_queue_receiver,
        work_queue_sender,
    );
}

fn orchestrator(
    start_from: String,
    site_map: Bound<'_, PySet>,
    result_queue: std::sync::mpsc::Receiver<Option<BTreeSet<String>>>,
    work_queue: crossbeam::channel::Sender<String>,
) {
    let python = site_map.py();
    let site_map = site_map.unbind();
    python.allow_threads(move || {
        let mut visited: BTreeSet<String> = BTreeSet::from_iter([start_from.clone()]);
        let mut n_pending = 1;
        let _ = work_queue.send(start_from);
        loop {
            match result_queue.try_recv() {
                Ok(links) => {
                    n_pending -= 1;
                    let Some(links) = links else {
                        continue;
                    };
                    for url in links {
                        if visited.contains(&url) {
                            continue;
                        }
                        visited.insert(url.clone());
                        Python::with_gil(|py| {
                            let site_map = site_map.bind(py);
                            let _ = site_map.add(url.clone());
                        });
                        let _ = work_queue.send(url);
                        n_pending += 1;
                    }
                }
                _ => {
                    if n_pending == 0 {
                        break;
                    }
                }
            }
        }
    });
}

fn scrape_worker(
    result_queue: std::sync::mpsc::Sender<Option<BTreeSet<String>>>,
    work_queue: crossbeam::channel::Receiver<String>,
) {
    while let Ok(url) = work_queue.recv() {
        let links = scrape_page_links(&url).ok().flatten();
        let _ = result_queue.send(links);
    }
}

/// Fetch a page and extract all links from it, if it's an HTML page.
fn scrape_page_links(url: &str) -> Result<Option<BTreeSet<String>>, ureq::Error> {
    let response = ureq::get(url).call()?;
    if response.content_type() != "text/html" {
        return Ok(None);
    }
    let page = response.into_string()?;
    Ok(Some(extract_links(&page, url)))
}

/// Extract all links from an HTML page.
///
/// If the link is relative, it gets converted to an absolute URL using the current
/// page's URL as the base.
fn extract_links(page: &str, page_url: &str) -> BTreeSet<String> {
    println!("Extracting links from {page_url}");
    let mut links = BTreeSet::new();
    let document = Html::parse_document(page);

    let link_selector = scraper::Selector::parse("a").unwrap();
    let Ok(base_url) = url::Url::parse(page_url) else {
        return links;
    };
    let Some(base_domain) = base_url.host_str() else {
        return links;
    };
    for link in document.select(&link_selector) {
        let Some(href) = link.value().attr("href") else {
            continue;
        };
        let url = url::Url::options().base_url(Some(&base_url)).parse(href);

        let Ok(mut url) = url else {
            continue;
        };
        // Strip anchors and query parameters, if any
        url.set_fragment(None);
        url.set_query(None);
        if url.host_str() == Some(base_domain) {
            links.insert(url.to_string());
        }
    }

    links
}

#[pymodule]
fn outro3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(site_map, m)?)?;
    Ok(())
}

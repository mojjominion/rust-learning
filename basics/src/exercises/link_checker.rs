use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};

use reqwest::blocking::get;
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

fn recursive(base_path: &str, tx: SyncSender<Url>) {
    let start_url = Url::parse(&base_path).unwrap();
    let response = get(start_url).unwrap();

    let base_url = response.url().to_owned();
    let document = response.text().unwrap();
    let html = Html::parse_document(&document);
    let selector = Selector::parse("a").unwrap();

    for element in html.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            match base_url.join(href) {
                Ok(url) => tx.send(url).unwrap(),
                Err(err) => {
                    println!("On {base_url}: could not parse {href:?}: {err} (ignored)",);
                }
            }
        }
    }
}

pub(crate) fn run_parallel() {
    let (tx, rx) = mpsc::sync_channel(100);
    tx.send(Url::parse("https://www.google.org").unwrap())
        .unwrap();
    let limit = Arc::new(Mutex::new(200));

    for rc in rx {
        let mut lim = limit.lock().unwrap();
        if *lim <= 0 {
            break;
        }
        *lim -= 1;

        println!("Link: {:#?} {:#?}", rc.to_string(), *lim);

        let link = rc.clone();
        let tx = tx.clone();

        std::thread::spawn(move || {
            recursive(link.as_str(), tx);
        });
    }

    drop(tx);
}

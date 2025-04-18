use std::process;
use html_parser::{Dom, Element, Node};
use anyhow::{Ok, Result};

fn is_element(node: &&Node) -> bool { 
    match node {
        Node::Element(..) => true,
        _ => false
    }
}

fn crawl_element(elem: Element) -> Result<Vec<String>> {

    let mut links: Vec<String>= Vec::new();
    for child in elem.children {
        match child {
            Node::Element(element) => {
                log::info!("Element found: {}", element.name);
                if element.name == "a" {
                    let url = 
                    match element.attributes.get("href") {
                        Some(href_opt) => href_opt,
                        None => &None
                    }.clone().unwrap();
                    log::info!("link found: {}", url);
                    links.push(url);
                }
                links.append(crawl_element(element)?.as_mut());
            }
            _ => {},
        }
    }
    Ok(links)
}

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    let html = reqwest::get(url)
        .await?
        .text()
        .await?;

    let dom = Dom::parse(&html)?;

    let mut res: Vec<String> = Vec::new();

    for child in dom.children {
        match child {
            Node::Element(elem) => {
                log::info!("Element found: {}", elem.name);
                res.append(crawl_element(elem)?.as_mut());
            },
            _ => {}
        }
    }

    Ok(res)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    match crawl_url("https://google.com").await {
        std::result::Result::Ok(res) => {
            log::info!("Links found: {:?}", res);
        },
        Err(e) => {
            log::error!("Error: {:?}", e);
            process::exit(-1);
        }
    }
}

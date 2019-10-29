use reqwest::Error;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn get_body(url: &str) -> Result<String, Error> {
    reqwest::get(url)?.text()
}

fn get_hrefs(doc: Document) -> Vec<Option<String>> {
    doc.find(Attr("id", "issues-list").descendant(Class("issue-title")).descendant(Name("a")))
        .map(|x| x.attr("href").map(|href| String::from(href)))
        .collect::<Vec<_>>()
}

fn process_doc(body: String) -> Vec<String> {
    let document: Document = Document::from(&*body);
    let issues: Vec<Option<String>> = get_hrefs(document);

    let mut issues_refs: Vec<String> = Vec::new();
    for issue in &issues {
        match issue {
            Some(du) => issues_refs.push(du.to_string()),
            _ => println!("Empty"),
        }
    }

    issues_refs
}

fn grab_issues(issues_url: &str) -> Vec<String> {
    let resp = get_body(issues_url);
    resp.map(|body| process_doc(body)).unwrap()
}

fn main() {
    for i in grab_issues("http://weekly.sfdata.io/") {
        println!("Issue link: {}", i);
    }
}

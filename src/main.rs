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

fn grab_issues(issues_url: &str) {
    let resp = get_body(issues_url);
    let document = resp.map(|body| Document::from(&*body));
    let issues: Result<Vec<Option<String>>, Error> = document.map(|doc| get_hrefs(doc));

    issues.map(|x| {
        println!("Length {}", x.len());

        for issue in &x {
            match issue {
                Some(du) => println!("{}", du),
                None => println!("Empty"),
            }
        }
    });
}

fn main() {
    grab_issues("http://weekly.sfdata.io/")
}

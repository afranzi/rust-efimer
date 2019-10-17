use reqwest::Error;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

fn get_body(url: &str) -> Result<String, Error> {
    reqwest::get(url)?.text()
}

fn grab_issues(issues_url: &str) {
    let resp = get_body(issues_url);
    let document = resp.map(|body| Document::from(&*body));

    let issues: Vec<Option<String>> = document.into_iter().map(|doc|
        doc.find(Attr("id", "issues-list").descendant(Class("issue-title")).descendant(Name("a")))
            .map(|x| x.attr("href"))
            .collect::<Vec<_>>()
    );

    issues.map(|x| x.into_iter().map(|issue| println!("{}", issue.unwrap())));
}

fn main() {
    grab_issues("http://weekly.sfdata.io/")
}

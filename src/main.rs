mod crawl;

fn grab_issues(issues_url: &str) -> Vec<String> {
    let resp = crawl::get_body(issues_url);
    resp.map(|body| crawl::process_doc(body)).unwrap()
}

fn main() {
    for i in grab_issues("http://weekly.sfdata.io/") {
        println!("Issue link: {}", i);
    }
}

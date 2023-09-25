// test me with: 
// python -c "import pylychee; print(pylychee.check_website('https://bartnick.eu'))"

use futures::StreamExt;
use lychee_lib::ClientBuilder;
use lychee_lib::{Collector, Input, InputSource, Request};
use pyo3::prelude::*;
use reqwest::Url;
use tokio::runtime::Builder;
use std::collections::HashSet;
use lychee_lib::Uri;
use futures::{stream};

async fn fetch_urls_from_url(url: &str)-> HashSet<Uri> {
    // Collect all links from the following inputs
    let inputs = vec![Input {
        source: InputSource::RemoteUrl(Box::new(Url::parse(url).unwrap())),
        file_type_hint: None,
        excluded_paths: None,
    }];

    let links = Collector::new(None) // base
        .skip_missing_inputs(false) // don't skip missing inputs? (default=false)
        .use_html5ever(false) // use html5ever for parsing? (default=false)
        .collect_links(inputs); // base url or directory
    let responses = links.await;
    responses.map(|r| r.unwrap().uri).collect().await
}


async fn inner_check(url: &str) {
    let requests = fetch_urls_from_url(url).await;
    let client = ClientBuilder::builder().build().client().unwrap();
    // panics if there are no requests
    let the_next_best_uri = requests.iter().next().unwrap();
    // weird that the uri cannot be converted to a request directly - I would have expected that
    let result = client.check(the_next_best_uri.to_string()).await.unwrap();
    dbg!("{}", result);
    let mut stream = stream::FuturesUnordered::new();
    for req in requests.iter(){
        stream.push(client.check(req.to_string()));
    }
    while let Some(result) = stream.next().await {
        dbg!("{}", result.unwrap());
    }
}

#[pyfunction]
pub fn check_website(url: &str) -> PyResult<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(inner_check(url));
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (url, *, exclude_all_private=false, exclude_private_ips=false, exclude_link_local_ips=false, exclude_loopback_ips=false, exclude_mail=false, max_redirects=10, max_retries=10, user_agent="pylyche", allow_insecure=false, require_https=false))]
pub fn check(
    url: String,
    // github_token: Option<SecretString>,
    // remaps: Option<Remaps>,
    // includes: Option<RegexSet>,
    // excludes: Option<RegexSet>,
    exclude_all_private: bool,
    exclude_private_ips: bool,
    exclude_link_local_ips: bool,
    exclude_loopback_ips: bool,
    exclude_mail: bool,
    max_redirects: usize,
    max_retries: u64,
    user_agent: &str,
    allow_insecure: bool,
    // schemes: HashSet<String>,
    // custom_headers: HeaderMap,
    // method: reqwest::Method,
    // accepted: Option<HashSet<StatusCode>>,
    // timeout: Option<Duration>,
    // retry_wait_time: Duration,
    require_https: bool,
) -> PyResult<()> {
    let builder = ClientBuilder::builder()
        // .github_token(github_token)
        // .remaps(remaps)
        // .includes(includes)
        // .excludes(excludes)
        .exclude_all_private(exclude_all_private)
        .exclude_private_ips(exclude_private_ips)
        .exclude_link_local_ips(exclude_link_local_ips)
        .exclude_loopback_ips(exclude_loopback_ips)
        .exclude_mail(exclude_mail)
        .max_redirects(max_redirects)
        .max_retries(max_retries)
        .user_agent(user_agent)
        .allow_insecure(allow_insecure)
        // .schemes(schemes)
        // .custom_headers(custom_headers)
        // .method(method)
        // .accepted(accepted)
        // .timeout(timeout)
        // .retry_wait_time(retry_wait_time)
        .require_https(require_https)
        .build();
    let client = builder.client().unwrap();
    let rt = Builder::new_current_thread().enable_all().build()?;

    let response = rt.block_on(client.check(url)).unwrap();
    println!("{response}");
    assert!(response.status().is_success());
    Ok(())
}

#[pymodule]
fn pylychee(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check, m)?)?;
    m.add_function(wrap_pyfunction!(check_website, m)?)?;
    Ok(())
}

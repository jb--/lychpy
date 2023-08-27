use lychee_lib::{remap::Remaps, ClientBuilder};
use pyo3::prelude::*;
use std::{collections::HashSet, time::Duration};
use tokio::runtime::Builder;

use http::{header::HeaderMap, StatusCode};
use regex::RegexSet;
use secrecy::SecretString;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

pub fn create_client(
    url: String,
    github_token: Option<SecretString>,
    remaps: Option<Remaps>,
    includes: Option<RegexSet>,
    excludes: Option<RegexSet>,
    exclude_all_private: bool,
    exclude_private_ips: bool,
    exclude_link_local_ips: bool,
    exclude_loopback_ips: bool,
    exclude_mail: bool,
    max_redirects: usize,
    max_retries: u64,
    user_agent: String,
    allow_insecure: bool,
    schemes: HashSet<String>,
    custom_headers: HeaderMap,
    method: reqwest::Method,
    accepted: Option<HashSet<StatusCode>>,
    timeout: Option<Duration>,
    retry_wait_time: Duration,
    require_https: bool,
) -> PyResult<()> {
    let builder = ClientBuilder::builder()
        .github_token(github_token)
        .remaps(remaps)
        .includes(includes)
        .excludes(excludes)
        .exclude_all_private(exclude_all_private)
        .exclude_private_ips(exclude_private_ips)
        .exclude_link_local_ips(exclude_link_local_ips)
        .exclude_loopback_ips(exclude_loopback_ips)
        .exclude_mail(exclude_mail)
        .max_redirects(max_redirects)
        .max_retries(max_retries)
        .user_agent(user_agent)
        .allow_insecure(allow_insecure)
        .schemes(schemes)
        .custom_headers(custom_headers)
        .method(method)
        .accepted(accepted)
        .timeout(timeout)
        .retry_wait_time(retry_wait_time)
        .require_https(require_https)
        .build();
    let client = builder.client().unwrap();
    let rt = Builder::new_current_thread().enable_all().build()?;

    let response = rt.block_on(client.check(url)).unwrap();
    println!("{response}");
    assert!(response.status().is_success());
    Ok(())
}

#[pyfunction]
fn check(url: String) -> PyResult<()> {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let response = rt.block_on(lychee_lib::check(url)).unwrap();
    println!("{response}");
    assert!(response.status().is_success());
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pylychee(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(check, m)?)?;
    Ok(())
}

// test me with: 
// python -c "import lychpy; print(lychpy.check(['https://example.com']))"


use futures::StreamExt;
use lychee_lib::ClientBuilder;
use lychee_lib::Request;
use lychee_lib::Response;
use pyo3::prelude::*;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use pyo3::types::{PyList, PyDict, PyString};

const CONCURRENT_REQUESTS: usize = 4;

#[pyclass]
struct PyResponse {
    uri: String,
    status: String,
    is_success: bool,
    is_failure: bool,
    is_excluded: bool,
    is_timeout: bool,
    is_unsupported: bool,
    icon: String,
    repr: String,
}

impl PyResponse {
    fn from(resp:  Response) -> Self {
        let response_body = resp.1;
        let status = response_body.status.code_as_string();
        let uri = response_body.uri.to_string();
        let is_success = response_body.status.is_success();
        let is_failure = response_body.status.is_failure();
        let is_excluded = response_body.status.is_excluded();
        let is_timeout = response_body.status.is_timeout();
        let is_unsupported = response_body.status.is_unsupported();
        let icon = response_body.status.icon().to_string();
        let repr = response_body.to_string();
        PyResponse { uri, status, is_success, is_failure, is_excluded, is_timeout, is_unsupported, icon , repr }
    }
}

#[pymethods]
impl PyResponse {
    #[getter(url)]
    fn uri(&self) -> PyResult<String> {
       Ok(self.uri.clone())
    }
    
    #[getter(status)]
    fn status(&self) -> PyResult<String> {
       Ok(self.status.clone())
    }

    #[getter(is_success)]
    fn is_success(&self) -> PyResult<bool> {
       Ok(self.is_success)
    }

    #[getter(is_failure)]
    fn is_failure(&self) -> PyResult<bool> {
       Ok(self.is_failure)
    }

    #[getter(is_excluded)]
    fn is_excluded(&self) -> PyResult<bool> {
       Ok(self.is_excluded)
    }

    #[getter(is_timeout)]
    fn is_timeout(&self) -> PyResult<bool> {
       Ok(self.is_timeout)
    }

    #[getter(is_unsupported)]
    fn is_unsupported(&self) -> PyResult<bool> {
       Ok(self.is_unsupported)
    }

    #[getter(icon)]
    fn icon(&self) -> PyResult<String> {
       Ok(self.icon.clone())
    }

    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        Ok(slf.repr.clone())
    }

    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String> {
        PyResponse::__str__(slf)
    }
}


#[pyfunction]
pub fn check(py: Python<'_>, urls: &PyList) -> PyResult<Py<PyDict>>{
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut array : Vec<String> = vec![String::new(); urls.len()];
    for (i, url) in urls.iter().enumerate() {
        let url = url.downcast::<PyString>()?;
        let url = url.to_string_lossy();
        array[i] = url.clone().to_string();
    }

    let (send_req, recv_req) = mpsc::channel(CONCURRENT_REQUESTS);
    let (send_resp, mut recv_resp) = mpsc::channel(CONCURRENT_REQUESTS);

    // Queue requests
    rt.spawn(async move {
        for url in array {
            if let Ok(request) = Request::try_from(url) {
                send_req.send(request).await.unwrap();
            }            
        }
    });

    // Create a default lychee client
    let client = ClientBuilder::default().client().unwrap();

    // Start receiving requests
    // Requests get streamed into the client and run concurrently
    rt.spawn(async move {
        let mystream = ReceiverStream::new(recv_req);
        mystream.for_each_concurrent(CONCURRENT_REQUESTS, 
            |req| async {
                let resp = client.check(req).await.unwrap();
                send_resp.send(resp).await.unwrap();
            },
        )
        .await;
    });


    let pymap = rt.block_on(async {
        let map = PyDict::new(py);

        while let Some(response) = recv_resp.recv().await {
            let url = response.0.to_string().clone();
            let pyresp = PyResponse::from(response);
            let pyresp_wrapped = Py::new(py, pyresp).unwrap();
            map.set_item(url, pyresp_wrapped).unwrap();
        }
        map
    });

    Ok(pymap.into())

}

#[pymodule]
fn lychpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyResponse>()?;
    m.add_function(wrap_pyfunction!(check, m)?)?;
    Ok(())
}

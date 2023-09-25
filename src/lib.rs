// test me with: 
// python -c "import pylychee; print(pylychee.check(['https://example.com']))"

use futures::StreamExt;
use lychee_lib::ClientBuilder;
use lychee_lib::Request;
use pyo3::prelude::*;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use pyo3::types::{PyList, PyDict, PyString};

const CONCURRENT_REQUESTS: usize = 4;

#[pyclass]
struct PyResponse {
    url: String,
    status: String,
}

#[pymethods]
impl PyResponse {
    #[getter(url)]
    fn url(&self) -> PyResult<String> {
       Ok(self.url.clone())
    }
    
    #[getter(status)]
    fn status(&self) -> PyResult<String> {
       Ok(self.status.clone())
    }

    fn __str__(slf: PyRef<'_, Self>) -> PyResult<String> {
        Ok(slf.url.clone())
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
            let request = Request::try_from(url).unwrap();
            send_req.send(request).await.unwrap();
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
            let pyresp = PyResponse{url:response.1.to_string(), status:response.1.to_string()} ;
            let pyresp_wrapped = Py::new(py, pyresp).unwrap();
            map.set_item(response.0.to_string(), pyresp_wrapped).unwrap();
        }
        map
    });

    Ok(pymap.into())

}

#[pymodule]
fn pylychee(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyResponse>()?;
    m.add_function(wrap_pyfunction!(check, m)?)?;
    Ok(())
}

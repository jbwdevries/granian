use pyo3::prelude::*;

use crate::{
    workers::{
        WorkerConfig,
        serve_rth,
        serve_wth,
        serve_rth_ssl,
        serve_wth_ssl
    }
};
use super::http::{handle_rtb, handle_rtt, handle_rtb_ws, handle_rtt_ws};

#[pyclass(module="granian._granian")]
pub struct RSGIWorker {
    config: WorkerConfig
}

impl RSGIWorker {
    serve_rth!(_serve_rth, handle_rtb);
    serve_rth!(_serve_rth_ws, handle_rtb_ws);
    serve_wth!(_serve_wth, handle_rtt);
    serve_wth!(_serve_wth_ws, handle_rtt_ws);
    serve_rth_ssl!(_serve_rth_ssl, handle_rtb);
    serve_rth_ssl!(_serve_rth_ssl_ws, handle_rtb_ws);
    serve_wth_ssl!(_serve_wth_ssl, handle_rtt);
    serve_wth_ssl!(_serve_wth_ssl_ws, handle_rtt_ws);
}

#[pymethods]
impl RSGIWorker {
    #[new]
    #[args(socket_fd, threads="1", http1_buffer_max="65535")]
    fn new(
        worker_id: i32,
        socket_fd: i32,
        threads: usize,
        pthreads: usize,
        http_mode: String,
        http1_buffer_max: usize,
        websockets_enabled: bool,
        ssl_enabled: bool,
        ssl_cert: Option<String>,
        ssl_key: Option<String>
    ) -> PyResult<Self> {
        Ok(Self {
            config: WorkerConfig::new(
                worker_id,
                socket_fd,
                threads,
                pthreads,
                http_mode,
                http1_buffer_max,
                websockets_enabled,
                ssl_enabled,
                ssl_cert,
                ssl_key
            )
        })
    }

    fn serve_rth(
        &self,
        callback: PyObject,
        event_loop: &PyAny,
        context: &PyAny,
        signal_rx: PyObject
    ) {
        match (self.config.websockets_enabled, self.config.ssl_enabled) {
            (false, false) => self._serve_rth(callback, event_loop, context, signal_rx),
            (true, false) => self._serve_rth_ws(callback, event_loop, context, signal_rx),
            (false, true) => self._serve_rth_ssl(callback, event_loop, context, signal_rx),
            (true, true) => self._serve_rth_ssl_ws(callback, event_loop, context, signal_rx)
        }
    }

    fn serve_wth(
        &self,
        callback: PyObject,
        event_loop: &PyAny,
        context: &PyAny,
        signal_rx: PyObject
    ) {
        match (self.config.websockets_enabled, self.config.ssl_enabled) {
            (false, false) => self._serve_wth(callback, event_loop, context, signal_rx),
            (true, false) => self._serve_wth_ws(callback, event_loop, context, signal_rx),
            (false, true) => self._serve_wth_ssl(callback, event_loop, context, signal_rx),
            (true, true) => self._serve_wth_ssl_ws(callback, event_loop, context, signal_rx)
        }
    }
}

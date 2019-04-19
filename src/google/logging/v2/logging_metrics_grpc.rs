// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_METRICS_SERVICE_V2_LIST_LOG_METRICS: ::grpcio::Method<super::logging_metrics::ListLogMetricsRequest, super::logging_metrics::ListLogMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.MetricsServiceV2/ListLogMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRICS_SERVICE_V2_GET_LOG_METRIC: ::grpcio::Method<super::logging_metrics::GetLogMetricRequest, super::logging_metrics::LogMetric> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.MetricsServiceV2/GetLogMetric",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRICS_SERVICE_V2_CREATE_LOG_METRIC: ::grpcio::Method<super::logging_metrics::CreateLogMetricRequest, super::logging_metrics::LogMetric> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.MetricsServiceV2/CreateLogMetric",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRICS_SERVICE_V2_UPDATE_LOG_METRIC: ::grpcio::Method<super::logging_metrics::UpdateLogMetricRequest, super::logging_metrics::LogMetric> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.MetricsServiceV2/UpdateLogMetric",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_METRICS_SERVICE_V2_DELETE_LOG_METRIC: ::grpcio::Method<super::logging_metrics::DeleteLogMetricRequest, super::super::super::protobuf::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.MetricsServiceV2/DeleteLogMetric",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct MetricsServiceV2Client {
    client: ::grpcio::Client,
}

impl MetricsServiceV2Client {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        MetricsServiceV2Client {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_log_metrics_opt(&self, req: &super::logging_metrics::ListLogMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_metrics::ListLogMetricsResponse> {
        self.client.unary_call(&METHOD_METRICS_SERVICE_V2_LIST_LOG_METRICS, req, opt)
    }

    pub fn list_log_metrics(&self, req: &super::logging_metrics::ListLogMetricsRequest) -> ::grpcio::Result<super::logging_metrics::ListLogMetricsResponse> {
        self.list_log_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_log_metrics_async_opt(&self, req: &super::logging_metrics::ListLogMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::ListLogMetricsResponse>> {
        self.client.unary_call_async(&METHOD_METRICS_SERVICE_V2_LIST_LOG_METRICS, req, opt)
    }

    pub fn list_log_metrics_async(&self, req: &super::logging_metrics::ListLogMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::ListLogMetricsResponse>> {
        self.list_log_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_log_metric_opt(&self, req: &super::logging_metrics::GetLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_metrics::LogMetric> {
        self.client.unary_call(&METHOD_METRICS_SERVICE_V2_GET_LOG_METRIC, req, opt)
    }

    pub fn get_log_metric(&self, req: &super::logging_metrics::GetLogMetricRequest) -> ::grpcio::Result<super::logging_metrics::LogMetric> {
        self.get_log_metric_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_log_metric_async_opt(&self, req: &super::logging_metrics::GetLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::LogMetric>> {
        self.client.unary_call_async(&METHOD_METRICS_SERVICE_V2_GET_LOG_METRIC, req, opt)
    }

    pub fn get_log_metric_async(&self, req: &super::logging_metrics::GetLogMetricRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::LogMetric>> {
        self.get_log_metric_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_log_metric_opt(&self, req: &super::logging_metrics::CreateLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_metrics::LogMetric> {
        self.client.unary_call(&METHOD_METRICS_SERVICE_V2_CREATE_LOG_METRIC, req, opt)
    }

    pub fn create_log_metric(&self, req: &super::logging_metrics::CreateLogMetricRequest) -> ::grpcio::Result<super::logging_metrics::LogMetric> {
        self.create_log_metric_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_log_metric_async_opt(&self, req: &super::logging_metrics::CreateLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::LogMetric>> {
        self.client.unary_call_async(&METHOD_METRICS_SERVICE_V2_CREATE_LOG_METRIC, req, opt)
    }

    pub fn create_log_metric_async(&self, req: &super::logging_metrics::CreateLogMetricRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::LogMetric>> {
        self.create_log_metric_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_log_metric_opt(&self, req: &super::logging_metrics::UpdateLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_metrics::LogMetric> {
        self.client.unary_call(&METHOD_METRICS_SERVICE_V2_UPDATE_LOG_METRIC, req, opt)
    }

    pub fn update_log_metric(&self, req: &super::logging_metrics::UpdateLogMetricRequest) -> ::grpcio::Result<super::logging_metrics::LogMetric> {
        self.update_log_metric_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_log_metric_async_opt(&self, req: &super::logging_metrics::UpdateLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::LogMetric>> {
        self.client.unary_call_async(&METHOD_METRICS_SERVICE_V2_UPDATE_LOG_METRIC, req, opt)
    }

    pub fn update_log_metric_async(&self, req: &super::logging_metrics::UpdateLogMetricRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_metrics::LogMetric>> {
        self.update_log_metric_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_log_metric_opt(&self, req: &super::logging_metrics::DeleteLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::super::super::protobuf::empty::Empty> {
        self.client.unary_call(&METHOD_METRICS_SERVICE_V2_DELETE_LOG_METRIC, req, opt)
    }

    pub fn delete_log_metric(&self, req: &super::logging_metrics::DeleteLogMetricRequest) -> ::grpcio::Result<super::super::super::protobuf::empty::Empty> {
        self.delete_log_metric_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_log_metric_async_opt(&self, req: &super::logging_metrics::DeleteLogMetricRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::super::super::protobuf::empty::Empty>> {
        self.client.unary_call_async(&METHOD_METRICS_SERVICE_V2_DELETE_LOG_METRIC, req, opt)
    }

    pub fn delete_log_metric_async(&self, req: &super::logging_metrics::DeleteLogMetricRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::super::super::protobuf::empty::Empty>> {
        self.delete_log_metric_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait MetricsServiceV2 {
    fn list_log_metrics(&self, ctx: ::grpcio::RpcContext, req: super::logging_metrics::ListLogMetricsRequest, sink: ::grpcio::UnarySink<super::logging_metrics::ListLogMetricsResponse>);
    fn get_log_metric(&self, ctx: ::grpcio::RpcContext, req: super::logging_metrics::GetLogMetricRequest, sink: ::grpcio::UnarySink<super::logging_metrics::LogMetric>);
    fn create_log_metric(&self, ctx: ::grpcio::RpcContext, req: super::logging_metrics::CreateLogMetricRequest, sink: ::grpcio::UnarySink<super::logging_metrics::LogMetric>);
    fn update_log_metric(&self, ctx: ::grpcio::RpcContext, req: super::logging_metrics::UpdateLogMetricRequest, sink: ::grpcio::UnarySink<super::logging_metrics::LogMetric>);
    fn delete_log_metric(&self, ctx: ::grpcio::RpcContext, req: super::logging_metrics::DeleteLogMetricRequest, sink: ::grpcio::UnarySink<super::super::super::protobuf::empty::Empty>);
}

pub fn create_metrics_service_v2<S: MetricsServiceV2 + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRICS_SERVICE_V2_LIST_LOG_METRICS, move |ctx, req, resp| {
        instance.list_log_metrics(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRICS_SERVICE_V2_GET_LOG_METRIC, move |ctx, req, resp| {
        instance.get_log_metric(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRICS_SERVICE_V2_CREATE_LOG_METRIC, move |ctx, req, resp| {
        instance.create_log_metric(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRICS_SERVICE_V2_UPDATE_LOG_METRIC, move |ctx, req, resp| {
        instance.update_log_metric(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_METRICS_SERVICE_V2_DELETE_LOG_METRIC, move |ctx, req, resp| {
        instance.delete_log_metric(ctx, req, resp)
    });
    builder.build()
}

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

const METHOD_CONFIG_SERVICE_V2_LIST_SINKS: ::grpcio::Method<super::logging_config::ListSinksRequest, super::logging_config::ListSinksResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/ListSinks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_GET_SINK: ::grpcio::Method<super::logging_config::GetSinkRequest, super::logging_config::LogSink> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/GetSink",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_CREATE_SINK: ::grpcio::Method<super::logging_config::CreateSinkRequest, super::logging_config::LogSink> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/CreateSink",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_UPDATE_SINK: ::grpcio::Method<super::logging_config::UpdateSinkRequest, super::logging_config::LogSink> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/UpdateSink",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_DELETE_SINK: ::grpcio::Method<super::logging_config::DeleteSinkRequest, super::super::super::protobuf::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/DeleteSink",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_LIST_EXCLUSIONS: ::grpcio::Method<super::logging_config::ListExclusionsRequest, super::logging_config::ListExclusionsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/ListExclusions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_GET_EXCLUSION: ::grpcio::Method<super::logging_config::GetExclusionRequest, super::logging_config::LogExclusion> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/GetExclusion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_CREATE_EXCLUSION: ::grpcio::Method<super::logging_config::CreateExclusionRequest, super::logging_config::LogExclusion> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/CreateExclusion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_UPDATE_EXCLUSION: ::grpcio::Method<super::logging_config::UpdateExclusionRequest, super::logging_config::LogExclusion> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/UpdateExclusion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CONFIG_SERVICE_V2_DELETE_EXCLUSION: ::grpcio::Method<super::logging_config::DeleteExclusionRequest, super::super::super::protobuf::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.logging.v2.ConfigServiceV2/DeleteExclusion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ConfigServiceV2Client {
    client: ::grpcio::Client,
}

impl ConfigServiceV2Client {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ConfigServiceV2Client {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_sinks_opt(&self, req: &super::logging_config::ListSinksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::ListSinksResponse> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_LIST_SINKS, req, opt)
    }

    pub fn list_sinks(&self, req: &super::logging_config::ListSinksRequest) -> ::grpcio::Result<super::logging_config::ListSinksResponse> {
        self.list_sinks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_sinks_async_opt(&self, req: &super::logging_config::ListSinksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::ListSinksResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_LIST_SINKS, req, opt)
    }

    pub fn list_sinks_async(&self, req: &super::logging_config::ListSinksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::ListSinksResponse>> {
        self.list_sinks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_sink_opt(&self, req: &super::logging_config::GetSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::LogSink> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_GET_SINK, req, opt)
    }

    pub fn get_sink(&self, req: &super::logging_config::GetSinkRequest) -> ::grpcio::Result<super::logging_config::LogSink> {
        self.get_sink_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_sink_async_opt(&self, req: &super::logging_config::GetSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogSink>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_GET_SINK, req, opt)
    }

    pub fn get_sink_async(&self, req: &super::logging_config::GetSinkRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogSink>> {
        self.get_sink_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_sink_opt(&self, req: &super::logging_config::CreateSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::LogSink> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_CREATE_SINK, req, opt)
    }

    pub fn create_sink(&self, req: &super::logging_config::CreateSinkRequest) -> ::grpcio::Result<super::logging_config::LogSink> {
        self.create_sink_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_sink_async_opt(&self, req: &super::logging_config::CreateSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogSink>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_CREATE_SINK, req, opt)
    }

    pub fn create_sink_async(&self, req: &super::logging_config::CreateSinkRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogSink>> {
        self.create_sink_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_sink_opt(&self, req: &super::logging_config::UpdateSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::LogSink> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_UPDATE_SINK, req, opt)
    }

    pub fn update_sink(&self, req: &super::logging_config::UpdateSinkRequest) -> ::grpcio::Result<super::logging_config::LogSink> {
        self.update_sink_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_sink_async_opt(&self, req: &super::logging_config::UpdateSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogSink>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_UPDATE_SINK, req, opt)
    }

    pub fn update_sink_async(&self, req: &super::logging_config::UpdateSinkRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogSink>> {
        self.update_sink_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_sink_opt(&self, req: &super::logging_config::DeleteSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::super::super::protobuf::empty::Empty> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_DELETE_SINK, req, opt)
    }

    pub fn delete_sink(&self, req: &super::logging_config::DeleteSinkRequest) -> ::grpcio::Result<super::super::super::protobuf::empty::Empty> {
        self.delete_sink_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_sink_async_opt(&self, req: &super::logging_config::DeleteSinkRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::super::super::protobuf::empty::Empty>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_DELETE_SINK, req, opt)
    }

    pub fn delete_sink_async(&self, req: &super::logging_config::DeleteSinkRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::super::super::protobuf::empty::Empty>> {
        self.delete_sink_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_exclusions_opt(&self, req: &super::logging_config::ListExclusionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::ListExclusionsResponse> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_LIST_EXCLUSIONS, req, opt)
    }

    pub fn list_exclusions(&self, req: &super::logging_config::ListExclusionsRequest) -> ::grpcio::Result<super::logging_config::ListExclusionsResponse> {
        self.list_exclusions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_exclusions_async_opt(&self, req: &super::logging_config::ListExclusionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::ListExclusionsResponse>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_LIST_EXCLUSIONS, req, opt)
    }

    pub fn list_exclusions_async(&self, req: &super::logging_config::ListExclusionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::ListExclusionsResponse>> {
        self.list_exclusions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_exclusion_opt(&self, req: &super::logging_config::GetExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::LogExclusion> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_GET_EXCLUSION, req, opt)
    }

    pub fn get_exclusion(&self, req: &super::logging_config::GetExclusionRequest) -> ::grpcio::Result<super::logging_config::LogExclusion> {
        self.get_exclusion_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_exclusion_async_opt(&self, req: &super::logging_config::GetExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogExclusion>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_GET_EXCLUSION, req, opt)
    }

    pub fn get_exclusion_async(&self, req: &super::logging_config::GetExclusionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogExclusion>> {
        self.get_exclusion_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_exclusion_opt(&self, req: &super::logging_config::CreateExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::LogExclusion> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_CREATE_EXCLUSION, req, opt)
    }

    pub fn create_exclusion(&self, req: &super::logging_config::CreateExclusionRequest) -> ::grpcio::Result<super::logging_config::LogExclusion> {
        self.create_exclusion_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_exclusion_async_opt(&self, req: &super::logging_config::CreateExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogExclusion>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_CREATE_EXCLUSION, req, opt)
    }

    pub fn create_exclusion_async(&self, req: &super::logging_config::CreateExclusionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogExclusion>> {
        self.create_exclusion_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_exclusion_opt(&self, req: &super::logging_config::UpdateExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::logging_config::LogExclusion> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_UPDATE_EXCLUSION, req, opt)
    }

    pub fn update_exclusion(&self, req: &super::logging_config::UpdateExclusionRequest) -> ::grpcio::Result<super::logging_config::LogExclusion> {
        self.update_exclusion_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_exclusion_async_opt(&self, req: &super::logging_config::UpdateExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogExclusion>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_UPDATE_EXCLUSION, req, opt)
    }

    pub fn update_exclusion_async(&self, req: &super::logging_config::UpdateExclusionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::logging_config::LogExclusion>> {
        self.update_exclusion_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_exclusion_opt(&self, req: &super::logging_config::DeleteExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::super::super::protobuf::empty::Empty> {
        self.client.unary_call(&METHOD_CONFIG_SERVICE_V2_DELETE_EXCLUSION, req, opt)
    }

    pub fn delete_exclusion(&self, req: &super::logging_config::DeleteExclusionRequest) -> ::grpcio::Result<super::super::super::protobuf::empty::Empty> {
        self.delete_exclusion_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_exclusion_async_opt(&self, req: &super::logging_config::DeleteExclusionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::super::super::protobuf::empty::Empty>> {
        self.client.unary_call_async(&METHOD_CONFIG_SERVICE_V2_DELETE_EXCLUSION, req, opt)
    }

    pub fn delete_exclusion_async(&self, req: &super::logging_config::DeleteExclusionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::super::super::protobuf::empty::Empty>> {
        self.delete_exclusion_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ConfigServiceV2 {
    fn list_sinks(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::ListSinksRequest, sink: ::grpcio::UnarySink<super::logging_config::ListSinksResponse>);
    fn get_sink(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::GetSinkRequest, sink: ::grpcio::UnarySink<super::logging_config::LogSink>);
    fn create_sink(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::CreateSinkRequest, sink: ::grpcio::UnarySink<super::logging_config::LogSink>);
    fn update_sink(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::UpdateSinkRequest, sink: ::grpcio::UnarySink<super::logging_config::LogSink>);
    fn delete_sink(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::DeleteSinkRequest, sink: ::grpcio::UnarySink<super::super::super::protobuf::empty::Empty>);
    fn list_exclusions(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::ListExclusionsRequest, sink: ::grpcio::UnarySink<super::logging_config::ListExclusionsResponse>);
    fn get_exclusion(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::GetExclusionRequest, sink: ::grpcio::UnarySink<super::logging_config::LogExclusion>);
    fn create_exclusion(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::CreateExclusionRequest, sink: ::grpcio::UnarySink<super::logging_config::LogExclusion>);
    fn update_exclusion(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::UpdateExclusionRequest, sink: ::grpcio::UnarySink<super::logging_config::LogExclusion>);
    fn delete_exclusion(&self, ctx: ::grpcio::RpcContext, req: super::logging_config::DeleteExclusionRequest, sink: ::grpcio::UnarySink<super::super::super::protobuf::empty::Empty>);
}

pub fn create_config_service_v2<S: ConfigServiceV2 + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_LIST_SINKS, move |ctx, req, resp| {
        instance.list_sinks(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_GET_SINK, move |ctx, req, resp| {
        instance.get_sink(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_CREATE_SINK, move |ctx, req, resp| {
        instance.create_sink(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_UPDATE_SINK, move |ctx, req, resp| {
        instance.update_sink(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_DELETE_SINK, move |ctx, req, resp| {
        instance.delete_sink(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_LIST_EXCLUSIONS, move |ctx, req, resp| {
        instance.list_exclusions(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_GET_EXCLUSION, move |ctx, req, resp| {
        instance.get_exclusion(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_CREATE_EXCLUSION, move |ctx, req, resp| {
        instance.create_exclusion(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_UPDATE_EXCLUSION, move |ctx, req, resp| {
        instance.update_exclusion(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CONFIG_SERVICE_V2_DELETE_EXCLUSION, move |ctx, req, resp| {
        instance.delete_exclusion(ctx, req, resp)
    });
    builder.build()
}

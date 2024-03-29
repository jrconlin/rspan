// This file is generated by rust-protobuf 2.0.2. Do not edit
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const http: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MethodOptions, ::protobuf::types::ProtobufTypeMessage<super::super::http::HttpRule>> = ::protobuf::ext::ExtFieldOptional { field_number: 72295728, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/api/annotations.proto\x12\ngoogle.api\x1a\x15google/api/htt\
    p.proto\x1a\x20google/protobuf/descriptor.proto:K\n\x04http\x18\xb0\xca\
    \xbc\"\x20\x01(\x0b2\x14.google.api.HttpRule\x12\x1e.google.protobuf.Met\
    hodOptionsR\x04httpBn\n\x0ecom.google.apiB\x10AnnotationsProtoP\x01ZAgoo\
    gle.golang.org/genproto/googleapis/api/annotations;annotations\xa2\x02\
    \x04GAPIJ\xed\x08\n\x06\x12\x04\x0e\0\x1e\x01\n\xc2\x04\n\x01\x0c\x12\
    \x03\x0e\0\x122\xb7\x04\x20Copyright\x20(c)\x202015,\x20Google\x20Inc.\n\
    \n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20file\
    \x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20ma\
    y\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x12\n\t\n\x02\x03\0\x12\
    \x03\x12\x07\x1e\n\t\n\x02\x03\x01\x12\x03\x13\x07)\n\x08\n\x01\x08\x12\
    \x03\x15\0X\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x15\0X\n\x0c\n\x05\x08\xe7\
    \x07\0\x02\x12\x03\x15\x07\x11\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x15\
    \x07\x11\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x15\x07\x11\n\x0c\n\
    \x05\x08\xe7\x07\0\x07\x12\x03\x15\x14W\n\x08\n\x01\x08\x12\x03\x16\0\"\
    \n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x16\0\"\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03\x16\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x16\x07\
    \x1a\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x16\x07\x1a\n\x0c\n\
    \x05\x08\xe7\x07\x01\x03\x12\x03\x16\x1d!\n\x08\n\x01\x08\x12\x03\x17\01\
    \n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x17\01\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03\x17\x07\x1b\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x17\x07\
    \x1b\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x17\x07\x1b\n\x0c\n\
    \x05\x08\xe7\x07\x02\x07\x12\x03\x17\x1e0\n\x08\n\x01\x08\x12\x03\x18\0'\
    \n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x18\0'\n\x0c\n\x05\x08\xe7\x07\x03\
    \x02\x12\x03\x18\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x18\x07\
    \x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x18\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\x03\x07\x12\x03\x18\x16&\n\x08\n\x01\x08\x12\x03\x19\0\
    \"\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x19\0\"\n\x0c\n\x05\x08\xe7\x07\
    \x04\x02\x12\x03\x19\x07\x18\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03\x19\
    \x07\x18\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\x19\x07\x18\n\x0c\
    \n\x05\x08\xe7\x07\x04\x07\x12\x03\x19\x1b!\n\t\n\x01\x07\x12\x04\x1b\0\
    \x1e\x01\n\x1c\n\x02\x07\0\x12\x03\x1d\x02\x1b\x1a\x11\x20See\x20`HttpRu\
    le`.\n\n\n\n\x03\x07\0\x02\x12\x03\x1b\x07$\n\x0b\n\x03\x07\0\x04\x12\
    \x04\x1d\x02\x1b&\n\n\n\x03\x07\0\x06\x12\x03\x1d\x02\n\n\n\n\x03\x07\0\
    \x01\x12\x03\x1d\x0b\x0f\n\n\n\x03\x07\0\x03\x12\x03\x1d\x12\x1ab\x06pro\
    to3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

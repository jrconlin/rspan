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

#[derive(PartialEq,Clone,Default)]
pub struct SystemParameters {
    // message fields
    pub rules: ::protobuf::RepeatedField<SystemParameterRule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl SystemParameters {
    pub fn new() -> SystemParameters {
        ::std::default::Default::default()
    }

    // repeated .google.api.SystemParameterRule rules = 1;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<SystemParameterRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<SystemParameterRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<SystemParameterRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[SystemParameterRule] {
        &self.rules
    }
}

impl ::protobuf::Message for SystemParameters {
    fn is_initialized(&self) -> bool {
        for v in &self.rules {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rules {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SystemParameters {
        SystemParameters::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SystemParameterRule>>(
                    "rules",
                    |m: &SystemParameters| { &m.rules },
                    |m: &mut SystemParameters| { &mut m.rules },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SystemParameters>(
                    "SystemParameters",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SystemParameters {
        static mut instance: ::protobuf::lazy::Lazy<SystemParameters> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SystemParameters,
        };
        unsafe {
            instance.get(SystemParameters::new)
        }
    }
}

impl ::protobuf::Clear for SystemParameters {
    fn clear(&mut self) {
        self.clear_rules();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemParameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemParameters {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SystemParameterRule {
    // message fields
    pub selector: ::std::string::String,
    pub parameters: ::protobuf::RepeatedField<SystemParameter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl SystemParameterRule {
    pub fn new() -> SystemParameterRule {
        ::std::default::Default::default()
    }

    // string selector = 1;

    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::string::String) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::string::String {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.selector, ::std::string::String::new())
    }

    pub fn get_selector(&self) -> &str {
        &self.selector
    }

    // repeated .google.api.SystemParameter parameters = 2;

    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameters(&mut self, v: ::protobuf::RepeatedField<SystemParameter>) {
        self.parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameters(&mut self) -> &mut ::protobuf::RepeatedField<SystemParameter> {
        &mut self.parameters
    }

    // Take field
    pub fn take_parameters(&mut self) -> ::protobuf::RepeatedField<SystemParameter> {
        ::std::mem::replace(&mut self.parameters, ::protobuf::RepeatedField::new())
    }

    pub fn get_parameters(&self) -> &[SystemParameter] {
        &self.parameters
    }
}

impl ::protobuf::Message for SystemParameterRule {
    fn is_initialized(&self) -> bool {
        for v in &self.parameters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.parameters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.selector.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.selector);
        }
        for value in &self.parameters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.selector.is_empty() {
            os.write_string(1, &self.selector)?;
        }
        for v in &self.parameters {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SystemParameterRule {
        SystemParameterRule::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "selector",
                    |m: &SystemParameterRule| { &m.selector },
                    |m: &mut SystemParameterRule| { &mut m.selector },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SystemParameter>>(
                    "parameters",
                    |m: &SystemParameterRule| { &m.parameters },
                    |m: &mut SystemParameterRule| { &mut m.parameters },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SystemParameterRule>(
                    "SystemParameterRule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SystemParameterRule {
        static mut instance: ::protobuf::lazy::Lazy<SystemParameterRule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SystemParameterRule,
        };
        unsafe {
            instance.get(SystemParameterRule::new)
        }
    }
}

impl ::protobuf::Clear for SystemParameterRule {
    fn clear(&mut self) {
        self.clear_selector();
        self.clear_parameters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemParameterRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemParameterRule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SystemParameter {
    // message fields
    pub name: ::std::string::String,
    pub http_header: ::std::string::String,
    pub url_query_parameter: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl SystemParameter {
    pub fn new() -> SystemParameter {
        ::std::default::Default::default()
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // string http_header = 2;

    pub fn clear_http_header(&mut self) {
        self.http_header.clear();
    }

    // Param is passed by value, moved
    pub fn set_http_header(&mut self, v: ::std::string::String) {
        self.http_header = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_header(&mut self) -> &mut ::std::string::String {
        &mut self.http_header
    }

    // Take field
    pub fn take_http_header(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.http_header, ::std::string::String::new())
    }

    pub fn get_http_header(&self) -> &str {
        &self.http_header
    }

    // string url_query_parameter = 3;

    pub fn clear_url_query_parameter(&mut self) {
        self.url_query_parameter.clear();
    }

    // Param is passed by value, moved
    pub fn set_url_query_parameter(&mut self, v: ::std::string::String) {
        self.url_query_parameter = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url_query_parameter(&mut self) -> &mut ::std::string::String {
        &mut self.url_query_parameter
    }

    // Take field
    pub fn take_url_query_parameter(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.url_query_parameter, ::std::string::String::new())
    }

    pub fn get_url_query_parameter(&self) -> &str {
        &self.url_query_parameter
    }
}

impl ::protobuf::Message for SystemParameter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.http_header)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.url_query_parameter)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.http_header.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.http_header);
        }
        if !self.url_query_parameter.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url_query_parameter);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.http_header.is_empty() {
            os.write_string(2, &self.http_header)?;
        }
        if !self.url_query_parameter.is_empty() {
            os.write_string(3, &self.url_query_parameter)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SystemParameter {
        SystemParameter::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    |m: &SystemParameter| { &m.name },
                    |m: &mut SystemParameter| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_header",
                    |m: &SystemParameter| { &m.http_header },
                    |m: &mut SystemParameter| { &mut m.http_header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url_query_parameter",
                    |m: &SystemParameter| { &m.url_query_parameter },
                    |m: &mut SystemParameter| { &mut m.url_query_parameter },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SystemParameter>(
                    "SystemParameter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SystemParameter {
        static mut instance: ::protobuf::lazy::Lazy<SystemParameter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SystemParameter,
        };
        unsafe {
            instance.get(SystemParameter::new)
        }
    }
}

impl ::protobuf::Clear for SystemParameter {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_http_header();
        self.clear_url_query_parameter();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SystemParameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SystemParameter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!google/api/system_parameter.proto\x12\ngoogle.api\"I\n\x10SystemParam\
    eters\x125\n\x05rules\x18\x01\x20\x03(\x0b2\x1f.google.api.SystemParamet\
    erRuleR\x05rules\"n\n\x13SystemParameterRule\x12\x1a\n\x08selector\x18\
    \x01\x20\x01(\tR\x08selector\x12;\n\nparameters\x18\x02\x20\x03(\x0b2\
    \x1b.google.api.SystemParameterR\nparameters\"v\n\x0fSystemParameter\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1f\n\x0bhttp_header\x18\
    \x02\x20\x01(\tR\nhttpHeader\x12.\n\x13url_query_parameter\x18\x03\x20\
    \x01(\tR\x11urlQueryParameterBv\n\x0ecom.google.apiB\x14SystemParameterP\
    rotoP\x01ZEgoogle.golang.org/genproto/googleapis/api/serviceconfig;servi\
    ceconfig\xa2\x02\x04GAPIJ\xae\x1c\n\x06\x12\x04\x0e\0_\x01\n\xbd\x04\n\
    \x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20Google\x20In\
    c.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Version\x202\
    .0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\x20fil\
    e\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20You\x20m\
    ay\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\x20\x20\
    \x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\x20requ\
    ired\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20writing,\
    \x20software\n\x20distributed\x20under\x20the\x20License\x20is\x20distri\
    buted\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\
    \x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\x20or\
    \x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\x20lan\
    guage\x20governing\x20permissions\x20and\n\x20limitations\x20under\x20th\
    e\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x12\n\x08\n\x01\x08\x12\
    \x03\x12\0\\\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x12\0\\\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x12\x07\x11\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x12\x07\x11\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x12\x07\x11\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x12\x14[\n\x08\n\x01\x08\x12\x03\
    \x13\0\"\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x13\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x13\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x13\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x13\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\x13\x1d!\n\x08\n\x01\x08\x12\x03\
    \x14\05\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x14\05\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\x14\x07\x1b\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\
    \x14\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x14\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03\x14\x1e4\n\x08\n\x01\x08\x12\x03\
    \x15\0'\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x15\0'\n\x0c\n\x05\x08\xe7\
    \x07\x03\x02\x12\x03\x15\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\
    \x15\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x15\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x15\x16&\n\x08\n\x01\x08\x12\x03\
    \x16\0\"\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x16\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x04\x02\x12\x03\x16\x07\x18\n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03\
    \x16\x07\x18\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\x03\x16\x07\x18\n\
    \x0c\n\x05\x08\xe7\x07\x04\x07\x12\x03\x16\x1b!\n\xba\x02\n\x02\x04\0\
    \x12\x04\x1f\0>\x01\x1a\xad\x02\x20###\x20System\x20parameter\x20configu\
    ration\n\n\x20A\x20system\x20parameter\x20is\x20a\x20special\x20kind\x20\
    of\x20parameter\x20defined\x20by\x20the\x20API\n\x20system,\x20not\x20by\
    \x20an\x20individual\x20API.\x20It\x20is\x20typically\x20mapped\x20to\
    \x20an\x20HTTP\x20header\n\x20and/or\x20a\x20URL\x20query\x20parameter.\
    \x20This\x20configuration\x20specifies\x20which\x20methods\n\x20change\
    \x20the\x20names\x20of\x20the\x20system\x20parameters.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\x1f\x08\x18\n\xd7\x06\n\x04\x04\0\x02\0\x12\x03=\x02)\x1a\
    \xc9\x06\x20Define\x20system\x20parameters.\n\n\x20The\x20parameters\x20\
    defined\x20here\x20will\x20override\x20the\x20default\x20parameters\n\
    \x20implemented\x20by\x20the\x20system.\x20If\x20this\x20field\x20is\x20\
    missing\x20from\x20the\x20service\n\x20config,\x20default\x20system\x20p\
    arameters\x20will\x20be\x20used.\x20Default\x20system\x20parameters\n\
    \x20and\x20names\x20is\x20implementation-dependent.\n\n\x20Example:\x20d\
    efine\x20api\x20key\x20for\x20all\x20methods\n\n\x20\x20\x20\x20\x20syst\
    em_parameters\n\x20\x20\x20\x20\x20\x20\x20rules:\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20-\x20selector:\x20\"*\"\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20parameters:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20-\x20name:\x20api_key\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20url_query_parameter:\x20api_key\n\n\n\x20Example\
    :\x20define\x202\x20api\x20key\x20names\x20for\x20a\x20specific\x20metho\
    d.\n\n\x20\x20\x20\x20\x20system_parameters\n\x20\x20\x20\x20\x20\x20\
    \x20rules:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20selector:\x20\"/Lis\
    tShelves\"\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20parameters:\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20name:\x20api_ke\
    y\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20http_head\
    er:\x20Api-Key1\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20-\
    \x20name:\x20api_key\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20http_header:\x20Api-Key2\n\n\x20**NOTE:**\x20All\x20service\
    \x20configuration\x20rules\x20follow\x20\"last\x20one\x20wins\"\x20order\
    .\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03=\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03=\x0b\x1e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03=\x1f$\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03='(\n^\n\x02\x04\x01\x12\x04B\0O\x01\x1aR\
    \x20Define\x20a\x20system\x20parameter\x20rule\x20mapping\x20system\x20p\
    arameter\x20definitions\x20to\n\x20methods.\n\n\n\n\x03\x04\x01\x01\x12\
    \x03B\x08\x1b\n\xbe\x01\n\x04\x04\x01\x02\0\x12\x03G\x02\x16\x1a\xb0\x01\
    \x20Selects\x20the\x20methods\x20to\x20which\x20this\x20rule\x20applies.\
    \x20Use\x20'*'\x20to\x20indicate\x20all\n\x20methods\x20in\x20all\x20API\
    s.\n\n\x20Refer\x20to\x20[selector][google.api.DocumentationRule.selecto\
    r]\x20for\x20syntax\x20details.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04G\
    \x02B\x1d\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03G\x02\x08\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03G\t\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03G\x14\
    \x15\n\xa4\x02\n\x04\x04\x01\x02\x01\x12\x03N\x02*\x1a\x96\x02\x20Define\
    \x20parameters.\x20Multiple\x20names\x20may\x20be\x20defined\x20for\x20a\
    \x20parameter.\n\x20For\x20a\x20given\x20method\x20call,\x20only\x20one\
    \x20of\x20them\x20should\x20be\x20used.\x20If\x20multiple\n\x20names\x20\
    are\x20used\x20the\x20behavior\x20is\x20implementation-dependent.\n\x20I\
    f\x20none\x20of\x20the\x20specified\x20names\x20are\x20present\x20the\
    \x20behavior\x20is\n\x20parameter-dependent.\n\n\x0c\n\x05\x04\x01\x02\
    \x01\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03N\x0b\x1a\
    \n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03N\x1b%\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03N()\n\xc8\x01\n\x02\x04\x02\x12\x04T\0_\x01\x1a\xbb\x01\
    \x20Define\x20a\x20parameter's\x20name\x20and\x20location.\x20The\x20par\
    ameter\x20may\x20be\x20passed\x20as\x20either\n\x20an\x20HTTP\x20header\
    \x20or\x20a\x20URL\x20query\x20parameter,\x20and\x20if\x20both\x20are\
    \x20passed\x20the\x20behavior\n\x20is\x20implementation-dependent.\n\n\n\
    \n\x03\x04\x02\x01\x12\x03T\x08\x17\nZ\n\x04\x04\x02\x02\0\x12\x03V\x02\
    \x12\x1aM\x20Define\x20the\x20name\x20of\x20the\x20parameter,\x20such\
    \x20as\x20\"api_key\"\x20.\x20It\x20is\x20case\x20sensitive.\n\n\r\n\x05\
    \x04\x02\x02\0\x04\x12\x04V\x02T\x19\n\x0c\n\x05\x04\x02\x02\0\x05\x12\
    \x03V\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03V\t\r\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03V\x10\x11\n]\n\x04\x04\x02\x02\x01\x12\x03Z\x02\
    \x19\x1aP\x20Define\x20the\x20HTTP\x20header\x20name\x20to\x20use\x20for\
    \x20the\x20parameter.\x20It\x20is\x20case\n\x20insensitive.\n\n\r\n\x05\
    \x04\x02\x02\x01\x04\x12\x04Z\x02V\x12\n\x0c\n\x05\x04\x02\x02\x01\x05\
    \x12\x03Z\x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03Z\t\x14\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03Z\x17\x18\nc\n\x04\x04\x02\x02\x02\x12\
    \x03^\x02!\x1aV\x20Define\x20the\x20URL\x20query\x20parameter\x20name\
    \x20to\x20use\x20for\x20the\x20parameter.\x20It\x20is\x20case\n\x20sensi\
    tive.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04^\x02Z\x19\n\x0c\n\x05\x04\
    \x02\x02\x02\x05\x12\x03^\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\
    \x03^\t\x1c\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03^\x1f\x20b\x06proto3\
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

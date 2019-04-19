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
pub struct SetIamPolicyRequest {
    // message fields
    pub resource: ::std::string::String,
    pub policy: ::protobuf::SingularPtrField<super::policy::Policy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl SetIamPolicyRequest {
    pub fn new() -> SetIamPolicyRequest {
        ::std::default::Default::default()
    }

    // string resource = 1;

    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        &mut self.resource
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource, ::std::string::String::new())
    }

    pub fn get_resource(&self) -> &str {
        &self.resource
    }

    // .google.iam.v1.Policy policy = 2;

    pub fn clear_policy(&mut self) {
        self.policy.clear();
    }

    pub fn has_policy(&self) -> bool {
        self.policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policy(&mut self, v: super::policy::Policy) {
        self.policy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_policy(&mut self) -> &mut super::policy::Policy {
        if self.policy.is_none() {
            self.policy.set_default();
        }
        self.policy.as_mut().unwrap()
    }

    // Take field
    pub fn take_policy(&mut self) -> super::policy::Policy {
        self.policy.take().unwrap_or_else(|| super::policy::Policy::new())
    }

    pub fn get_policy(&self) -> &super::policy::Policy {
        self.policy.as_ref().unwrap_or_else(|| super::policy::Policy::default_instance())
    }
}

impl ::protobuf::Message for SetIamPolicyRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.policy {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resource)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.policy)?;
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
        if !self.resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource);
        }
        if let Some(ref v) = self.policy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.resource.is_empty() {
            os.write_string(1, &self.resource)?;
        }
        if let Some(ref v) = self.policy.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> SetIamPolicyRequest {
        SetIamPolicyRequest::new()
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
                    "resource",
                    |m: &SetIamPolicyRequest| { &m.resource },
                    |m: &mut SetIamPolicyRequest| { &mut m.resource },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::policy::Policy>>(
                    "policy",
                    |m: &SetIamPolicyRequest| { &m.policy },
                    |m: &mut SetIamPolicyRequest| { &mut m.policy },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetIamPolicyRequest>(
                    "SetIamPolicyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SetIamPolicyRequest {
        static mut instance: ::protobuf::lazy::Lazy<SetIamPolicyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetIamPolicyRequest,
        };
        unsafe {
            instance.get(SetIamPolicyRequest::new)
        }
    }
}

impl ::protobuf::Clear for SetIamPolicyRequest {
    fn clear(&mut self) {
        self.clear_resource();
        self.clear_policy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetIamPolicyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetIamPolicyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetIamPolicyRequest {
    // message fields
    pub resource: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl GetIamPolicyRequest {
    pub fn new() -> GetIamPolicyRequest {
        ::std::default::Default::default()
    }

    // string resource = 1;

    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        &mut self.resource
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource, ::std::string::String::new())
    }

    pub fn get_resource(&self) -> &str {
        &self.resource
    }
}

impl ::protobuf::Message for GetIamPolicyRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resource)?;
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
        if !self.resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.resource.is_empty() {
            os.write_string(1, &self.resource)?;
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

    fn new() -> GetIamPolicyRequest {
        GetIamPolicyRequest::new()
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
                    "resource",
                    |m: &GetIamPolicyRequest| { &m.resource },
                    |m: &mut GetIamPolicyRequest| { &mut m.resource },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetIamPolicyRequest>(
                    "GetIamPolicyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GetIamPolicyRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetIamPolicyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetIamPolicyRequest,
        };
        unsafe {
            instance.get(GetIamPolicyRequest::new)
        }
    }
}

impl ::protobuf::Clear for GetIamPolicyRequest {
    fn clear(&mut self) {
        self.clear_resource();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetIamPolicyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetIamPolicyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestIamPermissionsRequest {
    // message fields
    pub resource: ::std::string::String,
    pub permissions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl TestIamPermissionsRequest {
    pub fn new() -> TestIamPermissionsRequest {
        ::std::default::Default::default()
    }

    // string resource = 1;

    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        &mut self.resource
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.resource, ::std::string::String::new())
    }

    pub fn get_resource(&self) -> &str {
        &self.resource
    }

    // repeated string permissions = 2;

    pub fn clear_permissions(&mut self) {
        self.permissions.clear();
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.permissions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_permissions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.permissions
    }

    // Take field
    pub fn take_permissions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.permissions, ::protobuf::RepeatedField::new())
    }

    pub fn get_permissions(&self) -> &[::std::string::String] {
        &self.permissions
    }
}

impl ::protobuf::Message for TestIamPermissionsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.resource)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.permissions)?;
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
        if !self.resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.resource);
        }
        for value in &self.permissions {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.resource.is_empty() {
            os.write_string(1, &self.resource)?;
        }
        for v in &self.permissions {
            os.write_string(2, &v)?;
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

    fn new() -> TestIamPermissionsRequest {
        TestIamPermissionsRequest::new()
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
                    "resource",
                    |m: &TestIamPermissionsRequest| { &m.resource },
                    |m: &mut TestIamPermissionsRequest| { &mut m.resource },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "permissions",
                    |m: &TestIamPermissionsRequest| { &m.permissions },
                    |m: &mut TestIamPermissionsRequest| { &mut m.permissions },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestIamPermissionsRequest>(
                    "TestIamPermissionsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TestIamPermissionsRequest {
        static mut instance: ::protobuf::lazy::Lazy<TestIamPermissionsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestIamPermissionsRequest,
        };
        unsafe {
            instance.get(TestIamPermissionsRequest::new)
        }
    }
}

impl ::protobuf::Clear for TestIamPermissionsRequest {
    fn clear(&mut self) {
        self.clear_resource();
        self.clear_permissions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestIamPermissionsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestIamPermissionsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestIamPermissionsResponse {
    // message fields
    pub permissions: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl TestIamPermissionsResponse {
    pub fn new() -> TestIamPermissionsResponse {
        ::std::default::Default::default()
    }

    // repeated string permissions = 1;

    pub fn clear_permissions(&mut self) {
        self.permissions.clear();
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.permissions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_permissions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.permissions
    }

    // Take field
    pub fn take_permissions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.permissions, ::protobuf::RepeatedField::new())
    }

    pub fn get_permissions(&self) -> &[::std::string::String] {
        &self.permissions
    }
}

impl ::protobuf::Message for TestIamPermissionsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.permissions)?;
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
        for value in &self.permissions {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.permissions {
            os.write_string(1, &v)?;
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

    fn new() -> TestIamPermissionsResponse {
        TestIamPermissionsResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "permissions",
                    |m: &TestIamPermissionsResponse| { &m.permissions },
                    |m: &mut TestIamPermissionsResponse| { &mut m.permissions },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TestIamPermissionsResponse>(
                    "TestIamPermissionsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TestIamPermissionsResponse {
        static mut instance: ::protobuf::lazy::Lazy<TestIamPermissionsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TestIamPermissionsResponse,
        };
        unsafe {
            instance.get(TestIamPermissionsResponse::new)
        }
    }
}

impl ::protobuf::Clear for TestIamPermissionsResponse {
    fn clear(&mut self) {
        self.clear_permissions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestIamPermissionsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestIamPermissionsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egoogle/iam/v1/iam_policy.proto\x12\rgoogle.iam.v1\x1a\x1cgoogle/ap\
    i/annotations.proto\x1a\x1agoogle/iam/v1/policy.proto\"`\n\x13SetIamPoli\
    cyRequest\x12\x1a\n\x08resource\x18\x01\x20\x01(\tR\x08resource\x12-\n\
    \x06policy\x18\x02\x20\x01(\x0b2\x15.google.iam.v1.PolicyR\x06policy\"1\
    \n\x13GetIamPolicyRequest\x12\x1a\n\x08resource\x18\x01\x20\x01(\tR\x08r\
    esource\"Y\n\x19TestIamPermissionsRequest\x12\x1a\n\x08resource\x18\x01\
    \x20\x01(\tR\x08resource\x12\x20\n\x0bpermissions\x18\x02\x20\x03(\tR\
    \x0bpermissions\">\n\x1aTestIamPermissionsResponse\x12\x20\n\x0bpermissi\
    ons\x18\x01\x20\x03(\tR\x0bpermissions2\x94\x03\n\tIAMPolicy\x12t\n\x0cS\
    etIamPolicy\x12\".google.iam.v1.SetIamPolicyRequest\x1a\x15.google.iam.v\
    1.Policy\")\x82\xd3\xe4\x93\x02#\"\x1e/v1/{resource=**}:setIamPolicy:\
    \x01*\x12t\n\x0cGetIamPolicy\x12\".google.iam.v1.GetIamPolicyRequest\x1a\
    \x15.google.iam.v1.Policy\")\x82\xd3\xe4\x93\x02#\"\x1e/v1/{resource=**}\
    :getIamPolicy:\x01*\x12\x9a\x01\n\x12TestIamPermissions\x12(.google.iam.\
    v1.TestIamPermissionsRequest\x1a).google.iam.v1.TestIamPermissionsRespon\
    se\"/\x82\xd3\xe4\x93\x02)\"$/v1/{resource=**}:testIamPermissions:\x01*B\
    \x86\x01\n\x11com.google.iam.v1B\x0eIamPolicyProtoP\x01Z0google.golang.o\
    rg/genproto/googleapis/iam/v1;iam\xf8\x01\x01\xaa\x02\x13Google.Cloud.Ia\
    m.V1\xca\x02\x13Google\\Cloud\\Iam\\V1J\xfa%\n\x06\x12\x04\x0e\0v\x01\n\
    \xbd\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202016\x20Go\
    ogle\x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Ve\
    rsion\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20t\
    his\x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\
    \x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\
    \x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Un\
    less\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\
    \x20writing,\x20software\n\x20distributed\x20under\x20the\x20License\x20\
    is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20\
    WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20expres\
    s\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x15\n\t\n\x02\x03\
    \0\x12\x03\x12\x07%\n\t\n\x02\x03\x01\x12\x03\x13\x07#\n\x08\n\x01\x08\
    \x12\x03\x15\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x15\0\x1f\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x15\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x15\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x15\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x15\x1a\x1e\n\x08\n\x01\x08\
    \x12\x03\x16\00\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x16\00\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\x16\x07\x17\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\x16\x07\x17\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x16\
    \x07\x17\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x16\x1a/\n\x08\n\x01\
    \x08\x12\x03\x17\0G\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x17\0G\n\x0c\n\
    \x05\x08\xe7\x07\x02\x02\x12\x03\x17\x07\x11\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x17\x07\x11\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x17\x07\x11\n\x0c\n\x05\x08\xe7\x07\x02\x07\x12\x03\x17\x14F\n\x08\n\
    \x01\x08\x12\x03\x18\0\"\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x18\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x18\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x18\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x18\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x03\x03\x12\x03\x18\x1d!\n\x08\
    \n\x01\x08\x12\x03\x19\0/\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03\x19\0/\n\
    \x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03\x19\x07\x1b\n\r\n\x06\x08\xe7\x07\
    \x04\x02\0\x12\x03\x19\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\
    \x03\x19\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x04\x07\x12\x03\x19\x1e.\n\x08\
    \n\x01\x08\x12\x03\x1a\0*\n\x0b\n\x04\x08\xe7\x07\x05\x12\x03\x1a\0*\n\
    \x0c\n\x05\x08\xe7\x07\x05\x02\x12\x03\x1a\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x05\x02\0\x12\x03\x1a\x07\x13\n\x0e\n\x07\x08\xe7\x07\x05\x02\0\x01\x12\
    \x03\x1a\x07\x13\n\x0c\n\x05\x08\xe7\x07\x05\x07\x12\x03\x1a\x16)\n\x08\
    \n\x01\x08\x12\x03\x1b\00\n\x0b\n\x04\x08\xe7\x07\x06\x12\x03\x1b\00\n\
    \x0c\n\x05\x08\xe7\x07\x06\x02\x12\x03\x1b\x07\x14\n\r\n\x06\x08\xe7\x07\
    \x06\x02\0\x12\x03\x1b\x07\x14\n\x0e\n\x07\x08\xe7\x07\x06\x02\0\x01\x12\
    \x03\x1b\x07\x14\n\x0c\n\x05\x08\xe7\x07\x06\x07\x12\x03\x1b\x17/\n\xbb\
    \x07\n\x02\x06\0\x12\x047\0K\x01\x1a\xae\x07\x20##\x20API\x20Overview\n\
    \n\x20Manages\x20Identity\x20and\x20Access\x20Management\x20(IAM)\x20pol\
    icies.\n\n\x20Any\x20implementation\x20of\x20an\x20API\x20that\x20offers\
    \x20access\x20control\x20features\n\x20implements\x20the\x20google.iam.v\
    1.IAMPolicy\x20interface.\n\n\x20##\x20Data\x20model\n\n\x20Access\x20co\
    ntrol\x20is\x20applied\x20when\x20a\x20principal\x20(user\x20or\x20servi\
    ce\x20account),\x20takes\n\x20some\x20action\x20on\x20a\x20resource\x20e\
    xposed\x20by\x20a\x20service.\x20Resources,\x20identified\x20by\n\x20URI\
    -like\x20names,\x20are\x20the\x20unit\x20of\x20access\x20control\x20spec\
    ification.\x20Service\n\x20implementations\x20can\x20choose\x20the\x20gr\
    anularity\x20of\x20access\x20control\x20and\x20the\n\x20supported\x20per\
    missions\x20for\x20their\x20resources.\n\x20For\x20example\x20one\x20dat\
    abase\x20service\x20may\x20allow\x20access\x20control\x20to\x20be\n\x20s\
    pecified\x20only\x20at\x20the\x20Table\x20level,\x20whereas\x20another\
    \x20might\x20allow\x20access\x20control\n\x20to\x20also\x20be\x20specifi\
    ed\x20at\x20the\x20Column\x20level.\n\n\x20##\x20Policy\x20Structure\n\n\
    \x20See\x20google.iam.v1.Policy\n\n\x20This\x20is\x20intentionally\x20no\
    t\x20a\x20CRUD\x20style\x20API\x20because\x20access\x20control\x20polici\
    es\n\x20are\x20created\x20and\x20deleted\x20implicitly\x20with\x20the\
    \x20resources\x20to\x20which\x20they\x20are\n\x20attached.\n\n\n\n\x03\
    \x06\0\x01\x12\x037\x08\x11\nh\n\x04\x06\0\x02\0\x12\x04:\x02<\x03\x1aZ\
    \x20Sets\x20the\x20access\x20control\x20policy\x20on\x20the\x20specified\
    \x20resource.\x20Replaces\x20any\n\x20existing\x20policy.\n\n\x0c\n\x05\
    \x06\0\x02\0\x01\x12\x03:\x06\x12\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03:\
    \x13&\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03:17\n\x0c\n\x05\x06\0\x02\0\x04\
    \x12\x03;\x04T\n\x0f\n\x08\x06\0\x02\0\x04\xe7\x07\0\x12\x03;\x04T\n\x10\
    \n\t\x06\0\x02\0\x04\xe7\x07\0\x02\x12\x03;\x0b\x1c\n\x11\n\n\x06\0\x02\
    \0\x04\xe7\x07\0\x02\0\x12\x03;\x0b\x1c\n\x12\n\x0b\x06\0\x02\0\x04\xe7\
    \x07\0\x02\0\x01\x12\x03;\x0c\x1b\n\x10\n\t\x06\0\x02\0\x04\xe7\x07\0\
    \x08\x12\x03;\x1fS\n\x90\x01\n\x04\x06\0\x02\x01\x12\x04A\x02C\x03\x1a\
    \x81\x01\x20Gets\x20the\x20access\x20control\x20policy\x20for\x20a\x20re\
    source.\n\x20Returns\x20an\x20empty\x20policy\x20if\x20the\x20resource\
    \x20exists\x20and\x20does\x20not\x20have\x20a\x20policy\n\x20set.\n\n\
    \x0c\n\x05\x06\0\x02\x01\x01\x12\x03A\x06\x12\n\x0c\n\x05\x06\0\x02\x01\
    \x02\x12\x03A\x13&\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03A17\n\x0c\n\x05\
    \x06\0\x02\x01\x04\x12\x03B\x04T\n\x0f\n\x08\x06\0\x02\x01\x04\xe7\x07\0\
    \x12\x03B\x04T\n\x10\n\t\x06\0\x02\x01\x04\xe7\x07\0\x02\x12\x03B\x0b\
    \x1c\n\x11\n\n\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x12\x03B\x0b\x1c\n\x12\
    \n\x0b\x06\0\x02\x01\x04\xe7\x07\0\x02\0\x01\x12\x03B\x0c\x1b\n\x10\n\t\
    \x06\0\x02\x01\x04\xe7\x07\0\x08\x12\x03B\x1fS\n\xb8\x01\n\x04\x06\0\x02\
    \x02\x12\x04H\x02J\x03\x1a\xa9\x01\x20Returns\x20permissions\x20that\x20\
    a\x20caller\x20has\x20on\x20the\x20specified\x20resource.\n\x20If\x20the\
    \x20resource\x20does\x20not\x20exist,\x20this\x20will\x20return\x20an\
    \x20empty\x20set\x20of\n\x20permissions,\x20not\x20a\x20NOT_FOUND\x20err\
    or.\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03H\x06\x18\n\x0c\n\x05\x06\0\
    \x02\x02\x02\x12\x03H\x192\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03H=W\n\
    \x0c\n\x05\x06\0\x02\x02\x04\x12\x03I\x04Z\n\x0f\n\x08\x06\0\x02\x02\x04\
    \xe7\x07\0\x12\x03I\x04Z\n\x10\n\t\x06\0\x02\x02\x04\xe7\x07\0\x02\x12\
    \x03I\x0b\x1c\n\x11\n\n\x06\0\x02\x02\x04\xe7\x07\0\x02\0\x12\x03I\x0b\
    \x1c\n\x12\n\x0b\x06\0\x02\x02\x04\xe7\x07\0\x02\0\x01\x12\x03I\x0c\x1b\
    \n\x10\n\t\x06\0\x02\x02\x04\xe7\x07\0\x08\x12\x03I\x1fY\n8\n\x02\x04\0\
    \x12\x04N\0Y\x01\x1a,\x20Request\x20message\x20for\x20`SetIamPolicy`\x20\
    method.\n\n\n\n\x03\x04\0\x01\x12\x03N\x08\x1b\n\xc2\x01\n\x04\x04\0\x02\
    \0\x12\x03R\x02\x16\x1a\xb4\x01\x20REQUIRED:\x20The\x20resource\x20for\
    \x20which\x20the\x20policy\x20is\x20being\x20specified.\n\x20`resource`\
    \x20is\x20usually\x20specified\x20as\x20a\x20path.\x20For\x20example,\
    \x20a\x20Project\n\x20resource\x20is\x20specified\x20as\x20`projects/{pr\
    oject}`.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04R\x02N\x1d\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03R\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03R\t\x11\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03R\x14\x15\n\xf3\x01\n\x04\x04\0\x02\
    \x01\x12\x03X\x02\x14\x1a\xe5\x01\x20REQUIRED:\x20The\x20complete\x20pol\
    icy\x20to\x20be\x20applied\x20to\x20the\x20`resource`.\x20The\x20size\
    \x20of\n\x20the\x20policy\x20is\x20limited\x20to\x20a\x20few\x2010s\x20o\
    f\x20KB.\x20An\x20empty\x20policy\x20is\x20a\n\x20valid\x20policy\x20but\
    \x20certain\x20Cloud\x20Platform\x20services\x20(such\x20as\x20Projects)\
    \n\x20might\x20reject\x20them.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04X\
    \x02R\x16\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03X\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03X\t\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03X\x12\
    \x13\n8\n\x02\x04\x01\x12\x04\\\0a\x01\x1a,\x20Request\x20message\x20for\
    \x20`GetIamPolicy`\x20method.\n\n\n\n\x03\x04\x01\x01\x12\x03\\\x08\x1b\
    \n\xc2\x01\n\x04\x04\x01\x02\0\x12\x03`\x02\x16\x1a\xb4\x01\x20REQUIRED:\
    \x20The\x20resource\x20for\x20which\x20the\x20policy\x20is\x20being\x20r\
    equested.\n\x20`resource`\x20is\x20usually\x20specified\x20as\x20a\x20pa\
    th.\x20For\x20example,\x20a\x20Project\n\x20resource\x20is\x20specified\
    \x20as\x20`projects/{project}`.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04`\
    \x02\\\x1d\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03`\x02\x08\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03`\t\x11\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03`\
    \x14\x15\n>\n\x02\x04\x02\x12\x04d\0o\x01\x1a2\x20Request\x20message\x20\
    for\x20`TestIamPermissions`\x20method.\n\n\n\n\x03\x04\x02\x01\x12\x03d\
    \x08!\n\xc9\x01\n\x04\x04\x02\x02\0\x12\x03h\x02\x16\x1a\xbb\x01\x20REQU\
    IRED:\x20The\x20resource\x20for\x20which\x20the\x20policy\x20detail\x20i\
    s\x20being\x20requested.\n\x20`resource`\x20is\x20usually\x20specified\
    \x20as\x20a\x20path.\x20For\x20example,\x20a\x20Project\n\x20resource\
    \x20is\x20specified\x20as\x20`projects/{project}`.\n\n\r\n\x05\x04\x02\
    \x02\0\x04\x12\x04h\x02d#\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03h\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03h\t\x11\n\x0c\n\x05\x04\x02\x02\0\
    \x03\x12\x03h\x14\x15\n\xf0\x01\n\x04\x04\x02\x02\x01\x12\x03n\x02\"\x1a\
    \xe2\x01\x20The\x20set\x20of\x20permissions\x20to\x20check\x20for\x20the\
    \x20`resource`.\x20Permissions\x20with\n\x20wildcards\x20(such\x20as\x20\
    '*'\x20or\x20'storage.*')\x20are\x20not\x20allowed.\x20For\x20more\n\x20\
    information\x20see\n\x20[IAM\x20Overview](https://cloud.google.com/iam/d\
    ocs/overview#permissions).\n\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03n\
    \x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03n\x0b\x11\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03n\x12\x1d\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03n\x20!\n?\n\x02\x04\x03\x12\x04r\0v\x01\x1a3\x20Response\x20message\
    \x20for\x20`TestIamPermissions`\x20method.\n\n\n\n\x03\x04\x03\x01\x12\
    \x03r\x08\"\n\\\n\x04\x04\x03\x02\0\x12\x03u\x02\"\x1aO\x20A\x20subset\
    \x20of\x20`TestPermissionsRequest.permissions`\x20that\x20the\x20caller\
    \x20is\n\x20allowed.\n\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03u\x02\n\n\
    \x0c\n\x05\x04\x03\x02\0\x05\x12\x03u\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03u\x12\x1d\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03u\x20!b\x06pr\
    oto3\
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

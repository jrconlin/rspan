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
pub struct Empty {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Empty>(
                    "Empty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Empty {
        static mut instance: ::protobuf::lazy::Lazy<Empty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Empty,
        };
        unsafe {
            instance.get(Empty::new)
        }
    }
}

impl ::protobuf::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Empty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgoogle/protobuf/empty.proto\x12\x0fgoogle.protobuf\"\x07\n\x05Empt\
    yBv\n\x13com.google.protobufB\nEmptyProtoP\x01Z'github.com/golang/protob\
    uf/ptypes/empty\xf8\x01\x01\xa2\x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.We\
    llKnownTypesJ\xa9\x14\n\x06\x12\x04\x1e\03\x10\n\xcc\x0c\n\x01\x0c\x12\
    \x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20data\
    \x20interchange\x20format\n\x20Copyright\x202008\x20Google\x20Inc.\x20\
    \x20All\x20rights\x20reserved.\n\x20https://developers.google.com/protoc\
    ol-buffers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\x20and\
    \x20binary\x20forms,\x20with\x20or\x20without\n\x20modification,\x20are\
    \x20permitted\x20provided\x20that\x20the\x20following\x20conditions\x20a\
    re\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\x20source\
    \x20code\x20must\x20retain\x20the\x20above\x20copyright\n\x20notice,\x20\
    this\x20list\x20of\x20conditions\x20and\x20the\x20following\x20disclaime\
    r.\n\x20\x20\x20\x20\x20*\x20Redistributions\x20in\x20binary\x20form\x20\
    must\x20reproduce\x20the\x20above\n\x20copyright\x20notice,\x20this\x20l\
    ist\x20of\x20conditions\x20and\x20the\x20following\x20disclaimer\n\x20in\
    \x20the\x20documentation\x20and/or\x20other\x20materials\x20provided\x20\
    with\x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\x20Neither\x20the\
    \x20name\x20of\x20Google\x20Inc.\x20nor\x20the\x20names\x20of\x20its\n\
    \x20contributors\x20may\x20be\x20used\x20to\x20endorse\x20or\x20promote\
    \x20products\x20derived\x20from\n\x20this\x20software\x20without\x20spec\
    ific\x20prior\x20written\x20permission.\n\n\x20THIS\x20SOFTWARE\x20IS\
    \x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTORS\
    \n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANTI\
    ES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\
    \x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\
    \x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20\
    SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIAB\
    LE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\
    \x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\
    \x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\
    \x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\
    \x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\
    \x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CONTRACT,\
    \x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGENCE\x20\
    OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20U\
    SE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\
    \x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\x02\x12\x03\x20\
    \x08\x17\n\x08\n\x01\x08\x12\x03\"\0;\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\
    \"\0;\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\"\x07\x17\n\r\n\x06\x08\xe7\
    \x07\0\x02\0\x12\x03\"\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\"\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\"\x1a:\n\x08\n\x01\
    \x08\x12\x03#\0>\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03#\0>\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03#\x07\x11\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03#\x07\x11\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03#\x07\x11\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03#\x14=\n\x08\n\x01\x08\x12\x03$\0,\
    \n\x0b\n\x04\x08\xe7\x07\x02\x12\x03$\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\
    \x12\x03$\x07\x13\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03$\x07\x13\n\x0e\
    \n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03$\x07\x13\n\x0c\n\x05\x08\xe7\
    \x07\x02\x07\x12\x03$\x16+\n\x08\n\x01\x08\x12\x03%\0+\n\x0b\n\x04\x08\
    \xe7\x07\x03\x12\x03%\0+\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03%\x07\
    \x1b\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03%\x07\x1b\n\x0e\n\x07\x08\
    \xe7\x07\x03\x02\0\x01\x12\x03%\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x03\x07\
    \x12\x03%\x1e*\n\x08\n\x01\x08\x12\x03&\0\"\n\x0b\n\x04\x08\xe7\x07\x04\
    \x12\x03&\0\"\n\x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03&\x07\x1a\n\r\n\x06\
    \x08\xe7\x07\x04\x02\0\x12\x03&\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x04\x02\
    \0\x01\x12\x03&\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x04\x03\x12\x03&\x1d!\n\
    \x08\n\x01\x08\x12\x03'\0!\n\x0b\n\x04\x08\xe7\x07\x05\x12\x03'\0!\n\x0c\
    \n\x05\x08\xe7\x07\x05\x02\x12\x03'\x07\x18\n\r\n\x06\x08\xe7\x07\x05\
    \x02\0\x12\x03'\x07\x18\n\x0e\n\x07\x08\xe7\x07\x05\x02\0\x01\x12\x03'\
    \x07\x18\n\x0c\n\x05\x08\xe7\x07\x05\x07\x12\x03'\x1b\x20\n\x08\n\x01\
    \x08\x12\x03(\0\x1f\n\x0b\n\x04\x08\xe7\x07\x06\x12\x03(\0\x1f\n\x0c\n\
    \x05\x08\xe7\x07\x06\x02\x12\x03(\x07\x17\n\r\n\x06\x08\xe7\x07\x06\x02\
    \0\x12\x03(\x07\x17\n\x0e\n\x07\x08\xe7\x07\x06\x02\0\x01\x12\x03(\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\x06\x03\x12\x03(\x1a\x1e\n\xfb\x02\n\x02\
    \x04\0\x12\x033\0\x10\x1a\xef\x02\x20A\x20generic\x20empty\x20message\
    \x20that\x20you\x20can\x20re-use\x20to\x20avoid\x20defining\x20duplicate\
    d\n\x20empty\x20messages\x20in\x20your\x20APIs.\x20A\x20typical\x20examp\
    le\x20is\x20to\x20use\x20it\x20as\x20the\x20request\n\x20or\x20the\x20re\
    sponse\x20type\x20of\x20an\x20API\x20method.\x20For\x20instance:\n\n\x20\
    \x20\x20\x20\x20service\x20Foo\x20{\n\x20\x20\x20\x20\x20\x20\x20rpc\x20\
    Bar(google.protobuf.Empty)\x20returns\x20(google.protobuf.Empty);\n\x20\
    \x20\x20\x20\x20}\n\n\x20The\x20JSON\x20representation\x20for\x20`Empty`\
    \x20is\x20empty\x20JSON\x20object\x20`{}`.\n\n\n\n\x03\x04\0\x01\x12\x03\
    3\x08\rb\x06proto3\
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

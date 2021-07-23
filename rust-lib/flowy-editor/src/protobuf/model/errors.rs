// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `errors.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct EditorError {
    // message fields
    pub code: EditorErrorCode,
    pub msg: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EditorError {
    fn default() -> &'a EditorError {
        <EditorError as ::protobuf::Message>::default_instance()
    }
}

impl EditorError {
    pub fn new() -> EditorError {
        ::std::default::Default::default()
    }

    // .EditorErrorCode code = 1;


    pub fn get_code(&self) -> EditorErrorCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = EditorErrorCode::Unknown;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: EditorErrorCode) {
        self.code = v;
    }

    // string msg = 2;


    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }
}

impl ::protobuf::Message for EditorError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
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
        if self.code != EditorErrorCode::Unknown {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != EditorErrorCode::Unknown {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.code))?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EditorError {
        EditorError::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EditorErrorCode>>(
                "code",
                |m: &EditorError| { &m.code },
                |m: &mut EditorError| { &mut m.code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg",
                |m: &EditorError| { &m.msg },
                |m: &mut EditorError| { &mut m.msg },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<EditorError>(
                "EditorError",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static EditorError {
        static instance: ::protobuf::rt::LazyV2<EditorError> = ::protobuf::rt::LazyV2::INIT;
        instance.get(EditorError::new)
    }
}

impl ::protobuf::Clear for EditorError {
    fn clear(&mut self) {
        self.code = EditorErrorCode::Unknown;
        self.msg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EditorError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EditorError {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EditorErrorCode {
    Unknown = 0,
    EditorDBInternalError = 1,
    EditorDBConnFailed = 2,
    DocNameInvalid = 10,
    DocViewIdInvalid = 11,
    DocDescTooLong = 12,
    DocFileError = 13,
    EditorUserNotLoginYet = 100,
}

impl ::protobuf::ProtobufEnum for EditorErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EditorErrorCode> {
        match value {
            0 => ::std::option::Option::Some(EditorErrorCode::Unknown),
            1 => ::std::option::Option::Some(EditorErrorCode::EditorDBInternalError),
            2 => ::std::option::Option::Some(EditorErrorCode::EditorDBConnFailed),
            10 => ::std::option::Option::Some(EditorErrorCode::DocNameInvalid),
            11 => ::std::option::Option::Some(EditorErrorCode::DocViewIdInvalid),
            12 => ::std::option::Option::Some(EditorErrorCode::DocDescTooLong),
            13 => ::std::option::Option::Some(EditorErrorCode::DocFileError),
            100 => ::std::option::Option::Some(EditorErrorCode::EditorUserNotLoginYet),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EditorErrorCode] = &[
            EditorErrorCode::Unknown,
            EditorErrorCode::EditorDBInternalError,
            EditorErrorCode::EditorDBConnFailed,
            EditorErrorCode::DocNameInvalid,
            EditorErrorCode::DocViewIdInvalid,
            EditorErrorCode::DocDescTooLong,
            EditorErrorCode::DocFileError,
            EditorErrorCode::EditorUserNotLoginYet,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<EditorErrorCode>("EditorErrorCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for EditorErrorCode {
}

impl ::std::default::Default for EditorErrorCode {
    fn default() -> Self {
        EditorErrorCode::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for EditorErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cerrors.proto\"E\n\x0bEditorError\x12$\n\x04code\x18\x01\x20\x01(\
    \x0e2\x10.EditorErrorCodeR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\
    \x03msg*\xbc\x01\n\x0fEditorErrorCode\x12\x0b\n\x07Unknown\x10\0\x12\x19\
    \n\x15EditorDBInternalError\x10\x01\x12\x16\n\x12EditorDBConnFailed\x10\
    \x02\x12\x12\n\x0eDocNameInvalid\x10\n\x12\x14\n\x10DocViewIdInvalid\x10\
    \x0b\x12\x12\n\x0eDocDescTooLong\x10\x0c\x12\x10\n\x0cDocFileError\x10\r\
    \x12\x19\n\x15EditorUserNotLoginYet\x10dJ\xf8\x03\n\x06\x12\x04\0\0\x0f\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x03\x04\x1d\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x03\x04\x13\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x03\x14\x18\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x03\x1b\x1c\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x13\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x04\x0b\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x11\x12\n\
    \n\n\x02\x05\0\x12\x04\x06\0\x0f\x01\n\n\n\x03\x05\0\x01\x12\x03\x06\x05\
    \x14\n\x0b\n\x04\x05\0\x02\0\x12\x03\x07\x04\x10\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x07\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x07\x0e\x0f\
    \n\x0b\n\x04\x05\0\x02\x01\x12\x03\x08\x04\x1e\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x08\x04\x19\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x08\x1c\
    \x1d\n\x0b\n\x04\x05\0\x02\x02\x12\x03\t\x04\x1b\n\x0c\n\x05\x05\0\x02\
    \x02\x01\x12\x03\t\x04\x16\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\t\x19\
    \x1a\n\x0b\n\x04\x05\0\x02\x03\x12\x03\n\x04\x18\n\x0c\n\x05\x05\0\x02\
    \x03\x01\x12\x03\n\x04\x12\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\n\x15\
    \x17\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x0b\x04\x1a\n\x0c\n\x05\x05\0\x02\
    \x04\x01\x12\x03\x0b\x04\x14\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x0b\
    \x17\x19\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x0c\x04\x18\n\x0c\n\x05\x05\0\
    \x02\x05\x01\x12\x03\x0c\x04\x12\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\
    \x0c\x15\x17\n\x0b\n\x04\x05\0\x02\x06\x12\x03\r\x04\x16\n\x0c\n\x05\x05\
    \0\x02\x06\x01\x12\x03\r\x04\x10\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\r\
    \x13\x15\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x0e\x04\x20\n\x0c\n\x05\x05\0\
    \x02\x07\x01\x12\x03\x0e\x04\x19\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\
    \x0e\x1c\x1fb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}

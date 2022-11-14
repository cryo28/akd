// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.19.4
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `audit.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:NodeLabel)
pub struct NodeLabel {
    // message fields
    // @@protoc_insertion_point(field:NodeLabel.label_val)
    pub label_val: ::std::option::Option<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:NodeLabel.label_len)
    pub label_len: ::std::option::Option<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:NodeLabel.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NodeLabel {
    fn default() -> &'a NodeLabel {
        <NodeLabel as ::protobuf::Message>::default_instance()
    }
}

impl NodeLabel {
    pub fn new() -> NodeLabel {
        ::std::default::Default::default()
    }

    // optional bytes label_val = 1;

    pub fn label_val(&self) -> &[u8] {
        match self.label_val.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_label_val(&mut self) {
        self.label_val = ::std::option::Option::None;
    }

    pub fn has_label_val(&self) -> bool {
        self.label_val.is_some()
    }

    // Param is passed by value, moved
    pub fn set_label_val(&mut self, v: ::std::vec::Vec<u8>) {
        self.label_val = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label_val(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.label_val.is_none() {
            self.label_val = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.label_val.as_mut().unwrap()
    }

    // Take field
    pub fn take_label_val(&mut self) -> ::std::vec::Vec<u8> {
        self.label_val.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional uint32 label_len = 2;

    pub fn label_len(&self) -> u32 {
        self.label_len.unwrap_or(0)
    }

    pub fn clear_label_len(&mut self) {
        self.label_len = ::std::option::Option::None;
    }

    pub fn has_label_len(&self) -> bool {
        self.label_len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_label_len(&mut self, v: u32) {
        self.label_len = ::std::option::Option::Some(v);
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "label_val",
            |m: &NodeLabel| { &m.label_val },
            |m: &mut NodeLabel| { &mut m.label_val },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "label_len",
            |m: &NodeLabel| { &m.label_len },
            |m: &mut NodeLabel| { &mut m.label_len },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NodeLabel>(
            "NodeLabel",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NodeLabel {
    const NAME: &'static str = "NodeLabel";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.label_val = ::std::option::Option::Some(is.read_bytes()?);
                },
                16 => {
                    self.label_len = ::std::option::Option::Some(is.read_uint32()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.label_val.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.label_len {
            my_size += ::protobuf::rt::uint32_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.label_val.as_ref() {
            os.write_bytes(1, v)?;
        }
        if let Some(v) = self.label_len {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NodeLabel {
        NodeLabel::new()
    }

    fn clear(&mut self) {
        self.label_val = ::std::option::Option::None;
        self.label_len = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NodeLabel {
        static instance: NodeLabel = NodeLabel {
            label_val: ::std::option::Option::None,
            label_len: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NodeLabel {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NodeLabel").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NodeLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeLabel {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:Node)
pub struct Node {
    // message fields
    // @@protoc_insertion_point(field:Node.label)
    pub label: ::protobuf::MessageField<NodeLabel>,
    // @@protoc_insertion_point(field:Node.hash)
    pub hash: ::std::option::Option<::std::vec::Vec<u8>>,
    // special fields
    // @@protoc_insertion_point(special_field:Node.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Node {
    fn default() -> &'a Node {
        <Node as ::protobuf::Message>::default_instance()
    }
}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }

    // optional bytes hash = 2;

    pub fn hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => v,
            None => &[],
        }
    }

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::std::option::Option::Some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash = ::std::option::Option::Some(::std::vec::Vec::new());
        }
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, NodeLabel>(
            "label",
            |m: &Node| { &m.label },
            |m: &mut Node| { &mut m.label },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "hash",
            |m: &Node| { &m.hash },
            |m: &mut Node| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Node>(
            "Node",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Node {
    const NAME: &'static str = "Node";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.label)?;
                },
                18 => {
                    self.hash = ::std::option::Option::Some(is.read_bytes()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.label.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.label.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.hash.as_ref() {
            os.write_bytes(2, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Node {
        Node::new()
    }

    fn clear(&mut self) {
        self.label.clear();
        self.hash = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Node {
        static instance: Node = Node {
            label: ::protobuf::MessageField::none(),
            hash: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Node {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Node").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Node {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:SingleEncodedProof)
pub struct SingleEncodedProof {
    // message fields
    // @@protoc_insertion_point(field:SingleEncodedProof.inserted)
    pub inserted: ::std::vec::Vec<Node>,
    // @@protoc_insertion_point(field:SingleEncodedProof.unchanged)
    pub unchanged: ::std::vec::Vec<Node>,
    // special fields
    // @@protoc_insertion_point(special_field:SingleEncodedProof.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SingleEncodedProof {
    fn default() -> &'a SingleEncodedProof {
        <SingleEncodedProof as ::protobuf::Message>::default_instance()
    }
}

impl SingleEncodedProof {
    pub fn new() -> SingleEncodedProof {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "inserted",
            |m: &SingleEncodedProof| { &m.inserted },
            |m: &mut SingleEncodedProof| { &mut m.inserted },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unchanged",
            |m: &SingleEncodedProof| { &m.unchanged },
            |m: &mut SingleEncodedProof| { &mut m.unchanged },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SingleEncodedProof>(
            "SingleEncodedProof",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SingleEncodedProof {
    const NAME: &'static str = "SingleEncodedProof";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.inserted.push(is.read_message()?);
                },
                18 => {
                    self.unchanged.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.inserted {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.unchanged {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.inserted {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.unchanged {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SingleEncodedProof {
        SingleEncodedProof::new()
    }

    fn clear(&mut self) {
        self.inserted.clear();
        self.unchanged.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SingleEncodedProof {
        static instance: SingleEncodedProof = SingleEncodedProof {
            inserted: ::std::vec::Vec::new(),
            unchanged: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SingleEncodedProof {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SingleEncodedProof").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SingleEncodedProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SingleEncodedProof {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0baudit.proto\"E\n\tNodeLabel\x12\x1b\n\tlabel_val\x18\x01\x20\x01(\
    \x0cR\x08labelVal\x12\x1b\n\tlabel_len\x18\x02\x20\x01(\rR\x08labelLen\"\
    <\n\x04Node\x12\x20\n\x05label\x18\x01\x20\x01(\x0b2\n.NodeLabelR\x05lab\
    el\x12\x12\n\x04hash\x18\x02\x20\x01(\x0cR\x04hash\"\\\n\x12SingleEncode\
    dProof\x12!\n\x08inserted\x18\x01\x20\x03(\x0b2\x05.NodeR\x08inserted\
    \x12#\n\tunchanged\x18\x02\x20\x03(\x0b2\x05.NodeR\tunchanged\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(NodeLabel::generated_message_descriptor_data());
            messages.push(Node::generated_message_descriptor_data());
            messages.push(SingleEncodedProof::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
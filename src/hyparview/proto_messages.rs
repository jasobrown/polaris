// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[deriving(Clone,Default)]
pub struct Join {
    sender: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Join {
    pub fn new() -> Join {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Join {
        static mut instance: ::protobuf::lazy::Lazy<Join> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Join,
        };
        unsafe {
            instance.get(|| {
                Join {
                    sender: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.sender.is_none() {
            self.sender.set_default();
        };
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        self.sender.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender<'a>(&'a self) -> &'a str {
        match self.sender.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for Join {
    fn new() -> Join {
        Join::new()
    }

    fn is_initialized(&self) -> bool {
        if self.sender.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.sender.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.sender.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sender.as_ref() {
            try!(os.write_string(1, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Join>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sender",
                    Join::has_sender,
                    Join::get_sender,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Join>(
                    "Join",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Join>()
    }
}

impl ::protobuf::Clear for Join {
    fn clear(&mut self) {
        self.clear_sender();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Join {
    fn eq(&self, other: &Join) -> bool {
        self.sender == other.sender &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for Join {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_impl(f)
    }
}

#[deriving(Clone,Default)]
pub struct ForwardJoin {
    sender: ::protobuf::SingularField<::std::string::String>,
    ttl: ::std::option::Option<i32>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ForwardJoin {
    pub fn new() -> ForwardJoin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ForwardJoin {
        static mut instance: ::protobuf::lazy::Lazy<ForwardJoin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ForwardJoin,
        };
        unsafe {
            instance.get(|| {
                ForwardJoin {
                    sender: ::protobuf::SingularField::none(),
                    ttl: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.sender.is_none() {
            self.sender.set_default();
        };
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        self.sender.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender<'a>(&'a self) -> &'a str {
        match self.sender.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required int32 ttl = 2;

    pub fn clear_ttl(&mut self) {
        self.ttl = ::std::option::Option::None;
    }

    pub fn has_ttl(&self) -> bool {
        self.ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: i32) {
        self.ttl = ::std::option::Option::Some(v);
    }

    pub fn get_ttl<'a>(&self) -> i32 {
        self.ttl.unwrap_or(0)
    }
}

impl ::protobuf::Message for ForwardJoin {
    fn new() -> ForwardJoin {
        ForwardJoin::new()
    }

    fn is_initialized(&self) -> bool {
        if self.sender.is_none() {
            return false;
        };
        if self.ttl.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.sender.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.ttl = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.sender.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.ttl.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sender.as_ref() {
            try!(os.write_string(1, v.as_slice()));
        };
        if let Some(v) = self.ttl {
            try!(os.write_int32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ForwardJoin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sender",
                    ForwardJoin::has_sender,
                    ForwardJoin::get_sender,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "ttl",
                    ForwardJoin::has_ttl,
                    ForwardJoin::get_ttl,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ForwardJoin>(
                    "ForwardJoin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ForwardJoin>()
    }
}

impl ::protobuf::Clear for ForwardJoin {
    fn clear(&mut self) {
        self.clear_sender();
        self.clear_ttl();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ForwardJoin {
    fn eq(&self, other: &ForwardJoin) -> bool {
        self.sender == other.sender &&
        self.ttl == other.ttl &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for ForwardJoin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_impl(f)
    }
}

#[deriving(Clone,Default)]
pub struct Disconnect {
    sender: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Disconnect {
    pub fn new() -> Disconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Disconnect {
        static mut instance: ::protobuf::lazy::Lazy<Disconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Disconnect,
        };
        unsafe {
            instance.get(|| {
                Disconnect {
                    sender: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.sender.is_none() {
            self.sender.set_default();
        };
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        self.sender.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender<'a>(&'a self) -> &'a str {
        match self.sender.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for Disconnect {
    fn new() -> Disconnect {
        Disconnect::new()
    }

    fn is_initialized(&self) -> bool {
        if self.sender.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.sender.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.sender.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sender.as_ref() {
            try!(os.write_string(1, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Disconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sender",
                    Disconnect::has_sender,
                    Disconnect::get_sender,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Disconnect>(
                    "Disconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Disconnect>()
    }
}

impl ::protobuf::Clear for Disconnect {
    fn clear(&mut self) {
        self.clear_sender();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Disconnect {
    fn eq(&self, other: &Disconnect) -> bool {
        self.sender == other.sender &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for Disconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_impl(f)
    }
}

#[deriving(Clone,Default)]
pub struct Neighbor {
    sender: ::protobuf::SingularField<::std::string::String>,
    priority: ::std::option::Option<Neighbor_Priority>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Neighbor {
    pub fn new() -> Neighbor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Neighbor {
        static mut instance: ::protobuf::lazy::Lazy<Neighbor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Neighbor,
        };
        unsafe {
            instance.get(|| {
                Neighbor {
                    sender: ::protobuf::SingularField::none(),
                    priority: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.sender.is_none() {
            self.sender.set_default();
        };
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        self.sender.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender<'a>(&'a self) -> &'a str {
        match self.sender.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required .Neighbor.Priority priority = 2;

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: Neighbor_Priority) {
        self.priority = ::std::option::Option::Some(v);
    }

    pub fn get_priority<'a>(&self) -> Neighbor_Priority {
        self.priority.unwrap_or(Neighbor_Priority::high)
    }
}

impl ::protobuf::Message for Neighbor {
    fn new() -> Neighbor {
        Neighbor::new()
    }

    fn is_initialized(&self) -> bool {
        if self.sender.is_none() {
            return false;
        };
        if self.priority.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.sender.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.priority = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.sender.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.priority.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sender.as_ref() {
            try!(os.write_string(1, v.as_slice()));
        };
        if let Some(v) = self.priority {
            try!(os.write_enum(2, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Neighbor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sender",
                    Neighbor::has_sender,
                    Neighbor::get_sender,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "priority",
                    Neighbor::has_priority,
                    Neighbor::get_priority,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Neighbor>(
                    "Neighbor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Neighbor>()
    }
}

impl ::protobuf::Clear for Neighbor {
    fn clear(&mut self) {
        self.clear_sender();
        self.clear_priority();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Neighbor {
    fn eq(&self, other: &Neighbor) -> bool {
        self.sender == other.sender &&
        self.priority == other.priority &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for Neighbor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_impl(f)
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum Neighbor_Priority {
    high = 1,
    low = 2,
}

impl ::protobuf::ProtobufEnum for Neighbor_Priority {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Neighbor_Priority> {
        match value {
            1 => ::std::option::Option::Some(Neighbor_Priority::high),
            2 => ::std::option::Option::Some(Neighbor_Priority::low),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Neighbor_Priority>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Neighbor_Priority", file_descriptor_proto())
            })
        }
    }
}

impl ::std::kinds::Copy for Neighbor_Priority {
}

#[deriving(Clone,Default)]
pub struct NeighborReject {
    sender: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NeighborReject {
    pub fn new() -> NeighborReject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NeighborReject {
        static mut instance: ::protobuf::lazy::Lazy<NeighborReject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NeighborReject,
        };
        unsafe {
            instance.get(|| {
                NeighborReject {
                    sender: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string sender = 1;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    pub fn has_sender(&self) -> bool {
        self.sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.sender.is_none() {
            self.sender.set_default();
        };
        self.sender.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        self.sender.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender<'a>(&'a self) -> &'a str {
        match self.sender.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for NeighborReject {
    fn new() -> NeighborReject {
        NeighborReject::new()
    }

    fn is_initialized(&self) -> bool {
        if self.sender.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.sender.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.sender.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sender.as_ref() {
            try!(os.write_string(1, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<NeighborReject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "sender",
                    NeighborReject::has_sender,
                    NeighborReject::get_sender,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NeighborReject>(
                    "NeighborReject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<NeighborReject>()
    }
}

impl ::protobuf::Clear for NeighborReject {
    fn clear(&mut self) {
        self.clear_sender();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NeighborReject {
    fn eq(&self, other: &NeighborReject) -> bool {
        self.sender == other.sender &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Show for NeighborReject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.fmt_impl(f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x16, 0x0a, 0x04, 0x4a, 0x6f, 0x69, 0x6e, 0x12, 0x0e,
    0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x2a,
    0x0a, 0x0b, 0x46, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x4a, 0x6f, 0x69, 0x6e, 0x12, 0x0e, 0x0a,
    0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0b, 0x0a,
    0x03, 0x74, 0x74, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x22, 0x1c, 0x0a, 0x0a, 0x44, 0x69,
    0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x5f, 0x0a, 0x08, 0x4e, 0x65, 0x69, 0x67,
    0x68, 0x62, 0x6f, 0x72, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x24, 0x0a, 0x08, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x4e, 0x65, 0x69, 0x67, 0x68, 0x62, 0x6f,
    0x72, 0x2e, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x22, 0x1d, 0x0a, 0x08, 0x50, 0x72,
    0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12, 0x08, 0x0a, 0x04, 0x68, 0x69, 0x67, 0x68, 0x10, 0x01,
    0x12, 0x07, 0x0a, 0x03, 0x6c, 0x6f, 0x77, 0x10, 0x02, 0x22, 0x20, 0x0a, 0x0e, 0x4e, 0x65, 0x69,
    0x67, 0x68, 0x62, 0x6f, 0x72, 0x52, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x0e, 0x0a, 0x06, 0x73,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x4a, 0xf8, 0x07, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00,
    0x02, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x01, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x01, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x01, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x01, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x04, 0x00, 0x07, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x04, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x05, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x06, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x06, 0x19, 0x1a, 0x0a, 0x45, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0a, 0x00, 0x0c,
    0x01, 0x1a, 0x39, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x72, 0x65, 0x27, 0x73, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2c, 0x20, 0x74, 0x62, 0x68, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x0b, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x0e, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x0f,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03,
    0x04, 0x00, 0x12, 0x04, 0x11, 0x04, 0x14, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x09, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x12, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x12, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x12, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x13, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x13, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x16, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x21, 0x22, 0x0a, 0xe9, 0x01, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x1b, 0x00, 0x1d, 0x01, 0x1a, 0xdc, 0x01, 0x20, 0x77, 0x68, 0x69, 0x6c, 0x65, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x68, 0x79, 0x70, 0x61, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x70, 0x61, 0x70, 0x65,
    0x72, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x20, 0x61, 0x20, 0x70, 0x65, 0x65, 0x72,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x64, 0x64, 0x20, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x69,
    0x72, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x68,
    0x61, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x64, 0x65, 0x6e, 0x69, 0x65, 0x64, 0x2e, 0x20,
    0x6f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65,
    0x27, 0x73, 0x20, 0x6e, 0x6f, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x64, 0x69,
    0x63, 0x61, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x6c,
    0x6c, 0x65, 0x72, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x6f, 0x75, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1b, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x04, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1c, 0x1d, 0x1e,
];

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

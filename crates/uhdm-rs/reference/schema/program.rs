pub mod program {
  #[derive(Copy, Clone)]
  pub struct Owned(());
  impl ::capnp::introspect::Introspect for Owned { fn introspect() -> ::capnp::introspect::Type { ::capnp::introspect::TypeVariant::Struct(::capnp::introspect::RawBrandedStructSchema { generic: &_private::RAW_SCHEMA, field_types: _private::get_field_types, annotation_types: _private::get_annotation_types }).into() } }
  impl ::capnp::traits::Owned for Owned { type Reader<'a> = Reader<'a>; type Builder<'a> = Builder<'a>; }
  impl ::capnp::traits::OwnedStruct for Owned { type Reader<'a> = Reader<'a>; type Builder<'a> = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }
  impl <'a,> ::core::marker::Copy for Reader<'a,>  {}
  impl <'a,> ::core::clone::Clone for Reader<'a,>  {
    fn clone(&self) -> Self { *self }
  }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    const TYPE_ID: u64 = _private::TYPE_ID;
  }
  impl <'a,> ::core::convert::From<::capnp::private::layout::StructReader<'a>> for Reader<'a,>  {
    fn from(reader: ::capnp::private::layout::StructReader<'a>) -> Self {
      Self { reader,  }
    }
  }

  impl <'a,> ::core::convert::From<Reader<'a,>> for ::capnp::dynamic_value::Reader<'a>  {
    fn from(reader: Reader<'a,>) -> Self {
      Self::Struct(::capnp::dynamic_struct::Reader::new(reader.reader, ::capnp::schema::StructSchema::new(::capnp::introspect::RawBrandedStructSchema { generic: &_private::RAW_SCHEMA, field_types: _private::get_field_types::<>, annotation_types: _private::get_annotation_types::<>})))
    }
  }

  impl <'a,> ::core::fmt::Debug for Reader<'a,>  {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
      core::fmt::Debug::fmt(&::core::convert::Into::<::capnp::dynamic_value::Reader<'_>>::into(*self), f)
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Self> {
      ::core::result::Result::Ok(reader.get_struct(default)?.into())
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<'_,> {
      Self { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::instance::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_vpi_index(self) -> i64 {
      self.reader.get_data_field::<i64>(0)
    }
    #[inline]
    pub fn get_defaultclocking(self) -> u64 {
      self.reader.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn get_instancearray(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_instancearray(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_interfaces(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_interfaces(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_exprdist(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_exprdist(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_interfacearrays(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_interfacearrays(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_process(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_process(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_contassigns(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_contassigns(&self) -> bool {
      !self.reader.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_clockingblocks(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_clockingblocks(&self) -> bool {
      !self.reader.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_ports(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_ports(&self) -> bool {
      !self.reader.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_genscopearrays(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_genscopearrays(&self) -> bool {
      !self.reader.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_vpi_end_label(self) -> u64 {
      self.reader.get_data_field::<u64>(2)
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    const STRUCT_SIZE: ::capnp::private::layout::StructSize = ::capnp::private::layout::StructSize { data: 3, pointers: 10 };
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    const TYPE_ID: u64 = _private::TYPE_ID;
  }
  impl <'a,> ::core::convert::From<::capnp::private::layout::StructBuilder<'a>> for Builder<'a,>  {
    fn from(builder: ::capnp::private::layout::StructBuilder<'a>) -> Self {
      Self { builder,  }
    }
  }

  impl <'a,> ::core::convert::From<Builder<'a,>> for ::capnp::dynamic_value::Builder<'a>  {
    fn from(builder: Builder<'a,>) -> Self {
      Self::Struct(::capnp::dynamic_struct::Builder::new(builder.builder, ::capnp::schema::StructSchema::new(::capnp::introspect::RawBrandedStructSchema { generic: &_private::RAW_SCHEMA, field_types: _private::get_field_types::<>, annotation_types: _private::get_annotation_types::<>})))
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Self {
      builder.init_struct(<Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE).into()
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [::capnp::Word]>) -> ::capnp::Result<Self> {
      ::core::result::Result::Ok(builder.get_struct(<Self as ::capnp::traits::HasStructSize>::STRUCT_SIZE, default)?.into())
    }
  }

  impl <'a,> ::capnp::traits::SetterInput<Owned<>> for Reader<'a,>  {
    fn set_pointer_builder(mut pointer: ::capnp::private::layout::PointerBuilder<'_>, value: Self, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      self.builder.into_reader().into()
    }
    pub fn reborrow(&mut self) -> Builder<'_,> {
      Builder { builder: self.builder.reborrow() }
    }
    pub fn reborrow_as_reader(&self) -> Reader<'_,> {
      self.builder.as_reader().into()
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::instance::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_base(&mut self, value: crate::uhdm_capnp::instance::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_base(self, ) -> crate::uhdm_capnp::instance::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.builder.is_pointer_field_null(0)
    }
    #[inline]
    pub fn get_vpi_index(self) -> i64 {
      self.builder.get_data_field::<i64>(0)
    }
    #[inline]
    pub fn set_vpi_index(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(0, value);
    }
    #[inline]
    pub fn get_defaultclocking(self) -> u64 {
      self.builder.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn set_defaultclocking(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(1, value);
    }
    #[inline]
    pub fn get_instancearray(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_instancearray(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(1), value, false)
    }
    #[inline]
    pub fn init_instancearray(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), 0)
    }
    #[inline]
    pub fn has_instancearray(&self) -> bool {
      !self.builder.is_pointer_field_null(1)
    }
    #[inline]
    pub fn get_interfaces(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_interfaces(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(2), value, false)
    }
    #[inline]
    pub fn init_interfaces(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(2), size)
    }
    #[inline]
    pub fn has_interfaces(&self) -> bool {
      !self.builder.is_pointer_field_null(2)
    }
    #[inline]
    pub fn get_exprdist(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_exprdist(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(3), value, false)
    }
    #[inline]
    pub fn init_exprdist(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(3), 0)
    }
    #[inline]
    pub fn has_exprdist(&self) -> bool {
      !self.builder.is_pointer_field_null(3)
    }
    #[inline]
    pub fn get_interfacearrays(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_interfacearrays(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(4), value, false)
    }
    #[inline]
    pub fn init_interfacearrays(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(4), size)
    }
    #[inline]
    pub fn has_interfacearrays(&self) -> bool {
      !self.builder.is_pointer_field_null(4)
    }
    #[inline]
    pub fn get_process(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_process(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(5), value, false)
    }
    #[inline]
    pub fn init_process(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(5), size)
    }
    #[inline]
    pub fn has_process(&self) -> bool {
      !self.builder.is_pointer_field_null(5)
    }
    #[inline]
    pub fn get_contassigns(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_contassigns(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(6), value, false)
    }
    #[inline]
    pub fn init_contassigns(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(6), size)
    }
    #[inline]
    pub fn has_contassigns(&self) -> bool {
      !self.builder.is_pointer_field_null(6)
    }
    #[inline]
    pub fn get_clockingblocks(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_clockingblocks(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(7), value, false)
    }
    #[inline]
    pub fn init_clockingblocks(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(7), size)
    }
    #[inline]
    pub fn has_clockingblocks(&self) -> bool {
      !self.builder.is_pointer_field_null(7)
    }
    #[inline]
    pub fn get_ports(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_ports(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(8), value, false)
    }
    #[inline]
    pub fn init_ports(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(8), size)
    }
    #[inline]
    pub fn has_ports(&self) -> bool {
      !self.builder.is_pointer_field_null(8)
    }
    #[inline]
    pub fn get_genscopearrays(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_genscopearrays(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(9), value, false)
    }
    #[inline]
    pub fn init_genscopearrays(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(9), size)
    }
    #[inline]
    pub fn has_genscopearrays(&self) -> bool {
      !self.builder.is_pointer_field_null(9)
    }
    #[inline]
    pub fn get_vpi_end_label(self) -> u64 {
      self.builder.get_data_field::<u64>(2)
    }
    #[inline]
    pub fn set_vpi_end_label(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(2, value);
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
      Self { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_base(&self) -> crate::uhdm_capnp::instance::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
    pub fn get_instancearray(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(1))
    }
    pub fn get_exprdist(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(3))
    }
  }
  mod _private {
    pub static ENCODED_NODE: [::capnp::Word; 252] = [
      ::capnp::word(0, 0, 0, 0, 6, 0, 6, 0),
      ::capnp::word(127, 153, 225, 135, 39, 139, 160, 184),
      ::capnp::word(18, 0, 0, 0, 1, 0, 3, 0),
      ::capnp::word(119, 104, 85, 41, 145, 41, 247, 255),
      ::capnp::word(10, 0, 7, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(200, 140, 0, 0, 112, 142, 0, 0),
      ::capnp::word(21, 0, 0, 0, 210, 0, 0, 0),
      ::capnp::word(33, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(29, 0, 0, 0, 223, 2, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(115, 99, 104, 101, 109, 97, 47, 117),
      ::capnp::word(104, 100, 109, 46, 99, 97, 112, 110),
      ::capnp::word(112, 58, 80, 114, 111, 103, 114, 97),
      ::capnp::word(109, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 1, 0, 1, 0),
      ::capnp::word(52, 0, 0, 0, 3, 0, 4, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(93, 1, 0, 0, 42, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(88, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(100, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(97, 1, 0, 0, 74, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(96, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(108, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(2, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(105, 1, 0, 0, 130, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(104, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(116, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(3, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(113, 1, 0, 0, 114, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(112, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(124, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(4, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(121, 1, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(120, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(148, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(5, 0, 0, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(145, 1, 0, 0, 74, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(144, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(156, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(6, 0, 0, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(153, 1, 0, 0, 130, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(152, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(180, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(7, 0, 0, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(177, 1, 0, 0, 66, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(172, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(200, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(8, 0, 0, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(197, 1, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(196, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(224, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(221, 1, 0, 0, 122, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(220, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(248, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(10, 0, 0, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 10, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(245, 1, 0, 0, 50, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(240, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(12, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(11, 0, 0, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 11, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(9, 2, 0, 0, 122, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(8, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(36, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(12, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 12, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(33, 2, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(32, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(44, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(98, 97, 115, 101, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(135, 192, 67, 67, 26, 131, 116, 238),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 73, 110, 100, 101, 120),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(100, 101, 102, 97, 117, 108, 116, 99),
      ::capnp::word(108, 111, 99, 107, 105, 110, 103, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(105, 110, 115, 116, 97, 110, 99, 101),
      ::capnp::word(97, 114, 114, 97, 121, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(105, 110, 116, 101, 114, 102, 97, 99),
      ::capnp::word(101, 115, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(101, 120, 112, 114, 100, 105, 115, 116),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(105, 110, 116, 101, 114, 102, 97, 99),
      ::capnp::word(101, 97, 114, 114, 97, 121, 115, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(112, 114, 111, 99, 101, 115, 115, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(99, 111, 110, 116, 97, 115, 115, 105),
      ::capnp::word(103, 110, 115, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(99, 108, 111, 99, 107, 105, 110, 103),
      ::capnp::word(98, 108, 111, 99, 107, 115, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(112, 111, 114, 116, 115, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(103, 101, 110, 115, 99, 111, 112, 101),
      ::capnp::word(97, 114, 114, 97, 121, 115, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 69, 110, 100, 76, 97),
      ::capnp::word(98, 101, 108, 0, 0, 0, 0, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
    ];
    pub fn get_field_types(index: u16) -> ::capnp::introspect::Type {
      match index {
        0 => <crate::uhdm_capnp::instance::Owned as ::capnp::introspect::Introspect>::introspect(),
        1 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        2 => <u64 as ::capnp::introspect::Introspect>::introspect(),
        3 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
        4 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        5 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
        6 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        7 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        8 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        9 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        10 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        11 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        12 => <u64 as ::capnp::introspect::Introspect>::introspect(),
        _ => panic!("invalid field index {}", index),
      }
    }
    pub fn get_annotation_types(child_index: Option<u16>, index: u32) -> ::capnp::introspect::Type {
      panic!("invalid annotation indices ({:?}, {}) ", child_index, index)
    }
    pub static RAW_SCHEMA: ::capnp::introspect::RawStructSchema = ::capnp::introspect::RawStructSchema {
      encoded_node: &ENCODED_NODE,
      nonunion_members: NONUNION_MEMBERS,
      members_by_discriminant: MEMBERS_BY_DISCRIMINANT,
      members_by_name: MEMBERS_BY_NAME,
    };
    pub static NONUNION_MEMBERS : &[u16] = &[0,1,2,3,4,5,6,7,8,9,10,11,12];
    pub static MEMBERS_BY_DISCRIMINANT : &[u16] = &[];
    pub static MEMBERS_BY_NAME : &[u16] = &[0,9,8,2,5,11,3,6,4,10,7,12,1];
    pub const TYPE_ID: u64 = 0xb8a0_8b27_87e1_997f;
  }
}


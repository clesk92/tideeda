pub mod forstmt {
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
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::scope::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_vpi_local_var_decls(self) -> i64 {
      self.reader.get_data_field::<i64>(0)
    }
    #[inline]
    pub fn get_vpi_for_init_stmts(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_vpi_for_init_stmts(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_vpi_for_inc_stmts(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_vpi_for_inc_stmts(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_vpi_for_init_stmt(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_vpi_for_init_stmt(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_vpi_condition(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_vpi_condition(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_vpi_for_inc_stmt(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_vpi_for_inc_stmt(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_vpi_stmt(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_vpi_stmt(&self) -> bool {
      !self.reader.get_pointer_field(6).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    const STRUCT_SIZE: ::capnp::private::layout::StructSize = ::capnp::private::layout::StructSize { data: 1, pointers: 7 };
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
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::scope::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_base(&mut self, value: crate::uhdm_capnp::scope::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_base(self, ) -> crate::uhdm_capnp::scope::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.builder.is_pointer_field_null(0)
    }
    #[inline]
    pub fn get_vpi_local_var_decls(self) -> i64 {
      self.builder.get_data_field::<i64>(0)
    }
    #[inline]
    pub fn set_vpi_local_var_decls(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(0, value);
    }
    #[inline]
    pub fn get_vpi_for_init_stmts(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vpi_for_init_stmts(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(1), value, false)
    }
    #[inline]
    pub fn init_vpi_for_init_stmts(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), size)
    }
    #[inline]
    pub fn has_vpi_for_init_stmts(&self) -> bool {
      !self.builder.is_pointer_field_null(1)
    }
    #[inline]
    pub fn get_vpi_for_inc_stmts(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vpi_for_inc_stmts(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(2), value, false)
    }
    #[inline]
    pub fn init_vpi_for_inc_stmts(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(2), size)
    }
    #[inline]
    pub fn has_vpi_for_inc_stmts(&self) -> bool {
      !self.builder.is_pointer_field_null(2)
    }
    #[inline]
    pub fn get_vpi_for_init_stmt(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vpi_for_init_stmt(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(3), value, false)
    }
    #[inline]
    pub fn init_vpi_for_init_stmt(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(3), 0)
    }
    #[inline]
    pub fn has_vpi_for_init_stmt(&self) -> bool {
      !self.builder.is_pointer_field_null(3)
    }
    #[inline]
    pub fn get_vpi_condition(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vpi_condition(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(4), value, false)
    }
    #[inline]
    pub fn init_vpi_condition(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(4), 0)
    }
    #[inline]
    pub fn has_vpi_condition(&self) -> bool {
      !self.builder.is_pointer_field_null(4)
    }
    #[inline]
    pub fn get_vpi_for_inc_stmt(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vpi_for_inc_stmt(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(5), value, false)
    }
    #[inline]
    pub fn init_vpi_for_inc_stmt(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(5), 0)
    }
    #[inline]
    pub fn has_vpi_for_inc_stmt(&self) -> bool {
      !self.builder.is_pointer_field_null(5)
    }
    #[inline]
    pub fn get_vpi_stmt(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vpi_stmt(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(6), value, false)
    }
    #[inline]
    pub fn init_vpi_stmt(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(6), 0)
    }
    #[inline]
    pub fn has_vpi_stmt(&self) -> bool {
      !self.builder.is_pointer_field_null(6)
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
      Self { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_base(&self) -> crate::uhdm_capnp::scope::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
    pub fn get_vpi_for_init_stmt(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(3))
    }
    pub fn get_vpi_condition(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(4))
    }
    pub fn get_vpi_for_inc_stmt(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(5))
    }
    pub fn get_vpi_stmt(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(6))
    }
  }
  mod _private {
    pub static ENCODED_NODE: [::capnp::Word; 154] = [
      ::capnp::word(0, 0, 0, 0, 6, 0, 6, 0),
      ::capnp::word(83, 156, 101, 205, 207, 87, 11, 222),
      ::capnp::word(18, 0, 0, 0, 1, 0, 1, 0),
      ::capnp::word(119, 104, 85, 41, 145, 41, 247, 255),
      ::capnp::word(7, 0, 7, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(116, 47, 0, 0, 146, 48, 0, 0),
      ::capnp::word(21, 0, 0, 0, 210, 0, 0, 0),
      ::capnp::word(33, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(29, 0, 0, 0, 199, 1, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(115, 99, 104, 101, 109, 97, 47, 117),
      ::capnp::word(104, 100, 109, 46, 99, 97, 112, 110),
      ::capnp::word(112, 58, 70, 111, 114, 115, 116, 109),
      ::capnp::word(116, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 1, 0, 1, 0),
      ::capnp::word(32, 0, 0, 0, 3, 0, 4, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(209, 0, 0, 0, 42, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(204, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(216, 0, 0, 0, 2, 0, 1, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(213, 0, 0, 0, 138, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(216, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(228, 0, 0, 0, 2, 0, 1, 0),
      ::capnp::word(2, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(225, 0, 0, 0, 130, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(224, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(252, 0, 0, 0, 2, 0, 1, 0),
      ::capnp::word(3, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(249, 0, 0, 0, 122, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(248, 0, 0, 0, 3, 0, 1, 0),
      ::capnp::word(20, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(4, 0, 0, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(17, 1, 0, 0, 122, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(28, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(5, 0, 0, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(25, 1, 0, 0, 106, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(24, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(36, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(6, 0, 0, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(33, 1, 0, 0, 114, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(32, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(44, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(7, 0, 0, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(41, 1, 0, 0, 66, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(36, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(48, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(98, 97, 115, 101, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(211, 210, 12, 49, 234, 138, 157, 128),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 76, 111, 99, 97, 108),
      ::capnp::word(86, 97, 114, 68, 101, 99, 108, 115),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 70, 111, 114, 73, 110),
      ::capnp::word(105, 116, 83, 116, 109, 116, 115, 0),
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
      ::capnp::word(118, 112, 105, 70, 111, 114, 73, 110),
      ::capnp::word(99, 83, 116, 109, 116, 115, 0, 0),
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
      ::capnp::word(118, 112, 105, 70, 111, 114, 73, 110),
      ::capnp::word(105, 116, 83, 116, 109, 116, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 67, 111, 110, 100, 105),
      ::capnp::word(116, 105, 111, 110, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 70, 111, 114, 73, 110),
      ::capnp::word(99, 83, 116, 109, 116, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 83, 116, 109, 116, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
    ];
    pub fn get_field_types(index: u16) -> ::capnp::introspect::Type {
      match index {
        0 => <crate::uhdm_capnp::scope::Owned as ::capnp::introspect::Introspect>::introspect(),
        1 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        2 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        3 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        4 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
        5 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
        6 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
        7 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
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
    pub static NONUNION_MEMBERS : &[u16] = &[0,1,2,3,4,5,6,7];
    pub static MEMBERS_BY_DISCRIMINANT : &[u16] = &[];
    pub static MEMBERS_BY_NAME : &[u16] = &[0,5,6,3,4,2,1,7];
    pub const TYPE_ID: u64 = 0xde0b_57cf_cd65_9c53;
  }
}


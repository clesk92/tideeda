pub mod design {
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
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::any::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_vpi_elaborated(self) -> bool {
      self.reader.get_bool_field(0)
    }
    #[inline]
    pub fn get_vpi_name(self) -> u64 {
      self.reader.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn get_includefileinfos(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_includefileinfos(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_all_packages(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_all_packages(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_top_packages(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_top_packages(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_all_classes(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_all_classes(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_all_interfaces(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_all_interfaces(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_all_udps(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_all_udps(&self) -> bool {
      !self.reader.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_all_programs(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_all_programs(&self) -> bool {
      !self.reader.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_all_modules(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_all_modules(&self) -> bool {
      !self.reader.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_typespecs(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_typespecs(&self) -> bool {
      !self.reader.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_letdecls(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(10), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_letdecls(&self) -> bool {
      !self.reader.get_pointer_field(10).is_null()
    }
    #[inline]
    pub fn get_taskfuncs(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(11), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_taskfuncs(&self) -> bool {
      !self.reader.get_pointer_field(11).is_null()
    }
    #[inline]
    pub fn get_parameters(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(12), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_parameters(&self) -> bool {
      !self.reader.get_pointer_field(12).is_null()
    }
    #[inline]
    pub fn get_paramassigns(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(13), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_paramassigns(&self) -> bool {
      !self.reader.get_pointer_field(13).is_null()
    }
    #[inline]
    pub fn get_top_modules(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(14), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_top_modules(&self) -> bool {
      !self.reader.get_pointer_field(14).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    const STRUCT_SIZE: ::capnp::private::layout::StructSize = ::capnp::private::layout::StructSize { data: 2, pointers: 15 };
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
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::any::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_base(&mut self, value: crate::uhdm_capnp::any::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_base(self, ) -> crate::uhdm_capnp::any::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.builder.is_pointer_field_null(0)
    }
    #[inline]
    pub fn get_vpi_elaborated(self) -> bool {
      self.builder.get_bool_field(0)
    }
    #[inline]
    pub fn set_vpi_elaborated(&mut self, value: bool)  {
      self.builder.set_bool_field(0, value);
    }
    #[inline]
    pub fn get_vpi_name(self) -> u64 {
      self.builder.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn set_vpi_name(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(1, value);
    }
    #[inline]
    pub fn get_includefileinfos(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_includefileinfos(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(1), value, false)
    }
    #[inline]
    pub fn init_includefileinfos(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), size)
    }
    #[inline]
    pub fn has_includefileinfos(&self) -> bool {
      !self.builder.is_pointer_field_null(1)
    }
    #[inline]
    pub fn get_all_packages(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_all_packages(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(2), value, false)
    }
    #[inline]
    pub fn init_all_packages(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(2), size)
    }
    #[inline]
    pub fn has_all_packages(&self) -> bool {
      !self.builder.is_pointer_field_null(2)
    }
    #[inline]
    pub fn get_top_packages(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_top_packages(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(3), value, false)
    }
    #[inline]
    pub fn init_top_packages(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(3), size)
    }
    #[inline]
    pub fn has_top_packages(&self) -> bool {
      !self.builder.is_pointer_field_null(3)
    }
    #[inline]
    pub fn get_all_classes(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_all_classes(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(4), value, false)
    }
    #[inline]
    pub fn init_all_classes(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(4), size)
    }
    #[inline]
    pub fn has_all_classes(&self) -> bool {
      !self.builder.is_pointer_field_null(4)
    }
    #[inline]
    pub fn get_all_interfaces(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_all_interfaces(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(5), value, false)
    }
    #[inline]
    pub fn init_all_interfaces(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(5), size)
    }
    #[inline]
    pub fn has_all_interfaces(&self) -> bool {
      !self.builder.is_pointer_field_null(5)
    }
    #[inline]
    pub fn get_all_udps(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_all_udps(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(6), value, false)
    }
    #[inline]
    pub fn init_all_udps(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(6), size)
    }
    #[inline]
    pub fn has_all_udps(&self) -> bool {
      !self.builder.is_pointer_field_null(6)
    }
    #[inline]
    pub fn get_all_programs(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_all_programs(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(7), value, false)
    }
    #[inline]
    pub fn init_all_programs(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(7), size)
    }
    #[inline]
    pub fn has_all_programs(&self) -> bool {
      !self.builder.is_pointer_field_null(7)
    }
    #[inline]
    pub fn get_all_modules(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_all_modules(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(8), value, false)
    }
    #[inline]
    pub fn init_all_modules(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(8), size)
    }
    #[inline]
    pub fn has_all_modules(&self) -> bool {
      !self.builder.is_pointer_field_null(8)
    }
    #[inline]
    pub fn get_typespecs(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_typespecs(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(9), value, false)
    }
    #[inline]
    pub fn init_typespecs(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(9), size)
    }
    #[inline]
    pub fn has_typespecs(&self) -> bool {
      !self.builder.is_pointer_field_null(9)
    }
    #[inline]
    pub fn get_letdecls(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(10), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_letdecls(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(10), value, false)
    }
    #[inline]
    pub fn init_letdecls(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(10), size)
    }
    #[inline]
    pub fn has_letdecls(&self) -> bool {
      !self.builder.is_pointer_field_null(10)
    }
    #[inline]
    pub fn get_taskfuncs(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(11), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_taskfuncs(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(11), value, false)
    }
    #[inline]
    pub fn init_taskfuncs(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(11), size)
    }
    #[inline]
    pub fn has_taskfuncs(&self) -> bool {
      !self.builder.is_pointer_field_null(11)
    }
    #[inline]
    pub fn get_parameters(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(12), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_parameters(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(12), value, false)
    }
    #[inline]
    pub fn init_parameters(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(12), size)
    }
    #[inline]
    pub fn has_parameters(&self) -> bool {
      !self.builder.is_pointer_field_null(12)
    }
    #[inline]
    pub fn get_paramassigns(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(13), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_paramassigns(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(13), value, false)
    }
    #[inline]
    pub fn init_paramassigns(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(13), size)
    }
    #[inline]
    pub fn has_paramassigns(&self) -> bool {
      !self.builder.is_pointer_field_null(13)
    }
    #[inline]
    pub fn get_top_modules(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(14), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_top_modules(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(14), value, false)
    }
    #[inline]
    pub fn init_top_modules(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(14), size)
    }
    #[inline]
    pub fn has_top_modules(&self) -> bool {
      !self.builder.is_pointer_field_null(14)
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
      Self { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_base(&self) -> crate::uhdm_capnp::any::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
  }
  mod _private {
    pub static ENCODED_NODE: [::capnp::Word; 345] = [
      ::capnp::word(0, 0, 0, 0, 6, 0, 6, 0),
      ::capnp::word(4, 3, 9, 82, 251, 142, 186, 188),
      ::capnp::word(18, 0, 0, 0, 1, 0, 2, 0),
      ::capnp::word(119, 104, 85, 41, 145, 41, 247, 255),
      ::capnp::word(15, 0, 7, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(85, 172, 0, 0, 136, 174, 0, 0),
      ::capnp::word(21, 0, 0, 0, 202, 0, 0, 0),
      ::capnp::word(33, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(29, 0, 0, 0, 191, 3, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(115, 99, 104, 101, 109, 97, 47, 117),
      ::capnp::word(104, 100, 109, 46, 99, 97, 112, 110),
      ::capnp::word(112, 58, 68, 101, 115, 105, 103, 110),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 1, 0, 1, 0),
      ::capnp::word(68, 0, 0, 0, 3, 0, 4, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(205, 1, 0, 0, 42, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(200, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(212, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(209, 1, 0, 0, 114, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(208, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(220, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(2, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(217, 1, 0, 0, 66, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(212, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(224, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(3, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(221, 1, 0, 0, 138, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(224, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(252, 1, 0, 0, 2, 0, 1, 0),
      ::capnp::word(4, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(249, 1, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(248, 1, 0, 0, 3, 0, 1, 0),
      ::capnp::word(20, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(5, 0, 0, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(17, 2, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(44, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(6, 0, 0, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(41, 2, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(40, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(68, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(7, 0, 0, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(65, 2, 0, 0, 114, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(64, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(92, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(8, 0, 0, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(89, 2, 0, 0, 66, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(84, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(112, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(109, 2, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(108, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(136, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(10, 0, 0, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 10, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(133, 2, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(132, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(160, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(11, 0, 0, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 11, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(157, 2, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(156, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(184, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(12, 0, 0, 0, 10, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 12, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(181, 2, 0, 0, 74, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(180, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(208, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(13, 0, 0, 0, 11, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 13, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(205, 2, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(204, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(232, 2, 0, 0, 2, 0, 1, 0),
      ::capnp::word(14, 0, 0, 0, 12, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 14, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(229, 2, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(228, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(0, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(15, 0, 0, 0, 13, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 15, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(253, 2, 0, 0, 106, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(252, 2, 0, 0, 3, 0, 1, 0),
      ::capnp::word(24, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(16, 0, 0, 0, 14, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 16, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(21, 3, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(20, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(48, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(98, 97, 115, 101, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(174, 247, 179, 5, 226, 209, 252, 175),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 69, 108, 97, 98, 111),
      ::capnp::word(114, 97, 116, 101, 100, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 78, 97, 109, 101, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(105, 110, 99, 108, 117, 100, 101, 102),
      ::capnp::word(105, 108, 101, 105, 110, 102, 111, 115),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(97, 108, 108, 80, 97, 99, 107, 97),
      ::capnp::word(103, 101, 115, 0, 0, 0, 0, 0),
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
      ::capnp::word(116, 111, 112, 80, 97, 99, 107, 97),
      ::capnp::word(103, 101, 115, 0, 0, 0, 0, 0),
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
      ::capnp::word(97, 108, 108, 67, 108, 97, 115, 115),
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
      ::capnp::word(97, 108, 108, 73, 110, 116, 101, 114),
      ::capnp::word(102, 97, 99, 101, 115, 0, 0, 0),
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
      ::capnp::word(97, 108, 108, 85, 100, 112, 115, 0),
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
      ::capnp::word(97, 108, 108, 80, 114, 111, 103, 114),
      ::capnp::word(97, 109, 115, 0, 0, 0, 0, 0),
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
      ::capnp::word(97, 108, 108, 77, 111, 100, 117, 108),
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
      ::capnp::word(116, 121, 112, 101, 115, 112, 101, 99),
      ::capnp::word(115, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(108, 101, 116, 100, 101, 99, 108, 115),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(116, 97, 115, 107, 102, 117, 110, 99),
      ::capnp::word(115, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(112, 97, 114, 97, 109, 101, 116, 101),
      ::capnp::word(114, 115, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(112, 97, 114, 97, 109, 97, 115, 115),
      ::capnp::word(105, 103, 110, 115, 0, 0, 0, 0),
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
      ::capnp::word(116, 111, 112, 77, 111, 100, 117, 108),
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
    ];
    pub fn get_field_types(index: u16) -> ::capnp::introspect::Type {
      match index {
        0 => <crate::uhdm_capnp::any::Owned as ::capnp::introspect::Introspect>::introspect(),
        1 => <bool as ::capnp::introspect::Introspect>::introspect(),
        2 => <u64 as ::capnp::introspect::Introspect>::introspect(),
        3 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        4 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        5 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        6 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        7 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        8 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        9 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        10 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        11 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        12 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        13 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        14 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        15 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        16 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
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
    pub static NONUNION_MEMBERS : &[u16] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    pub static MEMBERS_BY_DISCRIMINANT : &[u16] = &[];
    pub static MEMBERS_BY_NAME : &[u16] = &[6,7,10,4,9,8,0,3,12,15,14,13,16,5,11,1,2];
    pub const TYPE_ID: u64 = 0xbcba_8efb_5209_0304;
  }
}

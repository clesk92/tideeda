pub mod nets {
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
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::simpleexpr::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_vpi_array_member(self) -> bool {
      self.reader.get_bool_field(0)
    }
    #[inline]
    pub fn get_vpi_constant_select(self) -> bool {
      self.reader.get_bool_field(1)
    }
    #[inline]
    pub fn get_vpi_expanded(self) -> bool {
      self.reader.get_bool_field(2)
    }
    #[inline]
    pub fn get_vpi_implicit_decl(self) -> bool {
      self.reader.get_bool_field(3)
    }
    #[inline]
    pub fn get_vpi_name(self) -> u64 {
      self.reader.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn get_vpi_full_name(self) -> u64 {
      self.reader.get_data_field::<u64>(2)
    }
    #[inline]
    pub fn get_vpi_net_decl_assign(self) -> bool {
      self.reader.get_bool_field(4)
    }
    #[inline]
    pub fn get_vpi_net_type(self) -> i64 {
      self.reader.get_data_field::<i64>(3)
    }
    #[inline]
    pub fn get_vpi_resolved_net_type(self) -> i64 {
      self.reader.get_data_field::<i64>(4)
    }
    #[inline]
    pub fn get_vpi_scalar(self) -> bool {
      self.reader.get_bool_field(5)
    }
    #[inline]
    pub fn get_vpi_explicit_scalared(self) -> bool {
      self.reader.get_bool_field(6)
    }
    #[inline]
    pub fn get_vpi_signed(self) -> bool {
      self.reader.get_bool_field(7)
    }
    #[inline]
    pub fn get_vpi_strength0(self) -> i64 {
      self.reader.get_data_field::<i64>(5)
    }
    #[inline]
    pub fn get_vpi_strength1(self) -> i64 {
      self.reader.get_data_field::<i64>(6)
    }
    #[inline]
    pub fn get_vpi_charge_strength(self) -> i64 {
      self.reader.get_data_field::<i64>(7)
    }
    #[inline]
    pub fn get_vpi_vector(self) -> bool {
      self.reader.get_bool_field(8)
    }
    #[inline]
    pub fn get_vpi_explicit_vectored(self) -> bool {
      self.reader.get_bool_field(9)
    }
    #[inline]
    pub fn get_vpi_struct_union_member(self) -> bool {
      self.reader.get_bool_field(10)
    }
    #[inline]
    pub fn get_ports(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_ports(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_drivers(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_drivers(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_loads(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_loads(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
    #[inline]
    pub fn get_localdrivers(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_localdrivers(&self) -> bool {
      !self.reader.get_pointer_field(4).is_null()
    }
    #[inline]
    pub fn get_localloads(self) -> ::capnp::Result<::capnp::struct_list::Reader<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_localloads(&self) -> bool {
      !self.reader.get_pointer_field(5).is_null()
    }
    #[inline]
    pub fn get_primterms(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_primterms(&self) -> bool {
      !self.reader.get_pointer_field(6).is_null()
    }
    #[inline]
    pub fn get_contassigns(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_contassigns(&self) -> bool {
      !self.reader.get_pointer_field(7).is_null()
    }
    #[inline]
    pub fn get_pathterms(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_pathterms(&self) -> bool {
      !self.reader.get_pointer_field(8).is_null()
    }
    #[inline]
    pub fn get_tchkterms(self) -> ::capnp::Result<::capnp::primitive_list::Reader<'a,u64>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_tchkterms(&self) -> bool {
      !self.reader.get_pointer_field(9).is_null()
    }
    #[inline]
    pub fn get_simnet(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(10), ::core::option::Option::None)
    }
    #[inline]
    pub fn has_simnet(&self) -> bool {
      !self.reader.get_pointer_field(10).is_null()
    }
    #[inline]
    pub fn get_moduleinst(self) -> u64 {
      self.reader.get_data_field::<u64>(8)
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    const STRUCT_SIZE: ::capnp::private::layout::StructSize = ::capnp::private::layout::StructSize { data: 9, pointers: 11 };
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
    pub fn get_base(self) -> ::capnp::Result<crate::uhdm_capnp::simpleexpr::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_base(&mut self, value: crate::uhdm_capnp::simpleexpr::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(0), value, false)
    }
    #[inline]
    pub fn init_base(self, ) -> crate::uhdm_capnp::simpleexpr::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(0), 0)
    }
    #[inline]
    pub fn has_base(&self) -> bool {
      !self.builder.is_pointer_field_null(0)
    }
    #[inline]
    pub fn get_vpi_array_member(self) -> bool {
      self.builder.get_bool_field(0)
    }
    #[inline]
    pub fn set_vpi_array_member(&mut self, value: bool)  {
      self.builder.set_bool_field(0, value);
    }
    #[inline]
    pub fn get_vpi_constant_select(self) -> bool {
      self.builder.get_bool_field(1)
    }
    #[inline]
    pub fn set_vpi_constant_select(&mut self, value: bool)  {
      self.builder.set_bool_field(1, value);
    }
    #[inline]
    pub fn get_vpi_expanded(self) -> bool {
      self.builder.get_bool_field(2)
    }
    #[inline]
    pub fn set_vpi_expanded(&mut self, value: bool)  {
      self.builder.set_bool_field(2, value);
    }
    #[inline]
    pub fn get_vpi_implicit_decl(self) -> bool {
      self.builder.get_bool_field(3)
    }
    #[inline]
    pub fn set_vpi_implicit_decl(&mut self, value: bool)  {
      self.builder.set_bool_field(3, value);
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
    pub fn get_vpi_full_name(self) -> u64 {
      self.builder.get_data_field::<u64>(2)
    }
    #[inline]
    pub fn set_vpi_full_name(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(2, value);
    }
    #[inline]
    pub fn get_vpi_net_decl_assign(self) -> bool {
      self.builder.get_bool_field(4)
    }
    #[inline]
    pub fn set_vpi_net_decl_assign(&mut self, value: bool)  {
      self.builder.set_bool_field(4, value);
    }
    #[inline]
    pub fn get_vpi_net_type(self) -> i64 {
      self.builder.get_data_field::<i64>(3)
    }
    #[inline]
    pub fn set_vpi_net_type(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(3, value);
    }
    #[inline]
    pub fn get_vpi_resolved_net_type(self) -> i64 {
      self.builder.get_data_field::<i64>(4)
    }
    #[inline]
    pub fn set_vpi_resolved_net_type(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(4, value);
    }
    #[inline]
    pub fn get_vpi_scalar(self) -> bool {
      self.builder.get_bool_field(5)
    }
    #[inline]
    pub fn set_vpi_scalar(&mut self, value: bool)  {
      self.builder.set_bool_field(5, value);
    }
    #[inline]
    pub fn get_vpi_explicit_scalared(self) -> bool {
      self.builder.get_bool_field(6)
    }
    #[inline]
    pub fn set_vpi_explicit_scalared(&mut self, value: bool)  {
      self.builder.set_bool_field(6, value);
    }
    #[inline]
    pub fn get_vpi_signed(self) -> bool {
      self.builder.get_bool_field(7)
    }
    #[inline]
    pub fn set_vpi_signed(&mut self, value: bool)  {
      self.builder.set_bool_field(7, value);
    }
    #[inline]
    pub fn get_vpi_strength0(self) -> i64 {
      self.builder.get_data_field::<i64>(5)
    }
    #[inline]
    pub fn set_vpi_strength0(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(5, value);
    }
    #[inline]
    pub fn get_vpi_strength1(self) -> i64 {
      self.builder.get_data_field::<i64>(6)
    }
    #[inline]
    pub fn set_vpi_strength1(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(6, value);
    }
    #[inline]
    pub fn get_vpi_charge_strength(self) -> i64 {
      self.builder.get_data_field::<i64>(7)
    }
    #[inline]
    pub fn set_vpi_charge_strength(&mut self, value: i64)  {
      self.builder.set_data_field::<i64>(7, value);
    }
    #[inline]
    pub fn get_vpi_vector(self) -> bool {
      self.builder.get_bool_field(8)
    }
    #[inline]
    pub fn set_vpi_vector(&mut self, value: bool)  {
      self.builder.set_bool_field(8, value);
    }
    #[inline]
    pub fn get_vpi_explicit_vectored(self) -> bool {
      self.builder.get_bool_field(9)
    }
    #[inline]
    pub fn set_vpi_explicit_vectored(&mut self, value: bool)  {
      self.builder.set_bool_field(9, value);
    }
    #[inline]
    pub fn get_vpi_struct_union_member(self) -> bool {
      self.builder.get_bool_field(10)
    }
    #[inline]
    pub fn set_vpi_struct_union_member(&mut self, value: bool)  {
      self.builder.set_bool_field(10, value);
    }
    #[inline]
    pub fn get_ports(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_ports(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(1), value, false)
    }
    #[inline]
    pub fn init_ports(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), size)
    }
    #[inline]
    pub fn has_ports(&self) -> bool {
      !self.builder.is_pointer_field_null(1)
    }
    #[inline]
    pub fn get_drivers(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_drivers(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(2), value, false)
    }
    #[inline]
    pub fn init_drivers(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(2), size)
    }
    #[inline]
    pub fn has_drivers(&self) -> bool {
      !self.builder.is_pointer_field_null(2)
    }
    #[inline]
    pub fn get_loads(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_loads(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(3), value, false)
    }
    #[inline]
    pub fn init_loads(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(3), size)
    }
    #[inline]
    pub fn has_loads(&self) -> bool {
      !self.builder.is_pointer_field_null(3)
    }
    #[inline]
    pub fn get_localdrivers(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(4), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_localdrivers(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(4), value, false)
    }
    #[inline]
    pub fn init_localdrivers(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(4), size)
    }
    #[inline]
    pub fn has_localdrivers(&self) -> bool {
      !self.builder.is_pointer_field_null(4)
    }
    #[inline]
    pub fn get_localloads(self) -> ::capnp::Result<::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(5), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_localloads(&mut self, value: ::capnp::struct_list::Reader<'_,crate::uhdm_capnp::obj_index_type::Owned>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(5), value, false)
    }
    #[inline]
    pub fn init_localloads(self, size: u32) -> ::capnp::struct_list::Builder<'a,crate::uhdm_capnp::obj_index_type::Owned> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(5), size)
    }
    #[inline]
    pub fn has_localloads(&self) -> bool {
      !self.builder.is_pointer_field_null(5)
    }
    #[inline]
    pub fn get_primterms(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(6), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_primterms(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(6), value, false)
    }
    #[inline]
    pub fn init_primterms(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(6), size)
    }
    #[inline]
    pub fn has_primterms(&self) -> bool {
      !self.builder.is_pointer_field_null(6)
    }
    #[inline]
    pub fn get_contassigns(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(7), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_contassigns(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(7), value, false)
    }
    #[inline]
    pub fn init_contassigns(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(7), size)
    }
    #[inline]
    pub fn has_contassigns(&self) -> bool {
      !self.builder.is_pointer_field_null(7)
    }
    #[inline]
    pub fn get_pathterms(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(8), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_pathterms(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(8), value, false)
    }
    #[inline]
    pub fn init_pathterms(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(8), size)
    }
    #[inline]
    pub fn has_pathterms(&self) -> bool {
      !self.builder.is_pointer_field_null(8)
    }
    #[inline]
    pub fn get_tchkterms(self) -> ::capnp::Result<::capnp::primitive_list::Builder<'a,u64>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(9), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_tchkterms(&mut self, value: impl ::capnp::traits::SetterInput<::capnp::primitive_list::Owned<u64>>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(9), value, false)
    }
    #[inline]
    pub fn init_tchkterms(self, size: u32) -> ::capnp::primitive_list::Builder<'a,u64> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(9), size)
    }
    #[inline]
    pub fn has_tchkterms(&self) -> bool {
      !self.builder.is_pointer_field_null(9)
    }
    #[inline]
    pub fn get_simnet(self) -> ::capnp::Result<crate::uhdm_capnp::obj_index_type::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(10), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_simnet(&mut self, value: crate::uhdm_capnp::obj_index_type::Reader<'_>) -> ::capnp::Result<()> {
      ::capnp::traits::SetterInput::set_pointer_builder(self.builder.reborrow().get_pointer_field(10), value, false)
    }
    #[inline]
    pub fn init_simnet(self, ) -> crate::uhdm_capnp::obj_index_type::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(10), 0)
    }
    #[inline]
    pub fn has_simnet(&self) -> bool {
      !self.builder.is_pointer_field_null(10)
    }
    #[inline]
    pub fn get_moduleinst(self) -> u64 {
      self.builder.get_data_field::<u64>(8)
    }
    #[inline]
    pub fn set_moduleinst(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(8, value);
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Self {
      Self { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_base(&self) -> crate::uhdm_capnp::simpleexpr::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(0))
    }
    pub fn get_simnet(&self) -> crate::uhdm_capnp::obj_index_type::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(10))
    }
  }
  mod _private {
    pub static ENCODED_NODE: [::capnp::Word; 535] = [
      ::capnp::word(0, 0, 0, 0, 6, 0, 6, 0),
      ::capnp::word(120, 56, 39, 106, 37, 45, 223, 130),
      ::capnp::word(18, 0, 0, 0, 1, 0, 9, 0),
      ::capnp::word(119, 104, 85, 41, 145, 41, 247, 255),
      ::capnp::word(11, 0, 7, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(21, 120, 0, 0, 176, 123, 0, 0),
      ::capnp::word(21, 0, 0, 0, 186, 0, 0, 0),
      ::capnp::word(29, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(25, 0, 0, 0, 151, 6, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(115, 99, 104, 101, 109, 97, 47, 117),
      ::capnp::word(104, 100, 109, 46, 99, 97, 112, 110),
      ::capnp::word(112, 58, 78, 101, 116, 115, 0, 0),
      ::capnp::word(0, 0, 0, 0, 1, 0, 1, 0),
      ::capnp::word(120, 0, 0, 0, 3, 0, 4, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(57, 3, 0, 0, 42, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(52, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(64, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(61, 3, 0, 0, 122, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(60, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(72, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(2, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(69, 3, 0, 0, 146, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(72, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(84, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(3, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(81, 3, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(80, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(92, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(4, 0, 0, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(89, 3, 0, 0, 130, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(88, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(100, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(5, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(97, 3, 0, 0, 66, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(92, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(104, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(6, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(101, 3, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(100, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(112, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(7, 0, 0, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(109, 3, 0, 0, 138, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(112, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(124, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(8, 0, 0, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(121, 3, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(120, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(132, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(9, 0, 0, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(129, 3, 0, 0, 154, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(132, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(144, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(10, 0, 0, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 10, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(141, 3, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(140, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(152, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(11, 0, 0, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 11, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(149, 3, 0, 0, 162, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(152, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(164, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(12, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 12, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(161, 3, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(160, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(172, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(13, 0, 0, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 13, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(169, 3, 0, 0, 106, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(168, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(180, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(14, 0, 0, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 14, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(177, 3, 0, 0, 106, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(176, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(188, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(15, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 15, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(185, 3, 0, 0, 146, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(188, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(200, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(16, 0, 0, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 16, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(197, 3, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(196, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(208, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(17, 0, 0, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 17, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(205, 3, 0, 0, 162, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(208, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(220, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(18, 0, 0, 0, 10, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 18, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(217, 3, 0, 0, 170, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(220, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(232, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(19, 0, 0, 0, 1, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 19, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(229, 3, 0, 0, 50, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(224, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(252, 3, 0, 0, 2, 0, 1, 0),
      ::capnp::word(20, 0, 0, 0, 2, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 20, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(249, 3, 0, 0, 66, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(244, 3, 0, 0, 3, 0, 1, 0),
      ::capnp::word(16, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(21, 0, 0, 0, 3, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 21, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(13, 4, 0, 0, 50, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(8, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(36, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(22, 0, 0, 0, 4, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 22, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(33, 4, 0, 0, 106, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(32, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(60, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(23, 0, 0, 0, 5, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 23, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(57, 4, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(56, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(84, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(24, 0, 0, 0, 6, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 24, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(81, 4, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(80, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(108, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(25, 0, 0, 0, 7, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 25, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(105, 4, 0, 0, 98, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(104, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(132, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(26, 0, 0, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 26, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(129, 4, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(128, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(156, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(27, 0, 0, 0, 9, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 27, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(153, 4, 0, 0, 82, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(152, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(180, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(28, 0, 0, 0, 10, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 28, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(177, 4, 0, 0, 58, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(172, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(184, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(29, 0, 0, 0, 8, 0, 0, 0),
      ::capnp::word(0, 0, 1, 0, 29, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(181, 4, 0, 0, 90, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(180, 4, 0, 0, 3, 0, 1, 0),
      ::capnp::word(192, 4, 0, 0, 2, 0, 1, 0),
      ::capnp::word(98, 97, 115, 101, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(219, 77, 207, 4, 39, 241, 12, 221),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 65, 114, 114, 97, 121),
      ::capnp::word(77, 101, 109, 98, 101, 114, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 67, 111, 110, 115, 116),
      ::capnp::word(97, 110, 116, 83, 101, 108, 101, 99),
      ::capnp::word(116, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 69, 120, 112, 97, 110),
      ::capnp::word(100, 101, 100, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 73, 109, 112, 108, 105),
      ::capnp::word(99, 105, 116, 68, 101, 99, 108, 0),
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
      ::capnp::word(118, 112, 105, 70, 117, 108, 108, 78),
      ::capnp::word(97, 109, 101, 0, 0, 0, 0, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 78, 101, 116, 68, 101),
      ::capnp::word(99, 108, 65, 115, 115, 105, 103, 110),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 78, 101, 116, 84, 121),
      ::capnp::word(112, 101, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 82, 101, 115, 111, 108),
      ::capnp::word(118, 101, 100, 78, 101, 116, 84, 121),
      ::capnp::word(112, 101, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 83, 99, 97, 108, 97),
      ::capnp::word(114, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 69, 120, 112, 108, 105),
      ::capnp::word(99, 105, 116, 83, 99, 97, 108, 97),
      ::capnp::word(114, 101, 100, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 83, 105, 103, 110, 101),
      ::capnp::word(100, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 83, 116, 114, 101, 110),
      ::capnp::word(103, 116, 104, 48, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 83, 116, 114, 101, 110),
      ::capnp::word(103, 116, 104, 49, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 67, 104, 97, 114, 103),
      ::capnp::word(101, 83, 116, 114, 101, 110, 103, 116),
      ::capnp::word(104, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 86, 101, 99, 116, 111),
      ::capnp::word(114, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 69, 120, 112, 108, 105),
      ::capnp::word(99, 105, 116, 86, 101, 99, 116, 111),
      ::capnp::word(114, 101, 100, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(118, 112, 105, 83, 116, 114, 117, 99),
      ::capnp::word(116, 85, 110, 105, 111, 110, 77, 101),
      ::capnp::word(109, 98, 101, 114, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(112, 111, 114, 116, 115, 0, 0, 0),
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
      ::capnp::word(100, 114, 105, 118, 101, 114, 115, 0),
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
      ::capnp::word(108, 111, 97, 100, 115, 0, 0, 0),
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
      ::capnp::word(108, 111, 99, 97, 108, 100, 114, 105),
      ::capnp::word(118, 101, 114, 115, 0, 0, 0, 0),
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
      ::capnp::word(108, 111, 99, 97, 108, 108, 111, 97),
      ::capnp::word(100, 115, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(112, 114, 105, 109, 116, 101, 114, 109),
      ::capnp::word(115, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(112, 97, 116, 104, 116, 101, 114, 109),
      ::capnp::word(115, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(116, 99, 104, 107, 116, 101, 114, 109),
      ::capnp::word(115, 0, 0, 0, 0, 0, 0, 0),
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
      ::capnp::word(115, 105, 109, 110, 101, 116, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(91, 83, 69, 32, 250, 22, 98, 182),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(16, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
      ::capnp::word(109, 111, 100, 117, 108, 101, 105, 110),
      ::capnp::word(115, 116, 0, 0, 0, 0, 0, 0),
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
        0 => <crate::uhdm_capnp::simpleexpr::Owned as ::capnp::introspect::Introspect>::introspect(),
        1 => <bool as ::capnp::introspect::Introspect>::introspect(),
        2 => <bool as ::capnp::introspect::Introspect>::introspect(),
        3 => <bool as ::capnp::introspect::Introspect>::introspect(),
        4 => <bool as ::capnp::introspect::Introspect>::introspect(),
        5 => <u64 as ::capnp::introspect::Introspect>::introspect(),
        6 => <u64 as ::capnp::introspect::Introspect>::introspect(),
        7 => <bool as ::capnp::introspect::Introspect>::introspect(),
        8 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        9 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        10 => <bool as ::capnp::introspect::Introspect>::introspect(),
        11 => <bool as ::capnp::introspect::Introspect>::introspect(),
        12 => <bool as ::capnp::introspect::Introspect>::introspect(),
        13 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        14 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        15 => <i64 as ::capnp::introspect::Introspect>::introspect(),
        16 => <bool as ::capnp::introspect::Introspect>::introspect(),
        17 => <bool as ::capnp::introspect::Introspect>::introspect(),
        18 => <bool as ::capnp::introspect::Introspect>::introspect(),
        19 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        20 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        21 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        22 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        23 => <::capnp::struct_list::Owned<crate::uhdm_capnp::obj_index_type::Owned> as ::capnp::introspect::Introspect>::introspect(),
        24 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        25 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        26 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        27 => <::capnp::primitive_list::Owned<u64> as ::capnp::introspect::Introspect>::introspect(),
        28 => <crate::uhdm_capnp::obj_index_type::Owned as ::capnp::introspect::Introspect>::introspect(),
        29 => <u64 as ::capnp::introspect::Introspect>::introspect(),
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
    pub static NONUNION_MEMBERS : &[u16] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29];
    pub static MEMBERS_BY_DISCRIMINANT : &[u16] = &[];
    pub static MEMBERS_BY_NAME : &[u16] = &[0,25,20,21,22,23,29,26,19,24,28,27,1,15,2,3,11,17,6,4,5,7,8,9,10,12,13,14,18,16];
    pub const TYPE_ID: u64 = 0x82df_2d25_6a27_3878;
  }
}


use serde::Deserialize;
enum DeEnum<B, C, D> {
    Unit,
    Seq(i8, B, C, D),
    Map { a: i8, b: B, c: C, d: D },
    _Unit2,
    _Seq2(i8, B, C, D),
    _Map2 { a: i8, b: B, c: C, d: D },
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<B: ::core::fmt::Debug, C: ::core::fmt::Debug, D: ::core::fmt::Debug> ::core::fmt::Debug
    for DeEnum<B, C, D>
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&DeEnum::Unit,) => {
                let mut debug_trait_builder = f.debug_tuple("Unit");
                debug_trait_builder.finish()
            }
            (&DeEnum::Seq(ref __self_0, ref __self_1, ref __self_2, ref __self_3),) => {
                let mut debug_trait_builder = f.debug_tuple("Seq");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                let _ = debug_trait_builder.field(&&(*__self_3));
                debug_trait_builder.finish()
            }
            (&DeEnum::Map {
                a: ref __self_0,
                b: ref __self_1,
                c: ref __self_2,
                d: ref __self_3,
            },) => {
                let mut debug_trait_builder = f.debug_struct("Map");
                let _ = debug_trait_builder.field("a", &&(*__self_0));
                let _ = debug_trait_builder.field("b", &&(*__self_1));
                let _ = debug_trait_builder.field("c", &&(*__self_2));
                let _ = debug_trait_builder.field("d", &&(*__self_3));
                debug_trait_builder.finish()
            }
            (&DeEnum::_Unit2,) => {
                let mut debug_trait_builder = f.debug_tuple("_Unit2");
                debug_trait_builder.finish()
            }
            (&DeEnum::_Seq2(ref __self_0, ref __self_1, ref __self_2, ref __self_3),) => {
                let mut debug_trait_builder = f.debug_tuple("_Seq2");
                let _ = debug_trait_builder.field(&&(*__self_0));
                let _ = debug_trait_builder.field(&&(*__self_1));
                let _ = debug_trait_builder.field(&&(*__self_2));
                let _ = debug_trait_builder.field(&&(*__self_3));
                debug_trait_builder.finish()
            }
            (&DeEnum::_Map2 {
                a: ref __self_0,
                b: ref __self_1,
                c: ref __self_2,
                d: ref __self_3,
            },) => {
                let mut debug_trait_builder = f.debug_struct("_Map2");
                let _ = debug_trait_builder.field("a", &&(*__self_0));
                let _ = debug_trait_builder.field("b", &&(*__self_1));
                let _ = debug_trait_builder.field("c", &&(*__self_2));
                let _ = debug_trait_builder.field("d", &&(*__self_3));
                debug_trait_builder.finish()
            }
        }
    }
}
impl<B, C, D> ::core::marker::StructuralPartialEq for DeEnum<B, C, D> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<B: ::core::cmp::PartialEq, C: ::core::cmp::PartialEq, D: ::core::cmp::PartialEq>
    ::core::cmp::PartialEq for DeEnum<B, C, D>
{
    #[inline]
    fn eq(&self, other: &DeEnum<B, C, D>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &DeEnum::Seq(ref __self_0, ref __self_1, ref __self_2, ref __self_3),
                        &DeEnum::Seq(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2, ref __arg_1_3),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                    }
                    (
                        &DeEnum::Map {
                            a: ref __self_0,
                            b: ref __self_1,
                            c: ref __self_2,
                            d: ref __self_3,
                        },
                        &DeEnum::Map {
                            a: ref __arg_1_0,
                            b: ref __arg_1_1,
                            c: ref __arg_1_2,
                            d: ref __arg_1_3,
                        },
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                    }
                    (
                        &DeEnum::_Seq2(ref __self_0, ref __self_1, ref __self_2, ref __self_3),
                        &DeEnum::_Seq2(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2, ref __arg_1_3),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                    }
                    (
                        &DeEnum::_Map2 {
                            a: ref __self_0,
                            b: ref __self_1,
                            c: ref __self_2,
                            d: ref __self_3,
                        },
                        &DeEnum::_Map2 {
                            a: ref __arg_1_0,
                            b: ref __arg_1_1,
                            c: ref __arg_1_2,
                            d: ref __arg_1_3,
                        },
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &DeEnum<B, C, D>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &DeEnum::Seq(ref __self_0, ref __self_1, ref __self_2, ref __self_3),
                        &DeEnum::Seq(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2, ref __arg_1_3),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                    }
                    (
                        &DeEnum::Map {
                            a: ref __self_0,
                            b: ref __self_1,
                            c: ref __self_2,
                            d: ref __self_3,
                        },
                        &DeEnum::Map {
                            a: ref __arg_1_0,
                            b: ref __arg_1_1,
                            c: ref __arg_1_2,
                            d: ref __arg_1_3,
                        },
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                    }
                    (
                        &DeEnum::_Seq2(ref __self_0, ref __self_1, ref __self_2, ref __self_3),
                        &DeEnum::_Seq2(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2, ref __arg_1_3),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                    }
                    (
                        &DeEnum::_Map2 {
                            a: ref __self_0,
                            b: ref __self_1,
                            c: ref __self_2,
                            d: ref __self_3,
                        },
                        &DeEnum::_Map2 {
                            a: ref __arg_1_0,
                            b: ref __arg_1_1,
                            c: ref __arg_1_2,
                            d: ref __arg_1_3,
                        },
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                    }
                    _ => false,
                }
            } else {
                true
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_DeEnum: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de, B, C, D> _serde::Deserialize<'de> for DeEnum<B, C, D>
    where
        B: _serde::Deserialize<'de>,
        C: _serde::Deserialize<'de>,
        D: _serde::Deserialize<'de>,
    {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        3u64 => _serde::export::Ok(__Field::__field3),
                        4u64 => _serde::export::Ok(__Field::__field4),
                        5u64 => _serde::export::Ok(__Field::__field5),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 6",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Unit" => _serde::export::Ok(__Field::__field0),
                        "Seq" => _serde::export::Ok(__Field::__field1),
                        "Map" => _serde::export::Ok(__Field::__field2),
                        "_Unit2" => _serde::export::Ok(__Field::__field3),
                        "_Seq2" => _serde::export::Ok(__Field::__field4),
                        "_Map2" => _serde::export::Ok(__Field::__field5),
                        _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Unit" => _serde::export::Ok(__Field::__field0),
                        b"Seq" => _serde::export::Ok(__Field::__field1),
                        b"Map" => _serde::export::Ok(__Field::__field2),
                        b"_Unit2" => _serde::export::Ok(__Field::__field3),
                        b"_Seq2" => _serde::export::Ok(__Field::__field4),
                        b"_Map2" => _serde::export::Ok(__Field::__field5),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de, B, C, D>
            where
                B: _serde::Deserialize<'de>,
                C: _serde::Deserialize<'de>,
                D: _serde::Deserialize<'de>,
            {
                marker: _serde::export::PhantomData<DeEnum<B, C, D>>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de, B, C, D> _serde::de::Visitor<'de> for __Visitor<'de, B, C, D>
            where
                B: _serde::Deserialize<'de>,
                C: _serde::Deserialize<'de>,
                D: _serde::Deserialize<'de>,
            {
                type Value = DeEnum<B, C, D>;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "enum DeEnum")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(DeEnum::Unit)
                        }
                        (__Field::__field1, __variant) => {
                            struct __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                marker: _serde::export::PhantomData<DeEnum<B, C, D>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de, B, C, D> _serde::de::Visitor<'de> for __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                type Value = DeEnum<B, C, D>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "tuple variant DeEnum::Seq",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        i8,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"tuple variant DeEnum::Seq with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        B,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"tuple variant DeEnum::Seq with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        C,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"tuple variant DeEnum::Seq with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        D,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"tuple variant DeEnum::Seq with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(DeEnum::Seq(
                                        __field0, __field1, __field2, __field3,
                                    ))
                                }
                            }
                            _serde::de::VariantAccess::tuple_variant(
                                __variant,
                                4usize,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<DeEnum<B, C, D>>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                        (__Field::__field2, __variant) => {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 4",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "a" => _serde::export::Ok(__Field::__field0),
                                        "b" => _serde::export::Ok(__Field::__field1),
                                        "c" => _serde::export::Ok(__Field::__field2),
                                        "d" => _serde::export::Ok(__Field::__field3),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"a" => _serde::export::Ok(__Field::__field0),
                                        b"b" => _serde::export::Ok(__Field::__field1),
                                        b"c" => _serde::export::Ok(__Field::__field2),
                                        b"d" => _serde::export::Ok(__Field::__field3),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                marker: _serde::export::PhantomData<DeEnum<B, C, D>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de, B, C, D> _serde::de::Visitor<'de> for __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                type Value = DeEnum<B, C, D>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct variant DeEnum::Map",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        i8,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant DeEnum::Map with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        B,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct variant DeEnum::Map with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        C,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct variant DeEnum::Map with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        D,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct variant DeEnum::Map with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(DeEnum::Map {
                                        a: __field0,
                                        b: __field1,
                                        c: __field2,
                                        d: __field3,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::export::Option<i8> =
                                        _serde::export::None;
                                    let mut __field1: _serde::export::Option<B> =
                                        _serde::export::None;
                                    let mut __field2: _serde::export::Option<C> =
                                        _serde::export::None;
                                    let mut __field3: _serde::export::Option<D> =
                                        _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "a" ) ) ;
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<i8>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "b" ) ) ;
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<B>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "c" ) ) ;
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<C>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "d" ) ) ;
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<D>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("a") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("b") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("c") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("d") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(DeEnum::Map {
                                        a: __field0,
                                        b: __field1,
                                        c: __field2,
                                        d: __field3,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &["a", "b", "c", "d"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<DeEnum<B, C, D>>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(DeEnum::_Unit2)
                        }
                        (__Field::__field4, __variant) => {
                            struct __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                marker: _serde::export::PhantomData<DeEnum<B, C, D>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de, B, C, D> _serde::de::Visitor<'de> for __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                type Value = DeEnum<B, C, D>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "tuple variant DeEnum::_Seq2",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        i8,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"tuple variant DeEnum::_Seq2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        B,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"tuple variant DeEnum::_Seq2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        C,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"tuple variant DeEnum::_Seq2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        D,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"tuple variant DeEnum::_Seq2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(DeEnum::_Seq2(
                                        __field0, __field1, __field2, __field3,
                                    ))
                                }
                            }
                            _serde::de::VariantAccess::tuple_variant(
                                __variant,
                                4usize,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<DeEnum<B, C, D>>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                        (__Field::__field5, __variant) => {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 4",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "a" => _serde::export::Ok(__Field::__field0),
                                        "b" => _serde::export::Ok(__Field::__field1),
                                        "c" => _serde::export::Ok(__Field::__field2),
                                        "d" => _serde::export::Ok(__Field::__field3),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"a" => _serde::export::Ok(__Field::__field0),
                                        b"b" => _serde::export::Ok(__Field::__field1),
                                        b"c" => _serde::export::Ok(__Field::__field2),
                                        b"d" => _serde::export::Ok(__Field::__field3),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                marker: _serde::export::PhantomData<DeEnum<B, C, D>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de, B, C, D> _serde::de::Visitor<'de> for __Visitor<'de, B, C, D>
                            where
                                B: _serde::Deserialize<'de>,
                                C: _serde::Deserialize<'de>,
                                D: _serde::Deserialize<'de>,
                            {
                                type Value = DeEnum<B, C, D>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct variant DeEnum::_Map2",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        i8,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant DeEnum::_Map2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        B,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct variant DeEnum::_Map2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        C,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct variant DeEnum::_Map2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        D,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct variant DeEnum::_Map2 with 4 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(DeEnum::_Map2 {
                                        a: __field0,
                                        b: __field1,
                                        c: __field2,
                                        d: __field3,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::export::Option<i8> =
                                        _serde::export::None;
                                    let mut __field1: _serde::export::Option<B> =
                                        _serde::export::None;
                                    let mut __field2: _serde::export::Option<C> =
                                        _serde::export::None;
                                    let mut __field3: _serde::export::Option<D> =
                                        _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "a" ) ) ;
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<i8>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "b" ) ) ;
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<B>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "c" ) ) ;
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<C>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "d" ) ) ;
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<D>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("a") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("b") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("c") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("d") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(DeEnum::_Map2 {
                                        a: __field0,
                                        b: __field1,
                                        c: __field2,
                                        d: __field3,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &["a", "b", "c", "d"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<DeEnum<B, C, D>>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] =
                &["Unit", "Seq", "Map", "_Unit2", "_Seq2", "_Map2"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "DeEnum",
                VARIANTS,
                __Visitor {
                    marker: _serde::export::PhantomData::<DeEnum<B, C, D>>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
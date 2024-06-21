#![feature(prelude_import)]
//! Core data structures and algorithms used throughout Fyrox.
//!
//! Some of them can be useful separately outside the engine.
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate memoffset;
#[macro_use]
extern crate lazy_static;
pub use arrayvec;
pub use byteorder;
pub use nalgebra as algebra;
pub use num_traits;
pub use parking_lot;
pub use rand;
pub use sstorage::ImmutableString;
pub use uuid;
use crate::visitor::{Visit, VisitResult, Visitor};
use fxhash::FxHashMap;
use std::ffi::OsString;
use std::hash::Hasher;
use std::{borrow::Borrow, cmp, hash::Hash, path::{Path, PathBuf}};
pub mod color {
    use crate::{
        algebra::{Vector3, Vector4},
        reflect::prelude::*, uuid_provider, visitor::{Visit, VisitResult, Visitor},
    };
    use num_traits::Zero;
    use std::ops::{Add, AddAssign, Sub, SubAssign};
    #[repr(C)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Color {}
    #[automatically_derived]
    impl ::core::clone::Clone for Color {
        #[inline]
        fn clone(&self) -> Color {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Color {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Color",
                "r",
                &self.r,
                "g",
                &self.g,
                "b",
                &self.b,
                "a",
                &&self.a,
            )
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Color {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Color,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.r, &other.r) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    match ::core::cmp::PartialOrd::partial_cmp(&self.g, &other.g) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &self.b,
                                &other.b,
                            ) {
                                ::core::option::Option::Some(
                                    ::core::cmp::Ordering::Equal,
                                ) => ::core::cmp::PartialOrd::partial_cmp(&self.a, &other.a),
                                cmp => cmp,
                            }
                        }
                        cmp => cmp,
                    }
                }
                cmp => cmp,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Color {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Color {
        #[inline]
        fn eq(&self, other: &Color) -> bool {
            self.r == other.r && self.g == other.g && self.b == other.b
                && self.a == other.a
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Color {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for Color
    where
        u8: Visit,
        u8: Visit,
        u8: Visit,
        u8: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            if let Err(err) = self.r.visit("R", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.g.visit("G", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.b.visit("B", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.a.visit("A", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for Color
    where
        Self: 'static,
        u8: Reflect,
        u8: Reflect,
        u8: Reflect,
        u8: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\color.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "r",
                        display_name: "R",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.r,
                        reflect_value: &self.r,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<u8>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "g",
                        display_name: "G",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.g,
                        reflect_value: &self.g,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<u8>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "b",
                        display_name: "B",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.b,
                        reflect_value: &self.b,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<u8>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "a",
                        display_name: "A",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.a,
                        reflect_value: &self.a,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<u8>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(
                &[
                    &self.r as &dyn Reflect,
                    &self.g as &dyn Reflect,
                    &self.b as &dyn Reflect,
                    &self.a as &dyn Reflect,
                ],
            )
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(
                &mut [
                    &mut self.r as &mut dyn Reflect,
                    &mut self.g as &mut dyn Reflect,
                    &mut self.b as &mut dyn Reflect,
                    &mut self.a as &mut dyn Reflect,
                ],
            )
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "r" => Some(&self.r as &dyn Reflect),
                    "g" => Some(&self.g as &dyn Reflect),
                    "b" => Some(&self.b as &dyn Reflect),
                    "a" => Some(&self.a as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "r" => Some(&mut self.r as &mut dyn Reflect),
                    "g" => Some(&mut self.g as &mut dyn Reflect),
                    "b" => Some(&mut self.b as &mut dyn Reflect),
                    "a" => Some(&mut self.a as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(missing_docs)]
    impl Color {
        pub const R: &'static str = "r";
        pub const G: &'static str = "g";
        pub const B: &'static str = "b";
        pub const A: &'static str = "a";
    }
    impl crate::type_traits::TypeUuidProvider for Color {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "74e898aa-de19-44bd-8213-3b6d450b1bf8",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl Default for Color {
        #[inline]
        fn default() -> Self {
            Self::WHITE
        }
    }
    impl Into<u32> for Color {
        #[inline]
        fn into(self) -> u32 {
            ((self.a as u32) << 24) | ((self.b as u32) << 16) | ((self.g as u32) << 8)
                | (self.r as u32)
        }
    }
    impl From<Vector3<f32>> for Color {
        fn from(v: Vector3<f32>) -> Self {
            Self {
                r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
                g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
                b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
                a: 255,
            }
        }
    }
    impl From<Vector4<f32>> for Color {
        fn from(v: Vector4<f32>) -> Self {
            Self {
                r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
                g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
                b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
                a: (v.w.clamp(0.0, 1.0) * 255.0) as u8,
            }
        }
    }
    pub struct Hsv {
        /// [0; 360] range
        hue: f32,
        /// [0; 100] range
        saturation: f32,
        /// [0; 100] range
        brightness: f32,
    }
    #[automatically_derived]
    impl ::core::default::Default for Hsv {
        #[inline]
        fn default() -> Hsv {
            Hsv {
                hue: ::core::default::Default::default(),
                saturation: ::core::default::Default::default(),
                brightness: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Hsv {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Hsv",
                "hue",
                &self.hue,
                "saturation",
                &self.saturation,
                "brightness",
                &&self.brightness,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Hsv {}
    #[automatically_derived]
    impl ::core::clone::Clone for Hsv {
        #[inline]
        fn clone(&self) -> Hsv {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Hsv {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Hsv {
        #[inline]
        fn eq(&self, other: &Hsv) -> bool {
            self.hue == other.hue && self.saturation == other.saturation
                && self.brightness == other.brightness
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for Hsv
    where
        f32: Visit,
        f32: Visit,
        f32: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            if let Err(err) = self.hue.visit("Hue", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.saturation.visit("Saturation", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.brightness.visit("Brightness", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for Hsv
    where
        Self: 'static,
        f32: Reflect,
        f32: Reflect,
        f32: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\color.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "hue",
                        display_name: "Hue",
                        doc: " [0; 360] range",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.hue,
                        reflect_value: &self.hue,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "saturation",
                        display_name: "Saturation",
                        doc: " [0; 100] range",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.saturation,
                        reflect_value: &self.saturation,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "brightness",
                        display_name: "Brightness",
                        doc: " [0; 100] range",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.brightness,
                        reflect_value: &self.brightness,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(
                &[
                    &self.hue as &dyn Reflect,
                    &self.saturation as &dyn Reflect,
                    &self.brightness as &dyn Reflect,
                ],
            )
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(
                &mut [
                    &mut self.hue as &mut dyn Reflect,
                    &mut self.saturation as &mut dyn Reflect,
                    &mut self.brightness as &mut dyn Reflect,
                ],
            )
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "hue" => Some(&self.hue as &dyn Reflect),
                    "saturation" => Some(&self.saturation as &dyn Reflect),
                    "brightness" => Some(&self.brightness as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "hue" => Some(&mut self.hue as &mut dyn Reflect),
                    "saturation" => Some(&mut self.saturation as &mut dyn Reflect),
                    "brightness" => Some(&mut self.brightness as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(missing_docs)]
    impl Hsv {
        pub const HUE: &'static str = "hue";
        pub const SATURATION: &'static str = "saturation";
        pub const BRIGHTNESS: &'static str = "brightness";
    }
    impl Hsv {
        #[inline]
        pub fn new(hue: f32, saturation: f32, brightness: f32) -> Self {
            Self {
                hue: hue.clamp(0.0, 360.0),
                saturation: saturation.clamp(0.0, 100.0),
                brightness: brightness.clamp(0.0, 100.0),
            }
        }
        #[inline]
        pub fn hue(&self) -> f32 {
            self.hue
        }
        #[inline]
        pub fn set_hue(&mut self, hue: f32) {
            self.hue = hue.clamp(0.0, 360.0);
        }
        #[inline]
        pub fn saturation(&self) -> f32 {
            self.saturation
        }
        #[inline]
        pub fn set_saturation(&mut self, saturation: f32) {
            self.saturation = saturation.clamp(0.0, 100.0);
        }
        #[inline]
        pub fn brightness(&self) -> f32 {
            self.brightness
        }
        #[inline]
        pub fn set_brightness(&mut self, brightness: f32) {
            self.brightness = brightness.clamp(0.0, 100.0);
        }
    }
    impl From<Color> for Hsv {
        #[inline]
        fn from(color: Color) -> Self {
            let r = color.r as f32 / 255.0;
            let g = color.g as f32 / 255.0;
            let b = color.b as f32 / 255.0;
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let hue = if max.eq(&min) {
                0.0
            } else if max.eq(&r) && g >= b {
                60.0 * (g - b) / (max - min)
            } else if max.eq(&r) && g < b {
                60.0 * (g - b) / (max - min) + 360.0
            } else if max.eq(&g) {
                60.0 * (b - r) / (max - min) + 120.0
            } else if max.eq(&b) {
                60.0 * (r - g) / (max - min) + 240.0
            } else {
                0.0
            };
            let saturation = if max.eq(&0.0) { 0.0 } else { 1.0 - min / max };
            let brightness = max;
            Self {
                hue,
                saturation: saturation * 100.0,
                brightness: brightness * 100.0,
            }
        }
    }
    impl From<Hsv> for Color {
        #[inline]
        fn from(hsv: Hsv) -> Self {
            let hi = ((hsv.hue / 60.0) % 6.0) as i32;
            let vmin = ((100.0 - hsv.saturation) * hsv.brightness) / 100.0;
            let a = (hsv.brightness - vmin) * ((hsv.hue % 60.0) / 60.0);
            let vinc = vmin + a;
            let vdec = hsv.brightness - a;
            Self::from(
                match hi {
                    0 => Vector3::new(hsv.brightness, vinc, vmin),
                    1 => Vector3::new(vdec, hsv.brightness, vmin),
                    2 => Vector3::new(vmin, hsv.brightness, vinc),
                    3 => Vector3::new(vmin, vdec, hsv.brightness),
                    4 => Vector3::new(vinc, vmin, hsv.brightness),
                    5 => Vector3::new(hsv.brightness, vmin, vdec),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
                    .scale(1.0 / 100.0),
            )
        }
    }
    pub struct Hsl {
        /// [0; 360] range
        hue: f32,
        /// [0; 1] range
        saturation: f32,
        /// [0; 1] range
        lightness: f32,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Hsl {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Hsl {
        #[inline]
        fn eq(&self, other: &Hsl) -> bool {
            self.hue == other.hue && self.saturation == other.saturation
                && self.lightness == other.lightness
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Hsl {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Hsl",
                "hue",
                &self.hue,
                "saturation",
                &self.saturation,
                "lightness",
                &&self.lightness,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Hsl {}
    #[automatically_derived]
    impl ::core::clone::Clone for Hsl {
        #[inline]
        fn clone(&self) -> Hsl {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    impl Hsl {
        /// Hue: [0; 360] range
        /// Saturation: [0; 1] range
        /// Lightness: [0; 1] range
        pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
            Self {
                hue: hue.abs() % 360.0,
                saturation: saturation.clamp(0.0, 1.0),
                lightness: lightness.clamp(0.0, 1.0),
            }
        }
        pub fn hue(&self) -> f32 {
            self.hue
        }
        pub fn set_hue(&mut self, hue: f32) {
            self.hue = hue.abs() % 360.0;
        }
        pub fn saturation(&self) -> f32 {
            self.saturation
        }
        pub fn set_saturation(&mut self, saturation: f32) {
            self.saturation = saturation.clamp(0.0, 1.0);
        }
        pub fn lightness(&self) -> f32 {
            self.lightness
        }
        pub fn set_lightness(&mut self, lightness: f32) {
            self.lightness = lightness.clamp(0.0, 1.0);
        }
    }
    impl From<Hsl> for Color {
        #[allow(clippy::manual_range_contains)]
        #[inline]
        fn from(v: Hsl) -> Self {
            let h = v.hue;
            let s = v.saturation;
            let l = v.lightness;
            let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
            let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
            let m = l - c / 2.0;
            let (r, g, b) = if h >= 0.0 && h < 60.0 {
                (c, x, 0.0)
            } else if h >= 60.0 && h < 120.0 {
                (x, c, 0.0)
            } else if h >= 120.0 && h < 180.0 {
                (0.0, c, x)
            } else if h >= 180.0 && h < 240.0 {
                (0.0, x, c)
            } else if h >= 240.0 && h < 300.0 {
                (x, 0.0, c)
            } else if h >= 300.0 && h < 360.0 {
                (c, 0.0, x)
            } else {
                (0.0, 0.0, 0.0)
            };
            Color::from(Vector4::new(r + m, g + m, b + m, 1.0))
        }
    }
    impl From<Color> for Hsl {
        fn from(v: Color) -> Self {
            let f = v.as_frgb();
            let r = f.x;
            let g = f.y;
            let b = f.z;
            let cmax = r.max(g).max(b);
            let cmin = r.min(g).min(b);
            let d = cmax - cmin;
            let h = if d.is_zero() {
                0.0
            } else if cmax.eq(&r) {
                let k = 60.0 * (((g - b) / d) % 6.0);
                if g >= b { k } else { k + 360.0 }
            } else if cmax.eq(&g) {
                60.0 * ((b - r) / d + 2.0)
            } else if cmax.eq(&b) {
                60.0 * ((r - g) / d + 4.0)
            } else {
                0.0
            };
            let l = (cmax + cmin) / 2.0;
            let s = if d.is_zero() { 0.0 } else { d / (1.0 - (2.0 * l - 1.0).abs()) };
            Hsl {
                hue: h,
                saturation: s,
                lightness: l,
            }
        }
    }
    impl Color {
        pub const WHITE: Self = Self::repeat(255);
        pub const BLACK: Self = Self::opaque(0, 0, 0);
        pub const RED: Self = Self::opaque(255, 0, 0);
        pub const GREEN: Self = Self::opaque(0, 255, 0);
        pub const BLUE: Self = Self::opaque(0, 0, 255);
        pub const TRANSPARENT: Self = Self::repeat(0);
        pub const MAROON: Self = Self::opaque(128, 0, 0);
        pub const DARK_RED: Self = Self::opaque(139, 0, 0);
        pub const BROWN: Self = Self::opaque(165, 42, 42);
        pub const FIREBRICK: Self = Self::opaque(178, 34, 34);
        pub const CRIMSON: Self = Self::opaque(220, 20, 60);
        pub const TOMATO: Self = Self::opaque(255, 99, 71);
        pub const CORAL: Self = Self::opaque(255, 127, 80);
        pub const INDIAN_RED: Self = Self::opaque(205, 92, 92);
        pub const LIGHT_CORAL: Self = Self::opaque(240, 128, 128);
        pub const DARK_SALMON: Self = Self::opaque(233, 150, 122);
        pub const SALMON: Self = Self::opaque(250, 128, 114);
        pub const LIGHT_SALMON: Self = Self::opaque(255, 160, 122);
        pub const ORANGE_RED: Self = Self::opaque(255, 69, 0);
        pub const DARK_ORANGE: Self = Self::opaque(255, 140, 0);
        pub const ORANGE: Self = Self::opaque(255, 165, 0);
        pub const GOLD: Self = Self::opaque(255, 215, 0);
        pub const DARK_GOLDEN_ROD: Self = Self::opaque(184, 134, 11);
        pub const GOLDEN_ROD: Self = Self::opaque(218, 165, 32);
        pub const PALE_GOLDEN_ROD: Self = Self::opaque(238, 232, 170);
        pub const DARK_KHAKI: Self = Self::opaque(189, 183, 107);
        pub const KHAKI: Self = Self::opaque(240, 230, 140);
        pub const OLIVE: Self = Self::opaque(128, 128, 0);
        pub const YELLOW: Self = Self::opaque(255, 255, 0);
        pub const YELLOW_GREEN: Self = Self::opaque(154, 205, 50);
        pub const DARK_OLIVE_GREEN: Self = Self::opaque(85, 107, 47);
        pub const OLIVE_DRAB: Self = Self::opaque(107, 142, 35);
        pub const LAWN_GREEN: Self = Self::opaque(124, 252, 0);
        pub const CHARTREUSE: Self = Self::opaque(127, 255, 0);
        pub const GREEN_YELLOW: Self = Self::opaque(173, 255, 47);
        pub const DARK_GREEN: Self = Self::opaque(0, 100, 0);
        pub const FOREST_GREEN: Self = Self::opaque(34, 139, 34);
        pub const LIME: Self = Self::opaque(0, 255, 0);
        pub const LIME_GREEN: Self = Self::opaque(50, 205, 50);
        pub const LIGHT_GREEN: Self = Self::opaque(144, 238, 144);
        pub const PALE_GREEN: Self = Self::opaque(152, 251, 152);
        pub const DARK_SEA_GREEN: Self = Self::opaque(143, 188, 143);
        pub const MEDIUM_SPRING_GREEN: Self = Self::opaque(0, 250, 154);
        pub const SPRING_GREEN: Self = Self::opaque(0, 255, 127);
        pub const SEA_GREEN: Self = Self::opaque(46, 139, 87);
        pub const MEDIUM_AQUA_MARINE: Self = Self::opaque(102, 205, 170);
        pub const MEDIUM_SEA_GREEN: Self = Self::opaque(60, 179, 113);
        pub const LIGHT_SEA_GREEN: Self = Self::opaque(32, 178, 170);
        pub const DARK_SLATE_GRAY: Self = Self::opaque(47, 79, 79);
        pub const TEAL: Self = Self::opaque(0, 128, 128);
        pub const DARK_CYAN: Self = Self::opaque(0, 139, 139);
        pub const AQUA: Self = Self::opaque(0, 255, 255);
        pub const CYAN: Self = Self::opaque(0, 255, 255);
        pub const LIGHT_CYAN: Self = Self::opaque(224, 255, 255);
        pub const DARK_TURQUOISE: Self = Self::opaque(0, 206, 209);
        pub const TURQUOISE: Self = Self::opaque(64, 224, 208);
        pub const MEDIUM_TURQUOISE: Self = Self::opaque(72, 209, 204);
        pub const PALE_TURQUOISE: Self = Self::opaque(175, 238, 238);
        pub const AQUA_MARINE: Self = Self::opaque(127, 255, 212);
        pub const POWDER_BLUE: Self = Self::opaque(176, 224, 230);
        pub const CADET_BLUE: Self = Self::opaque(95, 158, 160);
        pub const STEEL_BLUE: Self = Self::opaque(70, 130, 180);
        pub const CORN_FLOWER_BLUE: Self = Self::opaque(100, 149, 237);
        pub const DEEP_SKY_BLUE: Self = Self::opaque(0, 191, 255);
        pub const DODGER_BLUE: Self = Self::opaque(30, 144, 255);
        pub const LIGHT_BLUE: Self = Self::opaque(173, 216, 230);
        pub const SKY_BLUE: Self = Self::opaque(135, 206, 235);
        pub const LIGHT_SKY_BLUE: Self = Self::opaque(135, 206, 250);
        pub const MIDNIGHT_BLUE: Self = Self::opaque(25, 25, 112);
        pub const NAVY: Self = Self::opaque(0, 0, 128);
        pub const DARK_BLUE: Self = Self::opaque(0, 0, 139);
        pub const MEDIUM_BLUE: Self = Self::opaque(0, 0, 205);
        pub const ROYAL_BLUE: Self = Self::opaque(65, 105, 225);
        pub const BLUE_VIOLET: Self = Self::opaque(138, 43, 226);
        pub const INDIGO: Self = Self::opaque(75, 0, 130);
        pub const DARK_SLATE_BLUE: Self = Self::opaque(72, 61, 139);
        pub const SLATE_BLUE: Self = Self::opaque(106, 90, 205);
        pub const MEDIUM_SLATE_BLUE: Self = Self::opaque(123, 104, 238);
        pub const MEDIUM_PURPLE: Self = Self::opaque(147, 112, 219);
        pub const DARK_MAGENTA: Self = Self::opaque(139, 0, 139);
        pub const DARK_VIOLET: Self = Self::opaque(148, 0, 211);
        pub const DARK_ORCHID: Self = Self::opaque(153, 50, 204);
        pub const MEDIUM_ORCHID: Self = Self::opaque(186, 85, 211);
        pub const PURPLE: Self = Self::opaque(128, 0, 128);
        pub const THISTLE: Self = Self::opaque(216, 191, 216);
        pub const PLUM: Self = Self::opaque(221, 160, 221);
        pub const VIOLET: Self = Self::opaque(238, 130, 238);
        pub const MAGENTA: Self = Self::opaque(255, 0, 255);
        pub const ORCHID: Self = Self::opaque(218, 112, 214);
        pub const MEDIUM_VIOLET_RED: Self = Self::opaque(199, 21, 133);
        pub const PALE_VIOLET_RED: Self = Self::opaque(219, 112, 147);
        pub const DEEP_PINK: Self = Self::opaque(255, 20, 147);
        pub const HOT_PINK: Self = Self::opaque(255, 105, 180);
        pub const LIGHT_PINK: Self = Self::opaque(255, 182, 193);
        pub const PINK: Self = Self::opaque(255, 192, 203);
        pub const ANTIQUE_WHITE: Self = Self::opaque(250, 235, 215);
        pub const BEIGE: Self = Self::opaque(245, 245, 220);
        pub const BISQUE: Self = Self::opaque(255, 228, 196);
        pub const BLANCHED_ALMOND: Self = Self::opaque(255, 235, 205);
        pub const WHEAT: Self = Self::opaque(245, 222, 179);
        pub const CORN_SILK: Self = Self::opaque(255, 248, 220);
        pub const LEMON_CHIFFON: Self = Self::opaque(255, 250, 205);
        pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::opaque(250, 250, 210);
        pub const LIGHT_YELLOW: Self = Self::opaque(255, 255, 224);
        pub const SADDLE_BROWN: Self = Self::opaque(139, 69, 19);
        pub const SIENNA: Self = Self::opaque(160, 82, 45);
        pub const CHOCOLATE: Self = Self::opaque(210, 105, 30);
        pub const PERU: Self = Self::opaque(205, 133, 63);
        pub const SANDY_BROWN: Self = Self::opaque(244, 164, 96);
        pub const BURLY_WOOD: Self = Self::opaque(222, 184, 135);
        pub const TAN: Self = Self::opaque(210, 180, 140);
        pub const ROSY_BROWN: Self = Self::opaque(188, 143, 143);
        pub const MOCCASIN: Self = Self::opaque(255, 228, 181);
        pub const NAVAJO_WHITE: Self = Self::opaque(255, 222, 173);
        pub const PEACH_PUFF: Self = Self::opaque(255, 218, 185);
        pub const MISTY_ROSE: Self = Self::opaque(255, 228, 225);
        pub const LAVENDER_BLUSH: Self = Self::opaque(255, 240, 245);
        pub const LINEN: Self = Self::opaque(250, 240, 230);
        pub const OLD_LACE: Self = Self::opaque(253, 245, 230);
        pub const PAPAYA_WHIP: Self = Self::opaque(255, 239, 213);
        pub const SEA_SHELL: Self = Self::opaque(255, 245, 238);
        pub const MINT_CREAM: Self = Self::opaque(245, 255, 250);
        pub const SLATE_GRAY: Self = Self::opaque(112, 128, 144);
        pub const LIGHT_SLATE_GRAY: Self = Self::opaque(119, 136, 153);
        pub const LIGHT_STEEL_BLUE: Self = Self::opaque(176, 196, 222);
        pub const LAVENDER: Self = Self::opaque(230, 230, 250);
        pub const FLORAL_WHITE: Self = Self::opaque(255, 250, 240);
        pub const ALICE_BLUE: Self = Self::opaque(240, 248, 255);
        pub const GHOST_WHITE: Self = Self::opaque(248, 248, 255);
        pub const HONEYDEW: Self = Self::opaque(240, 255, 240);
        pub const IVORY: Self = Self::opaque(255, 255, 240);
        pub const AZURE: Self = Self::opaque(240, 255, 255);
        pub const SNOW: Self = Self::opaque(255, 250, 250);
        pub const DIM_GRAY: Self = Self::opaque(105, 105, 105);
        pub const GRAY: Self = Self::opaque(128, 128, 128);
        pub const DARK_GRAY: Self = Self::opaque(169, 169, 169);
        pub const SILVER: Self = Self::opaque(192, 192, 192);
        pub const LIGHT_GRAY: Self = Self::opaque(211, 211, 211);
        pub const GAINSBORO: Self = Self::opaque(220, 220, 220);
        pub const WHITE_SMOKE: Self = Self::opaque(245, 245, 245);
        pub const COLORS: [Self; 140] = [
            Self::TRANSPARENT,
            Self::WHITE,
            Self::BLACK,
            Self::RED,
            Self::GREEN,
            Self::BLUE,
            Self::MAROON,
            Self::DARK_RED,
            Self::BROWN,
            Self::FIREBRICK,
            Self::CRIMSON,
            Self::TOMATO,
            Self::CORAL,
            Self::INDIAN_RED,
            Self::LIGHT_CORAL,
            Self::DARK_SALMON,
            Self::SALMON,
            Self::LIGHT_SALMON,
            Self::ORANGE_RED,
            Self::DARK_ORANGE,
            Self::ORANGE,
            Self::GOLD,
            Self::DARK_GOLDEN_ROD,
            Self::GOLDEN_ROD,
            Self::PALE_GOLDEN_ROD,
            Self::DARK_KHAKI,
            Self::KHAKI,
            Self::OLIVE,
            Self::YELLOW,
            Self::YELLOW_GREEN,
            Self::DARK_OLIVE_GREEN,
            Self::OLIVE_DRAB,
            Self::LAWN_GREEN,
            Self::CHARTREUSE,
            Self::GREEN_YELLOW,
            Self::DARK_GREEN,
            Self::FOREST_GREEN,
            Self::LIME,
            Self::LIME_GREEN,
            Self::LIGHT_GREEN,
            Self::PALE_GREEN,
            Self::DARK_SEA_GREEN,
            Self::MEDIUM_SPRING_GREEN,
            Self::SPRING_GREEN,
            Self::SEA_GREEN,
            Self::MEDIUM_AQUA_MARINE,
            Self::MEDIUM_SEA_GREEN,
            Self::LIGHT_SEA_GREEN,
            Self::DARK_SLATE_GRAY,
            Self::TEAL,
            Self::DARK_CYAN,
            Self::AQUA,
            Self::CYAN,
            Self::LIGHT_CYAN,
            Self::DARK_TURQUOISE,
            Self::TURQUOISE,
            Self::MEDIUM_TURQUOISE,
            Self::PALE_TURQUOISE,
            Self::AQUA_MARINE,
            Self::POWDER_BLUE,
            Self::CADET_BLUE,
            Self::STEEL_BLUE,
            Self::CORN_FLOWER_BLUE,
            Self::DEEP_SKY_BLUE,
            Self::DODGER_BLUE,
            Self::LIGHT_BLUE,
            Self::SKY_BLUE,
            Self::LIGHT_SKY_BLUE,
            Self::MIDNIGHT_BLUE,
            Self::NAVY,
            Self::DARK_BLUE,
            Self::MEDIUM_BLUE,
            Self::ROYAL_BLUE,
            Self::BLUE_VIOLET,
            Self::INDIGO,
            Self::DARK_SLATE_BLUE,
            Self::SLATE_BLUE,
            Self::MEDIUM_SLATE_BLUE,
            Self::MEDIUM_PURPLE,
            Self::DARK_MAGENTA,
            Self::DARK_VIOLET,
            Self::DARK_ORCHID,
            Self::MEDIUM_ORCHID,
            Self::PURPLE,
            Self::THISTLE,
            Self::PLUM,
            Self::VIOLET,
            Self::MAGENTA,
            Self::ORCHID,
            Self::MEDIUM_VIOLET_RED,
            Self::PALE_VIOLET_RED,
            Self::DEEP_PINK,
            Self::HOT_PINK,
            Self::LIGHT_PINK,
            Self::PINK,
            Self::ANTIQUE_WHITE,
            Self::BEIGE,
            Self::BISQUE,
            Self::BLANCHED_ALMOND,
            Self::WHEAT,
            Self::CORN_SILK,
            Self::LEMON_CHIFFON,
            Self::LIGHT_GOLDEN_ROD_YELLOW,
            Self::LIGHT_YELLOW,
            Self::SADDLE_BROWN,
            Self::SIENNA,
            Self::CHOCOLATE,
            Self::PERU,
            Self::SANDY_BROWN,
            Self::BURLY_WOOD,
            Self::TAN,
            Self::ROSY_BROWN,
            Self::MOCCASIN,
            Self::NAVAJO_WHITE,
            Self::PEACH_PUFF,
            Self::MISTY_ROSE,
            Self::LAVENDER_BLUSH,
            Self::LINEN,
            Self::OLD_LACE,
            Self::PAPAYA_WHIP,
            Self::SEA_SHELL,
            Self::MINT_CREAM,
            Self::SLATE_GRAY,
            Self::LIGHT_SLATE_GRAY,
            Self::LIGHT_STEEL_BLUE,
            Self::LAVENDER,
            Self::FLORAL_WHITE,
            Self::ALICE_BLUE,
            Self::GHOST_WHITE,
            Self::HONEYDEW,
            Self::IVORY,
            Self::AZURE,
            Self::SNOW,
            Self::DIM_GRAY,
            Self::GRAY,
            Self::DARK_GRAY,
            Self::SILVER,
            Self::LIGHT_GRAY,
            Self::GAINSBORO,
            Self::WHITE_SMOKE,
        ];
        #[inline]
        pub const fn opaque(r: u8, g: u8, b: u8) -> Self {
            Self { r, g, b, a: 255 }
        }
        #[inline]
        pub const fn repeat(c: u8) -> Self {
            Self { r: c, g: c, b: c, a: c }
        }
        #[inline]
        pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self { r, g, b, a }
        }
        #[must_use]
        #[inline]
        pub fn srgb_to_linear(self) -> Self {
            let r = ((self.r as f32 / 255.0).powf(2.2).clamp(0.0, 1.0) * 255.0) as u8;
            let g = ((self.g as f32 / 255.0).powf(2.2).clamp(0.0, 1.0) * 255.0) as u8;
            let b = ((self.b as f32 / 255.0).powf(2.2).clamp(0.0, 1.0) * 255.0) as u8;
            Self::from_rgba(r, g, b, self.a)
        }
        #[must_use]
        #[inline]
        pub fn srgb_to_linear_f32(self) -> Vector4<f32> {
            let r = (self.r as f32 / 255.0).powf(2.2).clamp(0.0, 1.0);
            let g = (self.g as f32 / 255.0).powf(2.2).clamp(0.0, 1.0);
            let b = (self.b as f32 / 255.0).powf(2.2).clamp(0.0, 1.0);
            Vector4::new(r, g, b, self.a as f32 / 255.0)
        }
        #[must_use]
        #[inline]
        pub fn linear_to_srgb(self) -> Self {
            let r = ((self.r as f32 / 255.0).powf(1.0 / 2.2).clamp(0.0, 1.0) * 255.0)
                as u8;
            let g = ((self.g as f32 / 255.0).powf(1.0 / 2.2).clamp(0.0, 1.0) * 255.0)
                as u8;
            let b = ((self.b as f32 / 255.0).powf(1.0 / 2.2).clamp(0.0, 1.0) * 255.0)
                as u8;
            Self::from_rgba(r, g, b, self.a)
        }
        #[inline]
        pub fn as_frgba(self) -> Vector4<f32> {
            Vector4::new(
                f32::from(self.r) / 255.0,
                f32::from(self.g) / 255.0,
                f32::from(self.b) / 255.0,
                f32::from(self.a) / 255.0,
            )
        }
        #[inline]
        pub fn as_frgb(self) -> Vector3<f32> {
            Vector3::new(
                f32::from(self.r) / 255.0,
                f32::from(self.g) / 255.0,
                f32::from(self.b) / 255.0,
            )
        }
        #[inline]
        pub fn to_opaque(self) -> Self {
            Self {
                r: self.r,
                g: self.g,
                b: self.b,
                a: 255,
            }
        }
        #[inline]
        pub fn lerp(self, other: Self, t: f32) -> Self {
            let dr = (t * (i32::from(other.r) - i32::from(self.r)) as f32) as i32;
            let dg = (t * (i32::from(other.g) - i32::from(self.g)) as f32) as i32;
            let db = (t * (i32::from(other.b) - i32::from(self.b)) as f32) as i32;
            let da = (t * (i32::from(other.a) - i32::from(self.a)) as f32) as i32;
            let red = (i32::from(self.r) + dr) as u8;
            let green = (i32::from(self.g) + dg) as u8;
            let blue = (i32::from(self.b) + db) as u8;
            let alpha = (i32::from(self.a) + da) as u8;
            Self {
                r: red,
                g: green,
                b: blue,
                a: alpha,
            }
        }
        #[inline]
        pub fn with_new_alpha(self, a: u8) -> Self {
            Self {
                r: self.r,
                g: self.g,
                b: self.b,
                a,
            }
        }
    }
    impl Add for Color {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                r: self.r.saturating_add(rhs.r),
                g: self.g.saturating_add(rhs.g),
                b: self.b.saturating_add(rhs.b),
                a: self.a.saturating_add(rhs.a),
            }
        }
    }
    impl AddAssign for Color {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Sub for Color {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self {
                r: self.r.saturating_sub(rhs.r),
                g: self.g.saturating_sub(rhs.g),
                b: self.b.saturating_sub(rhs.b),
                a: self.a.saturating_sub(rhs.a),
            }
        }
    }
    impl SubAssign for Color {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
}
pub mod color_gradient {
    use crate::{
        color::Color, reflect::prelude::*, visitor::{Visit, VisitResult, Visitor},
    };
    use std::cmp::Ordering;
    pub struct GradientPoint {
        location: f32,
        color: Color,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for GradientPoint {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for GradientPoint {
        #[inline]
        fn eq(&self, other: &GradientPoint) -> bool {
            self.location == other.location && self.color == other.color
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for GradientPoint {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "GradientPoint",
                "location",
                &self.location,
                "color",
                &&self.color,
            )
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for GradientPoint
    where
        f32: Visit,
        Color: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            if let Err(err) = self.location.visit("Location", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.color.visit("Color", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for GradientPoint
    where
        Self: 'static,
        f32: Reflect,
        Color: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\color_gradient.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "location",
                        display_name: "Location",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.location,
                        reflect_value: &self.location,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "color",
                        display_name: "Color",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.color,
                        reflect_value: &self.color,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<Color>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[&self.location as &dyn Reflect, &self.color as &dyn Reflect])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(
                &mut [
                    &mut self.location as &mut dyn Reflect,
                    &mut self.color as &mut dyn Reflect,
                ],
            )
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "location" => Some(&self.location as &dyn Reflect),
                    "color" => Some(&self.color as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "location" => Some(&mut self.location as &mut dyn Reflect),
                    "color" => Some(&mut self.color as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(missing_docs)]
    impl GradientPoint {
        pub const LOCATION: &'static str = "location";
        pub const COLOR: &'static str = "color";
    }
    impl GradientPoint {
        #[inline]
        pub fn new(location: f32, color: Color) -> Self {
            Self { location, color }
        }
        #[inline]
        pub fn color(&self) -> Color {
            self.color
        }
        #[inline]
        pub fn location(&self) -> f32 {
            self.location
        }
    }
    impl Default for GradientPoint {
        fn default() -> Self {
            Self {
                location: 0.0,
                color: Color::default(),
            }
        }
    }
    impl Clone for GradientPoint {
        fn clone(&self) -> Self {
            Self {
                location: self.location,
                color: self.color,
            }
        }
    }
    pub struct ColorGradient {
        points: Vec<GradientPoint>,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ColorGradient {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ColorGradient {
        #[inline]
        fn eq(&self, other: &ColorGradient) -> bool {
            self.points == other.points
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ColorGradient {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ColorGradient",
                "points",
                &&self.points,
            )
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for ColorGradient
    where
        Vec<GradientPoint>: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            if let Err(err) = self.points.visit("Points", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for ColorGradient
    where
        Self: 'static,
        Vec<GradientPoint>: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\color_gradient.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "points",
                        display_name: "Points",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.points,
                        reflect_value: &self.points,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<Vec<GradientPoint>>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[&self.points as &dyn Reflect])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(&mut [&mut self.points as &mut dyn Reflect])
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "points" => Some(&self.points as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "points" => Some(&mut self.points as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(missing_docs)]
    impl ColorGradient {
        pub const POINTS: &'static str = "points";
    }
    impl Clone for ColorGradient {
        fn clone(&self) -> Self {
            Self {
                points: self.points.clone(),
            }
        }
    }
    impl Default for ColorGradient {
        fn default() -> Self {
            Self::new()
        }
    }
    impl ColorGradient {
        pub const STUB_COLOR: Color = Color::WHITE;
        pub fn new() -> Self {
            Self { points: Vec::new() }
        }
        pub fn add_point(&mut self, pt: GradientPoint) {
            self.points.push(pt);
            self.points
                .sort_by(|a, b| {
                    a.location.partial_cmp(&b.location).unwrap_or(Ordering::Equal)
                });
        }
        pub fn get_color(&self, location: f32) -> Color {
            if self.points.is_empty() {
                return Self::STUB_COLOR;
            } else if self.points.len() == 1 {
                return self.points.first().unwrap().color;
            } else if self.points.len() == 2 {
                let pt_a = self.points.first().unwrap();
                let pt_b = self.points.get(1).unwrap();
                if location >= pt_a.location && location <= pt_b.location {
                    let span = pt_b.location - pt_a.location;
                    let t = (location - pt_a.location) / span;
                    return pt_a.color.lerp(pt_b.color, t);
                } else if location < pt_a.location {
                    return pt_a.color;
                } else {
                    return pt_b.color;
                }
            }
            let first = self.points.first().unwrap();
            let last = self.points.last().unwrap();
            if location <= first.location {
                first.color
            } else if location >= last.location {
                last.color
            } else {
                let mut pt_a_index = 0;
                for (i, pt) in self.points.iter().enumerate() {
                    if location >= pt.location {
                        pt_a_index = i;
                    }
                }
                let pt_b_index = pt_a_index + 1;
                let pt_a = self.points.get(pt_a_index).unwrap();
                let pt_b = self.points.get(pt_b_index).unwrap();
                let span = pt_b.location - pt_a.location;
                let t = (location - pt_a.location) / span;
                pt_a.color.lerp(pt_b.color, t)
            }
        }
        pub fn points(&self) -> &[GradientPoint] {
            &self.points
        }
        pub fn clear(&mut self) {
            self.points.clear()
        }
    }
    pub struct ColorGradientBuilder {
        points: Vec<GradientPoint>,
    }
    #[automatically_derived]
    impl ::core::default::Default for ColorGradientBuilder {
        #[inline]
        fn default() -> ColorGradientBuilder {
            ColorGradientBuilder {
                points: ::core::default::Default::default(),
            }
        }
    }
    impl ColorGradientBuilder {
        pub fn new() -> Self {
            Self { points: Default::default() }
        }
        pub fn with_point(mut self, point: GradientPoint) -> Self {
            self.points.push(point);
            self
        }
        pub fn build(mut self) -> ColorGradient {
            self.points
                .sort_by(|a, b| {
                    a.location.partial_cmp(&b.location).unwrap_or(Ordering::Equal)
                });
            ColorGradient {
                points: self.points,
            }
        }
    }
}
pub mod io {
    use std::{io::Error, path::Path};
    pub enum FileLoadError {
        Io(std::io::Error),
        Custom(String),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FileLoadError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                FileLoadError::Io(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Io", &__self_0)
                }
                FileLoadError::Custom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Custom",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl From<std::io::Error> for FileLoadError {
        fn from(e: Error) -> Self {
            Self::Io(e)
        }
    }
    pub async fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, FileLoadError> {
        #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
        {
            use std::fs::File;
            use std::io::Read;
            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
    pub async fn exists<P: AsRef<Path>>(path: P) -> bool {
        #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
        { path.as_ref().exists() }
    }
    pub async fn is_dir<P: AsRef<Path>>(#[allow(unused)] path: P) -> bool {
        #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
        { path.as_ref().is_dir() }
    }
    pub async fn is_file<P: AsRef<Path>>(path: P) -> bool {
        #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
        { path.as_ref().is_file() }
    }
}
pub mod log {
    //! Simple logger, it writes in file and in console at the same time.
    use crate::lazy_static::lazy_static;
    use crate::parking_lot::Mutex;
    use std::fmt::{Debug, Display};
    use crate::instant::Instant;
    #[cfg(not(target_arch = "wasm32"))]
    use std::io::{self, Write};
    use std::sync::mpsc::Sender;
    use std::time::Duration;
    /// A message that could be sent by the logger to all listeners.
    pub struct LogMessage {
        /// Kind of the message: information, warning or error.
        pub kind: MessageKind,
        /// The source message without logger prefixes.
        pub content: String,
        /// Time point at which the message was recorded. It is relative to the moment when the
        /// logger was initialized.
        pub time: Duration,
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct LOG {
        __private_field: (),
    }
    #[doc(hidden)]
    static LOG: LOG = LOG { __private_field: () };
    impl ::lazy_static::__Deref for LOG {
        type Target = Mutex<Log>;
        fn deref(&self) -> &Mutex<Log> {
            #[inline(always)]
            fn __static_ref_initialize() -> Mutex<Log> {
                Mutex::new(Log {
                    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
                    file: std::fs::File::create("fyrox.log").unwrap(),
                    verbosity: MessageKind::Information,
                    listeners: Default::default(),
                    time_origin: Instant::now(),
                })
            }
            #[inline(always)]
            fn __stability() -> &'static Mutex<Log> {
                static LAZY: ::lazy_static::lazy::Lazy<Mutex<Log>> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for LOG {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    /// A kind of message.
    #[repr(u32)]
    pub enum MessageKind {
        /// Some useful information.
        Information = 0,
        /// A warning.
        Warning = 1,
        /// An error of some kind.
        Error = 2,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MessageKind {}
    #[automatically_derived]
    impl ::core::clone::Clone for MessageKind {
        #[inline]
        fn clone(&self) -> MessageKind {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for MessageKind {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MessageKind,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MessageKind {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MessageKind {
        #[inline]
        fn eq(&self, other: &MessageKind) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MessageKind {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for MessageKind {
        #[inline]
        fn cmp(&self, other: &MessageKind) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MessageKind {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl MessageKind {
        fn as_str(self) -> &'static str {
            match self {
                MessageKind::Information => "[INFO]: ",
                MessageKind::Warning => "[WARNING]: ",
                MessageKind::Error => "[ERROR]: ",
            }
        }
    }
    /// See module docs.
    pub struct Log {
        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
        file: std::fs::File,
        verbosity: MessageKind,
        listeners: Vec<Sender<LogMessage>>,
        time_origin: Instant,
    }
    impl Log {
        fn write_internal<S>(&mut self, kind: MessageKind, message: S)
        where
            S: AsRef<str>,
        {
            let mut msg = message.as_ref().to_owned();
            if kind as u32 >= self.verbosity as u32 {
                self.listeners
                    .retain(|listener| {
                        listener
                            .send(LogMessage {
                                kind,
                                content: msg.clone(),
                                time: Instant::now() - self.time_origin,
                            })
                            .is_ok()
                    });
                msg.insert_str(0, kind.as_str());
                #[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
                {
                    let _ = io::stdout().write_all(msg.as_bytes());
                    let _ = self.file.write_all(msg.as_bytes());
                }
            }
        }
        fn writeln_internal<S>(&mut self, kind: MessageKind, message: S)
        where
            S: AsRef<str>,
        {
            let mut msg = message.as_ref().to_owned();
            msg.push('\n');
            self.write_internal(kind, msg)
        }
        /// Writes string into console and into file.
        pub fn write<S>(kind: MessageKind, msg: S)
        where
            S: AsRef<str>,
        {
            LOG.lock().write_internal(kind, msg);
        }
        /// Writes line into console and into file.
        pub fn writeln<S>(kind: MessageKind, msg: S)
        where
            S: AsRef<str>,
        {
            LOG.lock().writeln_internal(kind, msg);
        }
        /// Writes information message.
        pub fn info<S>(msg: S)
        where
            S: AsRef<str>,
        {
            Self::writeln(MessageKind::Information, msg)
        }
        /// Writes warning message.
        pub fn warn<S>(msg: S)
        where
            S: AsRef<str>,
        {
            Self::writeln(MessageKind::Warning, msg)
        }
        /// Writes error message.
        pub fn err<S>(msg: S)
        where
            S: AsRef<str>,
        {
            Self::writeln(MessageKind::Error, msg)
        }
        /// Sets verbosity level.
        pub fn set_verbosity(kind: MessageKind) {
            LOG.lock().verbosity = kind;
        }
        /// Adds a listener that will receive a copy of every message passed into the log.
        pub fn add_listener(listener: Sender<LogMessage>) {
            LOG.lock().listeners.push(listener)
        }
        /// Allows you to verify that the result of operation is Ok, or print the error in the log.
        ///
        /// # Use cases
        ///
        /// Typical use case for this method is that when you _can_ ignore errors, but want them to
        /// be in the log.
        pub fn verify<T, E>(result: Result<T, E>)
        where
            E: Debug,
        {
            if let Err(e) = result {
                Self::writeln(
                    MessageKind::Error,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("Operation failed! Reason: {0:?}", e),
                        );
                        res
                    },
                );
            }
        }
        /// Allows you to verify that the result of operation is Ok, or print the error in the log.
        ///
        /// # Use cases
        ///
        /// Typical use case for this method is that when you _can_ ignore errors, but want them to
        /// be in the log.
        pub fn verify_message<S, T, E>(result: Result<T, E>, msg: S)
        where
            E: Debug,
            S: Display,
        {
            if let Err(e) = result {
                Self::writeln(
                    MessageKind::Error,
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}. Reason: {1:?}", msg, e),
                        );
                        res
                    },
                );
            }
        }
    }
}
pub mod math {
    #![allow(clippy::many_single_char_names)]
    pub use fyrox_math::*;
    use crate::math::curve::Curve;
    use crate::math::curve::CurveKey;
    use crate::math::curve::CurveKeyKind;
    use crate::Uuid;
    use crate::{
        algebra::Scalar,
        math::{aabb::AxisAlignedBoundingBox, frustum::Frustum, plane::Plane},
        num_traits::NumAssign, reflect::prelude::*, visitor::prelude::*,
    };
    use fyrox_core_derive::{impl_reflect, impl_visit};
    use std::fmt::Debug;
    #[allow(warnings)]
    impl<T: Debug> Reflect for Rect<T>
    where
        Self: 'static,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\math\\mod.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(&[])
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(&mut [])
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    _ => None,
                }
            };
            func(value)
        }
    }
    impl<T> Visit for Rect<T>
    where
        T: NumAssign + Scalar + Visit + PartialOrd + Copy + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.position.x.visit("X", &mut region)?;
            self.position.y.visit("Y", &mut region)?;
            self.size.x.visit("W", &mut region)?;
            self.size.y.visit("H", &mut region)?;
            Ok(())
        }
    }
    impl Visit for TriangleDefinition {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.0[0].visit("A", &mut region)?;
            self.0[1].visit("B", &mut region)?;
            self.0[2].visit("C", &mut region)?;
            Ok(())
        }
    }
    impl Visit for AxisAlignedBoundingBox {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.min.visit("Min", &mut region)?;
            self.max.visit("Max", &mut region)?;
            Ok(())
        }
    }
    impl Visit for Frustum {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.planes[0].visit("Left", &mut region)?;
            self.planes[1].visit("Right", &mut region)?;
            self.planes[2].visit("Top", &mut region)?;
            self.planes[3].visit("Bottom", &mut region)?;
            self.planes[4].visit("Far", &mut region)?;
            self.planes[5].visit("Near", &mut region)?;
            Ok(())
        }
    }
    impl Visit for Plane {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.normal.visit("Normal", &mut region)?;
            self.d.visit("D", &mut region)?;
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for TriangleDefinition
    where
        Self: 'static,
        [u32; 3]: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\math\\mod.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "0",
                        display_name: "0",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.0,
                        reflect_value: &self.0,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<[u32; 3]>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[&self.0 as &dyn Reflect])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(&mut [&mut self.0 as &mut dyn Reflect])
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "0" => Some(&self.0 as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "0" => Some(&mut self.0 as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for SmoothAngle
    where
        f32: Visit,
        f32: Visit,
        f32: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            if let Err(err) = self.angle.visit("Angle", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.target.visit("Target", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.speed.visit("Speed", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for SmoothAngle
    where
        Self: 'static,
        f32: Reflect,
        f32: Reflect,
        f32: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\math\\mod.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "angle",
                        display_name: "Angle",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.angle,
                        reflect_value: &self.angle,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "target",
                        display_name: "Target",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.target,
                        reflect_value: &self.target,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "speed",
                        display_name: "Speed",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.speed,
                        reflect_value: &self.speed,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<f32>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(
                &[
                    &self.angle as &dyn Reflect,
                    &self.target as &dyn Reflect,
                    &self.speed as &dyn Reflect,
                ],
            )
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(
                &mut [
                    &mut self.angle as &mut dyn Reflect,
                    &mut self.target as &mut dyn Reflect,
                    &mut self.speed as &mut dyn Reflect,
                ],
            )
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "angle" => Some(&self.angle as &dyn Reflect),
                    "target" => Some(&self.target as &dyn Reflect),
                    "speed" => Some(&self.speed as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "angle" => Some(&mut self.angle as &mut dyn Reflect),
                    "target" => Some(&mut self.target as &mut dyn Reflect),
                    "speed" => Some(&mut self.speed as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(warnings)]
    impl Reflect for CurveKeyKind
    where
        Self: 'static,
        f32: Reflect,
        f32: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\math\\mod.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            match self {
                CurveKeyKind::Constant => func(&[]),
                CurveKeyKind::Linear => func(&[]),
                CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                    func(
                        &[
                            FieldInfo {
                                owner_type_id: std::any::TypeId::of::<Self>(),
                                name: "Cubic@left_tangent",
                                display_name: "Left Tangent",
                                doc: "",
                                read_only: false,
                                immutable_collection: false,
                                min_value: None,
                                max_value: None,
                                value: left_tangent,
                                reflect_value: left_tangent,
                                step: None,
                                precision: None,
                                description: "",
                                type_name: std::any::type_name::<f32>(),
                            },
                            FieldInfo {
                                owner_type_id: std::any::TypeId::of::<Self>(),
                                name: "Cubic@right_tangent",
                                display_name: "Right Tangent",
                                doc: "",
                                read_only: false,
                                immutable_collection: false,
                                min_value: None,
                                max_value: None,
                                value: right_tangent,
                                reflect_value: right_tangent,
                                step: None,
                                precision: None,
                                description: "",
                                type_name: std::any::type_name::<f32>(),
                            },
                        ],
                    )
                }
                _ => func(&[]),
            }
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            match self {
                CurveKeyKind::Constant => func(&[]),
                CurveKeyKind::Linear => func(&[]),
                CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                    func(&[left_tangent as &dyn Reflect, right_tangent as &dyn Reflect])
                }
                _ => func(&[]),
            }
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            match self {
                CurveKeyKind::Constant => func(&mut []),
                CurveKeyKind::Linear => func(&mut []),
                CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                    func(
                        &mut [
                            left_tangent as &mut dyn Reflect,
                            right_tangent as &mut dyn Reflect,
                        ],
                    )
                }
                _ => func(&mut []),
            }
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "Cubic@left_tangent" => {
                        match self {
                            CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                                Some(left_tangent as &dyn Reflect)
                            }
                            _ => None,
                        }
                    }
                    "Cubic@right_tangent" => {
                        match self {
                            CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                                Some(right_tangent as &dyn Reflect)
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "Cubic@left_tangent" => {
                        match self {
                            CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                                Some(left_tangent as &mut dyn Reflect)
                            }
                            _ => None,
                        }
                    }
                    "Cubic@right_tangent" => {
                        match self {
                            CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                                Some(right_tangent as &mut dyn Reflect)
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for CurveKeyKind
    where
        f32: Visit,
        f32: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let mut id = id(self);
            if let Err(err) = id.visit("Id", &mut region) {
                return Err(err);
            }
            if region.is_reading() {
                *self = match from_id(id) {
                    Ok(x) => x,
                    Err(s) => return Err(s.into()),
                };
            }
            match self {
                CurveKeyKind::Constant => {}
                CurveKeyKind::Linear => {}
                CurveKeyKind::Cubic { left_tangent, right_tangent } => {
                    if let Err(err) = left_tangent.visit("LeftTangent", &mut region) {
                        return Err(err);
                    }
                    if let Err(err) = right_tangent.visit("RightTangent", &mut region) {
                        return Err(err);
                    }
                }
            }
            return Ok(());
            fn id(me: &CurveKeyKind) -> u32 {
                match me {
                    CurveKeyKind::Constant => 0u32,
                    CurveKeyKind::Linear => 1u32,
                    CurveKeyKind::Cubic { .. } => 2u32,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("Unable to get ID from enum variant"),
                            ),
                        );
                    }
                }
            }
            fn from_id(id: u32) -> std::result::Result<CurveKeyKind, String> {
                match id {
                    id if id == 0u32 => Ok(CurveKeyKind::Constant),
                    id if id == 1u32 => Ok(CurveKeyKind::Linear),
                    id if id == 2u32 => {
                        Ok(CurveKeyKind::Cubic {
                            left_tangent: Default::default(),
                            right_tangent: Default::default(),
                        })
                    }
                    _ => {
                        Err({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Unknown ID for type `{0}`: `{1}`",
                                    "CurveKeyKind",
                                    id,
                                ),
                            );
                            res
                        })
                    }
                }
            }
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for CurveKey
    where
        Uuid: Visit,
        f32: Visit,
        f32: Visit,
        CurveKeyKind: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            if let Err(err) = self.id.visit("Id", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.location.visit("Location", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.value.visit("Value", &mut region) {
                return Err(err);
            }
            if let Err(err) = self.kind.visit("Kind", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
    #[allow(warnings)]
    impl Reflect for Curve
    where
        Self: 'static,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\math\\mod.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(&[])
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(&mut [])
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = { None };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = { None };
            func(value)
        }
    }
    #[allow(clippy::question_mark)]
    impl Visit for Curve
    where
        Uuid: Visit,
        String: Visit,
        Vec<CurveKey>: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = match visitor.enter_region(name) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            self.id.visit("Id", &mut region).ok();
            self.name.visit("Name", &mut region).ok();
            if let Err(err) = self.keys.visit("Keys", &mut region) {
                return Err(err);
            }
            Ok(())
        }
    }
}
pub mod net {
    use crate::log::Log;
    use byteorder::{LittleEndian, WriteBytesExt};
    use serde::{de::DeserializeOwned, Serialize};
    use std::{
        io::{self, ErrorKind, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    };
    pub struct NetListener {
        listener: TcpListener,
    }
    impl NetListener {
        pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
            let listener = TcpListener::bind(addr)?;
            listener.set_nonblocking(true)?;
            Ok(Self { listener })
        }
        pub fn local_address(&self) -> io::Result<SocketAddr> {
            self.listener.local_addr()
        }
        pub fn accept_connections(&self) -> Vec<NetStream> {
            let mut streams = Vec::new();
            while let Ok(result) = self.listener.accept() {
                streams.push(NetStream::from_inner(result.0).unwrap())
            }
            streams
        }
    }
    pub struct NetStream {
        stream: TcpStream,
        rx_buffer: Vec<u8>,
        tx_buffer: Vec<u8>,
    }
    impl NetStream {
        pub fn from_inner(stream: TcpStream) -> io::Result<Self> {
            stream.set_nonblocking(true)?;
            stream.set_nodelay(true)?;
            Ok(Self {
                stream,
                rx_buffer: Default::default(),
                tx_buffer: Default::default(),
            })
        }
        pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
            Self::from_inner(TcpStream::connect(addr)?)
        }
        pub fn send_message<T>(&mut self, data: &T) -> io::Result<()>
        where
            T: Serialize,
        {
            self.tx_buffer.clear();
            if self.tx_buffer.capacity() < std::mem::size_of::<T>() {
                self.tx_buffer.reserve(std::mem::size_of::<T>());
            }
            bincode::serialize_into(&mut self.tx_buffer, data)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            self.stream.write_u32::<LittleEndian>(self.tx_buffer.len() as u32)?;
            self.stream.write_all(&self.tx_buffer)?;
            Ok(())
        }
        pub fn peer_address(&self) -> io::Result<SocketAddr> {
            self.stream.peer_addr()
        }
        pub fn string_peer_address(&self) -> String {
            self.peer_address()
                .map(|addr| addr.to_string())
                .unwrap_or_else(|_| "Unknown".into())
        }
        fn next_message<M>(&mut self) -> Option<M>
        where
            M: DeserializeOwned,
        {
            if self.rx_buffer.len() < 4 {
                return None;
            }
            let length = u32::from_le_bytes([
                self.rx_buffer[0],
                self.rx_buffer[1],
                self.rx_buffer[2],
                self.rx_buffer[3],
            ]) as usize;
            let end = 4 + length;
            if let Some(data) = self.rx_buffer.as_slice().get(4..end) {
                let message = match bincode::deserialize::<M>(data) {
                    Ok(message) => Some(message),
                    Err(err) => {
                        Log::err({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to parse a network message of {0} bytes long. Reason: {1:?}",
                                    length,
                                    err,
                                ),
                            );
                            res
                        });
                        None
                    }
                };
                self.rx_buffer.drain(..end);
                message
            } else {
                None
            }
        }
        pub fn process_input<M>(&mut self, mut func: impl FnMut(M))
        where
            M: DeserializeOwned,
        {
            loop {
                let mut bytes = [0; 8192];
                match self.stream.read(&mut bytes) {
                    Ok(bytes_count) => {
                        if bytes_count == 0 {
                            break;
                        } else {
                            self.rx_buffer.extend(&bytes[..bytes_count])
                        }
                    }
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::WouldBlock => {
                                break;
                            }
                            ErrorKind::Interrupted => {}
                            _ => {
                                Log::err({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "An error occurred when reading data from socket: {0}",
                                            err,
                                        ),
                                    );
                                    res
                                });
                                self.rx_buffer.clear();
                                return;
                            }
                        }
                    }
                }
            }
            while let Some(message) = self.next_message() {
                func(message)
            }
        }
    }
}
pub mod numeric_range {
    use crate::num_traits::Num;
    use rand::{distributions::uniform::SampleUniform, Rng};
    use std::ops::Range;
    fn min<T>(a: T, b: T) -> T
    where
        T: PartialOrd,
    {
        if a > b { b } else { a }
    }
    fn max<T>(a: T, b: T) -> T
    where
        T: PartialOrd,
    {
        if a > b { a } else { b }
    }
    pub trait RangeExt<T>
    where
        T: Num + PartialOrd + SampleUniform + Copy,
    {
        fn random<R: Rng>(&self, rng: &mut R) -> T;
        fn clamp_value(&self, value: &mut T) -> T;
    }
    impl<T: Num + PartialOrd + SampleUniform + Copy> RangeExt<T> for Range<T> {
        #[inline]
        fn random<R: Rng>(&self, rng: &mut R) -> T {
            let start = min(self.start, self.end);
            let end = max(self.start, self.end);
            if start == end { start } else { rng.gen_range(Range { start, end }) }
        }
        #[inline]
        fn clamp_value(&self, value: &mut T) -> T {
            let start = min(self.start, self.end);
            let end = max(self.start, self.end);
            if *value < start { start } else if *value > end { end } else { *value }
        }
    }
}
pub mod pool {
    //! A generational arena - a contiguous growable array type which allows removing
    //! from the middle without shifting and therefore without invalidating other indices.
    //!
    //! Pool is a contiguous block of memory with fixed-size entries, each entry can be
    //! either vacant or occupied. When you put an object into the pool you get a handle to
    //! that object. You can use that handle later on to borrow a reference to an object.
    //! A handle can point to some object or be invalid, this may look similar to raw
    //! pointers, but there is two major differences:
    //!
    //! 1) We can check if a handle is valid before accessing the object it might point to.
    //! 2) We can ensure the handle we're using is still valid for the object it points to
    //! to make sure it hasn't been replaced with a different object on the same position.
    //! Each handle stores a special field called generation which is shared across the entry
    //! and the handle, so the handle is valid if these fields are the same on both the entry
    //! and the handle. This protects from situations where you have a handle that has
    //! a valid index of a record, but the payload in this record has been replaced.
    //!
    //! Contiguous memory block increases efficiency of memory operations - the CPU will
    //! load portions of data into its cache piece by piece, it will be free from any
    //! indirections that might cause cache invalidation. This is the so called cache
    //! friendliness.
    use crate::{reflect::prelude::*, visitor::prelude::*, ComponentProvider};
    use std::{
        any::{Any, TypeId},
        fmt::Debug, future::Future, marker::PhantomData, ops::{Index, IndexMut},
        sync::atomic::{self, AtomicIsize},
    };
    pub mod handle {
        use crate::{
            combine_uuids, pool::INVALID_GENERATION, reflect::prelude::*, uuid_provider,
            visitor::prelude::*, TypeUuidProvider,
        };
        use serde::{Deserialize, Serialize};
        use std::{
            cmp::Ordering, fmt::{Debug, Display, Formatter},
            hash::{Hash, Hasher},
            marker::PhantomData, sync::atomic::{self, AtomicUsize},
        };
        use uuid::Uuid;
        /// Handle is some sort of non-owning reference to content in a pool. It stores
        /// index of object and additional information that allows to ensure that handle
        /// is still valid (points to the same object as when handle was created).
        pub struct Handle<T> {
            /// Index of object in pool.
            #[reflect(read_only, description = "Index of an object in a pool.")]
            pub(super) index: u32,
            /// Generation number, if it is same as generation of pool record at
            /// index of handle then this is valid handle.
            #[reflect(read_only, description = "Generation of an object in a pool.")]
            pub(super) generation: u32,
            /// Type holder.
            #[reflect(hidden)]
            #[serde(skip)]
            pub(super) type_marker: PhantomData<T>,
        }
        #[allow(warnings)]
        impl<T> Reflect for Handle<T>
        where
            Self: 'static,
            u32: Reflect,
            u32: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\pool\\handle.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                " Handle is some sort of non-owning reference to content in a pool. It stores index of object and additional information that allows to ensure that handle is still valid (points to the same object as when handle was created)."
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(
                    &[
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "index",
                            display_name: "Index",
                            doc: " Index of object in pool.",
                            read_only: true,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.index,
                            reflect_value: &self.index,
                            step: None,
                            precision: None,
                            description: "Index of an object in a pool.",
                            type_name: std::any::type_name::<u32>(),
                        },
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "generation",
                            display_name: "Generation",
                            doc: " Generation number, if it is same as generation of pool record at index of handle then this is valid handle.",
                            read_only: true,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.generation,
                            reflect_value: &self.generation,
                            step: None,
                            precision: None,
                            description: "Generation of an object in a pool.",
                            type_name: std::any::type_name::<u32>(),
                        },
                    ],
                )
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[&self.index as &dyn Reflect, &self.generation as &dyn Reflect])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(
                    &mut [
                        &mut self.index as &mut dyn Reflect,
                        &mut self.generation as &mut dyn Reflect,
                    ],
                )
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "index" => Some(&self.index as &dyn Reflect),
                        "generation" => Some(&self.generation as &dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "index" => Some(&mut self.index as &mut dyn Reflect),
                        "generation" => Some(&mut self.generation as &mut dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(missing_docs)]
        impl<T> Handle<T> {
            pub const INDEX: &'static str = "index";
            pub const GENERATION: &'static str = "generation";
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<T> _serde::Serialize for Handle<T> {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Handle",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "index",
                        &self.index,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "generation",
                        &self.generation,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, T> _serde::Deserialize<'de> for Handle<T> {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "index" => _serde::__private::Ok(__Field::__field0),
                                "generation" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"index" => _serde::__private::Ok(__Field::__field0),
                                b"generation" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, T> {
                        marker: _serde::__private::PhantomData<Handle<T>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de, T> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                        type Value = Handle<T>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Handle",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Handle with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Handle with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = _serde::__private::Default::default();
                            _serde::__private::Ok(Handle {
                                index: __field0,
                                generation: __field1,
                                type_marker: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "generation",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("index")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("generation")?
                                }
                            };
                            _serde::__private::Ok(Handle {
                                index: __field0,
                                generation: __field1,
                                type_marker: _serde::__private::Default::default(),
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["index", "generation"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Handle",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Handle<T>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl<T> Copy for Handle<T> {}
        impl<T> Eq for Handle<T> {}
        impl<T> PartialEq for Handle<T> {
            #[inline]
            fn eq(&self, other: &Handle<T>) -> bool {
                self.generation == other.generation && self.index == other.index
            }
        }
        impl<T> Hash for Handle<T> {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: Hasher,
            {
                self.index.hash(state);
                self.generation.hash(state);
            }
        }
        impl<T> Handle<T> {
            pub const NONE: Handle<T> = Handle {
                index: 0,
                generation: INVALID_GENERATION,
                type_marker: PhantomData,
            };
            #[inline(always)]
            pub fn is_none(self) -> bool {
                self.index == 0 && self.generation == INVALID_GENERATION
            }
            #[inline(always)]
            pub fn is_some(self) -> bool {
                !self.is_none()
            }
            #[inline(always)]
            pub fn index(self) -> u32 {
                self.index
            }
            #[inline(always)]
            pub fn generation(self) -> u32 {
                self.generation
            }
            #[inline(always)]
            pub fn new(index: u32, generation: u32) -> Self {
                Handle {
                    index,
                    generation,
                    type_marker: PhantomData,
                }
            }
            #[inline(always)]
            pub fn transmute<U>(&self) -> Handle<U> {
                Handle {
                    index: self.index,
                    generation: self.generation,
                    type_marker: Default::default(),
                }
            }
            #[inline(always)]
            pub fn decode_from_u128(num: u128) -> Self {
                Self {
                    index: num as u32,
                    generation: (num >> 32) as u32,
                    type_marker: Default::default(),
                }
            }
            #[inline(always)]
            pub fn encode_to_u128(&self) -> u128 {
                (self.index as u128) | ((self.generation as u128) << 32)
            }
        }
        impl<T> TypeUuidProvider for Handle<T>
        where
            T: TypeUuidProvider,
        {
            #[inline]
            fn type_uuid() -> Uuid {
                combine_uuids(
                    {
                        const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                            "30c0668d-7a2c-47e6-8c7b-208fdcc905a1",
                        ) {
                            ::uuid::__macro_support::Ok(u) => u,
                            ::uuid::__macro_support::Err(_) => {
                                ::std::rt::begin_panic("invalid UUID");
                            }
                        };
                        OUTPUT
                    },
                    T::type_uuid(),
                )
            }
        }
        impl<T> PartialOrd for Handle<T> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<T> Ord for Handle<T> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.index.cmp(&other.index)
            }
        }
        unsafe impl<T> Send for Handle<T> {}
        unsafe impl<T> Sync for Handle<T> {}
        impl<T> Display for Handle<T> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}:{1}", self.index, self.generation))
            }
        }
        /// Atomic handle.
        pub struct AtomicHandle(AtomicUsize);
        impl Clone for AtomicHandle {
            #[inline]
            fn clone(&self) -> Self {
                Self(AtomicUsize::new(self.0.load(atomic::Ordering::Relaxed)))
            }
        }
        impl Default for AtomicHandle {
            #[inline]
            fn default() -> Self {
                Self::none()
            }
        }
        impl AtomicHandle {
            #[inline]
            pub fn none() -> Self {
                Self(AtomicUsize::new(0))
            }
            #[inline]
            pub fn new(index: u32, generation: u32) -> Self {
                let handle = Self(AtomicUsize::new(0));
                handle.set(index, generation);
                handle
            }
            #[inline]
            pub fn set(&self, index: u32, generation: u32) {
                let index = (index as usize) << (usize::BITS / 2) >> (usize::BITS / 2);
                let generation = (generation as usize) << (usize::BITS / 2);
                self.0.store(index | generation, atomic::Ordering::Relaxed);
            }
            #[inline]
            pub fn set_from_handle<T>(&self, handle: Handle<T>) {
                self.set(handle.index, handle.generation)
            }
            #[inline(always)]
            pub fn is_some(&self) -> bool {
                self.generation() != INVALID_GENERATION
            }
            #[inline(always)]
            pub fn is_none(&self) -> bool {
                !self.is_some()
            }
            #[inline]
            pub fn index(&self) -> u32 {
                let bytes = self.0.load(atomic::Ordering::Relaxed);
                ((bytes << (usize::BITS / 2)) >> (usize::BITS / 2)) as u32
            }
            #[inline]
            pub fn generation(&self) -> u32 {
                let bytes = self.0.load(atomic::Ordering::Relaxed);
                (bytes >> (usize::BITS / 2)) as u32
            }
        }
        impl Display for AtomicHandle {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}:{1}", self.index(), self.generation()))
            }
        }
        impl Debug for AtomicHandle {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(
                    format_args!("[Idx: {0}; Gen: {1}]", self.index(), self.generation()),
                )
            }
        }
        /// Type-erased handle.
        pub struct ErasedHandle {
            /// Index of object in pool.
            #[reflect(read_only)]
            index: u32,
            /// Generation number, if it is same as generation of pool record at
            /// index of handle then this is valid handle.
            #[reflect(read_only)]
            generation: u32,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ErasedHandle {}
        #[automatically_derived]
        impl ::core::clone::Clone for ErasedHandle {
            #[inline]
            fn clone(&self) -> ErasedHandle {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ErasedHandle {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ErasedHandle",
                    "index",
                    &self.index,
                    "generation",
                    &&self.generation,
                )
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for ErasedHandle {
            #[inline]
            fn cmp(&self, other: &ErasedHandle) -> ::core::cmp::Ordering {
                match ::core::cmp::Ord::cmp(&self.index, &other.index) {
                    ::core::cmp::Ordering::Equal => {
                        ::core::cmp::Ord::cmp(&self.generation, &other.generation)
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ErasedHandle {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ErasedHandle,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match ::core::cmp::PartialOrd::partial_cmp(&self.index, &other.index) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::cmp::PartialOrd::partial_cmp(
                            &self.generation,
                            &other.generation,
                        )
                    }
                    cmp => cmp,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ErasedHandle {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ErasedHandle {
            #[inline]
            fn eq(&self, other: &ErasedHandle) -> bool {
                self.index == other.index && self.generation == other.generation
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ErasedHandle {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ErasedHandle {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.index, state);
                ::core::hash::Hash::hash(&self.generation, state)
            }
        }
        #[allow(warnings)]
        impl Reflect for ErasedHandle
        where
            Self: 'static,
            u32: Reflect,
            u32: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\pool\\handle.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                " Type-erased handle."
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(
                    &[
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "index",
                            display_name: "Index",
                            doc: " Index of object in pool.",
                            read_only: true,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.index,
                            reflect_value: &self.index,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<u32>(),
                        },
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "generation",
                            display_name: "Generation",
                            doc: " Generation number, if it is same as generation of pool record at index of handle then this is valid handle.",
                            read_only: true,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.generation,
                            reflect_value: &self.generation,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<u32>(),
                        },
                    ],
                )
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[&self.index as &dyn Reflect, &self.generation as &dyn Reflect])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(
                    &mut [
                        &mut self.index as &mut dyn Reflect,
                        &mut self.generation as &mut dyn Reflect,
                    ],
                )
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "index" => Some(&self.index as &dyn Reflect),
                        "generation" => Some(&self.generation as &dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "index" => Some(&mut self.index as &mut dyn Reflect),
                        "generation" => Some(&mut self.generation as &mut dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(missing_docs)]
        impl ErasedHandle {
            pub const INDEX: &'static str = "index";
            pub const GENERATION: &'static str = "generation";
        }
        #[allow(clippy::question_mark)]
        impl Visit for ErasedHandle
        where
            u32: Visit,
            u32: Visit,
        {
            fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
                let mut region = match visitor.enter_region(name) {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                if let Err(err) = self.index.visit("Index", &mut region) {
                    return Err(err);
                }
                if let Err(err) = self.generation.visit("Generation", &mut region) {
                    return Err(err);
                }
                Ok(())
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ErasedHandle {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "ErasedHandle",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "index",
                        &self.index,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "generation",
                        &self.generation,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ErasedHandle {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "index" => _serde::__private::Ok(__Field::__field0),
                                "generation" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"index" => _serde::__private::Ok(__Field::__field0),
                                b"generation" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ErasedHandle>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ErasedHandle;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ErasedHandle",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ErasedHandle with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ErasedHandle with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ErasedHandle {
                                index: __field0,
                                generation: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "generation",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("index")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("generation")?
                                }
                            };
                            _serde::__private::Ok(ErasedHandle {
                                index: __field0,
                                generation: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["index", "generation"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ErasedHandle",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ErasedHandle>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl crate::type_traits::TypeUuidProvider for ErasedHandle {
            fn type_uuid() -> crate::uuid::Uuid {
                {
                    const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                        "50131acc-8b3b-40b5-b495-e2c552c94db3",
                    ) {
                        ::uuid::__macro_support::Ok(u) => u,
                        ::uuid::__macro_support::Err(_) => {
                            ::std::rt::begin_panic("invalid UUID");
                        }
                    };
                    OUTPUT
                }
            }
        }
        impl Display for ErasedHandle {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}:{1}", self.index, self.generation))
            }
        }
        impl Default for ErasedHandle {
            #[inline]
            fn default() -> Self {
                Self::none()
            }
        }
        impl<T> From<ErasedHandle> for Handle<T> {
            #[inline]
            fn from(erased_handle: ErasedHandle) -> Self {
                Handle {
                    index: erased_handle.index,
                    generation: erased_handle.generation,
                    type_marker: PhantomData,
                }
            }
        }
        impl<T> From<AtomicHandle> for Handle<T> {
            #[inline]
            fn from(atomic_handle: AtomicHandle) -> Self {
                Handle {
                    index: atomic_handle.index(),
                    generation: atomic_handle.generation(),
                    type_marker: PhantomData,
                }
            }
        }
        impl<T> From<Handle<T>> for ErasedHandle {
            #[inline]
            fn from(h: Handle<T>) -> Self {
                Self {
                    index: h.index,
                    generation: h.generation,
                }
            }
        }
        impl ErasedHandle {
            #[inline]
            pub fn none() -> Self {
                Self {
                    index: 0,
                    generation: INVALID_GENERATION,
                }
            }
            #[inline]
            pub fn new(index: u32, generation: u32) -> Self {
                Self { index, generation }
            }
            #[inline(always)]
            pub fn is_some(&self) -> bool {
                self.generation != INVALID_GENERATION
            }
            #[inline(always)]
            pub fn is_none(&self) -> bool {
                !self.is_some()
            }
            #[inline(always)]
            pub fn index(self) -> u32 {
                self.index
            }
            #[inline(always)]
            pub fn generation(self) -> u32 {
                self.generation
            }
        }
        impl<T> Visit for Handle<T> {
            #[inline]
            fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
                let mut region = visitor.enter_region(name)?;
                self.index.visit("Index", &mut region)?;
                self.generation.visit("Generation", &mut region)?;
                Ok(())
            }
        }
        impl<T> Default for Handle<T> {
            #[inline]
            fn default() -> Self {
                Self::NONE
            }
        }
        impl<T> Debug for Handle<T> {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(
                    format_args!("[Idx: {0}; Gen: {1}]", self.index, self.generation),
                )
            }
        }
    }
    pub mod multiborrow {
        use super::{Handle, PayloadContainer, Pool, RefCounter};
        use crate::ComponentProvider;
        use std::cell::RefCell;
        use std::cmp::Ordering;
        use std::{
            any::TypeId, fmt::{Debug, Display, Formatter},
            marker::PhantomData, ops::{Deref, DerefMut},
            sync::atomic,
        };
        pub struct Ref<'a, 'b, T>
        where
            T: ?Sized,
        {
            data: &'a T,
            ref_counter: &'a RefCounter,
            phantom: PhantomData<&'b ()>,
        }
        impl<'a, 'b, T> Debug for Ref<'a, 'b, T>
        where
            T: ?Sized + Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&self.data, f)
            }
        }
        impl<'a, 'b, T> Deref for Ref<'a, 'b, T>
        where
            T: ?Sized,
        {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                self.data
            }
        }
        impl<'a, 'b, T> Drop for Ref<'a, 'b, T>
        where
            T: ?Sized,
        {
            fn drop(&mut self) {
                self.ref_counter.decrement();
            }
        }
        pub struct RefMut<'a, 'b, T>
        where
            T: ?Sized,
        {
            data: &'a mut T,
            ref_counter: &'a RefCounter,
            phantom: PhantomData<&'b ()>,
        }
        impl<'a, 'b, T> Debug for RefMut<'a, 'b, T>
        where
            T: ?Sized + Debug,
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&self.data, f)
            }
        }
        impl<'a, 'b, T> Deref for RefMut<'a, 'b, T>
        where
            T: ?Sized,
        {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                self.data
            }
        }
        impl<'a, 'b, T> DerefMut for RefMut<'a, 'b, T>
        where
            T: ?Sized,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.data
            }
        }
        impl<'a, 'b, T> Drop for RefMut<'a, 'b, T>
        where
            T: ?Sized,
        {
            fn drop(&mut self) {
                self.ref_counter.increment();
            }
        }
        /// Multi-borrow context allows you to get as many **unique** references to elements in
        /// a pool as you want.
        pub struct MultiBorrowContext<'a, T, P = Option<T>>
        where
            T: Sized,
            P: PayloadContainer<Element = T> + 'static,
        {
            pool: &'a mut Pool<T, P>,
            free_indices: RefCell<Vec<u32>>,
        }
        pub enum MultiBorrowError<T> {
            Empty(Handle<T>),
            NoSuchComponent(Handle<T>),
            MutablyBorrowed(Handle<T>),
            ImmutablyBorrowed(Handle<T>),
            InvalidHandleIndex(Handle<T>),
            InvalidHandleGeneration(Handle<T>),
        }
        #[automatically_derived]
        impl<T> ::core::marker::StructuralPartialEq for MultiBorrowError<T> {}
        #[automatically_derived]
        impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for MultiBorrowError<T> {
            #[inline]
            fn eq(&self, other: &MultiBorrowError<T>) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            MultiBorrowError::Empty(__self_0),
                            MultiBorrowError::Empty(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            MultiBorrowError::NoSuchComponent(__self_0),
                            MultiBorrowError::NoSuchComponent(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            MultiBorrowError::MutablyBorrowed(__self_0),
                            MultiBorrowError::MutablyBorrowed(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            MultiBorrowError::ImmutablyBorrowed(__self_0),
                            MultiBorrowError::ImmutablyBorrowed(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            MultiBorrowError::InvalidHandleIndex(__self_0),
                            MultiBorrowError::InvalidHandleIndex(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            MultiBorrowError::InvalidHandleGeneration(__self_0),
                            MultiBorrowError::InvalidHandleGeneration(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl<T> Debug for MultiBorrowError<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Display::fmt(self, f)
            }
        }
        impl<T> Display for MultiBorrowError<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Empty(handle) => {
                        f.write_fmt(
                            format_args!("There\'s no object at {0} handle.", handle),
                        )
                    }
                    Self::NoSuchComponent(handle) => {
                        f.write_fmt(
                            format_args!(
                                "An object at {0} handle does not have such component.",
                                handle,
                            ),
                        )
                    }
                    Self::MutablyBorrowed(handle) => {
                        f.write_fmt(
                            format_args!(
                                "An object at {0} handle cannot be borrowed immutably, because it is already borrowed mutably.",
                                handle,
                            ),
                        )
                    }
                    Self::ImmutablyBorrowed(handle) => {
                        f.write_fmt(
                            format_args!(
                                "An object at {0} handle cannot be borrowed mutably, because it is already borrowed immutably.",
                                handle,
                            ),
                        )
                    }
                    Self::InvalidHandleIndex(handle) => {
                        f.write_fmt(
                            format_args!(
                                "The index {0} in {1} handle is out of bounds.",
                                handle.index,
                                handle,
                            ),
                        )
                    }
                    Self::InvalidHandleGeneration(handle) => {
                        f.write_fmt(
                            format_args!(
                                "The generation {0} in {1} handle does not match the record\'s generation. It means that the object at the handle was freed and it position was taken by some other object.",
                                handle.generation,
                                handle,
                            ),
                        )
                    }
                }
            }
        }
        impl<'a, T, P> Drop for MultiBorrowContext<'a, T, P>
        where
            T: Sized,
            P: PayloadContainer<Element = T> + 'static,
        {
            fn drop(&mut self) {
                self.pool.free_stack.extend_from_slice(&self.free_indices.borrow())
            }
        }
        impl<'a, T, P> MultiBorrowContext<'a, T, P>
        where
            T: Sized,
            P: PayloadContainer<Element = T> + 'static,
        {
            #[inline]
            pub fn new(pool: &'a mut Pool<T, P>) -> Self {
                Self {
                    pool,
                    free_indices: Default::default(),
                }
            }
            #[inline]
            fn try_get_internal<'b: 'a, C, F>(
                &'b self,
                handle: Handle<T>,
                func: F,
            ) -> Result<Ref<'a, 'b, C>, MultiBorrowError<T>>
            where
                C: ?Sized,
                F: FnOnce(&T) -> Result<&C, MultiBorrowError<T>>,
            {
                let Some(record) = self.pool.records_get(handle.index) else {
                    return Err(MultiBorrowError::InvalidHandleIndex(handle));
                };
                if handle.generation != record.generation {
                    return Err(MultiBorrowError::InvalidHandleGeneration(handle));
                }
                let current_ref_count = record
                    .ref_counter
                    .0
                    .load(atomic::Ordering::Relaxed);
                if current_ref_count < 0 {
                    return Err(MultiBorrowError::MutablyBorrowed(handle));
                }
                let payload_container = unsafe { &*record.payload.0.get() };
                let Some(payload) = payload_container.as_ref() else {
                    return Err(MultiBorrowError::Empty(handle));
                };
                if let Err(ref_count) = record
                    .ref_counter
                    .0
                    .compare_exchange(
                        current_ref_count,
                        current_ref_count + 1,
                        atomic::Ordering::Acquire,
                        atomic::Ordering::Relaxed,
                    )
                {
                    if ref_count < 0 {
                        return Err(MultiBorrowError::MutablyBorrowed(handle));
                    }
                }
                Ok(Ref {
                    data: func(payload)?,
                    ref_counter: &record.ref_counter,
                    phantom: PhantomData,
                })
            }
            /// Tries to get a mutable reference to a pool element located at the given handle. The method could
            /// fail in three main reasons:
            ///
            /// 1) A reference to an element is already taken - returning multiple mutable references to the
            /// same element is forbidden by Rust safety rules.
            /// 2) You're trying to get more references that the context could handle (there is not enough space
            /// in the internal handles storage) - in this case you must increase `N`.
            /// 3) A given handle is invalid.
            #[inline]
            pub fn try_get<'b: 'a>(
                &'b self,
                handle: Handle<T>,
            ) -> Result<Ref<'a, 'b, T>, MultiBorrowError<T>> {
                self.try_get_internal(handle, |obj| Ok(obj))
            }
            #[inline]
            pub fn get<'b: 'a>(&'b self, handle: Handle<T>) -> Ref<'a, 'b, T> {
                self.try_get(handle).unwrap()
            }
            #[inline]
            fn try_get_mut_internal<'b: 'a, C, F>(
                &'b self,
                handle: Handle<T>,
                func: F,
            ) -> Result<RefMut<'a, 'b, C>, MultiBorrowError<T>>
            where
                C: ?Sized,
                F: FnOnce(&mut T) -> Result<&mut C, MultiBorrowError<T>>,
            {
                let Some(record) = self.pool.records_get(handle.index) else {
                    return Err(MultiBorrowError::InvalidHandleIndex(handle));
                };
                if handle.generation != record.generation {
                    return Err(MultiBorrowError::InvalidHandleGeneration(handle));
                }
                let current_ref_count = record
                    .ref_counter
                    .0
                    .load(atomic::Ordering::Relaxed);
                match current_ref_count.cmp(&0) {
                    Ordering::Less => {
                        return Err(MultiBorrowError::MutablyBorrowed(handle));
                    }
                    Ordering::Greater => {
                        return Err(MultiBorrowError::ImmutablyBorrowed(handle));
                    }
                    _ => {}
                }
                let payload_container = unsafe { &mut *record.payload.0.get() };
                let Some(payload) = payload_container.as_mut() else {
                    return Err(MultiBorrowError::Empty(handle));
                };
                if let Err(ref_count) = record
                    .ref_counter
                    .0
                    .compare_exchange(
                        0,
                        -1,
                        atomic::Ordering::Acquire,
                        atomic::Ordering::Relaxed,
                    )
                {
                    match ref_count.cmp(&0) {
                        Ordering::Less => {
                            return Err(MultiBorrowError::MutablyBorrowed(handle));
                        }
                        Ordering::Greater => {
                            return Err(MultiBorrowError::ImmutablyBorrowed(handle));
                        }
                        _ => {}
                    }
                }
                Ok(RefMut {
                    data: func(payload)?,
                    ref_counter: &record.ref_counter,
                    phantom: PhantomData,
                })
            }
            #[inline]
            pub fn try_get_mut<'b: 'a>(
                &'b self,
                handle: Handle<T>,
            ) -> Result<RefMut<'a, 'b, T>, MultiBorrowError<T>> {
                self.try_get_mut_internal(handle, |obj| Ok(obj))
            }
            #[inline]
            pub fn get_mut<'b: 'a>(&'b self, handle: Handle<T>) -> RefMut<'a, 'b, T> {
                self.try_get_mut(handle).unwrap()
            }
            #[inline]
            pub fn free(&self, handle: Handle<T>) -> Result<T, MultiBorrowError<T>> {
                let Some(record) = self.pool.records_get(handle.index) else {
                    return Err(MultiBorrowError::InvalidHandleIndex(handle));
                };
                if handle.generation != record.generation {
                    return Err(MultiBorrowError::InvalidHandleGeneration(handle));
                }
                if let Err(ref_count) = record
                    .ref_counter
                    .0
                    .compare_exchange(
                        0,
                        -1,
                        atomic::Ordering::Acquire,
                        atomic::Ordering::Relaxed,
                    )
                {
                    match ref_count.cmp(&0) {
                        Ordering::Less => {
                            return Err(MultiBorrowError::MutablyBorrowed(handle));
                        }
                        Ordering::Greater => {
                            return Err(MultiBorrowError::ImmutablyBorrowed(handle));
                        }
                        _ => {}
                    }
                }
                let payload_container = unsafe { &mut *record.payload.0.get() };
                let Some(payload) = payload_container.take() else {
                    return Err(MultiBorrowError::Empty(handle));
                };
                self.free_indices.borrow_mut().push(handle.index);
                record.ref_counter.increment();
                Ok(payload)
            }
        }
        impl<'a, T, P> MultiBorrowContext<'a, T, P>
        where
            T: Sized + ComponentProvider,
            P: PayloadContainer<Element = T> + 'static,
        {
            /// Tries to mutably borrow an object and fetch its component of specified type.
            #[inline]
            pub fn try_get_component_of_type<'b: 'a, C>(
                &'b self,
                handle: Handle<T>,
            ) -> Result<Ref<'a, 'b, C>, MultiBorrowError<T>>
            where
                C: 'static,
            {
                self.try_get_internal(
                    handle,
                    move |obj| {
                        obj.query_component_ref(TypeId::of::<C>())
                            .and_then(|c| c.downcast_ref())
                            .ok_or(MultiBorrowError::NoSuchComponent(handle))
                    },
                )
            }
            /// Tries to mutably borrow an object and fetch its component of specified type.
            #[inline]
            pub fn try_get_component_of_type_mut<'b: 'a, C>(
                &'b self,
                handle: Handle<T>,
            ) -> Result<RefMut<'a, 'b, C>, MultiBorrowError<T>>
            where
                C: 'static,
            {
                self.try_get_mut_internal(
                    handle,
                    move |obj| {
                        obj.query_component_mut(TypeId::of::<C>())
                            .and_then(|c| c.downcast_mut())
                            .ok_or(MultiBorrowError::NoSuchComponent(handle))
                    },
                )
            }
        }
    }
    pub mod payload {
        use std::cell::UnsafeCell;
        pub trait PayloadContainer: Sized {
            type Element: Sized;
            fn new_empty() -> Self;
            fn new(element: Self::Element) -> Self;
            fn is_some(&self) -> bool;
            fn as_ref(&self) -> Option<&Self::Element>;
            fn as_mut(&mut self) -> Option<&mut Self::Element>;
            fn replace(&mut self, element: Self::Element) -> Option<Self::Element>;
            fn take(&mut self) -> Option<Self::Element>;
        }
        impl<T> PayloadContainer for Option<T> {
            type Element = T;
            #[inline]
            fn new_empty() -> Self {
                Self::None
            }
            #[inline]
            fn new(element: Self::Element) -> Self {
                Self::Some(element)
            }
            #[inline]
            fn is_some(&self) -> bool {
                Option::is_some(self)
            }
            #[inline]
            fn as_ref(&self) -> Option<&Self::Element> {
                Option::as_ref(self)
            }
            #[inline]
            fn as_mut(&mut self) -> Option<&mut Self::Element> {
                Option::as_mut(self)
            }
            #[inline]
            fn replace(&mut self, element: Self::Element) -> Option<Self::Element> {
                Option::replace(self, element)
            }
            #[inline]
            fn take(&mut self) -> Option<Self::Element> {
                Option::take(self)
            }
        }
        pub struct Payload<P>(pub UnsafeCell<P>);
        #[automatically_derived]
        impl<P: ::core::fmt::Debug> ::core::fmt::Debug for Payload<P> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Payload", &&self.0)
            }
        }
        impl<T, P> Clone for Payload<P>
        where
            T: Sized,
            P: PayloadContainer<Element = T> + Clone,
        {
            fn clone(&self) -> Self {
                Self(UnsafeCell::new(self.get().clone()))
            }
        }
        impl<T, P> Payload<P>
        where
            T: Sized,
            P: PayloadContainer<Element = T>,
        {
            pub fn new(data: T) -> Self {
                Self(UnsafeCell::new(P::new(data)))
            }
            pub fn new_empty() -> Self {
                Self(UnsafeCell::new(P::new_empty()))
            }
            pub fn get(&self) -> &P {
                unsafe { &*self.0.get() }
            }
            pub fn get_mut(&mut self) -> &mut P {
                self.0.get_mut()
            }
            pub fn is_some(&self) -> bool {
                self.get().is_some()
            }
            pub fn as_ref(&self) -> Option<&T> {
                self.get().as_ref()
            }
            pub fn as_mut(&mut self) -> Option<&mut T> {
                self.get_mut().as_mut()
            }
            pub fn replace(&mut self, element: T) -> Option<T> {
                self.get_mut().replace(element)
            }
            pub fn take(&mut self) -> Option<T> {
                self.get_mut().take()
            }
        }
        unsafe impl<T, P> Sync for Payload<P>
        where
            T: Sized,
            P: PayloadContainer<Element = T>,
        {}
        unsafe impl<T, P> Send for Payload<P>
        where
            T: Sized,
            P: PayloadContainer<Element = T>,
        {}
    }
    pub use handle::*;
    pub use multiborrow::*;
    pub use payload::*;
    const INVALID_GENERATION: u32 = 0;
    /// Pool allows to create as many objects as you want in contiguous memory
    /// block. It allows to create and delete objects much faster than if they'll
    /// be allocated on heap. Also since objects stored in contiguous memory block
    /// they can be effectively accessed because such memory layout is cache-friendly.
    pub struct Pool<T, P = Option<T>>
    where
        T: Sized,
        P: PayloadContainer<Element = T>,
    {
        records: Vec<PoolRecord<T, P>>,
        free_stack: Vec<u32>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug, P: ::core::fmt::Debug> ::core::fmt::Debug for Pool<T, P>
    where
        T: Sized,
        P: PayloadContainer<Element = T>,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Pool",
                "records",
                &self.records,
                "free_stack",
                &&self.free_stack,
            )
        }
    }
    impl<T, P> Reflect for Pool<T, P>
    where
        T: Reflect,
        P: PayloadContainer<Element = T> + Reflect,
    {
        #[inline]
        fn source_path() -> &'static str {
            "fyrox-core\\src\\pool\\mod.rs"
        }
        #[inline]
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        #[inline]
        fn doc(&self) -> &'static str {
            ""
        }
        #[inline]
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(&[])
        }
        #[inline]
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
        #[inline]
        fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
            func(self)
        }
        #[inline]
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
            func(self)
        }
        #[inline]
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self)
        }
        #[inline]
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self)
        }
        #[inline]
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let this = std::mem::replace(self, value.take()?);
            Ok(Box::new(this))
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        #[inline]
        fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
            func(Some(self))
        }
        #[inline]
        fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
            func(Some(self))
        }
    }
    impl<T, P> ReflectArray for Pool<T, P>
    where
        T: Reflect,
        P: PayloadContainer<Element = T> + Reflect,
    {
        #[inline]
        fn reflect_index(&self, index: usize) -> Option<&dyn Reflect> {
            self.at(index as u32).map(|p| p as &dyn Reflect)
        }
        #[inline]
        fn reflect_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
            self.at_mut(index as u32).map(|p| p as &mut dyn Reflect)
        }
        #[inline]
        fn reflect_len(&self) -> usize {
            self.get_capacity() as usize
        }
    }
    impl<T, P> PartialEq for Pool<T, P>
    where
        T: PartialEq,
        P: PayloadContainer<Element = T> + PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.records == other.records
        }
    }
    struct RefCounter(pub AtomicIsize);
    #[automatically_derived]
    impl ::core::default::Default for RefCounter {
        #[inline]
        fn default() -> RefCounter {
            RefCounter(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RefCounter {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "RefCounter", &&self.0)
        }
    }
    impl RefCounter {
        fn increment(&self) {
            self.0.fetch_add(1, atomic::Ordering::Relaxed);
        }
        fn decrement(&self) {
            self.0.fetch_sub(1, atomic::Ordering::Relaxed);
        }
    }
    struct PoolRecord<T, P = Option<T>>
    where
        T: Sized,
        P: PayloadContainer<Element = T>,
    {
        ref_counter: RefCounter,
        generation: u32,
        payload: Payload<P>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug, P: ::core::fmt::Debug> ::core::fmt::Debug
    for PoolRecord<T, P>
    where
        T: Sized,
        P: PayloadContainer<Element = T>,
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "PoolRecord",
                "ref_counter",
                &self.ref_counter,
                "generation",
                &self.generation,
                "payload",
                &&self.payload,
            )
        }
    }
    impl<T, P> PartialEq for PoolRecord<T, P>
    where
        T: PartialEq,
        P: PayloadContainer<Element = T> + PartialEq,
    {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.generation == other.generation
                && self.payload.get() == other.payload.get()
        }
    }
    impl<T, P> Default for PoolRecord<T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        #[inline]
        fn default() -> Self {
            Self {
                ref_counter: Default::default(),
                generation: INVALID_GENERATION,
                payload: Payload::new_empty(),
            }
        }
    }
    impl<T, P> Visit for PoolRecord<T, P>
    where
        T: Visit + 'static,
        P: PayloadContainer<Element = T> + Visit,
    {
        #[inline]
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.generation.visit("Generation", &mut region)?;
            self.payload.get_mut().visit("Payload", &mut region)?;
            Ok(())
        }
    }
    impl<T> Clone for Handle<T> {
        #[inline]
        fn clone(&self) -> Handle<T> {
            *self
        }
    }
    impl<T, P> Visit for Pool<T, P>
    where
        T: Visit + 'static,
        P: PayloadContainer<Element = T> + Default + Visit + 'static,
    {
        #[inline]
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.records.visit("Records", &mut region)?;
            self.free_stack.visit("FreeStack", &mut region)?;
            Ok(())
        }
    }
    impl<T, P> Default for Pool<T, P>
    where
        T: 'static,
        P: PayloadContainer<Element = T> + 'static,
    {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }
    pub struct Ticket<T> {
        index: u32,
        marker: PhantomData<T>,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Ticket<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Ticket",
                "index",
                &self.index,
                "marker",
                &&self.marker,
            )
        }
    }
    impl<T> Drop for Ticket<T> {
        fn drop(&mut self) {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "An object at index {0} must be returned to a pool it was taken from! Call Pool::forget_ticket if you don\'t need the object anymore.",
                        self.index,
                    ),
                );
            }
        }
    }
    impl<T: Clone> Clone for PoolRecord<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                ref_counter: Default::default(),
                generation: self.generation,
                payload: self.payload.clone(),
            }
        }
    }
    impl<T: Clone> Clone for Pool<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                records: self.records.clone(),
                free_stack: self.free_stack.clone(),
            }
        }
    }
    impl<T, P> Pool<T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        #[inline]
        pub fn new() -> Self {
            Pool {
                records: Vec::new(),
                free_stack: Vec::new(),
            }
        }
        #[inline]
        pub fn with_capacity(capacity: u32) -> Self {
            let capacity = usize::try_from(capacity).expect("capacity overflowed usize");
            Pool {
                records: Vec::with_capacity(capacity),
                free_stack: Vec::new(),
            }
        }
        fn records_len(&self) -> u32 {
            u32::try_from(self.records.len()).expect("Number of records overflowed u32")
        }
        fn records_get(&self, index: u32) -> Option<&PoolRecord<T, P>> {
            let index = usize::try_from(index).expect("Index overflowed usize");
            self.records.get(index)
        }
        fn records_get_mut(&mut self, index: u32) -> Option<&mut PoolRecord<T, P>> {
            let index = usize::try_from(index).expect("Index overflowed usize");
            self.records.get_mut(index)
        }
        #[inline]
        #[must_use]
        pub fn spawn(&mut self, payload: T) -> Handle<T> {
            self.spawn_with(|_| payload)
        }
        /// Tries to put an object in the pool at given position. Returns `Err(payload)` if a corresponding
        /// entry is occupied.
        ///
        /// # Performance
        ///
        /// The method has O(n) complexity in worst case, where `n` - amount of free records in the pool.
        /// In typical uses cases `n` is very low. It should be noted that if a pool is filled entirely
        /// and you trying to put an object at the end of pool, the method will have O(1) complexity.
        ///
        /// # Panics
        ///
        /// Panics if the index is occupied or reserved (e.g. by [`take_reserve`]).
        ///
        /// [`take_reserve`]: Pool::take_reserve
        #[inline]
        pub fn spawn_at(&mut self, index: u32, payload: T) -> Result<Handle<T>, T> {
            self.spawn_at_internal(index, INVALID_GENERATION, payload)
        }
        /// Tries to put an object in the pool at given handle. Returns `Err(payload)` if a corresponding
        /// entry is occupied.
        ///
        /// # Performance
        ///
        /// The method has O(n) complexity in worst case, where `n` - amount of free records in the pool.
        /// In typical uses cases `n` is very low. It should be noted that if a pool is filled entirely
        /// and you trying to put an object at the end of pool, the method will have O(1) complexity.
        ///
        /// # Panics
        ///
        /// Panics if the index is occupied or reserved (e.g. by [`take_reserve`]).
        ///
        /// [`take_reserve`]: Pool::take_reserve
        #[inline]
        pub fn spawn_at_handle(
            &mut self,
            handle: Handle<T>,
            payload: T,
        ) -> Result<Handle<T>, T> {
            self.spawn_at_internal(handle.index, handle.generation, payload)
        }
        fn spawn_at_internal(
            &mut self,
            index: u32,
            desired_generation: u32,
            payload: T,
        ) -> Result<Handle<T>, T> {
            let index_usize = usize::try_from(index).expect("index overflowed usize");
            match self.records.get_mut(index_usize) {
                Some(record) => {
                    match record.payload.as_ref() {
                        Some(_) => Err(payload),
                        None => {
                            let position = self
                                .free_stack
                                .iter()
                                .rposition(|i| *i == index)
                                .expect(
                                    "free_stack must contain the index of the empty record (most likely attempting to spawn at a reserved index)!",
                                );
                            self.free_stack.remove(position);
                            let generation = if desired_generation == INVALID_GENERATION
                            {
                                record.generation + 1
                            } else {
                                desired_generation
                            };
                            record.generation = generation;
                            record.payload = Payload::new(payload);
                            Ok(Handle::new(index, generation))
                        }
                    }
                }
                None => {
                    for i in self.records_len()..index {
                        self.records
                            .push(PoolRecord {
                                ref_counter: Default::default(),
                                generation: 1,
                                payload: Payload::new_empty(),
                            });
                        self.free_stack.push(i);
                    }
                    let generation = if desired_generation == INVALID_GENERATION {
                        1
                    } else {
                        desired_generation
                    };
                    self.records
                        .push(PoolRecord {
                            ref_counter: Default::default(),
                            generation,
                            payload: Payload::new(payload),
                        });
                    Ok(Handle::new(index, generation))
                }
            }
        }
        #[inline]
        #[must_use]
        /// Construct a value with the handle it would be given.
        /// Note: Handle is _not_ valid until function has finished executing.
        pub fn spawn_with<F: FnOnce(Handle<T>) -> T>(
            &mut self,
            callback: F,
        ) -> Handle<T> {
            if let Some(free_index) = self.free_stack.pop() {
                let record = self
                    .records_get_mut(free_index)
                    .expect("free stack contained invalid index");
                if record.payload.is_some() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to spawn an object at pool record with payload! Record index is {0}",
                                free_index,
                            ),
                        );
                    };
                }
                let generation = record.generation + 1;
                let handle = Handle {
                    index: free_index,
                    generation,
                    type_marker: PhantomData,
                };
                let payload = callback(handle);
                record.generation = generation;
                record.payload.replace(payload);
                handle
            } else {
                let generation = 1;
                let handle = Handle {
                    index: self.records.len() as u32,
                    generation,
                    type_marker: PhantomData,
                };
                let payload = callback(handle);
                let record = PoolRecord {
                    ref_counter: Default::default(),
                    generation,
                    payload: Payload::new(payload),
                };
                self.records.push(record);
                handle
            }
        }
        #[inline]
        /// Asynchronously construct a value with the handle it would be given.
        /// Note: Handle is _not_ valid until function has finished executing.
        pub async fn spawn_with_async<F, Fut>(&mut self, callback: F) -> Handle<T>
        where
            F: FnOnce(Handle<T>) -> Fut,
            Fut: Future<Output = T>,
        {
            if let Some(free_index) = self.free_stack.pop() {
                let record = self
                    .records_get_mut(free_index)
                    .expect("free stack contained invalid index");
                if record.payload.is_some() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to spawn an object at pool record with payload! Record index is {0}",
                                free_index,
                            ),
                        );
                    };
                }
                let generation = record.generation + 1;
                let handle = Handle {
                    index: free_index,
                    generation,
                    type_marker: PhantomData,
                };
                let payload = callback(handle).await;
                record.generation = generation;
                record.payload.replace(payload);
                handle
            } else {
                let generation = 1;
                let handle = Handle {
                    index: self.records.len() as u32,
                    generation,
                    type_marker: PhantomData,
                };
                let payload = callback(handle).await;
                let record = PoolRecord {
                    generation,
                    ref_counter: Default::default(),
                    payload: Payload::new(payload),
                };
                self.records.push(record);
                handle
            }
        }
        /// Generates a set of handles that could be used to spawn a set of objects. This method does not
        /// modify the pool and the generated handles could be used together with [`Self::spawn_at_handle`]
        /// method.
        #[inline]
        pub fn generate_free_handles(&self, amount: usize) -> Vec<Handle<T>> {
            let mut free_handles = Vec::with_capacity(amount);
            free_handles
                .extend(
                    self
                        .free_stack
                        .iter()
                        .take(amount)
                        .map(|i| Handle::new(
                            *i,
                            self.records[*i as usize].generation + 1,
                        )),
                );
            if free_handles.len() < amount {
                let remainder = amount - free_handles.len();
                free_handles
                    .extend(
                        (self.records.len()..self.records.len() + remainder)
                            .map(|i| Handle::new(i as u32, 1)),
                    );
            }
            free_handles
        }
        /// Borrows shared reference to an object by its handle.
        ///
        /// # Panics
        ///
        /// Panics if handle is out of bounds or generation of handle does not match with
        /// generation of pool record at handle index (in other words it means that object
        /// at handle's index is different than the object was there before).
        #[inline]
        #[must_use]
        pub fn borrow(&self, handle: Handle<T>) -> &T {
            if let Some(record) = self.records_get(handle.index) {
                if record.generation == handle.generation {
                    if let Some(payload) = record.payload.as_ref() {
                        payload
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Attempt to borrow destroyed object at {0:?} handle.",
                                    handle,
                                ),
                            );
                        };
                    }
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to use dangling handle {0:?}. Record has generation {1}!",
                                handle,
                                record.generation,
                            ),
                        );
                    };
                }
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Attempt to borrow object using out-of-bounds handle {0:?}! Record count is {1}",
                            handle,
                            self.records.len(),
                        ),
                    );
                };
            }
        }
        /// Borrows mutable reference to an object by its handle.
        ///
        /// # Panics
        ///
        /// Panics if handle is out of bounds or generation of handle does not match with
        /// generation of pool record at handle index (in other words it means that object
        /// at handle's index is different than the object was there before).
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// let a = pool.spawn(1);
        /// let a = pool.borrow_mut(a);
        /// *a = 11;
        /// ```
        #[inline]
        #[must_use]
        pub fn borrow_mut(&mut self, handle: Handle<T>) -> &mut T {
            let record_count = self.records.len();
            if let Some(record) = self.records_get_mut(handle.index) {
                if record.generation == handle.generation {
                    if let Some(payload) = record.payload.as_mut() {
                        payload
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Attempt to borrow destroyed object at {0:?} handle.",
                                    handle,
                                ),
                            );
                        };
                    }
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to borrow object using dangling handle {0:?}. Record has {1} generation!",
                                handle,
                                record.generation,
                            ),
                        );
                    };
                }
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Attempt to borrow object using out-of-bounds handle {0:?}! Record count is {1}",
                            handle,
                            record_count,
                        ),
                    );
                };
            }
        }
        /// Borrows shared reference to an object by its handle.
        ///
        /// Returns None if handle is out of bounds or generation of handle does not match with
        /// generation of pool record at handle index (in other words it means that object
        /// at handle's index is different than the object was there before).
        #[inline]
        #[must_use]
        pub fn try_borrow(&self, handle: Handle<T>) -> Option<&T> {
            self.records_get(handle.index)
                .and_then(|r| {
                    if r.generation == handle.generation {
                        r.payload.as_ref()
                    } else {
                        None
                    }
                })
        }
        /// Borrows mutable reference to an object by its handle.
        ///
        /// Returns None if handle is out of bounds or generation of handle does not match with
        /// generation of pool record at handle index (in other words it means that object
        /// at handle's index is different than the object was there before).
        #[inline]
        #[must_use]
        pub fn try_borrow_mut(&mut self, handle: Handle<T>) -> Option<&mut T> {
            self.records_get_mut(handle.index)
                .and_then(|r| {
                    if r.generation == handle.generation {
                        r.payload.as_mut()
                    } else {
                        None
                    }
                })
        }
        /// Borrows mutable references of objects at the same time. This method will succeed only
        /// if handles are unique (not equal). Borrowing multiple mutable references at the same
        /// time is useful in case if you need to mutate some objects at the same time.
        ///
        /// # Panics
        ///
        /// See [`borrow_mut`](Self::borrow_mut).
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// let a = pool.spawn(1);
        /// let b = pool.spawn(2);
        /// let (a, b) = pool.borrow_two_mut((a, b));
        /// *a = 11;
        /// *b = 22;
        /// ```
        #[inline]
        #[must_use = "Handle set must not be ignored"]
        pub fn borrow_two_mut(
            &mut self,
            handles: (Handle<T>, Handle<T>),
        ) -> (&mut T, &mut T) {
            match (&handles.0.index, &handles.1.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            unsafe {
                let this = self as *mut Self;
                ((*this).borrow_mut(handles.0), (*this).borrow_mut(handles.1))
            }
        }
        /// Borrows mutable references of objects at the same time. This method will succeed only
        /// if handles are unique (not equal). Borrowing multiple mutable references at the same
        /// time is useful in case if you need to mutate some objects at the same time.
        ///
        /// # Panics
        ///
        /// See [`borrow_mut`](Self::borrow_mut).
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// let a = pool.spawn(1);
        /// let b = pool.spawn(2);
        /// let c = pool.spawn(3);
        /// let (a, b, c) = pool.borrow_three_mut((a, b, c));
        /// *a = 11;
        /// *b = 22;
        /// *c = 33;
        /// ```
        #[inline]
        #[must_use = "Handle set must not be ignored"]
        pub fn borrow_three_mut(
            &mut self,
            handles: (Handle<T>, Handle<T>, Handle<T>),
        ) -> (&mut T, &mut T, &mut T) {
            match (&handles.0.index, &handles.1.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.0.index, &handles.2.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.1.index, &handles.2.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            unsafe {
                let this = self as *mut Self;
                (
                    (*this).borrow_mut(handles.0),
                    (*this).borrow_mut(handles.1),
                    (*this).borrow_mut(handles.2),
                )
            }
        }
        /// Borrows mutable references of objects at the same time. This method will succeed only
        /// if handles are unique (not equal). Borrowing multiple mutable references at the same
        /// time is useful in case if you need to mutate some objects at the same time.
        ///
        /// # Panics
        ///
        /// See [`borrow_mut`](Self::borrow_mut).
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// let a = pool.spawn(1);
        /// let b = pool.spawn(2);
        /// let c = pool.spawn(3);
        /// let d = pool.spawn(4);
        /// let (a, b, c, d) = pool.borrow_four_mut((a, b, c, d));
        /// *a = 11;
        /// *b = 22;
        /// *c = 33;
        /// *d = 44;
        /// ```
        #[inline]
        #[must_use = "Handle set must not be ignored"]
        pub fn borrow_four_mut(
            &mut self,
            handles: (Handle<T>, Handle<T>, Handle<T>, Handle<T>),
        ) -> (&mut T, &mut T, &mut T, &mut T) {
            match (&handles.0.index, &handles.1.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.0.index, &handles.2.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.0.index, &handles.3.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.1.index, &handles.2.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.1.index, &handles.3.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            match (&handles.2.index, &handles.3.index) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = ::core::panicking::AssertKind::Ne;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            unsafe {
                let this = self as *mut Self;
                (
                    (*this).borrow_mut(handles.0),
                    (*this).borrow_mut(handles.1),
                    (*this).borrow_mut(handles.2),
                    (*this).borrow_mut(handles.3),
                )
            }
        }
        /// Tries to borrow two objects when a handle to the second object stored in the first object.
        #[inline]
        pub fn try_borrow_dependant_mut<F>(
            &mut self,
            handle: Handle<T>,
            func: F,
        ) -> (Option<&mut T>, Option<&mut T>)
        where
            F: FnOnce(&T) -> Handle<T>,
        {
            let this = unsafe { &mut *(self as *mut Pool<T, P>) };
            let first = self.try_borrow_mut(handle);
            if let Some(first_object) = first.as_ref() {
                let second_handle = func(first_object);
                if second_handle != handle {
                    return (first, this.try_borrow_mut(second_handle));
                }
            }
            (first, None)
        }
        /// Moves object out of the pool using the given handle. All handles to the object will become invalid.
        ///
        /// # Panics
        ///
        /// Panics if the given handle is invalid.
        #[inline]
        pub fn free(&mut self, handle: Handle<T>) -> T {
            let index = usize::try_from(handle.index).expect("index overflowed usize");
            if let Some(record) = self.records.get_mut(index) {
                if record.generation == handle.generation {
                    self.free_stack.push(handle.index);
                    if let Some(payload) = record.payload.take() {
                        payload
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Attempt to double free object at handle {0:?}!",
                                    handle,
                                ),
                            );
                        };
                    }
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to free object using dangling handle {0:?}! Record generation is {1}",
                                handle,
                                record.generation,
                            ),
                        );
                    };
                }
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Attempt to free destroyed object using out-of-bounds handle {0:?}! Record count is {1}",
                            handle,
                            self.records.len(),
                        ),
                    );
                };
            }
        }
        /// Tries to move object out of the pool using the given handle. Returns None if given handle
        /// is invalid. After object is moved out if the pool, all handles to the object will become
        /// invalid.
        #[inline]
        pub fn try_free(&mut self, handle: Handle<T>) -> Option<T> {
            let index = usize::try_from(handle.index).expect("index overflowed usize");
            self.records
                .get_mut(index)
                .and_then(|record| {
                    if record.generation == handle.generation {
                        if let Some(payload) = record.payload.take() {
                            self.free_stack.push(handle.index);
                            Some(payload)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
        }
        /// Moves an object out of the pool using the given handle with a promise that the object will be returned back.
        /// Returns pair (ticket, value). The ticket must be used to put the value back!
        ///
        /// # Motivation
        ///
        /// This method is useful when you need to take temporary ownership of an object, do something
        /// with it and then put it back while preserving all handles to it and being able to put new objects into
        /// the pool without overriding the payload at its handle.
        ///
        /// # Notes
        ///
        /// All handles to the object will be temporarily invalid until the object is returned to the pool! The pool record will
        /// be reserved for a further [`put_back`] call, which means if you lose the ticket you will have an empty
        /// "unusable" pool record forever.
        ///
        /// # Panics
        ///
        /// Panics if the given handle is invalid.
        ///
        /// [`put_back`]: Pool::put_back
        #[inline]
        pub fn take_reserve(&mut self, handle: Handle<T>) -> (Ticket<T>, T) {
            if let Some(record) = self.records_get_mut(handle.index) {
                if record.generation == handle.generation {
                    if let Some(payload) = record.payload.take() {
                        let ticket = Ticket {
                            index: handle.index,
                            marker: PhantomData,
                        };
                        (ticket, payload)
                    } else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Attempt to take already taken object at handle {0:?}!",
                                    handle,
                                ),
                            );
                        };
                    }
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to take object using dangling handle {0:?}! Record generation is {1}",
                                handle,
                                record.generation,
                            ),
                        );
                    };
                }
            } else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Attempt to take destroyed object using out-of-bounds handle {0:?}! Record count is {1}",
                            handle,
                            self.records.len(),
                        ),
                    );
                };
            }
        }
        /// Does the same as [`take_reserve`] but returns an option, instead of panicking.
        ///
        /// [`take_reserve`]: Pool::take_reserve
        #[inline]
        pub fn try_take_reserve(&mut self, handle: Handle<T>) -> Option<(Ticket<T>, T)> {
            if let Some(record) = self.records_get_mut(handle.index) {
                if record.generation == handle.generation {
                    if let Some(payload) = record.payload.take() {
                        let ticket = Ticket {
                            index: handle.index,
                            marker: PhantomData,
                        };
                        Some((ticket, payload))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        /// Returns the value back into the pool using the given ticket. See [`take_reserve`] for more
        /// information.
        ///
        /// [`take_reserve`]: Pool::take_reserve
        #[inline]
        pub fn put_back(&mut self, ticket: Ticket<T>, value: T) -> Handle<T> {
            let record = self
                .records_get_mut(ticket.index)
                .expect("Ticket index was invalid");
            let old = record.payload.replace(value);
            if !old.is_none() {
                ::core::panicking::panic("assertion failed: old.is_none()")
            }
            let handle = Handle::new(ticket.index, record.generation);
            std::mem::forget(ticket);
            handle
        }
        /// Forgets that value at ticket was reserved and makes it usable again.
        /// Useful when you don't need to put value back by ticket, but just make
        /// pool record usable again.
        #[inline]
        pub fn forget_ticket(&mut self, ticket: Ticket<T>) {
            self.free_stack.push(ticket.index);
            std::mem::forget(ticket);
        }
        /// Returns total capacity of pool. Capacity has nothing about real amount of objects in pool!
        #[inline]
        #[must_use]
        pub fn get_capacity(&self) -> u32 {
            u32::try_from(self.records.len()).expect("records.len() overflowed u32")
        }
        /// Destroys all objects in pool. All handles to objects will become invalid.
        ///
        /// # Remarks
        ///
        /// Use this method cautiously if objects in pool have cross "references" (handles)
        /// to each other. This method will make all produced handles invalid and any further
        /// calls for [`borrow`](Self::borrow) or [`borrow_mut`](Self::borrow_mut) will raise panic.
        #[inline]
        pub fn clear(&mut self) {
            self.records.clear();
            self.free_stack.clear();
        }
        #[inline]
        #[must_use]
        pub fn at_mut(&mut self, n: u32) -> Option<&mut T> {
            self.records_get_mut(n).and_then(|rec| rec.payload.as_mut())
        }
        #[inline]
        #[must_use]
        pub fn at(&self, n: u32) -> Option<&T> {
            self.records_get(n).and_then(|rec| rec.payload.get().as_ref())
        }
        #[inline]
        #[must_use]
        pub fn handle_from_index(&self, n: u32) -> Handle<T> {
            if let Some(record) = self.records_get(n) {
                if record.generation != INVALID_GENERATION {
                    return Handle::new(n, record.generation);
                }
            }
            Handle::NONE
        }
        /// Returns the exact number of "alive" objects in the pool.
        ///
        /// Records that have been reserved (e.g. by [`take_reserve`]) are *not* counted.
        ///
        /// It iterates through the entire pool to count the live objects so the complexity is `O(n)`.
        ///
        /// See also [`total_count`].
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// pool.spawn(123);
        /// pool.spawn(321);
        /// assert_eq!(pool.alive_count(), 2);
        /// ```
        ///
        /// [`take_reserve`]: Pool::take_reserve
        /// [`total_count`]: Pool::total_count
        #[inline]
        #[must_use]
        pub fn alive_count(&self) -> u32 {
            let cnt = self.iter().count();
            u32::try_from(cnt).expect("alive_count overflowed u32")
        }
        /// Returns the number of allocated objects in the pool.
        ///
        /// It also counts records that have been reserved (e.g. by [`take_reserve`]).
        ///
        /// This method is `O(1)`.
        ///
        /// See also [`alive_count`].
        ///
        /// [`take_reserve`]: Pool::take_reserve
        /// [`alive_count`]: Pool::alive_count
        #[inline]
        pub fn total_count(&self) -> u32 {
            let free = u32::try_from(self.free_stack.len())
                .expect("free stack length overflowed u32");
            self.records_len() - free
        }
        #[inline]
        pub fn replace(&mut self, handle: Handle<T>, payload: T) -> Option<T> {
            let index_usize = usize::try_from(handle.index)
                .expect("index overflowed usize");
            if let Some(record) = self.records.get_mut(index_usize) {
                if record.generation == handle.generation {
                    self.free_stack.retain(|i| *i != handle.index);
                    record.payload.replace(payload)
                } else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Attempt to replace object in pool using dangling handle! Handle is {0:?}, but pool record has {1} generation",
                                handle,
                                record.generation,
                            ),
                        );
                    };
                }
            } else {
                None
            }
        }
        /// Returns a reference to the first element in the pool (if any).
        pub fn first_ref(&self) -> Option<&T> {
            self.iter().next()
        }
        /// Returns a reference to the first element in the pool (if any).
        pub fn first_mut(&mut self) -> Option<&mut T> {
            self.iter_mut().next()
        }
        /// Checks if given handle "points" to some object.
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// let handle = pool.spawn(123);
        /// assert_eq!(pool.is_valid_handle(handle), true)
        /// ```
        #[inline]
        pub fn is_valid_handle(&self, handle: Handle<T>) -> bool {
            if let Some(record) = self.records_get(handle.index) {
                record.payload.is_some() && record.generation == handle.generation
            } else {
                false
            }
        }
        /// Creates new pool iterator that iterates over filled records in pool.
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// pool.spawn(123);
        /// pool.spawn(321);
        /// let mut iter = pool.iter();
        /// assert_eq!(*iter.next().unwrap(), 123);
        /// assert_eq!(*iter.next().unwrap(), 321);
        /// ```
        #[must_use]
        #[inline]
        pub fn iter(&self) -> PoolIterator<T, P> {
            unsafe {
                PoolIterator {
                    ptr: self.records.as_ptr(),
                    end: self.records.as_ptr().add(self.records.len()),
                    marker: PhantomData,
                }
            }
        }
        /// Creates new pair iterator that iterates over filled records using pair (handle, payload)
        /// Can be useful when there is a need to iterate over pool records and know a handle of
        /// that record.
        #[inline]
        pub fn pair_iter(&self) -> PoolPairIterator<T, P> {
            PoolPairIterator {
                pool: self,
                current: 0,
            }
        }
        /// Creates new pool iterator that iterates over filled records in pool allowing
        /// to modify record payload.
        ///
        /// # Example
        ///
        /// ```
        /// use fyrox_core::pool::Pool;
        /// let mut pool = Pool::<u32>::new();
        /// pool.spawn(123);
        /// pool.spawn(321);
        /// let mut iter = pool.iter_mut();
        /// assert_eq!(*iter.next().unwrap(), 123);
        /// assert_eq!(*iter.next().unwrap(), 321);
        /// ```
        #[must_use]
        #[inline]
        pub fn iter_mut(&mut self) -> PoolIteratorMut<T, P> {
            unsafe {
                PoolIteratorMut {
                    ptr: self.records.as_mut_ptr(),
                    end: self.records.as_mut_ptr().add(self.records.len()),
                    marker: PhantomData,
                }
            }
        }
        /// Creates new pair iterator that iterates over filled records using pair (handle, payload)
        /// Can be useful when there is a need to iterate over pool records and know a handle of
        /// that record.
        #[inline]
        pub fn pair_iter_mut(&mut self) -> PoolPairIteratorMut<T, P> {
            unsafe {
                PoolPairIteratorMut {
                    current: 0,
                    ptr: self.records.as_mut_ptr(),
                    end: self.records.as_mut_ptr().add(self.records.len()),
                    marker: PhantomData,
                }
            }
        }
        /// Retains pool records selected by `pred`. Useful when you need to remove all pool records
        /// by some criteria.
        #[inline]
        pub fn retain<F>(&mut self, mut pred: F)
        where
            F: FnMut(&T) -> bool,
        {
            for (i, record) in self.records.iter_mut().enumerate() {
                if record.generation == INVALID_GENERATION {
                    continue;
                }
                let retain = if let Some(payload) = record.payload.as_ref() {
                    pred(payload)
                } else {
                    continue;
                };
                if !retain {
                    self.free_stack.push(i as u32);
                    record.payload.take();
                }
            }
        }
        /// Begins multi-borrow that allows you to borrow as many (`N`) **unique** references to the pool
        /// elements as you need. See [`MultiBorrowContext::try_get`] for more info.
        #[inline]
        pub fn begin_multi_borrow(&mut self) -> MultiBorrowContext<T, P> {
            MultiBorrowContext::new(self)
        }
        /// Removes all elements from the pool.
        #[inline]
        pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
            self.free_stack.clear();
            self.records.drain(..).filter_map(|mut r| r.payload.take())
        }
        fn end(&self) -> *const PoolRecord<T, P> {
            unsafe { self.records.as_ptr().add(self.records.len()) }
        }
        fn begin(&self) -> *const PoolRecord<T, P> {
            self.records.as_ptr()
        }
        #[inline]
        pub fn handle_of(&self, ptr: &T) -> Handle<T> {
            let begin = self.begin() as usize;
            let end = self.end() as usize;
            let val = ptr as *const T as usize;
            if val >= begin && val < end {
                let record_size = std::mem::size_of::<PoolRecord<T>>();
                let record_location = (val
                    - { { builtin # offset_of(PoolRecord < T >, payload) } }) - begin;
                if record_location % record_size == 0 {
                    let index = record_location / record_size;
                    let index = u32::try_from(index).expect("Index overflowed u32");
                    return self.handle_from_index(index);
                }
            }
            Handle::NONE
        }
    }
    impl<T, P> Pool<T, P>
    where
        T: ComponentProvider,
        P: PayloadContainer<Element = T> + 'static,
    {
        /// Tries to mutably borrow an object and fetch its component of specified type.
        #[inline]
        pub fn try_get_component_of_type<C>(&self, handle: Handle<T>) -> Option<&C>
        where
            C: 'static,
        {
            self.try_borrow(handle)
                .and_then(|n| n.query_component_ref(TypeId::of::<C>()))
                .and_then(|c| c.downcast_ref())
        }
        /// Tries to mutably borrow an object and fetch its component of specified type.
        #[inline]
        pub fn try_get_component_of_type_mut<C>(
            &mut self,
            handle: Handle<T>,
        ) -> Option<&mut C>
        where
            C: 'static,
        {
            self.try_borrow_mut(handle)
                .and_then(|n| n.query_component_mut(TypeId::of::<C>()))
                .and_then(|c| c.downcast_mut())
        }
    }
    impl<T> FromIterator<T> for Pool<T>
    where
        T: 'static,
    {
        #[inline]
        fn from_iter<C: IntoIterator<Item = T>>(iter: C) -> Self {
            let iter = iter.into_iter();
            let (lower_bound, upper_bound) = iter.size_hint();
            let lower_bound = u32::try_from(lower_bound)
                .expect("lower_bound overflowed u32");
            let upper_bound = upper_bound
                .map(|b| u32::try_from(b).expect("upper_bound overflowed u32"));
            let mut pool = Self::with_capacity(upper_bound.unwrap_or(lower_bound));
            for v in iter {
                let _ = pool.spawn(v);
            }
            pool
        }
    }
    impl<T, P> Index<Handle<T>> for Pool<T, P>
    where
        T: 'static,
        P: PayloadContainer<Element = T> + 'static,
    {
        type Output = T;
        #[inline]
        fn index(&self, index: Handle<T>) -> &Self::Output {
            self.borrow(index)
        }
    }
    impl<T, P> IndexMut<Handle<T>> for Pool<T, P>
    where
        T: 'static,
        P: PayloadContainer<Element = T> + 'static,
    {
        #[inline]
        fn index_mut(&mut self, index: Handle<T>) -> &mut Self::Output {
            self.borrow_mut(index)
        }
    }
    impl<'a, T, P> IntoIterator for &'a Pool<T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        type Item = &'a T;
        type IntoIter = PoolIterator<'a, T, P>;
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    impl<'a, T, P> IntoIterator for &'a mut Pool<T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        type Item = &'a mut T;
        type IntoIter = PoolIteratorMut<'a, T, P>;
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter_mut()
        }
    }
    pub struct PoolIterator<'a, T, P>
    where
        P: PayloadContainer<Element = T>,
    {
        ptr: *const PoolRecord<T, P>,
        end: *const PoolRecord<T, P>,
        marker: PhantomData<&'a T>,
    }
    impl<'a, T, P> Iterator for PoolIterator<'a, T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        type Item = &'a T;
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                while self.ptr != self.end {
                    let current = &*self.ptr;
                    if let Some(payload) = current.payload.as_ref() {
                        self.ptr = self.ptr.offset(1);
                        return Some(payload);
                    }
                    self.ptr = self.ptr.offset(1);
                }
                None
            }
        }
    }
    pub struct PoolPairIterator<'a, T, P: PayloadContainer<Element = T>> {
        pool: &'a Pool<T, P>,
        current: usize,
    }
    impl<'a, T, P> Iterator for PoolPairIterator<'a, T, P>
    where
        P: PayloadContainer<Element = T>,
    {
        type Item = (Handle<T>, &'a T);
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.pool.records.get(self.current) {
                    Some(record) => {
                        if let Some(payload) = record.payload.as_ref() {
                            let handle = Handle::new(
                                self.current as u32,
                                record.generation,
                            );
                            self.current += 1;
                            return Some((handle, payload));
                        }
                        self.current += 1;
                    }
                    None => return None,
                }
            }
        }
    }
    pub struct PoolIteratorMut<'a, T, P>
    where
        P: PayloadContainer<Element = T>,
    {
        ptr: *mut PoolRecord<T, P>,
        end: *mut PoolRecord<T, P>,
        marker: PhantomData<&'a mut T>,
    }
    impl<'a, T, P> Iterator for PoolIteratorMut<'a, T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        type Item = &'a mut T;
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                while self.ptr != self.end {
                    let current = &mut *self.ptr;
                    if let Some(payload) = current.payload.as_mut() {
                        self.ptr = self.ptr.offset(1);
                        return Some(payload);
                    }
                    self.ptr = self.ptr.offset(1);
                }
                None
            }
        }
    }
    pub struct PoolPairIteratorMut<'a, T, P>
    where
        P: PayloadContainer<Element = T>,
    {
        ptr: *mut PoolRecord<T, P>,
        end: *mut PoolRecord<T, P>,
        marker: PhantomData<&'a mut T>,
        current: usize,
    }
    impl<'a, T, P> Iterator for PoolPairIteratorMut<'a, T, P>
    where
        P: PayloadContainer<Element = T> + 'static,
    {
        type Item = (Handle<T>, &'a mut T);
        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            unsafe {
                while self.ptr != self.end {
                    let current = &mut *self.ptr;
                    if let Some(payload) = current.payload.as_mut() {
                        let handle = Handle::new(
                            self.current as u32,
                            current.generation,
                        );
                        self.ptr = self.ptr.offset(1);
                        self.current += 1;
                        return Some((handle, payload));
                    }
                    self.ptr = self.ptr.offset(1);
                    self.current += 1;
                }
                None
            }
        }
    }
}
pub mod profiler {
    //! Built-in scoped profiler. You must compile with feature "enable_profiler" to
    //! force profiler gather info! It is disabled by default because it is not cheap
    //! and takes 3-5% of performance for internal needs.
    #![allow(dead_code)]
    use fxhash::{FxHashMap, FxHashSet, FxHasher};
    use std::{
        fmt, fmt::Write, hash::{Hash, Hasher},
        sync::{Arc, Mutex},
    };
    pub fn print() -> Result<String, fmt::Error> {
        #[cfg(not(feature = "enable_profiler"))]
        {
            Ok(
                "Performance profiling results are not available, because feature 'enable_profiler' wasn't defined!"
                    .to_owned(),
            )
        }
    }
    pub fn print_hot_path() -> Result<String, fmt::Error> {
        #[cfg(not(feature = "enable_profiler"))]
        {
            Ok(
                "Performance profiling results are not available, because feature 'enable_profiler' wasn't defined!"
                    .to_owned(),
            )
        }
    }
    struct Sample {
        count: u64,
        time: f64,
        children: FxHashSet<ScopeMark>,
    }
    impl Sample {
        pub fn collect(&mut self, time: f64) {
            self.time += time;
            self.count += 1;
        }
    }
    impl Default for Sample {
        fn default() -> Self {
            Self {
                count: 0,
                time: 0.0,
                children: Default::default(),
            }
        }
    }
    struct ScopeMark {
        parent_scope_hash: u64,
        function_name: &'static str,
        line: u32,
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ScopeMark {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.parent_scope_hash, state);
            ::core::hash::Hash::hash(&self.function_name, state);
            ::core::hash::Hash::hash(&self.line, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ScopeMark {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ScopeMark {
        #[inline]
        fn eq(&self, other: &ScopeMark) -> bool {
            self.parent_scope_hash == other.parent_scope_hash
                && self.function_name == other.function_name && self.line == other.line
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ScopeMark {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u64>;
            let _: ::core::cmp::AssertParamIsEq<&'static str>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ScopeMark {}
    #[automatically_derived]
    impl ::core::clone::Clone for ScopeMark {
        #[inline]
        fn clone(&self) -> ScopeMark {
            let _: ::core::clone::AssertParamIsClone<u64>;
            let _: ::core::clone::AssertParamIsClone<&'static str>;
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ScopeMark {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ScopeMark",
                "parent_scope_hash",
                &self.parent_scope_hash,
                "function_name",
                &self.function_name,
                "line",
                &&self.line,
            )
        }
    }
    struct Profiler {
        start_time: std::time::Instant,
        samples: FxHashMap<ScopeMark, Sample>,
        scope_stack: Vec<ScopeMark>,
    }
    const ENTRY_SCOPE_MARK: ScopeMark = ScopeMark {
        parent_scope_hash: 0,
        function_name: "EntryPoint",
        line: 0,
    };
    impl Default for Profiler {
        #[inline]
        fn default() -> Self {
            let entry_sample = Sample {
                count: 0,
                time: 0.0,
                children: FxHashSet::default(),
            };
            let mut samples = FxHashMap::default();
            samples.insert(ENTRY_SCOPE_MARK, entry_sample);
            Self {
                start_time: std::time::Instant::now(),
                samples,
                scope_stack: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([ENTRY_SCOPE_MARK]),
                ),
            }
        }
    }
    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = FxHasher::default();
        t.hash(&mut s);
        s.finish()
    }
    impl Profiler {
        fn enter_scope(&mut self, scope: &mut ScopeMark) {
            let parent_scope_mark = *self.scope_stack.last().unwrap();
            scope.parent_scope_hash = calculate_hash(&parent_scope_mark);
            self.scope_stack.push(*scope);
            self.samples.entry(*scope).or_default();
            self.samples.get_mut(&parent_scope_mark).unwrap().children.insert(*scope);
        }
        fn leave_scope(&mut self, scope: ScopeMark, elapsed: f64) {
            self.scope_stack.pop();
            self.samples.get_mut(&scope).unwrap().collect(elapsed);
        }
        fn print(&self, buffer: &mut String) -> fmt::Result {
            let full_time = (std::time::Instant::now() - self.start_time).as_secs_f64();
            self.recursive_print(buffer, &ENTRY_SCOPE_MARK, 0, full_time)?;
            buffer
                .write_fmt(
                    format_args!(
                        "=========================================================================================================\n",
                    ),
                )?;
            Ok(())
        }
        fn recursive_print(
            &self,
            buffer: &mut String,
            scope_mark: &ScopeMark,
            offset: usize,
            full_time: f64,
        ) -> fmt::Result {
            let sample = self.samples.get(scope_mark).unwrap();
            if scope_mark == &ENTRY_SCOPE_MARK {
                buffer
                    .write_fmt(
                        format_args!(
                            "=========================================================================================================\n",
                        ),
                    )?;
                buffer
                    .write_fmt(
                        format_args!(
                            "Profiling took {0} seconds. Please note that profiling itself takes time so results are not 100% accurate.\n",
                            full_time,
                        ),
                    )?;
                buffer.write_fmt(format_args!("Entry Point\n"))?;
            } else {
                buffer
                    .write_fmt(
                        format_args!(
                            "{0}{1:.4}% - {2} at line {3} was called {4} times and took {5} seconds individually.\n",
                            "\t".repeat(offset),
                            (sample.time / full_time) * 100.0,
                            scope_mark.function_name,
                            scope_mark.line,
                            sample.count,
                            sample.time,
                        ),
                    )?;
            }
            for child_scope in self
                .samples
                .get(scope_mark)
                .as_ref()
                .unwrap()
                .children
                .iter()
            {
                self.recursive_print(buffer, child_scope, offset + 1, full_time)?;
            }
            Ok(())
        }
        fn print_hot_path(&self, buffer: &mut String) -> fmt::Result {
            let full_time = (std::time::Instant::now() - self.start_time).as_secs_f64();
            self.print_hot_path_recursive(buffer, &ENTRY_SCOPE_MARK, 0, full_time)?;
            buffer
                .write_fmt(
                    format_args!(
                        "=========================================================================================================\n",
                    ),
                )?;
            Ok(())
        }
        fn print_hot_path_recursive(
            &self,
            buffer: &mut String,
            scope_mark: &ScopeMark,
            offset: usize,
            full_time: f64,
        ) -> fmt::Result {
            let sample = self.samples.get(scope_mark).unwrap();
            if scope_mark == &ENTRY_SCOPE_MARK {
                buffer
                    .write_fmt(
                        format_args!(
                            "=========================================================================================================\n",
                        ),
                    )?;
                buffer
                    .write_fmt(
                        format_args!(
                            "Showing hot path only! Profiling took {0} seconds. Please note that profiling itself takes time so results are not 100% accurate.\n",
                            full_time,
                        ),
                    )?;
                buffer.write_fmt(format_args!("Entry Point\n"))?;
            } else {
                buffer
                    .write_fmt(
                        format_args!(
                            "{0}{1:.4}% - {2} at line {3} was called {4} times and took {5} seconds individually.\n",
                            "\t".repeat(offset),
                            (sample.time / full_time) * 100.0,
                            scope_mark.function_name,
                            scope_mark.line,
                            sample.count,
                            sample.time,
                        ),
                    )?;
            }
            let mut hot = None;
            let mut hot_time = 0.0;
            for child_scope in self
                .samples
                .get(scope_mark)
                .as_ref()
                .unwrap()
                .children
                .iter()
            {
                let time = self.samples.get(child_scope).as_ref().unwrap().time;
                if time > hot_time {
                    hot_time = time;
                    hot = Some(*child_scope);
                }
            }
            if let Some(hot) = hot.as_ref() {
                self.print_hot_path_recursive(buffer, hot, offset + 1, full_time)?;
            }
            Ok(())
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct PROFILER {
        __private_field: (),
    }
    #[doc(hidden)]
    static PROFILER: PROFILER = PROFILER { __private_field: () };
    impl ::lazy_static::__Deref for PROFILER {
        type Target = Arc<Mutex<Profiler>>;
        fn deref(&self) -> &Arc<Mutex<Profiler>> {
            #[inline(always)]
            fn __static_ref_initialize() -> Arc<Mutex<Profiler>> {
                Arc::new(Mutex::new(Profiler::default()))
            }
            #[inline(always)]
            fn __stability() -> &'static Arc<Mutex<Profiler>> {
                static LAZY: ::lazy_static::lazy::Lazy<Arc<Mutex<Profiler>>> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for PROFILER {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub struct ScopeDefinition {
        scope: ScopeMark,
        start_time: std::time::Instant,
    }
    impl ScopeDefinition {
        #[inline]
        fn elapsed(&self) -> f64 {
            (std::time::Instant::now() - self.start_time).as_secs_f64()
        }
    }
    impl Drop for ScopeDefinition {
        fn drop(&mut self) {
            let elapsed = self.elapsed();
            PROFILER.lock().unwrap().leave_scope(self.scope, elapsed);
        }
    }
    #[inline]
    pub fn type_name_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }
}
pub mod quadtree {
    pub use crate::math::quadtree::*;
}
pub mod rectpack {
    pub use crate::math::pack::RectPacker;
}
pub mod reflect {
    //! Runtime reflection
    mod external_impls {
        //! `Reflect` implementations for external types othern than `std` types
        use fyrox_core_derive::impl_reflect;
        use nalgebra::*;
        use std::fmt::Debug;
        use crate::reflect::prelude::*;
        #[allow(warnings)]
        impl<T: 'static, R: Dim + 'static, C: Dim + 'static, S: 'static> Reflect
        for Matrix<T, R, C, S>
        where
            Self: 'static,
            S: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\external_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(
                    &[
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "data",
                            display_name: "Data",
                            doc: "",
                            read_only: false,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.data,
                            reflect_value: &self.data,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<S>(),
                        },
                    ],
                )
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[&self.data as &dyn Reflect])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [&mut self.data as &mut dyn Reflect])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "data" => Some(&self.data as &dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "data" => Some(&mut self.data as &mut dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(warnings)]
        impl<T: Debug, const R: usize, const C: usize> Reflect for ArrayStorage<T, R, C>
        where
            Self: 'static,
            [[T; R]; C]: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\external_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(
                    &[
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "0",
                            display_name: "0",
                            doc: "",
                            read_only: false,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.0,
                            reflect_value: &self.0,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<[[T; R]; C]>(),
                        },
                    ],
                )
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[&self.0 as &dyn Reflect])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [&mut self.0 as &mut dyn Reflect])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "0" => Some(&self.0 as &dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "0" => Some(&mut self.0 as &mut dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(warnings)]
        impl<T: Debug + 'static> Reflect for Unit<T>
        where
            Self: 'static,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\external_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(warnings)]
        impl<T: Debug> Reflect for Quaternion<T>
        where
            Self: 'static,
            Vector4<T>: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\external_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(
                    &[
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "coords",
                            display_name: "Coords",
                            doc: "",
                            read_only: false,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.coords,
                            reflect_value: &self.coords,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<Vector4<T>>(),
                        },
                    ],
                )
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[&self.coords as &dyn Reflect])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [&mut self.coords as &mut dyn Reflect])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "coords" => Some(&self.coords as &dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "coords" => Some(&mut self.coords as &mut dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
        }
    }
    mod std_impls {
        //! `Reflect` implementations for `std` types
        use crate::{
            delegate_reflect, reflect::{blank_reflect, prelude::*},
            sstorage::ImmutableString, uuid::Uuid,
        };
        use fyrox_core_derive::impl_reflect;
        use std::{
            any::Any, cell::{Cell, RefCell},
            collections::HashMap, fmt::Debug, hash::{BuildHasher, Hash},
            ops::{Deref, DerefMut, Range},
            rc::Rc, sync::Arc, time::{Duration, Instant},
        };
        impl Reflect for f32 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for f64 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for usize {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for u8 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for u16 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for u32 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for u64 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for isize {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for i8 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for i16 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for i32 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for i64 {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for bool {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for char {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for String {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for std::path::PathBuf {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for Duration {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for Instant {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl Reflect for ImmutableString {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl<T0: Reflect> Reflect for (T0,) {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl<T0: Reflect, T1: Reflect> Reflect for (T0, T1) {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl<T0: Reflect, T1: Reflect, T2: Reflect> Reflect for (T0, T1, T2) {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl<T0: Reflect, T1: Reflect, T2: Reflect, T3: Reflect> Reflect
        for (T0, T1, T2, T3) {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl<T0: Reflect, T1: Reflect, T2: Reflect, T3: Reflect, T4: Reflect> Reflect
        for (T0, T1, T2, T3, T4) {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        impl<const N: usize, T: Reflect> Reflect for [T; N] {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                func(Some(self))
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                func(Some(self))
            }
        }
        impl<const N: usize, T: Reflect> ReflectArray for [T; N] {
            fn reflect_index(&self, index: usize) -> Option<&dyn Reflect> {
                if let Some(item) = self.get(index) { Some(item) } else { None }
            }
            fn reflect_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
                if let Some(item) = self.get_mut(index) { Some(item) } else { None }
            }
            fn reflect_len(&self) -> usize {
                self.len()
            }
        }
        #[allow(warnings)]
        impl<T: Reflect + 'static> Reflect for Vec<T>
        where
            Self: 'static,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                func(Some(self))
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                func(Some(self))
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                func(Some(self))
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                func(Some(self))
            }
        }
        impl<T: Reflect + 'static> ReflectArray for Vec<T> {
            fn reflect_index(&self, index: usize) -> Option<&dyn Reflect> {
                self.get(index).map(|x| x as &dyn Reflect)
            }
            fn reflect_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect> {
                self.get_mut(index).map(|x| x as &mut dyn Reflect)
            }
            fn reflect_len(&self) -> usize {
                self.len()
            }
        }
        /// REMARK: `Reflect` is implemented for `Vec<T>` where `T: Reflect` only.
        impl<T: Reflect + 'static> ReflectList for Vec<T> {
            fn reflect_push(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<(), Box<dyn Reflect>> {
                self.push(*value.downcast::<T>()?);
                Ok(())
            }
            fn reflect_pop(&mut self) -> Option<Box<dyn Reflect>> {
                if let Some(item) = self.pop() { Some(Box::new(item)) } else { None }
            }
            fn reflect_remove(&mut self, index: usize) -> Option<Box<dyn Reflect>> {
                if index < self.len() {
                    Some(Box::new(self.remove(index)))
                } else {
                    None
                }
            }
            fn reflect_insert(
                &mut self,
                index: usize,
                value: Box<dyn Reflect>,
            ) -> Result<(), Box<dyn Reflect>> {
                self.insert(index, *value.downcast::<T>()?);
                Ok(())
            }
        }
        impl<K, V, S> Reflect for HashMap<K, V, S>
        where
            K: Reflect + Debug + Eq + Hash + 'static,
            V: Reflect + Debug + 'static,
            S: BuildHasher + 'static,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                func(Some(self))
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                func(Some(self))
            }
        }
        impl<K, V, S> ReflectHashMap for HashMap<K, V, S>
        where
            K: Reflect + Debug + Eq + Hash + 'static,
            V: Reflect + Debug + 'static,
            S: BuildHasher + 'static,
        {
            fn reflect_insert(
                &mut self,
                key: Box<dyn Reflect>,
                value: Box<dyn Reflect>,
            ) -> Option<Box<dyn Reflect>> {
                if let Ok(key) = key.downcast::<K>() {
                    if let Ok(value) = value.downcast::<V>() {
                        if let Some(previous) = self.insert(*key, *value) {
                            return Some(Box::new(previous));
                        }
                    }
                }
                None
            }
            fn reflect_len(&self) -> usize {
                self.len()
            }
            fn reflect_get(
                &self,
                key: &dyn Reflect,
                func: &mut dyn FnMut(Option<&dyn Reflect>),
            ) {
                key.downcast_ref::<
                        K,
                    >(
                    &mut (|result| match result {
                        Some(key) => {
                            match self.get(key) {
                                Some(value) => func(Some(value as &dyn Reflect)),
                                None => func(None),
                            }
                        }
                        None => func(None),
                    }),
                )
            }
            fn reflect_get_mut(
                &mut self,
                key: &dyn Reflect,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                key.downcast_ref::<
                        K,
                    >(
                    &mut (|result| match result {
                        Some(key) => {
                            match self.get_mut(key) {
                                Some(value) => func(Some(value as &mut dyn Reflect)),
                                None => func(None),
                            }
                        }
                        None => func(None),
                    }),
                )
            }
            fn reflect_get_nth_value_ref(&self, index: usize) -> Option<&dyn Reflect> {
                self.values().nth(index).map(|v| v as &dyn Reflect)
            }
            fn reflect_get_nth_value_mut(
                &mut self,
                index: usize,
            ) -> Option<&mut dyn Reflect> {
                self.values_mut().nth(index).map(|v| v as &mut dyn Reflect)
            }
            fn reflect_get_at(
                &self,
                index: usize,
            ) -> Option<(&dyn Reflect, &dyn Reflect)> {
                self.iter()
                    .nth(index)
                    .map(|(k, v)| (k as &dyn Reflect, v as &dyn Reflect))
            }
            fn reflect_get_at_mut(
                &mut self,
                index: usize,
            ) -> Option<(&dyn Reflect, &mut dyn Reflect)> {
                self.iter_mut()
                    .nth(index)
                    .map(|(k, v)| (k as &dyn Reflect, v as &mut dyn Reflect))
            }
            fn reflect_remove(
                &mut self,
                key: &dyn Reflect,
                func: &mut dyn FnMut(Option<Box<dyn Reflect>>),
            ) {
                key.downcast_ref::<
                        K,
                    >(
                    &mut (|result| match result {
                        Some(key) => {
                            func(
                                self
                                    .remove(key)
                                    .map(|value| Box::new(value) as Box<dyn Reflect>),
                            )
                        }
                        None => func(None),
                    }),
                )
            }
        }
        impl Reflect for () {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                self
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                func(if name == "self" { Some(self) } else { None })
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let this = std::mem::replace(self, value.take()?);
                Ok(Box::new(this))
            }
        }
        #[allow(warnings)]
        impl Reflect for Uuid
        where
            Self: 'static,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(warnings)]
        impl<T: Debug + Copy> Reflect for Cell<T>
        where
            Self: 'static,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(&[])
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(&mut [])
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(warnings)]
        impl<T> Reflect for Option<T>
        where
            Self: 'static,
            T: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                match self {
                    Option::Some(f0) => {
                        func(
                            &[
                                FieldInfo {
                                    owner_type_id: std::any::TypeId::of::<Self>(),
                                    name: "Some@0",
                                    display_name: "0",
                                    doc: "",
                                    read_only: false,
                                    immutable_collection: false,
                                    min_value: None,
                                    max_value: None,
                                    value: f0,
                                    reflect_value: f0,
                                    step: None,
                                    precision: None,
                                    description: "",
                                    type_name: std::any::type_name::<T>(),
                                },
                            ],
                        )
                    }
                    Option::None => func(&[]),
                    _ => func(&[]),
                }
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                match self {
                    Option::Some(f0) => func(&[f0 as &dyn Reflect]),
                    Option::None => func(&[]),
                    _ => func(&[]),
                }
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                match self {
                    Option::Some(f0) => func(&mut [f0 as &mut dyn Reflect]),
                    Option::None => func(&mut []),
                    _ => func(&mut []),
                }
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "Some@0" => {
                            match self {
                                Option::Some(f0) => Some(f0 as &dyn Reflect),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "Some@0" => {
                            match self {
                                Option::Some(f0) => Some(f0 as &mut dyn Reflect),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                };
                func(value)
            }
        }
        #[allow(warnings)]
        impl<Idx> Reflect for Range<Idx>
        where
            Self: 'static,
            Idx: Reflect,
            Idx: Reflect,
        {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                func(
                    &[
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "start",
                            display_name: "Start",
                            doc: "",
                            read_only: false,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.start,
                            reflect_value: &self.start,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<Idx>(),
                        },
                        FieldInfo {
                            owner_type_id: std::any::TypeId::of::<Self>(),
                            name: "end",
                            display_name: "End",
                            doc: "",
                            read_only: false,
                            immutable_collection: false,
                            min_value: None,
                            max_value: None,
                            value: &self.end,
                            reflect_value: &self.end,
                            step: None,
                            precision: None,
                            description: "",
                            type_name: std::any::type_name::<Idx>(),
                        },
                    ],
                )
            }
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let value = match value.take() {
                    Ok(x) => x,
                    Err(err) => return Err(err),
                };
                let this = std::mem::replace(self, value);
                Ok(Box::new(this))
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
                func(self)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
                func(self)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                func(self as &dyn Reflect)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                func(self as &mut dyn Reflect)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                func(&[&self.start as &dyn Reflect, &self.end as &dyn Reflect])
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                func(
                    &mut [
                        &mut self.start as &mut dyn Reflect,
                        &mut self.end as &mut dyn Reflect,
                    ],
                )
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let value = {
                    match name {
                        "start" => Some(&self.start as &dyn Reflect),
                        "end" => Some(&self.end as &dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let value = {
                    match name {
                        "start" => Some(&mut self.start as &mut dyn Reflect),
                        "end" => Some(&mut self.end as &mut dyn Reflect),
                        _ => None,
                    }
                };
                func(value)
            }
        }
        impl<T: ?Sized + Reflect> Reflect for Box<T> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                self.deref().type_name()
            }
            fn doc(&self) -> &'static str {
                self.deref().doc()
            }
            fn assembly_name(&self) -> &'static str {
                self.deref().assembly_name()
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                self.deref().fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                (*self).into_any()
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                self.deref().as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                self.deref_mut().as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                self.deref().as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                self.deref_mut().as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                self.deref_mut().set(value)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                self.deref().field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                self.deref_mut().field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                self.deref().as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                self.deref_mut().as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                self.deref().as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                self.deref_mut().as_list_mut(func)
            }
        }
        impl<T: Reflect> Reflect for parking_lot::Mutex<T> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.lock() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = { self.into_inner() };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.lock() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.lock() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.lock() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.lock() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.lock() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.lock() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.lock() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.lock() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.lock() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.lock() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.lock() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.lock() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.lock() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.lock() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.lock() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.lock() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.lock() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.lock() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for parking_lot::RwLock<T> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.write() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = { self.into_inner() };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.write() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.write() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.write() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.write() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.write() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.write() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.write() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.write() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.write() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.write() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.write() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.write() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.write() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.write() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.write() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.write() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.write() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.write() };
                guard.as_hash_map_mut(func)
            }
        }
        #[allow(clippy::mut_mutex_lock)]
        impl<T: Reflect> Reflect for std::sync::Mutex<T> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.lock().unwrap() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = { self.into_inner() };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.lock().unwrap() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.lock().unwrap() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.lock().unwrap() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.lock().unwrap() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.lock().unwrap() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.lock().unwrap() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.lock().unwrap() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.lock().unwrap() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.lock().unwrap() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.lock().unwrap() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.lock().unwrap() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.lock().unwrap() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for std::sync::RwLock<T> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.write().unwrap() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = { self.into_inner() };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.write().unwrap() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.write().unwrap() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.write().unwrap() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.write().unwrap() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.write().unwrap() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.write().unwrap() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.write().unwrap() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.write().unwrap() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.write().unwrap() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.write().unwrap() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.write().unwrap() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.write().unwrap() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for Arc<parking_lot::Mutex<T>> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.lock() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = {
                    Arc::into_inner(*self).expect("Value cannot be shared!").into_inner()
                };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.lock() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.lock() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.lock() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.lock() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.lock() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.lock() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.lock() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.lock() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.lock() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.lock() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.lock() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.lock() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.lock() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.lock() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.lock() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.lock() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.lock() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.lock() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for Arc<std::sync::Mutex<T>> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.lock().unwrap() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = {
                    Arc::into_inner(*self).expect("Value cannot be shared!").into_inner()
                };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.lock().unwrap() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.lock().unwrap() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.lock().unwrap() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.lock().unwrap() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.lock().unwrap() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.lock().unwrap() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.lock().unwrap() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.lock().unwrap() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.lock().unwrap() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.lock().unwrap() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.lock().unwrap() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.lock().unwrap() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.lock().unwrap() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for Arc<std::sync::RwLock<T>> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.write().unwrap() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = {
                    Arc::into_inner(*self).expect("Value cannot be shared!").into_inner()
                };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.write().unwrap() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.write().unwrap() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.write().unwrap() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.write().unwrap() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.write().unwrap() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.write().unwrap() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.write().unwrap() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.write().unwrap() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.write().unwrap() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.write().unwrap() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.write().unwrap() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.write().unwrap() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.write().unwrap() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for Arc<parking_lot::RwLock<T>> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.write() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = {
                    Arc::into_inner(*self).expect("Value cannot be shared!").into_inner()
                };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.write() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.write() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.write() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.write() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.write() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.write() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.write() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.write() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.write() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.write() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.write() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.write() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.write() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.write() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.write() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.write() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.write() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.write() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for RefCell<T> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.borrow_mut() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = { self.into_inner() };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.borrow_mut() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.borrow_mut() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.borrow_mut() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.borrow_mut() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.borrow_mut() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.borrow_mut() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.borrow_mut() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.borrow_mut() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.borrow_mut() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.borrow_mut() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.borrow_mut() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.borrow_mut() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_hash_map_mut(func)
            }
        }
        impl<T: Reflect> Reflect for Rc<RefCell<T>> {
            fn source_path() -> &'static str {
                "fyrox-core\\src\\reflect\\std_impls.rs"
            }
            fn type_name(&self) -> &'static str {
                std::any::type_name::<T>()
            }
            fn doc(&self) -> &'static str {
                ""
            }
            fn assembly_name(&self) -> &'static str {
                "fyrox-core"
            }
            fn type_assembly_name() -> &'static str {
                "fyrox-core"
            }
            fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
                let guard = { self.borrow_mut() };
                guard.fields_info(func)
            }
            fn into_any(self: Box<Self>) -> Box<dyn Any> {
                let inner = {
                    Rc::into_inner(*self).expect("Value cannot be shared!").into_inner()
                };
                Box::new(inner)
            }
            fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
                let guard = { self.borrow_mut() };
                (*guard).as_any(func)
            }
            fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
                let mut guard = { self.borrow_mut() };
                (*guard).as_any_mut(func)
            }
            fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
                let guard = { self.borrow_mut() };
                (*guard).as_reflect(func)
            }
            fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
                let mut guard = { self.borrow_mut() };
                (*guard).as_reflect_mut(func)
            }
            fn set(
                &mut self,
                value: Box<dyn Reflect>,
            ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
                let mut guard = { self.borrow_mut() };
                guard.set(value)
            }
            fn set_field(
                &mut self,
                field: &str,
                value: Box<dyn Reflect>,
                func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.set_field(field, value, func)
            }
            fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
                let guard = { self.borrow_mut() };
                guard.fields(func)
            }
            fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
                let mut guard = { self.borrow_mut() };
                guard.fields_mut(func)
            }
            fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
                let guard = { self.borrow_mut() };
                guard.field(name, func)
            }
            fn field_mut(
                &mut self,
                name: &str,
                func: &mut dyn FnMut(Option<&mut dyn Reflect>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.field_mut(name, func)
            }
            fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
                let guard = { self.borrow_mut() };
                guard.as_array(func)
            }
            fn as_array_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectArray>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_array_mut(func)
            }
            fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
                let guard = { self.borrow_mut() };
                guard.as_list(func)
            }
            fn as_list_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectList>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_list_mut(func)
            }
            fn as_inheritable_variable(
                &self,
                func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
            ) {
                let guard = { self.borrow_mut() };
                guard.as_inheritable_variable(func)
            }
            fn as_inheritable_variable_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_inheritable_variable_mut(func)
            }
            fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
                let guard = { self.borrow_mut() };
                guard.as_hash_map(func)
            }
            fn as_hash_map_mut(
                &mut self,
                func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
            ) {
                let mut guard = { self.borrow_mut() };
                guard.as_hash_map_mut(func)
            }
        }
    }
    pub use fyrox_core_derive::Reflect;
    use std::{
        any::{Any, TypeId},
        fmt::{self, Debug, Display, Formatter},
        mem::ManuallyDrop,
    };
    pub mod prelude {
        pub use super::{
            FieldInfo, Reflect, ReflectArray, ReflectHashMap, ReflectInheritableVariable,
            ReflectList, ResolvePath, SetFieldByPathError,
        };
    }
    /// A value of a field..
    pub trait FieldValue: Any + 'static {
        /// Casts `self` to a `&dyn Any`
        fn as_any(&self) -> &dyn Any;
    }
    impl<T: 'static> FieldValue for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    /// An error that can occur during "type casting"
    pub enum CastError {
        /// Given type does not match expected.
        TypeMismatch {
            /// A name of the field.
            property_name: String,
            /// Expected type identifier.
            expected_type_id: TypeId,
            /// Actual type identifier.
            actual_type_id: TypeId,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CastError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                CastError::TypeMismatch {
                    property_name: __self_0,
                    expected_type_id: __self_1,
                    actual_type_id: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "TypeMismatch",
                        "property_name",
                        __self_0,
                        "expected_type_id",
                        __self_1,
                        "actual_type_id",
                        &__self_2,
                    )
                }
            }
        }
    }
    pub struct FieldInfo<'a, 'b> {
        /// A type id of the owner of the property.
        pub owner_type_id: TypeId,
        /// A name of the property.
        pub name: &'b str,
        /// A human-readable name of the property.
        pub display_name: &'b str,
        /// Description of the property.
        pub description: &'b str,
        /// Type name of the property.
        pub type_name: &'b str,
        /// Doc comment content.
        pub doc: &'b str,
        /// An reference to the actual value of the property. This is "non-mangled" reference, which
        /// means that while `field/fields/field_mut/fields_mut` might return a reference to other value,
        /// than the actual field, the `value` is guaranteed to be a reference to the real value.
        pub value: &'a dyn FieldValue,
        /// A reference to the value casted to `Reflect`.
        pub reflect_value: &'a dyn Reflect,
        /// A property is not meant to be edited.
        pub read_only: bool,
        /// Only for dynamic collections (Vec, etc) - means that its size cannot be changed, however the
        /// _items_ of the collection can still be changed.
        pub immutable_collection: bool,
        /// A minimal value of the property. Works only with numeric properties!
        pub min_value: Option<f64>,
        /// A minimal value of the property. Works only with numeric properties!
        pub max_value: Option<f64>,
        /// A minimal value of the property. Works only with numeric properties!
        pub step: Option<f64>,
        /// Maximum amount of decimal places for a numeric property.
        pub precision: Option<usize>,
    }
    impl<'a, 'b> FieldInfo<'a, 'b> {
        /// Tries to cast a value to a given type.
        pub fn cast_value<T: 'static>(&self) -> Result<&T, CastError> {
            match self.value.as_any().downcast_ref::<T>() {
                Some(value) => Ok(value),
                None => {
                    Err(CastError::TypeMismatch {
                        property_name: self.name.to_string(),
                        expected_type_id: TypeId::of::<T>(),
                        actual_type_id: self.value.type_id(),
                    })
                }
            }
        }
    }
    impl<'a, 'b> fmt::Debug for FieldInfo<'a, 'b> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PropertyInfo")
                .field("owner_type_id", &self.owner_type_id)
                .field("name", &self.name)
                .field("display_name", &self.display_name)
                .field("value", &format_args!("{0:?}", self.value as *const _))
                .field("read_only", &self.read_only)
                .field("min_value", &self.min_value)
                .field("max_value", &self.max_value)
                .field("step", &self.step)
                .field("precision", &self.precision)
                .field("description", &self.description)
                .finish()
        }
    }
    impl<'a, 'b> PartialEq<Self> for FieldInfo<'a, 'b> {
        fn eq(&self, other: &Self) -> bool {
            let value_ptr_a = self.value as *const _ as *const ();
            let value_ptr_b = other.value as *const _ as *const ();
            self.owner_type_id == other.owner_type_id && self.name == other.name
                && self.display_name == other.display_name
                && std::ptr::eq(value_ptr_a, value_ptr_b)
                && self.read_only == other.read_only && self.min_value == other.min_value
                && self.max_value == other.max_value && self.step == other.step
                && self.precision == other.precision
                && self.description == other.description
        }
    }
    pub trait ReflectBase: Any + Debug {
        fn as_any_raw(&self) -> &dyn Any;
        fn as_any_raw_mut(&mut self) -> &mut dyn Any;
    }
    impl<T: Reflect> ReflectBase for T {
        fn as_any_raw(&self) -> &dyn Any {
            self
        }
        fn as_any_raw_mut(&mut self) -> &mut dyn Any {
            self
        }
    }
    /// Trait for runtime reflection
    ///
    /// Derive macro is available.
    ///
    /// # Type attributes
    /// - `#[reflect(hide_all)]`: Hide all fields, just like `Any`
    /// - `#[reflect(bounds)]`: Add type boundary for `Reflect` impl
    ///
    /// # Field attributes
    /// - `#[reflect(deref)]`: Delegate the field access with deref
    /// - `#[reflect(field = <method call>)]`
    /// - `#[reflect(field_mut = <method call>)]`
    ///
    /// # Additional Trait Bounds
    ///
    /// `Reflect` restricted to types that implement `Debug` trait, this is needed to convert the actual value
    /// to string. `Display` isn't used here, because it can't be derived and it is very tedious to implement it
    /// for every type that should support `Reflect` trait. It is a good compromise between development speed
    /// and the quality of the string output.
    pub trait Reflect: ReflectBase {
        fn source_path() -> &'static str
        where
            Self: Sized;
        fn type_name(&self) -> &'static str;
        fn doc(&self) -> &'static str;
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo]));
        fn into_any(self: Box<Self>) -> Box<dyn Any>;
        fn as_any(&self, func: &mut dyn FnMut(&dyn Any));
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any));
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect));
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect));
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>>;
        /// Returns a parent assembly name of the type that implements this trait. **WARNING:** You should use
        /// proc-macro (`#[derive(Reflect)]`) to ensure that this method will return correct assembly
        /// name. In other words - there's no guarantee, that any implementation other than proc-macro
        /// will return a correct name of the assembly. Alternatively, you can use `env!("CARGO_PKG_NAME")`
        /// as an implementation.
        fn assembly_name(&self) -> &'static str;
        /// Returns a parent assembly name of the type that implements this trait. **WARNING:** You should use
        /// proc-macro (`#[derive(Reflect)]`) to ensure that this method will return correct assembly
        /// name. In other words - there's no guarantee, that any implementation other than proc-macro
        /// will return a correct name of the assembly. Alternatively, you can use `env!("CARGO_PKG_NAME")`
        /// as an implementation.
        fn type_assembly_name() -> &'static str
        where
            Self: Sized;
        /// Calls user method specified with `#[reflect(setter = ..)]` or falls back to
        /// [`Reflect::field_mut`]
        #[allow(clippy::type_complexity)]
        fn set_field(
            &mut self,
            field: &str,
            value: Box<dyn Reflect>,
            func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
        ) {
            let mut opt_value = Some(value);
            self.field_mut(
                field,
                &mut (move |field| {
                    let value = opt_value.take().unwrap();
                    match field {
                        Some(f) => func(f.set(value)),
                        None => func(Err(value)),
                    };
                }),
            );
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(&mut [])
        }
        fn field(
            &self,
            #[allow(unused_variables)]
            name: &str,
            func: &mut dyn FnMut(Option<&dyn Reflect>),
        ) {
            func(None)
        }
        fn field_mut(
            &mut self,
            #[allow(unused_variables)]
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            func(None)
        }
        fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
            func(None)
        }
        fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
            func(None)
        }
        fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
            func(None)
        }
        fn as_list_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectList>)) {
            func(None)
        }
        fn as_inheritable_variable(
            &self,
            func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
        ) {
            func(None)
        }
        fn as_inheritable_variable_mut(
            &mut self,
            func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
        ) {
            func(None)
        }
        fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
            func(None)
        }
        fn as_hash_map_mut(
            &mut self,
            func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>),
        ) {
            func(None)
        }
    }
    /// [`Reflect`] sub trait for working with slices.
    pub trait ReflectArray: Reflect {
        fn reflect_index(&self, index: usize) -> Option<&dyn Reflect>;
        fn reflect_index_mut(&mut self, index: usize) -> Option<&mut dyn Reflect>;
        fn reflect_len(&self) -> usize;
    }
    /// [`Reflect`] sub trait for working with `Vec`-like types
    pub trait ReflectList: ReflectArray {
        fn reflect_push(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<(), Box<dyn Reflect>>;
        fn reflect_pop(&mut self) -> Option<Box<dyn Reflect>>;
        fn reflect_remove(&mut self, index: usize) -> Option<Box<dyn Reflect>>;
        fn reflect_insert(
            &mut self,
            index: usize,
            value: Box<dyn Reflect>,
        ) -> Result<(), Box<dyn Reflect>>;
    }
    pub trait ReflectHashMap: Reflect {
        fn reflect_insert(
            &mut self,
            key: Box<dyn Reflect>,
            value: Box<dyn Reflect>,
        ) -> Option<Box<dyn Reflect>>;
        fn reflect_len(&self) -> usize;
        fn reflect_get(
            &self,
            key: &dyn Reflect,
            func: &mut dyn FnMut(Option<&dyn Reflect>),
        );
        fn reflect_get_mut(
            &mut self,
            key: &dyn Reflect,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        );
        fn reflect_get_nth_value_ref(&self, index: usize) -> Option<&dyn Reflect>;
        fn reflect_get_nth_value_mut(
            &mut self,
            index: usize,
        ) -> Option<&mut dyn Reflect>;
        fn reflect_get_at(&self, index: usize) -> Option<(&dyn Reflect, &dyn Reflect)>;
        fn reflect_get_at_mut(
            &mut self,
            index: usize,
        ) -> Option<(&dyn Reflect, &mut dyn Reflect)>;
        fn reflect_remove(
            &mut self,
            key: &dyn Reflect,
            func: &mut dyn FnMut(Option<Box<dyn Reflect>>),
        );
    }
    pub trait ReflectInheritableVariable: Reflect + Debug {
        /// Tries to inherit a value from parent. It will succeed only if the current variable is
        /// not marked as modified.
        fn try_inherit(
            &mut self,
            parent: &dyn ReflectInheritableVariable,
            ignored_types: &[TypeId],
        ) -> Result<Option<Box<dyn Reflect>>, InheritError>;
        /// Resets modified flag from the variable.
        fn reset_modified_flag(&mut self);
        /// Returns current variable flags.
        fn flags(&self) -> VariableFlags;
        fn set_flags(&mut self, flags: VariableFlags);
        /// Returns true if value was modified.
        fn is_modified(&self) -> bool;
        /// Returns true if value equals to other's value.
        fn value_equals(&self, other: &dyn ReflectInheritableVariable) -> bool;
        /// Clones self value.
        fn clone_value_box(&self) -> Box<dyn Reflect>;
        /// Marks value as modified, so its value won't be overwritten during property inheritance.
        fn mark_modified(&mut self);
        /// Returns a mutable reference to wrapped value without marking the variable itself as modified.
        fn inner_value_mut(&mut self) -> &mut dyn Reflect;
        /// Returns a shared reference to wrapped value without marking the variable itself as modified.
        fn inner_value_ref(&self) -> &dyn Reflect;
    }
    /// An error returned from a failed path string query.
    pub enum ReflectPathError<'a> {
        UnclosedBrackets { s: &'a str },
        InvalidIndexSyntax { s: &'a str },
        UnknownField { s: &'a str },
        NoItemForIndex { s: &'a str },
        InvalidDowncast,
        NotAnArray,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for ReflectPathError<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ReflectPathError::UnclosedBrackets { s: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnclosedBrackets",
                        "s",
                        &__self_0,
                    )
                }
                ReflectPathError::InvalidIndexSyntax { s: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidIndexSyntax",
                        "s",
                        &__self_0,
                    )
                }
                ReflectPathError::UnknownField { s: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnknownField",
                        "s",
                        &__self_0,
                    )
                }
                ReflectPathError::NoItemForIndex { s: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "NoItemForIndex",
                        "s",
                        &__self_0,
                    )
                }
                ReflectPathError::InvalidDowncast => {
                    ::core::fmt::Formatter::write_str(f, "InvalidDowncast")
                }
                ReflectPathError::NotAnArray => {
                    ::core::fmt::Formatter::write_str(f, "NotAnArray")
                }
            }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for ReflectPathError<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for ReflectPathError<'a> {
        #[inline]
        fn eq(&self, other: &ReflectPathError<'a>) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ReflectPathError::UnclosedBrackets { s: __self_0 },
                        ReflectPathError::UnclosedBrackets { s: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    (
                        ReflectPathError::InvalidIndexSyntax { s: __self_0 },
                        ReflectPathError::InvalidIndexSyntax { s: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    (
                        ReflectPathError::UnknownField { s: __self_0 },
                        ReflectPathError::UnknownField { s: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    (
                        ReflectPathError::NoItemForIndex { s: __self_0 },
                        ReflectPathError::NoItemForIndex { s: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for ReflectPathError<'a> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
        }
    }
    impl<'a> Display for ReflectPathError<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                ReflectPathError::UnclosedBrackets { s } => {
                    f.write_fmt(format_args!("unclosed brackets: `{0}`", s))
                }
                ReflectPathError::InvalidIndexSyntax { s } => {
                    f.write_fmt(format_args!("not index syntax: `{0}`", s))
                }
                ReflectPathError::UnknownField { s } => {
                    f.write_fmt(format_args!("given unknown field: `{0}`", s))
                }
                ReflectPathError::NoItemForIndex { s } => {
                    f.write_fmt(format_args!("no item for index: `{0}`", s))
                }
                ReflectPathError::InvalidDowncast => {
                    f.write_fmt(
                        format_args!(
                            "failed to downcast to the target type after path resolution",
                        ),
                    )
                }
                ReflectPathError::NotAnArray => {
                    f.write_fmt(
                        format_args!(
                            "tried to resolve index access, but the reflect type does not implement list API",
                        ),
                    )
                }
            }
        }
    }
    pub trait ResolvePath {
        fn resolve_path<'p>(
            &self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&dyn Reflect, ReflectPathError<'p>>),
        );
        fn resolve_path_mut<'p>(
            &mut self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&mut dyn Reflect, ReflectPathError<'p>>),
        );
        fn get_resolve_path<'p, T: Reflect>(
            &self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&T, ReflectPathError<'p>>),
        ) {
            self.resolve_path(
                path,
                &mut (|resolve_result| {
                    match resolve_result {
                        Ok(value) => {
                            value
                                .downcast_ref(
                                    &mut (|result| {
                                        match result {
                                            Some(value) => {
                                                func(Ok(value));
                                            }
                                            None => {
                                                func(Err(ReflectPathError::InvalidDowncast));
                                            }
                                        };
                                    }),
                                );
                        }
                        Err(err) => {
                            func(Err(err));
                        }
                    };
                }),
            )
        }
        fn get_resolve_path_mut<'p, T: Reflect>(
            &mut self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&mut T, ReflectPathError<'p>>),
        ) {
            self.resolve_path_mut(
                path,
                &mut (|result| match result {
                    Ok(value) => {
                        value
                            .downcast_mut(
                                &mut (|result| match result {
                                    Some(value) => func(Ok(value)),
                                    None => func(Err(ReflectPathError::InvalidDowncast)),
                                }),
                            )
                    }
                    Err(err) => func(Err(err)),
                }),
            )
        }
    }
    impl<T: Reflect> ResolvePath for T {
        fn resolve_path<'p>(
            &self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&dyn Reflect, ReflectPathError<'p>>),
        ) {
            (self as &dyn Reflect).resolve_path(path, func)
        }
        fn resolve_path_mut<'p>(
            &mut self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&mut dyn Reflect, ReflectPathError<'p>>),
        ) {
            (self as &mut dyn Reflect).resolve_path_mut(path, func)
        }
    }
    /// Splits property path into individual components.
    pub fn path_to_components(path: &str) -> Vec<Component> {
        let mut components = Vec::new();
        let mut current_path = path;
        while let Ok((component, sub_path)) = Component::next(current_path) {
            if let Component::Field(field) = component {
                if field.is_empty() {
                    break;
                }
            }
            current_path = sub_path;
            components.push(component);
        }
        components
    }
    /// Helper methods over [`Reflect`] types
    pub trait GetField {
        fn get_field<T: 'static>(&self, name: &str, func: &mut dyn FnMut(Option<&T>));
        fn get_field_mut<T: 'static>(
            &mut self,
            _name: &str,
            func: &mut dyn FnMut(Option<&mut T>),
        );
    }
    impl<R: Reflect> GetField for R {
        fn get_field<T: 'static>(&self, name: &str, func: &mut dyn FnMut(Option<&T>)) {
            self.field(
                name,
                &mut (|field| match field {
                    None => func(None),
                    Some(reflect) => {
                        reflect.as_any(&mut (|any| func(any.downcast_ref())))
                    }
                }),
            )
        }
        fn get_field_mut<T: 'static>(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut T>),
        ) {
            self.field_mut(
                name,
                &mut (|field| match field {
                    None => func(None),
                    Some(reflect) => {
                        reflect.as_any_mut(&mut (|any| func(any.downcast_mut())))
                    }
                }),
            )
        }
    }
    unsafe fn make_fake_string_from_slice(string: &str) -> ManuallyDrop<String> {
        ManuallyDrop::new(
            String::from_utf8_unchecked(
                Vec::from_raw_parts(
                    string.as_bytes().as_ptr() as *mut _,
                    string.as_bytes().len(),
                    string.as_bytes().len(),
                ),
            ),
        )
    }
    fn try_fetch_by_str_path_ref(
        hash_map: &dyn ReflectHashMap,
        path: &str,
        func: &mut dyn FnMut(Option<&dyn Reflect>),
    ) {
        let fake_string_key = unsafe { make_fake_string_from_slice(path) };
        hash_map
            .reflect_get(
                &*fake_string_key,
                &mut (|result| match result {
                    Some(value) => func(Some(value)),
                    None => {
                        hash_map
                            .reflect_get(
                                &ImmutableString::new(path) as &dyn Reflect,
                                func,
                            )
                    }
                }),
            );
    }
    fn try_fetch_by_str_path_mut(
        hash_map: &mut dyn ReflectHashMap,
        path: &str,
        func: &mut dyn FnMut(Option<&mut dyn Reflect>),
    ) {
        let fake_string_key = unsafe { make_fake_string_from_slice(path) };
        let mut succeeded = true;
        hash_map
            .reflect_get_mut(
                &*fake_string_key,
                &mut (|result| match result {
                    Some(value) => func(Some(value)),
                    None => succeeded = false,
                }),
            );
        if !succeeded {
            hash_map.reflect_get_mut(&ImmutableString::new(path) as &dyn Reflect, func)
        }
    }
    /// Simple path parser / reflect path component
    pub enum Component<'p> {
        Field(&'p str),
        Index(&'p str),
    }
    impl<'p> Component<'p> {
        fn next(mut path: &'p str) -> Result<(Self, &'p str), ReflectPathError<'p>> {
            if path.bytes().next() == Some(b'.') {
                path = &path[1..];
            }
            let mut bytes = path.bytes().enumerate();
            while let Some((i, b)) = bytes.next() {
                if b == b'.' {
                    let (l, r) = path.split_at(i);
                    return Ok((Self::Field(l), &r[1..]));
                }
                if b == b'[' {
                    if i != 0 {
                        let (l, r) = path.split_at(i);
                        return Ok((Self::Field(l), r));
                    }
                    if let Some((end, _)) = bytes.find(|(_, b)| *b == b']') {
                        let l = &path[1..end];
                        let r = &path[end + 1..];
                        return Ok((Self::Index(l), r));
                    } else {
                        return Err(ReflectPathError::UnclosedBrackets {
                            s: path,
                        });
                    }
                }
            }
            Ok((Self::Field(path), ""))
        }
        fn resolve(
            &self,
            reflect: &dyn Reflect,
            func: &mut dyn FnMut(Result<&dyn Reflect, ReflectPathError<'p>>),
        ) {
            match self {
                Self::Field(path) => {
                    reflect
                        .field(
                            path,
                            &mut (|field| {
                                func(
                                    field
                                        .ok_or(ReflectPathError::UnknownField {
                                            s: path,
                                        }),
                                )
                            }),
                        )
                }
                Self::Index(path) => {
                    reflect
                        .as_array(
                            &mut (|result| match result {
                                Some(array) => {
                                    match path.parse::<usize>() {
                                        Ok(index) => {
                                            match array.reflect_index(index) {
                                                None => {
                                                    func(
                                                        Err(ReflectPathError::NoItemForIndex {
                                                            s: path,
                                                        }),
                                                    )
                                                }
                                                Some(value) => func(Ok(value)),
                                            }
                                        }
                                        Err(_) => {
                                            func(
                                                Err(ReflectPathError::InvalidIndexSyntax {
                                                    s: path,
                                                }),
                                            )
                                        }
                                    }
                                }
                                None => {
                                    reflect
                                        .as_hash_map(
                                            &mut (|result| match result {
                                                Some(hash_map) => {
                                                    try_fetch_by_str_path_ref(
                                                        hash_map,
                                                        path,
                                                        &mut (|result| {
                                                            func(
                                                                result
                                                                    .ok_or(ReflectPathError::NoItemForIndex {
                                                                        s: path,
                                                                    }),
                                                            )
                                                        }),
                                                    )
                                                }
                                                None => func(Err(ReflectPathError::NotAnArray)),
                                            }),
                                        )
                                }
                            }),
                        );
                }
            }
        }
        fn resolve_mut(
            &self,
            reflect: &mut dyn Reflect,
            func: &mut dyn FnMut(Result<&mut dyn Reflect, ReflectPathError<'p>>),
        ) {
            match self {
                Self::Field(path) => {
                    reflect
                        .field_mut(
                            path,
                            &mut (|field| {
                                func(
                                    field
                                        .ok_or(ReflectPathError::UnknownField {
                                            s: path,
                                        }),
                                )
                            }),
                        )
                }
                Self::Index(path) => {
                    let mut succeeded = true;
                    reflect
                        .as_array_mut(
                            &mut (|array| match array {
                                Some(list) => {
                                    match path.parse::<usize>() {
                                        Ok(index) => {
                                            match list.reflect_index_mut(index) {
                                                None => {
                                                    func(
                                                        Err(ReflectPathError::NoItemForIndex {
                                                            s: path,
                                                        }),
                                                    )
                                                }
                                                Some(value) => func(Ok(value)),
                                            }
                                        }
                                        Err(_) => {
                                            func(
                                                Err(ReflectPathError::InvalidIndexSyntax {
                                                    s: path,
                                                }),
                                            )
                                        }
                                    }
                                }
                                None => succeeded = false,
                            }),
                        );
                    if !succeeded {
                        reflect
                            .as_hash_map_mut(
                                &mut (|result| match result {
                                    Some(hash_map) => {
                                        try_fetch_by_str_path_mut(
                                            hash_map,
                                            path,
                                            &mut (|result| {
                                                func(
                                                    result
                                                        .ok_or(ReflectPathError::NoItemForIndex {
                                                            s: path,
                                                        }),
                                                )
                                            }),
                                        )
                                    }
                                    None => func(Err(ReflectPathError::NotAnArray)),
                                }),
                            )
                    }
                }
            }
        }
    }
    impl ResolvePath for dyn Reflect {
        fn resolve_path<'p>(
            &self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&dyn Reflect, ReflectPathError<'p>>),
        ) {
            match Component::next(path) {
                Ok((component, r)) => {
                    component
                        .resolve(
                            self,
                            &mut (|result| match result {
                                Ok(child) => {
                                    if r.is_empty() {
                                        func(Ok(child))
                                    } else {
                                        child.resolve_path(r, func)
                                    }
                                }
                                Err(err) => func(Err(err)),
                            }),
                        )
                }
                Err(err) => func(Err(err)),
            }
        }
        fn resolve_path_mut<'p>(
            &mut self,
            path: &'p str,
            func: &mut dyn FnMut(Result<&mut dyn Reflect, ReflectPathError<'p>>),
        ) {
            match Component::next(path) {
                Ok((component, r)) => {
                    component
                        .resolve_mut(
                            self,
                            &mut (|result| match result {
                                Ok(child) => {
                                    if r.is_empty() {
                                        func(Ok(child))
                                    } else {
                                        child.resolve_path_mut(r, func)
                                    }
                                }
                                Err(err) => func(Err(err)),
                            }),
                        )
                }
                Err(err) => func(Err(err)),
            }
        }
    }
    pub enum SetFieldByPathError<'p> {
        InvalidPath { value: Box<dyn Reflect>, reason: ReflectPathError<'p> },
        InvalidValue(Box<dyn Reflect>),
    }
    /// Type-erased API
    impl dyn Reflect {
        pub fn downcast<T: Reflect>(
            self: Box<dyn Reflect>,
        ) -> Result<Box<T>, Box<dyn Reflect>> {
            if self.is::<T>() {
                Ok(self.into_any().downcast().unwrap())
            } else {
                Err(self)
            }
        }
        pub fn take<T: Reflect>(self: Box<dyn Reflect>) -> Result<T, Box<dyn Reflect>> {
            self.downcast::<T>().map(|value| *value)
        }
        #[inline]
        pub fn is<T: Reflect>(&self) -> bool {
            self.type_id() == TypeId::of::<T>()
        }
        #[inline]
        pub fn downcast_ref<T: Reflect>(&self, func: &mut dyn FnMut(Option<&T>)) {
            self.as_any(&mut (|any| func(any.downcast_ref::<T>())))
        }
        #[inline]
        pub fn downcast_mut<T: Reflect>(
            &mut self,
            func: &mut dyn FnMut(Option<&mut T>),
        ) {
            self.as_any_mut(&mut (|any| func(any.downcast_mut::<T>())))
        }
        /// Sets a field by its path in the given entity. This method always uses [`Reflect::set_field`] which means,
        /// that it will always call custom property setters.
        #[inline]
        pub fn set_field_by_path<'p>(
            &mut self,
            path: &'p str,
            value: Box<dyn Reflect>,
            func: &mut dyn FnMut(Result<Box<dyn Reflect>, SetFieldByPathError<'p>>),
        ) {
            if let Some(separator_position) = path.rfind('.') {
                let mut opt_value = Some(value);
                let parent_path = &path[..separator_position];
                let field = &path[(separator_position + 1)..];
                self.resolve_path_mut(
                    parent_path,
                    &mut (|result| match result {
                        Err(reason) => {
                            func(
                                Err(SetFieldByPathError::InvalidPath {
                                    reason,
                                    value: opt_value.take().unwrap(),
                                }),
                            );
                        }
                        Ok(property) => {
                            property
                                .set_field(
                                    field,
                                    opt_value.take().unwrap(),
                                    &mut (|result| match result {
                                        Ok(value) => func(Ok(value)),
                                        Err(e) => func(Err(SetFieldByPathError::InvalidValue(e))),
                                    }),
                                )
                        }
                    }),
                );
            } else {
                self.set_field(
                    path,
                    value,
                    &mut (|result| match result {
                        Ok(value) => func(Ok(value)),
                        Err(e) => func(Err(SetFieldByPathError::InvalidValue(e))),
                    }),
                );
            }
        }
        pub fn enumerate_fields_recursively<F>(
            &self,
            func: &mut F,
            ignored_types: &[TypeId],
        )
        where
            F: FnMut(&str, Option<&FieldInfo>, &dyn Reflect),
        {
            self.enumerate_fields_recursively_internal("", None, func, ignored_types)
        }
        fn enumerate_fields_recursively_internal<F>(
            &self,
            path: &str,
            field_info: Option<&FieldInfo>,
            func: &mut F,
            ignored_types: &[TypeId],
        )
        where
            F: FnMut(&str, Option<&FieldInfo>, &dyn Reflect),
        {
            if ignored_types.contains(&self.type_id()) {
                return;
            }
            func(path, field_info, self);
            let mut done = false;
            self.as_inheritable_variable(
                &mut (|variable| {
                    if let Some(variable) = variable {
                        variable
                            .inner_value_ref()
                            .enumerate_fields_recursively_internal(
                                path,
                                field_info,
                                func,
                                ignored_types,
                            );
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.as_array(
                &mut (|array| {
                    if let Some(array) = array {
                        for i in 0..array.reflect_len() {
                            if let Some(item) = array.reflect_index(i) {
                                let item_path = {
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}[{1}]", path, i),
                                    );
                                    res
                                };
                                item.enumerate_fields_recursively_internal(
                                    &item_path,
                                    field_info,
                                    func,
                                    ignored_types,
                                );
                            }
                        }
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.as_hash_map(
                &mut (|hash_map| {
                    if let Some(hash_map) = hash_map {
                        for i in 0..hash_map.reflect_len() {
                            if let Some((key, value)) = hash_map.reflect_get_at(i) {
                                let mut key_str = {
                                    let res = ::alloc::fmt::format(format_args!("{0:?}", key));
                                    res
                                };
                                let mut is_key_string = false;
                                key.downcast_ref::<
                                        String,
                                    >(&mut (|string| is_key_string |= string.is_some()));
                                key.downcast_ref::<
                                        ImmutableString,
                                    >(&mut (|string| { is_key_string |= string.is_some() }));
                                if is_key_string {
                                    key_str.remove(0);
                                    key_str.pop();
                                }
                                let item_path = {
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}[{1}]", path, key_str),
                                    );
                                    res
                                };
                                value
                                    .enumerate_fields_recursively_internal(
                                        &item_path,
                                        field_info,
                                        func,
                                        ignored_types,
                                    );
                            }
                        }
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.fields_info(
                &mut (|fields| {
                    for field in fields {
                        let compound_path;
                        let field_path = if path.is_empty() {
                            field.name
                        } else {
                            compound_path = {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}.{1}", path, field.name),
                                );
                                res
                            };
                            &compound_path
                        };
                        field
                            .reflect_value
                            .enumerate_fields_recursively_internal(
                                field_path,
                                Some(field),
                                func,
                                ignored_types,
                            );
                    }
                }),
            )
        }
        pub fn apply_recursively<F>(&self, func: &mut F, ignored_types: &[TypeId])
        where
            F: FnMut(&dyn Reflect),
        {
            if ignored_types.contains(&(*self).type_id()) {
                return;
            }
            func(self);
            let mut done = false;
            self.as_inheritable_variable(
                &mut (|variable| {
                    if let Some(variable) = variable {
                        variable
                            .inner_value_ref()
                            .apply_recursively(func, ignored_types);
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.as_array(
                &mut (|array| {
                    if let Some(array) = array {
                        for i in 0..array.reflect_len() {
                            if let Some(item) = array.reflect_index(i) {
                                item.apply_recursively(func, ignored_types);
                            }
                        }
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.as_hash_map(
                &mut (|hash_map| {
                    if let Some(hash_map) = hash_map {
                        for i in 0..hash_map.reflect_len() {
                            if let Some(item) = hash_map.reflect_get_nth_value_ref(i) {
                                item.apply_recursively(func, ignored_types);
                            }
                        }
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.fields(
                &mut (|fields| {
                    for field in fields {
                        field.apply_recursively(func, ignored_types);
                    }
                }),
            )
        }
        pub fn apply_recursively_mut<F>(
            &mut self,
            func: &mut F,
            ignored_types: &[TypeId],
        )
        where
            F: FnMut(&mut dyn Reflect),
        {
            if ignored_types.contains(&(*self).type_id()) {
                return;
            }
            func(self);
            let mut done = false;
            self.as_inheritable_variable_mut(
                &mut (|variable| {
                    if let Some(variable) = variable {
                        variable
                            .inner_value_mut()
                            .apply_recursively_mut(func, ignored_types);
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.as_array_mut(
                &mut (|array| {
                    if let Some(array) = array {
                        for i in 0..array.reflect_len() {
                            if let Some(item) = array.reflect_index_mut(i) {
                                item.apply_recursively_mut(func, ignored_types);
                            }
                        }
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.as_hash_map_mut(
                &mut (|hash_map| {
                    if let Some(hash_map) = hash_map {
                        for i in 0..hash_map.reflect_len() {
                            if let Some(item) = hash_map.reflect_get_nth_value_mut(i) {
                                item.apply_recursively_mut(func, ignored_types);
                            }
                        }
                        done = true;
                    }
                }),
            );
            if done {
                return;
            }
            self.fields_mut(
                &mut (|fields| {
                    for field in fields {
                        (*field).apply_recursively_mut(func, ignored_types);
                    }
                }),
            )
        }
    }
    pub fn is_path_to_array_element(path: &str) -> bool {
        path.chars().last().map_or(false, |c| c == ']')
    }
    impl dyn ReflectList {
        pub fn get_reflect_index<T: Reflect + 'static>(
            &self,
            index: usize,
            func: &mut dyn FnMut(Option<&T>),
        ) {
            if let Some(reflect) = self.reflect_index(index) {
                reflect.downcast_ref(func)
            } else {
                func(None)
            }
        }
        pub fn get_reflect_index_mut<T: Reflect + 'static>(
            &mut self,
            index: usize,
            func: &mut dyn FnMut(Option<&mut T>),
        ) {
            if let Some(reflect) = self.reflect_index_mut(index) {
                reflect.downcast_mut(func)
            } else {
                func(None)
            }
        }
    }
    use crate::sstorage::ImmutableString;
    use crate::variable::{InheritError, VariableFlags};
    pub use blank_reflect;
    pub use delegate_reflect;
}
pub mod sparse {
    use std::{fmt::Debug, sync::atomic::{AtomicUsize, Ordering}};
    pub struct AtomicIndex {
        index: AtomicUsize,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AtomicIndex {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "AtomicIndex",
                "index",
                &&self.index,
            )
        }
    }
    impl Clone for AtomicIndex {
        fn clone(&self) -> Self {
            Self {
                index: AtomicUsize::new(self.index.load(Ordering::SeqCst)),
            }
        }
    }
    impl Default for AtomicIndex {
        fn default() -> Self {
            Self::unassigned()
        }
    }
    impl AtomicIndex {
        pub const UNASSIGNED_INDEX: usize = usize::MAX;
        pub fn unassigned() -> Self {
            Self {
                index: AtomicUsize::new(Self::UNASSIGNED_INDEX),
            }
        }
        fn new(index: usize) -> Self {
            Self {
                index: AtomicUsize::new(index),
            }
        }
        pub fn set(&self, index: usize) {
            self.index.store(index, Ordering::SeqCst)
        }
        pub fn get(&self) -> usize {
            self.index.load(Ordering::SeqCst)
        }
    }
    pub struct SparseBuffer<T> {
        vec: Vec<Option<T>>,
        free: Vec<usize>,
    }
    impl<T> Default for SparseBuffer<T> {
        fn default() -> Self {
            Self {
                vec: Default::default(),
                free: Default::default(),
            }
        }
    }
    impl<T: Clone> Clone for SparseBuffer<T> {
        fn clone(&self) -> Self {
            Self {
                vec: self.vec.clone(),
                free: self.free.clone(),
            }
        }
    }
    impl<T> SparseBuffer<T> {
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                vec: Vec::with_capacity(capacity),
                free: ::alloc::vec::Vec::new(),
            }
        }
        pub fn spawn(&mut self, payload: T) -> AtomicIndex {
            match self.free.pop() {
                Some(free) => {
                    let old = self.vec[free].replace(payload);
                    if true {
                        if !old.is_none() {
                            ::core::panicking::panic("assertion failed: old.is_none()")
                        }
                    }
                    AtomicIndex::new(free)
                }
                None => {
                    let index = AtomicIndex::new(self.vec.len());
                    self.vec.push(Some(payload));
                    index
                }
            }
        }
        pub fn free(&mut self, index: &AtomicIndex) -> Option<T> {
            self.free_raw(index.get())
        }
        pub fn free_raw(&mut self, index: usize) -> Option<T> {
            match self.vec.get_mut(index) {
                Some(entry) => {
                    match entry.take() {
                        Some(payload) => {
                            self.free.push(index);
                            Some(payload)
                        }
                        None => None,
                    }
                }
                None => None,
            }
        }
        pub fn len(&self) -> usize {
            self.vec.len()
        }
        pub fn is_empty(&self) -> bool {
            self.filled() == 0
        }
        pub fn filled(&self) -> usize {
            self.vec.len() - self.free.len()
        }
        pub fn is_index_valid(&self, index: &AtomicIndex) -> bool {
            self.get(index).is_some()
        }
        pub fn get(&self, index: &AtomicIndex) -> Option<&T> {
            self.get_raw(index.get())
        }
        pub fn get_mut(&mut self, index: &AtomicIndex) -> Option<&mut T> {
            self.get_mut_raw(index.get())
        }
        pub fn get_raw(&self, index: usize) -> Option<&T> {
            self.vec.get(index).and_then(|entry| entry.as_ref())
        }
        pub fn get_mut_raw(&mut self, index: usize) -> Option<&mut T> {
            self.vec.get_mut(index).and_then(|entry| entry.as_mut())
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.vec.iter().filter_map(|entry| entry.as_ref())
        }
        pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
            self.vec.iter_mut().filter_map(|entry| entry.as_mut())
        }
        pub fn clear(&mut self) {
            self.vec.clear();
            self.free.clear();
        }
    }
}
pub mod sstorage {
    //! Immutable string + immutable string storage. See docs of [`ImmutableString`] and
    //! [`ImmutableStringStorage`] for more info.
    #![warn(missing_docs)]
    use crate::{
        parking_lot::Mutex, uuid_provider, visitor::{Visit, VisitResult, Visitor},
    };
    use fxhash::{FxHashMap, FxHasher};
    pub use fyrox_core_derive::TypeUuidProvider;
    use serde::{Deserialize, Serialize};
    use std::{
        fmt::{Debug, Display, Formatter},
        hash::{Hash, Hasher},
        ops::Deref, sync::Arc,
    };
    struct State {
        string: String,
        hash: u64,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for State {
        #[inline]
        fn clone(&self) -> State {
            State {
                string: ::core::clone::Clone::clone(&self.string),
                hash: ::core::clone::Clone::clone(&self.hash),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for State {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "State",
                "string",
                &self.string,
                "hash",
                &&self.hash,
            )
        }
    }
    /// Immutable string is a string with constant content. Immutability gives some nice properties:
    ///
    /// - Address of the string could be used as a hash, which improves hashing performance dramatically
    /// and basically making it constant in terms of complexity (O(1))
    /// - Equality comparison becomes constant in terms of complexity.
    /// - Uniqueness guarantees - means that calling multiple times will allocate memory only once
    /// `ImmutableString::new("foo")` and in consecutive calls existing string will be used.
    ///
    /// # Use cases
    ///
    /// Most common use case for immutable strings is hash map keys in performance-critical places.
    pub struct ImmutableString(Arc<State>);
    #[automatically_derived]
    impl ::core::clone::Clone for ImmutableString {
        #[inline]
        fn clone(&self) -> ImmutableString {
            ImmutableString(::core::clone::Clone::clone(&self.0))
        }
    }
    impl crate::type_traits::TypeUuidProvider for ImmutableString {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "452caac1-19f7-43d6-9e33-92c2c9163332",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl Display for ImmutableString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0.string.as_ref())
        }
    }
    impl Debug for ImmutableString {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&self.0.string, f)
        }
    }
    impl Visit for ImmutableString {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut string = self.0.string.clone();
            string.visit(name, visitor)?;
            if visitor.is_reading() {
                *self = SSTORAGE.lock().insert(string);
            }
            Ok(())
        }
    }
    impl Serialize for ImmutableString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> Deserialize<'de> for ImmutableString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(
                ImmutableString::new(
                    deserializer.deserialize_string(ImmutableStringVisitor {})?,
                ),
            )
        }
    }
    struct ImmutableStringVisitor {}
    impl serde::de::Visitor<'_> for ImmutableStringVisitor {
        type Value = ImmutableString;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_fmt(format_args!("a string"))
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(ImmutableString::new(v))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v.into())
        }
    }
    impl Default for ImmutableString {
        fn default() -> Self {
            Self::new("")
        }
    }
    impl AsRef<str> for ImmutableString {
        fn as_ref(&self) -> &str {
            self.deref()
        }
    }
    impl ImmutableString {
        /// Creates new immutable string from given string slice.
        ///
        /// # Performance
        ///
        /// This method has amortized O(1) complexity, in worst case (when there is no such string
        /// in backing storage) it allocates memory which could lead to complexity defined by current
        /// memory allocator.
        #[inline]
        pub fn new<S: AsRef<str>>(string: S) -> ImmutableString {
            SSTORAGE.lock().insert(string)
        }
        /// Returns unique identifier of the string. Keep in mind that uniqueness is guaranteed only
        /// for a single session, uniqueness is not preserved between application runs.
        #[inline]
        pub fn id(&self) -> u64 {
            self.0.hash
        }
        /// Clones content of inner immutable string to a mutable string.
        #[inline]
        pub fn to_mutable(&self) -> String {
            self.0.string.clone()
        }
        /// Get a reference to the inner str.
        pub fn as_str(&self) -> &str {
            self.deref()
        }
    }
    impl From<&str> for ImmutableString {
        fn from(value: &str) -> Self {
            Self::new(value)
        }
    }
    impl From<String> for ImmutableString {
        fn from(value: String) -> Self {
            SSTORAGE.lock().insert_owned(value)
        }
    }
    impl Deref for ImmutableString {
        type Target = str;
        #[inline]
        fn deref(&self) -> &Self::Target {
            self.0.string.as_ref()
        }
    }
    impl Hash for ImmutableString {
        #[inline]
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u64(self.id())
        }
    }
    impl PartialEq for ImmutableString {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.id() == other.id()
        }
    }
    impl Eq for ImmutableString {}
    /// Immutable string storage is a backing storage for every immutable string in the application,
    /// storage is a singleton. In normal circumstances you should never use it directly.
    pub struct ImmutableStringStorage {
        vec: FxHashMap<u64, Arc<State>>,
    }
    #[automatically_derived]
    impl ::core::default::Default for ImmutableStringStorage {
        #[inline]
        fn default() -> ImmutableStringStorage {
            ImmutableStringStorage {
                vec: ::core::default::Default::default(),
            }
        }
    }
    impl ImmutableStringStorage {
        #[inline]
        fn insert<S: AsRef<str>>(&mut self, string: S) -> ImmutableString {
            let mut hasher = FxHasher::default();
            string.as_ref().hash(&mut hasher);
            let hash = hasher.finish();
            if let Some(existing) = self.vec.get(&hash) {
                ImmutableString(existing.clone())
            } else {
                let immutable = Arc::new(State {
                    string: string.as_ref().to_owned(),
                    hash,
                });
                self.vec.insert(hash, immutable.clone());
                ImmutableString(immutable)
            }
        }
        /// Insert without copying the given String.
        #[inline]
        fn insert_owned(&mut self, string: String) -> ImmutableString {
            let mut hasher = FxHasher::default();
            string.hash(&mut hasher);
            let hash = hasher.finish();
            if let Some(existing) = self.vec.get(&hash) {
                ImmutableString(existing.clone())
            } else {
                let immutable = Arc::new(State { string, hash });
                self.vec.insert(hash, immutable.clone());
                ImmutableString(immutable)
            }
        }
    }
    impl ImmutableStringStorage {
        /// Returns total amount of immutable strings in the storage.
        pub fn entry_count() -> usize {
            SSTORAGE.lock().vec.len()
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct SSTORAGE {
        __private_field: (),
    }
    #[doc(hidden)]
    static SSTORAGE: SSTORAGE = SSTORAGE { __private_field: () };
    impl ::lazy_static::__Deref for SSTORAGE {
        type Target = Arc<Mutex<ImmutableStringStorage>>;
        fn deref(&self) -> &Arc<Mutex<ImmutableStringStorage>> {
            #[inline(always)]
            fn __static_ref_initialize() -> Arc<Mutex<ImmutableStringStorage>> {
                Arc::new(Mutex::new(ImmutableStringStorage::default()))
            }
            #[inline(always)]
            fn __stability() -> &'static Arc<Mutex<ImmutableStringStorage>> {
                static LAZY: ::lazy_static::lazy::Lazy<
                    Arc<Mutex<ImmutableStringStorage>>,
                > = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for SSTORAGE {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
}
pub mod task {
    #[cfg(not(target_arch = "wasm32"))]
    use crate::futures::executor::ThreadPool;
    use parking_lot::Mutex;
    use std::{any::Any, future::Future, sync::mpsc::{self, Receiver, Sender}};
    use uuid::Uuid;
    #[cfg(not(target_arch = "wasm32"))]
    pub trait AsyncTaskResult: Any + Send + 'static {
        fn into_any(self: Box<Self>) -> Box<dyn Any>;
    }
    #[cfg(not(target_arch = "wasm32"))]
    impl<T> AsyncTaskResult for T
    where
        T: Any + Send + 'static,
    {
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub trait AsyncTask<R: AsyncTaskResult>: Future<Output = R> + Send + 'static {}
    #[cfg(not(target_arch = "wasm32"))]
    impl<T, R: AsyncTaskResult> AsyncTask<R> for T
    where
        T: Future<Output = R> + Send + 'static,
    {}
    impl dyn AsyncTaskResult {
        pub fn downcast<T: AsyncTaskResult>(
            self: Box<Self>,
        ) -> Result<Box<T>, Box<dyn Any>> {
            self.into_any().downcast()
        }
    }
    pub struct TaskResult {
        pub id: Uuid,
        pub payload: Box<dyn AsyncTaskResult>,
    }
    pub struct TaskPool {
        #[cfg(not(target_arch = "wasm32"))]
        thread_pool: ThreadPool,
        sender: Sender<TaskResult>,
        receiver: Mutex<Receiver<TaskResult>>,
    }
    impl Default for TaskPool {
        fn default() -> Self {
            Self::new()
        }
    }
    impl TaskPool {
        #[inline]
        pub fn new() -> Self {
            let (sender, receiver) = mpsc::channel();
            Self {
                #[cfg(not(target_arch = "wasm32"))]
                thread_pool: ThreadPool::new().unwrap(),
                sender,
                receiver: Mutex::new(receiver),
            }
        }
        #[inline]
        #[cfg(not(target_arch = "wasm32"))]
        pub fn spawn_task<F>(&self, future: F)
        where
            F: Future<Output = ()> + Send + 'static,
        {
            self.thread_pool.spawn_ok(future);
        }
        #[inline]
        pub fn spawn_with_result<F, T>(&self, future: F) -> Uuid
        where
            F: AsyncTask<T>,
            T: AsyncTaskResult,
        {
            let id = Uuid::new_v4();
            let sender = self.sender.clone();
            self.spawn_task(async move {
                let result = future.await;
                sender
                    .send(TaskResult {
                        id,
                        payload: Box::new(result),
                    })
                    .unwrap();
            });
            id
        }
        #[inline]
        pub fn next_task_result(&self) -> Option<TaskResult> {
            self.receiver.lock().try_recv().ok()
        }
    }
}
pub mod type_traits {
    pub use fyrox_core_derive::ComponentProvider;
    pub use fyrox_core_derive::TypeUuidProvider;
    use std::any::{Any, TypeId};
    use std::path::PathBuf;
    use uuid::Uuid;
    pub mod prelude {
        pub use super::{combine_uuids, ComponentProvider, TypeUuidProvider};
        pub use uuid::{uuid, Uuid};
    }
    /// A trait for an entity that has unique type identifier.
    pub trait TypeUuidProvider: Sized {
        /// Return type UUID.
        fn type_uuid() -> Uuid;
    }
    impl crate::type_traits::TypeUuidProvider for u8 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "7a8c337c-0219-466b-92b5-81460fa9c836",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for i8 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "3036f00e-5986-4ac3-8763-19e51d0889d7",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for u16 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "c662169d-cc3b-453c-bdf3-e0104ac3b966",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for i16 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "abce35a9-5e7b-4f7e-a729-2620a9806a6b",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for u32 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "8c4d2541-76a5-4dd8-9eb1-10222d2d6912",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for i32 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "7413ddd4-71ce-484d-a808-4f3479f5712d",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for u64 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "d1a45bd5-5066-4b28-b103-95c59c230e77",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for i64 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "35b89368-805f-486d-b3b1-fd3e86b5d645",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for f32 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "479e29c6-85fd-4bb8-b311-7b98793b8bf6",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for f64 {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "dac09d54-d069-47f4-aa0e-aa0057cc2b52",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for usize {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "620e24e3-fb51-48c6-a885-91d65135c5c9",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for isize {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "0a06591a-1c66-4299-ba6f-2b205b795575",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for bool {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "3b104074-9d39-4a2b-b974-da8cc1759fe8",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for PathBuf {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "3b104074-9d39-4a2b-b974-da8cc1759666",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl crate::type_traits::TypeUuidProvider for String {
        fn type_uuid() -> crate::uuid::Uuid {
            {
                const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                    "3b104074-9d39-4a2b-b974-da8cc1759999",
                ) {
                    ::uuid::__macro_support::Ok(u) => u,
                    ::uuid::__macro_support::Err(_) => {
                        ::std::rt::begin_panic("invalid UUID");
                    }
                };
                OUTPUT
            }
        }
    }
    impl<T: TypeUuidProvider> TypeUuidProvider for Option<T> {
        fn type_uuid() -> Uuid {
            combine_uuids(
                {
                    const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                        "ffe06d3b-0d07-42cd-886b-5248f6ca7f7d",
                    ) {
                        ::uuid::__macro_support::Ok(u) => u,
                        ::uuid::__macro_support::Err(_) => {
                            ::std::rt::begin_panic("invalid UUID");
                        }
                    };
                    OUTPUT
                },
                T::type_uuid(),
            )
        }
    }
    impl<T: TypeUuidProvider> TypeUuidProvider for Vec<T> {
        fn type_uuid() -> Uuid {
            combine_uuids(
                {
                    const OUTPUT: ::uuid::Uuid = match ::uuid::Uuid::try_parse(
                        "51bc577b-5a50-4a97-9b31-eda2f3d46c9c",
                    ) {
                        ::uuid::__macro_support::Ok(u) => u,
                        ::uuid::__macro_support::Err(_) => {
                            ::std::rt::begin_panic("invalid UUID");
                        }
                    };
                    OUTPUT
                },
                T::type_uuid(),
            )
        }
    }
    #[inline]
    pub fn combine_uuids(a: Uuid, b: Uuid) -> Uuid {
        let mut combined_bytes = a.into_bytes();
        for (src, dest) in b.into_bytes().into_iter().zip(combined_bytes.iter_mut()) {
            *dest ^= src;
        }
        Uuid::from_bytes(combined_bytes)
    }
    /// Component provider provides dynamic access to inner components of an object by their type id.
    pub trait ComponentProvider {
        /// Allows an object to provide access to inner components.
        fn query_component_ref(&self, type_id: TypeId) -> Option<&dyn Any>;
        /// Allows an object to provide access to inner components.
        fn query_component_mut(&mut self, type_id: TypeId) -> Option<&mut dyn Any>;
    }
    impl dyn ComponentProvider {
        /// Tries to borrow a component of given type.
        #[inline]
        pub fn component_ref<T: Any>(&self) -> Option<&T> {
            ComponentProvider::query_component_ref(self, TypeId::of::<T>())
                .and_then(|c| c.downcast_ref())
        }
        /// Tries to borrow a component of given type.
        #[inline]
        pub fn component_mut<T: Any>(&mut self) -> Option<&mut T> {
            ComponentProvider::query_component_mut(self, TypeId::of::<T>())
                .and_then(|c| c.downcast_mut())
        }
    }
}
pub mod variable {
    //! A wrapper for a variable that hold additional flag that tells that initial value was changed in runtime.
    //!
    //! For more info see [`InheritableVariable`]
    use crate::{reflect::prelude::*, visitor::{prelude::*, VisitorFlags}};
    use bitflags::bitflags;
    use std::{
        any::{Any, TypeId},
        cell::Cell, fmt::Debug, ops::{Deref, DerefMut},
    };
    #[repr(transparent)]
    pub struct VariableFlags(u8);
    #[allow(warnings)]
    impl Reflect for VariableFlags
    where
        Self: 'static,
        u8: Reflect,
    {
        fn source_path() -> &'static str {
            "fyrox-core\\src\\variable.rs"
        }
        fn type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn doc(&self) -> &'static str {
            ""
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            func(
                &[
                    FieldInfo {
                        owner_type_id: std::any::TypeId::of::<Self>(),
                        name: "0",
                        display_name: "0",
                        doc: "",
                        read_only: false,
                        immutable_collection: false,
                        min_value: None,
                        max_value: None,
                        value: &self.0,
                        reflect_value: &self.0,
                        step: None,
                        precision: None,
                        description: "",
                        type_name: std::any::type_name::<u8>(),
                    },
                ],
            )
        }
        fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
            self
        }
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            let value = match value.take() {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            let this = std::mem::replace(self, value);
            Ok(Box::new(this))
        }
        fn as_any(&self, func: &mut dyn FnMut(&dyn ::core::any::Any)) {
            func(self)
        }
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn ::core::any::Any)) {
            func(self)
        }
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            func(self as &dyn Reflect)
        }
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            func(self as &mut dyn Reflect)
        }
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            func(&[&self.0 as &dyn Reflect])
        }
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            func(&mut [&mut self.0 as &mut dyn Reflect])
        }
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            let value = {
                match name {
                    "0" => Some(&self.0 as &dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            let value = {
                match name {
                    "0" => Some(&mut self.0 as &mut dyn Reflect),
                    _ => None,
                }
            };
            func(value)
        }
    }
    #[allow(missing_docs)]
    impl VariableFlags {
        pub const F_0: &'static str = "0";
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VariableFlags {}
    #[automatically_derived]
    impl ::core::clone::Clone for VariableFlags {
        #[inline]
        fn clone(&self) -> VariableFlags {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for VariableFlags {
        #[inline]
        fn cmp(&self, other: &VariableFlags) -> ::core::cmp::Ordering {
            ::core::cmp::Ord::cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for VariableFlags {
        #[inline]
        fn partial_cmp(
            &self,
            other: &VariableFlags,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for VariableFlags {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for VariableFlags {
        #[inline]
        fn eq(&self, other: &VariableFlags) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for VariableFlags {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    impl VariableFlags {
        /// Nothing.
        #[allow(deprecated, non_upper_case_globals)]
        pub const NONE: Self = Self::from_bits_retain(0);
        /// A variable was externally modified.
        #[allow(deprecated, non_upper_case_globals)]
        pub const MODIFIED: Self = Self::from_bits_retain(0b0000_0001);
        /// A variable must be synced with respective variable from data model.
        #[allow(deprecated, non_upper_case_globals)]
        pub const NEED_SYNC: Self = Self::from_bits_retain(0b0000_0010);
    }
    impl ::bitflags::Flags for VariableFlags {
        const FLAGS: &'static [::bitflags::Flag<VariableFlags>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("NONE", VariableFlags::NONE)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("MODIFIED", VariableFlags::MODIFIED)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("NEED_SYNC", VariableFlags::NEED_SYNC)
            },
        ];
        type Bits = u8;
        fn bits(&self) -> u8 {
            VariableFlags::bits(self)
        }
        fn from_bits_retain(bits: u8) -> VariableFlags {
            VariableFlags::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::iter_without_into_iter,
    )]
    const _: () = {
        #[allow(dead_code, deprecated, unused_attributes)]
        impl VariableFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                { Self(<u8 as ::bitflags::Bits>::EMPTY) }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                {
                    let mut truncated = <u8 as ::bitflags::Bits>::EMPTY;
                    let mut i = 0;
                    {
                        {
                            let flag = <VariableFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <VariableFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <VariableFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    let _ = i;
                    Self::from_bits_retain(truncated)
                }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u8 {
                let f = self;
                { f.0 }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u8,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    let truncated = Self::from_bits_truncate(bits).0;
                    if truncated == bits {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    } else {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u8) -> Self {
                let bits = bits;
                { Self(bits & Self::all().bits()) }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u8) -> Self {
                let bits = bits;
                { Self(bits) }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    {
                        if name == "NONE" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(VariableFlags::NONE.bits()),
                            );
                        }
                    };
                    {
                        if name == "MODIFIED" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(VariableFlags::MODIFIED.bits()),
                            );
                        }
                    };
                    {
                        if name == "NEED_SYNC" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(VariableFlags::NEED_SYNC.bits()),
                            );
                        }
                    };
                    let _ = name;
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                { f.bits() == <u8 as ::bitflags::Bits>::EMPTY }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                { Self::all().bits() | f.bits() == f.bits() }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.bits() & other.bits() != <u8 as ::bitflags::Bits>::EMPTY }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.bits() & other.bits() == other.bits() }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).union(other);
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).difference(other);
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
                }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                {
                    if value {
                        f.insert(other);
                    } else {
                        f.remove(other);
                    }
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() & other.bits()) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() | other.bits()) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() & !other.bits()) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() ^ other.bits()) }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                { Self::from_bits_truncate(!f.bits()) }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for VariableFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for VariableFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for VariableFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for VariableFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for VariableFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: VariableFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for VariableFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for VariableFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for VariableFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for VariableFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for VariableFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for VariableFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for VariableFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for VariableFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<VariableFlags> for VariableFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<VariableFlags>
        for VariableFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl VariableFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<VariableFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <VariableFlags as ::bitflags::Flags>::FLAGS,
                    VariableFlags::from_bits_retain(self.bits()),
                    VariableFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(
                &self,
            ) -> ::bitflags::iter::IterNames<VariableFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <VariableFlags as ::bitflags::Flags>::FLAGS,
                    VariableFlags::from_bits_retain(self.bits()),
                    VariableFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for VariableFlags {
            type Item = VariableFlags;
            type IntoIter = ::bitflags::iter::Iter<VariableFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
    impl Debug for VariableFlags {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if *self == VariableFlags::NONE {
                f.write_fmt(format_args!("NONE"))
            } else {
                for (i, flag) in self.iter().enumerate() {
                    if i != 0 {
                        f.write_fmt(format_args!("|"))?
                    }
                    match flag {
                        VariableFlags::MODIFIED => f.write_fmt(format_args!("MOD"))?,
                        VariableFlags::NEED_SYNC => f.write_fmt(format_args!("SYNC"))?,
                        _ => {}
                    }
                }
                Ok(())
            }
        }
    }
    /// An error that could occur during inheritance.
    pub enum InheritError {
        /// Types of properties mismatch.
        TypesMismatch {
            /// Type of left property.
            left_type: &'static str,
            /// Type of right property.
            right_type: &'static str,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InheritError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                InheritError::TypesMismatch {
                    left_type: __self_0,
                    right_type: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TypesMismatch",
                        "left_type",
                        __self_0,
                        "right_type",
                        &__self_1,
                    )
                }
            }
        }
    }
    /// A wrapper for a variable that hold additional flag that tells that initial value was changed in runtime.
    ///
    /// InheritableVariables are used for resource inheritance system. Resource inheritance may just sound weird,
    /// but the idea behind it is very simple - take property values from parent resource if the value in current
    /// hasn't changed in runtime.
    ///
    /// To get better understanding, let's look at very simple example. Imagine you have a scene with a 3d model
    /// instance. Now you realizes that the 3d model has a misplaced object and you need to fix it, you open a
    /// 3D modelling software (Blender, 3Ds max, etc) and move the object to a correct spot and re-save the 3D model.
    /// The question is: what should happen with the instance of the object in the scene? Logical answer would be:
    /// if it hasn't been modified, then just take the new position from the 3D model. This is where inheritable
    /// variable comes into play. If you've change the value of such variable, it will remember changes and the object
    /// will stay on its new position instead of changed.
    ///
    /// # Deref and DerefMut
    ///
    /// Access via Deref provides access to inner variable. **DerefMut marks variable as modified** and returns a
    /// mutable reference to inner variable.
    pub struct InheritableVariable<T> {
        value: T,
        flags: Cell<VariableFlags>,
    }
    impl<T: Debug> Debug for InheritableVariable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                format_args!("{0:?} (flags:{1:?})", self.value, self.flags.get()),
            )
        }
    }
    impl<T: Clone> Clone for InheritableVariable<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                value: self.value.clone(),
                flags: self.flags.clone(),
            }
        }
    }
    impl<T> From<T> for InheritableVariable<T> {
        #[inline]
        fn from(v: T) -> Self {
            InheritableVariable::new_modified(v)
        }
    }
    impl<T: PartialEq> PartialEq for InheritableVariable<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.value.eq(&other.value)
        }
    }
    impl<T: Eq> Eq for InheritableVariable<T> {}
    impl<T: Default> Default for InheritableVariable<T> {
        #[inline]
        fn default() -> Self {
            Self {
                value: T::default(),
                flags: Cell::new(VariableFlags::MODIFIED),
            }
        }
    }
    impl<T: Clone> InheritableVariable<T> {
        /// Clones wrapped value.
        #[inline]
        pub fn clone_inner(&self) -> T {
            self.value.clone()
        }
        /// Tries to sync a value in a data model with a value in the inheritable variable. The value
        /// will be synced only if it was marked as needs sync.
        #[inline]
        pub fn try_sync_model<S: FnOnce(T)>(&self, setter: S) -> bool {
            if self.need_sync() {
                let mut flags = self.flags.get();
                flags.remove(VariableFlags::NEED_SYNC);
                self.flags.set(flags);
                (setter)(self.value.clone());
                true
            } else {
                false
            }
        }
    }
    impl<T> InheritableVariable<T> {
        /// Creates new modified variable from given value. This method should always be used to create inheritable
        /// variables in the engine.
        #[inline]
        pub fn new_modified(value: T) -> Self {
            Self {
                value,
                flags: Cell::new(VariableFlags::MODIFIED),
            }
        }
        /// Creates new variable without any flags set.
        #[inline]
        pub fn new_non_modified(value: T) -> Self {
            Self {
                value,
                flags: Cell::new(VariableFlags::NONE),
            }
        }
        /// Creates new variable from a given value with custom flags.
        #[inline]
        pub fn new_with_flags(value: T, flags: VariableFlags) -> Self {
            Self {
                value,
                flags: Cell::new(flags),
            }
        }
        /// Replaces value and also raises the [`VariableFlags::MODIFIED`] flag.
        #[inline]
        pub fn set_value_and_mark_modified(&mut self, value: T) -> T {
            self.mark_modified_and_need_sync();
            std::mem::replace(&mut self.value, value)
        }
        /// Replaces value and flags.
        #[inline]
        pub fn set_value_with_flags(&mut self, value: T, flags: VariableFlags) -> T {
            self.flags.set(flags);
            std::mem::replace(&mut self.value, value)
        }
        /// Replaces current value without marking the variable modified.
        #[inline]
        pub fn set_value_silent(&mut self, value: T) -> T {
            std::mem::replace(&mut self.value, value)
        }
        /// Returns true if the respective data model's variable must be synced.
        #[inline]
        pub fn need_sync(&self) -> bool {
            self.flags.get().contains(VariableFlags::NEED_SYNC)
        }
        /// Returns a reference to the wrapped value.
        #[inline]
        pub fn get_value_ref(&self) -> &T {
            &self.value
        }
        /// Returns a mutable reference to the wrapped value.
        ///
        /// # Important notes.
        ///
        /// The method raises `modified` flag, no matter if actual modification was made!
        #[inline]
        pub fn get_value_mut_and_mark_modified(&mut self) -> &mut T {
            self.mark_modified_and_need_sync();
            &mut self.value
        }
        /// Returns a mutable reference to the wrapped value.
        ///
        /// # Important notes.
        ///
        /// This method does not mark the value as modified!
        #[inline]
        pub fn get_value_mut_silent(&mut self) -> &mut T {
            &mut self.value
        }
        /// Returns true if variable was modified and should not be overwritten during property inheritance.
        #[inline]
        pub fn is_modified(&self) -> bool {
            self.flags.get().contains(VariableFlags::MODIFIED)
        }
        /// Marks value as modified, so its value won't be overwritten during property inheritance.
        #[inline]
        pub fn mark_modified(&mut self) {
            self.flags
                .get_mut()
                .insert(VariableFlags::MODIFIED | VariableFlags::NEED_SYNC);
        }
        /// Deconstructs the variable and returns the wrapped value.
        #[inline]
        pub fn take(self) -> T {
            self.value
        }
        #[inline]
        fn mark_modified_and_need_sync(&mut self) {
            self.flags
                .get_mut()
                .insert(VariableFlags::MODIFIED | VariableFlags::NEED_SYNC);
        }
    }
    impl<T> Deref for InheritableVariable<T> {
        type Target = T;
        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }
    impl<T> DerefMut for InheritableVariable<T> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.mark_modified_and_need_sync();
            &mut self.value
        }
    }
    /// Special non-derived implementation of Visit to account for the special needs of InheritableVariable from Visitors.
    impl<T> Visit for InheritableVariable<T>
    where
        T: Visit,
    {
        /// Read or write this value, depending on whether [Visitor::is_reading()] is true or false.
        /// InheritableVariable uses the visit method in a very special way. Rather than just directly
        /// visiting the inner value and flags of the InheritableVariable, it allows for several distinct possibilities.
        ///
        /// # Cases when the visitor is reading:
        ///
        /// 1. If the visitor is reading, InheritableVariable allows for the possibilities that the data being read
        /// is not an InheritableVariable but is data of type T. It uses this data to set the inner value
        /// and adds [VariableFlags::MODIFIED] to [InheritableVariable::flags].
        ///
        /// 2. The data for this InheritableVariable may be missing entirely from the given visitor.
        /// If so, then leave inner value unmodified and remove the `MODIFIED` flag from `flags`.
        ///
        /// # Cases when the visitor is writing:
        ///
        /// 1. If the visitor is writing and the `MODIFIED` flag is not set, then InheritableVariable writes **nothing at all.**
        /// It does not even write an empty region.
        ///
        /// 2. If the visitor is writing and the `MODIFIED` flag is set, then the InheritableVariable writes itself to the Visitor
        /// as if InheritableVariable were a normal struct, writing a Field for "Flags" and causing `value` to write itself.
        ///
        /// If the [VisitorFlags::SERIALIZE_EVERYTHING] flag is set in the [Visitor::flags], this causes the InheritableVariable to act
        /// as if its `MODIFIED` flag were set.
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut visited = false;
            if visitor.is_reading() {
                visited = self.value.visit(name, visitor).is_ok();
                self.flags.get_mut().insert(VariableFlags::MODIFIED);
            }
            if !visited {
                if visitor.is_reading() {
                    if let Ok(mut region) = visitor.enter_region(name) {
                        let _ = self.value.visit("Value", &mut region);
                        self.flags.get_mut().0.visit("Flags", &mut region)?;
                    } else {
                        self.flags.get_mut().remove(VariableFlags::MODIFIED);
                    }
                } else if self.flags.get().contains(VariableFlags::MODIFIED)
                    || visitor.flags.contains(VisitorFlags::SERIALIZE_EVERYTHING)
                {
                    let mut region = visitor.enter_region(name)?;
                    self.value.visit("Value", &mut region)?;
                    self.flags.get_mut().0.visit("Flags", &mut region)?;
                } else {}
            }
            Ok(())
        }
    }
    impl<T> Reflect for InheritableVariable<T>
    where
        T: Reflect + Clone + PartialEq + Debug,
    {
        #[inline]
        fn source_path() -> &'static str {
            "fyrox-core\\src\\variable.rs"
        }
        #[inline]
        fn type_name(&self) -> &'static str {
            self.value.type_name()
        }
        #[inline]
        fn doc(&self) -> &'static str {
            self.value.doc()
        }
        fn assembly_name(&self) -> &'static str {
            "fyrox-core"
        }
        fn type_assembly_name() -> &'static str {
            "fyrox-core"
        }
        #[inline]
        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            self.value.fields_info(func)
        }
        #[inline]
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            Box::new(self.value).into_any()
        }
        #[inline]
        fn as_any(&self, func: &mut dyn FnMut(&dyn Any)) {
            self.value.as_any(func)
        }
        #[inline]
        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn Any)) {
            self.value.as_any_mut(func)
        }
        #[inline]
        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            self.value.as_reflect(func)
        }
        #[inline]
        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            self.value.as_reflect_mut(func)
        }
        #[inline]
        fn set(
            &mut self,
            value: Box<dyn Reflect>,
        ) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            self.mark_modified_and_need_sync();
            self.value.set(value)
        }
        #[inline]
        fn set_field(
            &mut self,
            field: &str,
            value: Box<dyn Reflect>,
            func: &mut dyn FnMut(Result<Box<dyn Reflect>, Box<dyn Reflect>>),
        ) {
            self.mark_modified_and_need_sync();
            self.value.set_field(field, value, func)
        }
        #[inline]
        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            self.value.fields(func)
        }
        #[inline]
        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            self.value.fields_mut(func)
        }
        #[inline]
        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            self.value.field(name, func)
        }
        #[inline]
        fn field_mut(
            &mut self,
            name: &str,
            func: &mut dyn FnMut(Option<&mut dyn Reflect>),
        ) {
            self.mark_modified_and_need_sync();
            self.value.field_mut(name, func)
        }
        #[inline]
        fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
            self.value.as_array(func)
        }
        #[inline]
        fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
            self.mark_modified_and_need_sync();
            self.value.as_array_mut(func)
        }
        #[inline]
        fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
            self.value.as_list(func)
        }
        #[inline]
        fn as_list_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectList>)) {
            self.mark_modified_and_need_sync();
            self.value.as_list_mut(func)
        }
        #[inline]
        fn as_inheritable_variable(
            &self,
            func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
        ) {
            func(Some(self))
        }
        #[inline]
        fn as_inheritable_variable_mut(
            &mut self,
            func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
        ) {
            func(Some(self))
        }
    }
    impl<T> ReflectInheritableVariable for InheritableVariable<T>
    where
        T: Reflect + Clone + PartialEq + Debug,
    {
        fn try_inherit(
            &mut self,
            parent: &dyn ReflectInheritableVariable,
            ignored_types: &[TypeId],
        ) -> Result<Option<Box<dyn Reflect>>, InheritError> {
            let mut result: Result<Option<Box<dyn Reflect>>, InheritError> = Ok(None);
            match parent.inner_value_ref().as_any_raw().downcast_ref::<T>() {
                Some(parent_value) => {
                    if !self.is_modified() {
                        let mut parent_value_clone = parent_value.clone();
                        mark_inheritable_properties_non_modified(
                            &mut parent_value_clone,
                            ignored_types,
                        );
                        result = Ok(
                            Some(
                                Box::new(
                                    std::mem::replace(&mut self.value, parent_value_clone),
                                ),
                            ),
                        );
                    }
                }
                None => {
                    result = Err(InheritError::TypesMismatch {
                        left_type: self.inner_value_ref().type_name(),
                        right_type: parent.inner_value_ref().type_name(),
                    });
                }
            }
            result
        }
        #[inline]
        fn reset_modified_flag(&mut self) {
            self.flags.get_mut().remove(VariableFlags::MODIFIED)
        }
        #[inline]
        fn flags(&self) -> VariableFlags {
            self.flags.get()
        }
        #[inline]
        fn set_flags(&mut self, flags: VariableFlags) {
            self.flags.set(flags)
        }
        #[inline]
        fn is_modified(&self) -> bool {
            self.is_modified()
        }
        #[inline]
        fn value_equals(&self, other: &dyn ReflectInheritableVariable) -> bool {
            let mut output_result = false;
            other
                .as_reflect(
                    &mut (|reflect| {
                        reflect
                            .downcast_ref::<
                                T,
                            >(
                                &mut (|result| {
                                    output_result = match result {
                                        Some(other) => &self.value == other,
                                        None => false,
                                    };
                                }),
                            )
                    }),
                );
            output_result
        }
        #[inline]
        fn clone_value_box(&self) -> Box<dyn Reflect> {
            Box::new(self.value.clone())
        }
        #[inline]
        fn mark_modified(&mut self) {
            self.mark_modified()
        }
        #[inline]
        fn inner_value_mut(&mut self) -> &mut dyn Reflect {
            &mut self.value
        }
        #[inline]
        fn inner_value_ref(&self) -> &dyn Reflect {
            &self.value
        }
    }
    /// Simultaneously walks over fields of given child and parent and tries to inherit values of properties
    /// of child with parent's properties. It is done recursively for every fields in entities.
    ///
    /// ## How it works
    ///
    /// In general, it uses reflection to iterate over child and parent properties and trying to inherit values.
    /// Child's field will take parent's field value only if child's field is **non-modified**. There are one
    /// edge case in inheritance: collections.
    ///
    /// Inheritance for collections itself works the same as described above, however the content of collections
    /// can only be inherited if their sizes are equal. Also, since inheritance uses plain copy of inner data of
    /// inheritable variables, it works in a special way.
    ///
    /// ```text
    /// Child                                       Parent (root)
    ///     InheritableVariableA            <-         InheritableVariableA*
    ///     InheritableCollection*          ->         InheritableCollection*
    ///         Item0                                       Item0
    ///             InheritableVariableB*   ->                  InheritableVariableB*
    ///             InheritableVariableC    <-                  InheritableVariableC*
    ///         Item1                                       Item1
    ///             ..                                          ..
    ///         ..                                          ..
    ///         ItemN                                       ItemN
    ///             ..                                          ..
    ///
    /// * - means that the variable was modified
    /// ```
    ///
    /// At first, `InheritableVariableA` will be copied from the parent as usual. Next, the inheritable collection
    /// won't be copied (because it is modified), however its items will be inherited separately.
    /// `InheritableVariableB` won't be copied either (since it is modified too), but `InheritableVariableC` **will**
    /// be copied from parent.
    pub fn try_inherit_properties(
        child: &mut dyn Reflect,
        parent: &dyn Reflect,
        ignored_types: &[TypeId],
    ) -> Result<(), InheritError> {
        let child_type_id = (*child).type_id();
        let parent_type_id = (*parent).type_id();
        if ignored_types.contains(&child_type_id)
            || ignored_types.contains(&parent_type_id)
        {
            return Ok(());
        }
        if child_type_id != parent_type_id {
            return Err(InheritError::TypesMismatch {
                left_type: (*child).type_name(),
                right_type: (*parent).type_name(),
            });
        }
        let mut result = None;
        child
            .as_inheritable_variable_mut(
                &mut (|inheritable_child| {
                    if let Some(inheritable_child) = inheritable_child {
                        parent
                            .as_inheritable_variable(
                                &mut (|inheritable_parent| {
                                    if let Some(inheritable_parent) = inheritable_parent {
                                        if let Err(e) = inheritable_child
                                            .try_inherit(inheritable_parent, ignored_types)
                                        {
                                            result = Some(Err(e));
                                        }
                                        if !match result {
                                            Some(Err(_)) => true,
                                            _ => false,
                                        } {
                                            result = Some(
                                                try_inherit_properties(
                                                    inheritable_child.inner_value_mut(),
                                                    inheritable_parent.inner_value_ref(),
                                                    ignored_types,
                                                ),
                                            );
                                        }
                                    }
                                }),
                            )
                    }
                }),
            );
        if result.is_none() {
            child
                .as_array_mut(
                    &mut (|child_collection| {
                        if let Some(child_collection) = child_collection {
                            parent
                                .as_array(
                                    &mut (|parent_collection| {
                                        if let Some(parent_collection) = parent_collection {
                                            if child_collection.reflect_len()
                                                == parent_collection.reflect_len()
                                            {
                                                for i in 0..child_collection.reflect_len() {
                                                    if let (Some(child_item), Some(parent_item)) = (
                                                        child_collection.reflect_index_mut(i),
                                                        parent_collection.reflect_index(i),
                                                    ) {
                                                        if let Err(e) = try_inherit_properties(
                                                            child_item,
                                                            parent_item,
                                                            ignored_types,
                                                        ) {
                                                            result = Some(Err(e));
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }),
                                )
                        }
                    }),
                )
        }
        if result.is_none() {
            child
                .fields_mut(
                    &mut (|child_fields| {
                        parent
                            .fields(
                                &mut (|parent_fields| {
                                    for (child_field, parent_field) in child_fields
                                        .iter_mut()
                                        .zip(parent_fields)
                                    {
                                        if let Err(e) = try_inherit_properties(
                                            *child_field,
                                            *parent_field,
                                            ignored_types,
                                        ) {
                                            result = Some(Err(e));
                                        }
                                        if match result {
                                            Some(Err(_)) => true,
                                            _ => false,
                                        } {
                                            break;
                                        }
                                    }
                                }),
                            )
                    }),
                );
        }
        result.unwrap_or(Ok(()))
    }
    pub fn do_with_inheritable_variables<F>(
        root: &mut dyn Reflect,
        func: &mut F,
        ignored_types: &[TypeId],
    )
    where
        F: FnMut(&mut dyn ReflectInheritableVariable),
    {
        root.apply_recursively_mut(
            &mut (|object| {
                object
                    .as_inheritable_variable_mut(
                        &mut (|variable| {
                            if let Some(variable) = variable {
                                func(variable);
                            }
                        }),
                    );
            }),
            ignored_types,
        )
    }
    pub fn mark_inheritable_properties_non_modified(
        object: &mut dyn Reflect,
        ignored_types: &[TypeId],
    ) {
        do_with_inheritable_variables(
            object,
            &mut (|variable| variable.reset_modified_flag()),
            ignored_types,
        );
    }
    pub fn mark_inheritable_properties_modified(
        object: &mut dyn Reflect,
        ignored_types: &[TypeId],
    ) {
        do_with_inheritable_variables(
            object,
            &mut (|variable| variable.mark_modified()),
            ignored_types,
        );
    }
}
pub mod visitor {
    //! Visitor is a tree-based serializer/deserializer.
    //!
    //! # Overview
    //!
    //! Visitor uses tree to create structured storage of data. Basic unit is a *node* - it is a container
    //! for data fields. Each node has name, handle to parent, set of handles to children nodes and some
    //! container for data fields. Data field is tuple of name and value, value can be any of simple Rust
    //! types and some of basic structures of the crate. Main criteria of what could be the field and what
    //! not is the ability to be represented as set of bytes without any aliasing issues.
    pub use fyrox_core_derive::Visit;
    pub mod prelude {
        //! Types to use `#[derive(Visit)]`
        pub use super::{Visit, VisitError, VisitResult, Visitor};
    }
    use crate::{
        algebra::{
            Complex, Const, Matrix, Matrix2, Matrix3, Matrix4, Quaternion, RawStorage,
            RawStorageMut, SVector, Scalar, UnitComplex, UnitQuaternion, Vector2,
            Vector3, Vector4, U1,
        },
        io::{self, FileLoadError},
        pool::{Handle, Pool},
        replace_slashes,
    };
    use base64::Engine;
    use bitflags::bitflags;
    use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
    use fxhash::FxHashMap;
    use std::any::TypeId;
    use std::error::Error;
    use std::{
        any::Any, cell::{Cell, RefCell},
        collections::{hash_map::Entry, HashMap, HashSet},
        fmt::{Display, Formatter},
        fs::File, hash::{BuildHasher, Hash},
        io::{BufWriter, Cursor, Read, Write},
        ops::{Deref, DerefMut, Range},
        path::{Path, PathBuf},
        rc::Rc, string::FromUtf8Error, sync::{Arc, Mutex, RwLock},
        time::Duration,
    };
    use uuid::Uuid;
    /// The internal data format of [Visitor]. Fields are limited to being one of these types.
    /// This means that all [Visit] values must be built from some assortment
    /// of these types.
    /// Fields can be accessed from a visitor using [Visit::visit] on a variable with the
    /// same type as the field.
    pub enum FieldKind {
        Bool(bool),
        U8(u8),
        I8(i8),
        U16(u16),
        I16(i16),
        U32(u32),
        I32(i32),
        U64(u64),
        I64(i64),
        F32(f32),
        F64(f64),
        UnitQuaternion(UnitQuaternion<f32>),
        Matrix4(Matrix4<f32>),
        /// A representation of some `Vec<T>` where `T` must be [Copy].
        /// It is mostly used to store the bytes of string types.
        BinaryBlob(Vec<u8>),
        Matrix3(Matrix3<f32>),
        Uuid(Uuid),
        UnitComplex(UnitComplex<f32>),
        /// A representation for arrays of [Pod] types as a `Vec<u8>`.
        PodArray {
            /// A code to identify the Pod type of the elements of the array.
            /// Taken from [Pod::type_id].
            type_id: u8,
            /// The number of bytes in each array element.
            element_size: u32,
            /// The bytes that store the data, with unspecified endianness.
            bytes: Vec<u8>,
        },
        Matrix2(Matrix2<f32>),
        Vector2F32(Vector2<f32>),
        Vector3F32(Vector3<f32>),
        Vector4F32(Vector4<f32>),
        Vector2F64(Vector2<f64>),
        Vector3F64(Vector3<f64>),
        Vector4F64(Vector4<f64>),
        Vector2U8(Vector2<u8>),
        Vector3U8(Vector3<u8>),
        Vector4U8(Vector4<u8>),
        Vector2I8(Vector2<i8>),
        Vector3I8(Vector3<i8>),
        Vector4I8(Vector4<i8>),
        Vector2U16(Vector2<u16>),
        Vector3U16(Vector3<u16>),
        Vector4U16(Vector4<u16>),
        Vector2I16(Vector2<i16>),
        Vector3I16(Vector3<i16>),
        Vector4I16(Vector4<i16>),
        Vector2U32(Vector2<u32>),
        Vector3U32(Vector3<u32>),
        Vector4U32(Vector4<u32>),
        Vector2I32(Vector2<i32>),
        Vector3I32(Vector3<i32>),
        Vector4I32(Vector4<i32>),
        Vector2U64(Vector2<u64>),
        Vector3U64(Vector3<u64>),
        Vector4U64(Vector4<u64>),
        Vector2I64(Vector2<i64>),
        Vector3I64(Vector3<i64>),
        Vector4I64(Vector4<i64>),
    }
    /// Trait for datatypes that can be converted directly into bytes.
    /// This is required for the type to be used in the Vec of a [PodVecView].
    pub trait Pod: Copy {
        /// A number which distinguishes each Pod type. Two distinct Pod types must not share the same `type_id` byte.
        /// The `type_id` is stored with the data when a [PodVecView] is visited and used to confirm that the stored
        /// data matches the expected type when reading. Otherwise garbage data could be read by interpreting an
        /// array of i8 as an array of f32 or any other mismatched combination.
        fn type_id() -> u8;
    }
    impl Pod for u8 {
        fn type_id() -> u8 {
            0
        }
    }
    impl Pod for i8 {
        fn type_id() -> u8 {
            1
        }
    }
    impl Pod for u16 {
        fn type_id() -> u8 {
            2
        }
    }
    impl Pod for i16 {
        fn type_id() -> u8 {
            3
        }
    }
    impl Pod for u32 {
        fn type_id() -> u8 {
            4
        }
    }
    impl Pod for i32 {
        fn type_id() -> u8 {
            5
        }
    }
    impl Pod for u64 {
        fn type_id() -> u8 {
            6
        }
    }
    impl Pod for i64 {
        fn type_id() -> u8 {
            7
        }
    }
    impl Pod for f32 {
        fn type_id() -> u8 {
            8
        }
    }
    impl Pod for f64 {
        fn type_id() -> u8 {
            9
        }
    }
    /// A [Visit] type for storing a whole Vec of [Pod] values as a single field within a Visitor.
    /// The Vec is reinterpreted as a Vec of bytes, with no consideration given for whether the bytes
    /// are in big-endian or little-endian order by using [std::ptr::copy_nonoverlapping].
    pub struct PodVecView<'a, T: Pod> {
        type_id: u8,
        vec: &'a mut Vec<T>,
    }
    impl<'a, T: Pod> PodVecView<'a, T> {
        pub fn from_pod_vec(vec: &'a mut Vec<T>) -> Self {
            Self { type_id: T::type_id(), vec }
        }
    }
    impl<'a, T: Pod> Visit for PodVecView<'a, T> {
        #[allow(clippy::uninit_vec)]
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match &field.kind {
                        FieldKind::PodArray { type_id, element_size, bytes } => {
                            if *type_id == self.type_id {
                                let len = bytes.len() / *element_size as usize;
                                let mut data = Vec::<T>::with_capacity(len);
                                unsafe {
                                    data.set_len(len);
                                    std::ptr::copy_nonoverlapping(
                                        bytes.as_ptr(),
                                        data.as_mut_ptr() as *mut u8,
                                        bytes.len(),
                                    );
                                }
                                *self.vec = data;
                                Ok(())
                            } else {
                                Err(VisitError::TypeMismatch)
                            }
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields
                    .push(
                        Field::new(
                            name,
                            FieldKind::PodArray {
                                type_id: T::type_id(),
                                element_size: std::mem::size_of::<T>() as u32,
                                bytes: unsafe {
                                    let mut data = self.vec.clone();
                                    let bytes = Vec::from_raw_parts(
                                        data.as_mut_ptr() as *mut u8,
                                        data.len() * std::mem::size_of::<T>(),
                                        data.capacity() * std::mem::size_of::<T>(),
                                    );
                                    std::mem::forget(data);
                                    bytes
                                },
                            },
                        ),
                    );
                Ok(())
            }
        }
    }
    impl FieldKind {
        fn as_string(&self) -> String {
            match self {
                Self::Bool(data) => {
                    let res = ::alloc::fmt::format(format_args!("<bool = {0}>, ", data));
                    res
                }
                Self::U8(data) => {
                    let res = ::alloc::fmt::format(format_args!("<u8 = {0}>, ", data));
                    res
                }
                Self::I8(data) => {
                    let res = ::alloc::fmt::format(format_args!("<i8 = {0}>, ", data));
                    res
                }
                Self::U16(data) => {
                    let res = ::alloc::fmt::format(format_args!("<u16 = {0}>, ", data));
                    res
                }
                Self::I16(data) => {
                    let res = ::alloc::fmt::format(format_args!("<i16 = {0}>, ", data));
                    res
                }
                Self::U32(data) => {
                    let res = ::alloc::fmt::format(format_args!("<u32 = {0}>, ", data));
                    res
                }
                Self::I32(data) => {
                    let res = ::alloc::fmt::format(format_args!("<i32 = {0}>, ", data));
                    res
                }
                Self::U64(data) => {
                    let res = ::alloc::fmt::format(format_args!("<u64 = {0}>, ", data));
                    res
                }
                Self::I64(data) => {
                    let res = ::alloc::fmt::format(format_args!("<i64 = {0}>, ", data));
                    res
                }
                Self::F32(data) => {
                    let res = ::alloc::fmt::format(format_args!("<f32 = {0}>, ", data));
                    res
                }
                Self::F64(data) => {
                    let res = ::alloc::fmt::format(format_args!("<f64 = {0}>, ", data));
                    res
                }
                Self::Vector2F32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2f32 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3F32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3f32 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4F32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4f32 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2F64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2f64 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3F64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3f64 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4F64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4f64 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2I8(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2i8 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3I8(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3i8 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4I8(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4i8 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2U8(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2u8 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3U8(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3u8 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4U8(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4u8 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2I16(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2i16 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3I16(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3i16 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4I16(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4i16 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2U16(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2u16 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3U16(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3u16 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4U16(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4u16 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2I32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2i32 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3I32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3i32 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4I32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4i32 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2U32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2u32 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3U32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3u32 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4U32(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4u32 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2I64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2i64 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3I64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3i64 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4I64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4i64 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Vector2U64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<vec2u64 = {0}; {1}>, ", data.x, data.y),
                    );
                    res
                }
                Self::Vector3U64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec3u64 = {0}; {1}; {2}>, ",
                            data.x,
                            data.y,
                            data.z,
                        ),
                    );
                    res
                }
                Self::Vector4U64(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<vec4u64 = {0}; {1}; {2}; {3}>, ",
                            data.x,
                            data.y,
                            data.z,
                            data.w,
                        ),
                    );
                    res
                }
                Self::UnitQuaternion(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<quat = {0}; {1}; {2}; {3}>, ",
                            data.i,
                            data.j,
                            data.k,
                            data.w,
                        ),
                    );
                    res
                }
                Self::Matrix4(data) => {
                    let mut out = String::from("<mat4 = ");
                    for f in data.iter() {
                        out
                            += {
                                let res = ::alloc::fmt::format(format_args!("{0}; ", f));
                                res
                            }
                                .as_str();
                    }
                    out
                }
                Self::BinaryBlob(data) => {
                    let out = match String::from_utf8(data.clone()) {
                        Ok(s) => s,
                        Err(_) => base64::engine::general_purpose::STANDARD.encode(data),
                    };
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("<data = {0}>, ", out),
                        );
                        res
                    }
                }
                Self::Matrix3(data) => {
                    let mut out = String::from("<mat3 = ");
                    for f in data.iter() {
                        out
                            += {
                                let res = ::alloc::fmt::format(format_args!("{0}; ", f));
                                res
                            }
                                .as_str();
                    }
                    out
                }
                Self::Uuid(uuid) => {
                    let res = ::alloc::fmt::format(format_args!("<uuid = {0}", uuid));
                    res
                }
                Self::UnitComplex(data) => {
                    let res = ::alloc::fmt::format(
                        format_args!("<complex = {0}; {1}>, ", data.re, data.im),
                    );
                    res
                }
                FieldKind::PodArray { type_id, element_size, bytes } => {
                    let base64_encoded = base64::engine::general_purpose::STANDARD
                        .encode(bytes);
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "<podarray = {0}; {1}; [{2}]>",
                                type_id,
                                element_size,
                                base64_encoded,
                            ),
                        );
                        res
                    }
                }
                Self::Matrix2(data) => {
                    let mut out = String::from("<mat2 = ");
                    for f in data.iter() {
                        out
                            += {
                                let res = ::alloc::fmt::format(format_args!("{0}; ", f));
                                res
                            }
                                .as_str();
                    }
                    out
                }
            }
        }
    }
    /// Proxy struct for plain data, we can't use `Vec<u8>` directly,
    /// because it will serialize each byte as separate node.
    /// BinaryBlob stores data very much like [PodVecView] except that BinaryBlob
    /// has less type safety. In practice it is used with T = u8 for Strings and Paths,
    /// but it accepts any type T that is Copy, and it lacks the type_id system that
    /// PodVecView has for checking that the data it is reading comes from the expected type.
    pub struct BinaryBlob<'a, T>
    where
        T: Copy,
    {
        pub vec: &'a mut Vec<T>,
    }
    impl Visit for u64 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::U64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::U64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for i64 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::I64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::I64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for u32 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::U32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::U32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for i32 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::I32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::I32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for u16 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::U16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::U16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for i16 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::I16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::I16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for u8 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::U8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::U8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for i8 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::I8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::I8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for f32 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::F32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::F32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for f64 {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::F64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::F64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for UnitQuaternion<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::UnitQuaternion(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields
                    .push(Field::new(name, FieldKind::UnitQuaternion(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Matrix4<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Matrix4(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Matrix4(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for bool {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Bool(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Bool(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Matrix3<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Matrix3(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Matrix3(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Uuid {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Uuid(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Uuid(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for UnitComplex<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::UnitComplex(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::UnitComplex(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Matrix2<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Matrix2(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Matrix2(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2F32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2F32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3F32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3F32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<f32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4F32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4F32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<f64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2F64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2F64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<f64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3F64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3F64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<f64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4F64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4F64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<i8> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2I8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2I8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<i8> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3I8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3I8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<i8> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4I8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4I8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<u8> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2U8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2U8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<u8> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3U8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3U8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<u8> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4U8(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4U8(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<i16> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2I16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2I16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<i16> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3I16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3I16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<i16> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4I16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4I16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<u16> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2U16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2U16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<u16> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3U16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3U16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<u16> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4U16(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4U16(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<i32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2I32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2I32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<i32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3I32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3I32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<i32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4I32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4I32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<u32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2U32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2U32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<u32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3U32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3U32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<u32> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4U32(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4U32(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<i64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2I64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2I64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<i64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3I64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3I64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<i64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4I64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4I64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector2<u64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector2U64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector2U64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector3<u64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector3U64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector3U64(self.clone())));
                Ok(())
            }
        }
    }
    impl Visit for Vector4<u64> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match field.kind {
                        FieldKind::Vector4U64(data) => {
                            *self = data.clone();
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                node.fields.push(Field::new(name, FieldKind::Vector4U64(self.clone())));
                Ok(())
            }
        }
    }
    impl<'a, T> Visit for BinaryBlob<'a, T>
    where
        T: Copy,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if visitor.reading {
                if let Some(field) = visitor.find_field(name) {
                    match &field.kind {
                        FieldKind::BinaryBlob(data) => {
                            let mut bytes = std::mem::ManuallyDrop::new(data.clone());
                            *self.vec = unsafe {
                                Vec::from_raw_parts(
                                    bytes.as_mut_ptr() as *mut T,
                                    bytes.len() / std::mem::size_of::<T>(),
                                    bytes.capacity() / std::mem::size_of::<T>(),
                                )
                            };
                            Ok(())
                        }
                        _ => Err(VisitError::FieldTypeDoesNotMatch),
                    }
                } else {
                    Err(VisitError::FieldDoesNotExist(name.to_owned()))
                }
            } else if visitor.find_field(name).is_some() {
                Err(VisitError::FieldAlreadyExists(name.to_owned()))
            } else {
                let node = visitor.current_node();
                let len_bytes = self.vec.len() * std::mem::size_of::<T>();
                let mut bytes = Vec::<u8>::with_capacity(len_bytes);
                bytes
                    .extend_from_slice(unsafe {
                        std::slice::from_raw_parts(
                            self.vec.as_ptr() as *const u8,
                            len_bytes,
                        )
                    });
                node.fields.push(Field::new(name, FieldKind::BinaryBlob(bytes)));
                Ok(())
            }
        }
    }
    /// Values within a visitor are constructed from Fields.
    /// Each Field has a name and a value. The name is used as a key to access the value
    /// within the visitor using the [Visit::visit] method, so each field within a value
    /// must have a unique name.
    pub struct Field {
        /// The key string that allows access to the field.
        name: String,
        /// The data stored in the visitor for this field.
        kind: FieldKind,
    }
    /// Errors that may occur while reading or writing [Visitor].
    pub enum VisitError {
        /// An [std::io::Error] occured while reading or writing a file with Visitor data.
        Io(std::io::Error),
        /// When a field is encoded as bytes, the field data is prefixed by an identifying byte
        /// to allow the bytes to be decoded. This error happens when an identifying byte is
        /// expected during decoding, but an unknown value is found in that byte.
        UnknownFieldType(u8),
        /// Attempting to visit a field on a read-mode Visitor when no field in the visitor data
        /// has the given name.
        FieldDoesNotExist(String),
        /// Attempting to visit a field on a write-mode Visitor when a field already has the
        /// given name.
        FieldAlreadyExists(String),
        /// Attempting to enter a region on a write-mode Visitor when a region already has the
        /// given name.
        RegionAlreadyExists(String),
        InvalidCurrentNode,
        /// Attempting to visit a field using a read-mode Visitor when when that field was originally
        /// written using a value of a different type.
        FieldTypeDoesNotMatch,
        /// Attempting to enter a region on a read-mode Visitor when no region in the visitor's data
        /// has the given name.
        RegionDoesNotExist(String),
        /// The Visitor tried to leave is current node, but somehow it had no current node. This should never happen.
        NoActiveNode,
        /// The [Visitor::MAGIC] bytes were missing from the beginning of encoded Visitor data.
        NotSupportedFormat,
        /// Some sequence of bytes was not in UTF8 format.
        InvalidName,
        /// Visitor data can be self-referential, such as when the data contains multiple [Rc] references
        /// to a single shared value. This causes the visitor to store the data once and then later references
        /// to the same value point back to its first occurrence. This error occurs if one of these references
        /// points to a value of the wrong type.
        TypeMismatch,
        /// Attempting to visit a mutably borrowed RefCell.
        RefCellAlreadyMutableBorrowed,
        /// A plain-text error message that could indicate almost anything.
        User(String),
        /// [Rc] and [Arc] values store an "Id" value in the Visitor data which is based in their internal pointer.
        /// This error indicates that while reading this data, one of those Id values was discovered by be 0.
        UnexpectedRcNullIndex,
        /// A poison error occurred while trying to visit a mutex.
        PoisonedMutex,
        /// A FileLoadError was encountered while trying to decode Visitor data from a file.
        FileLoadError(FileLoadError),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VisitError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                VisitError::Io(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Io", &__self_0)
                }
                VisitError::UnknownFieldType(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UnknownFieldType",
                        &__self_0,
                    )
                }
                VisitError::FieldDoesNotExist(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FieldDoesNotExist",
                        &__self_0,
                    )
                }
                VisitError::FieldAlreadyExists(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FieldAlreadyExists",
                        &__self_0,
                    )
                }
                VisitError::RegionAlreadyExists(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RegionAlreadyExists",
                        &__self_0,
                    )
                }
                VisitError::InvalidCurrentNode => {
                    ::core::fmt::Formatter::write_str(f, "InvalidCurrentNode")
                }
                VisitError::FieldTypeDoesNotMatch => {
                    ::core::fmt::Formatter::write_str(f, "FieldTypeDoesNotMatch")
                }
                VisitError::RegionDoesNotExist(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RegionDoesNotExist",
                        &__self_0,
                    )
                }
                VisitError::NoActiveNode => {
                    ::core::fmt::Formatter::write_str(f, "NoActiveNode")
                }
                VisitError::NotSupportedFormat => {
                    ::core::fmt::Formatter::write_str(f, "NotSupportedFormat")
                }
                VisitError::InvalidName => {
                    ::core::fmt::Formatter::write_str(f, "InvalidName")
                }
                VisitError::TypeMismatch => {
                    ::core::fmt::Formatter::write_str(f, "TypeMismatch")
                }
                VisitError::RefCellAlreadyMutableBorrowed => {
                    ::core::fmt::Formatter::write_str(f, "RefCellAlreadyMutableBorrowed")
                }
                VisitError::User(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "User",
                        &__self_0,
                    )
                }
                VisitError::UnexpectedRcNullIndex => {
                    ::core::fmt::Formatter::write_str(f, "UnexpectedRcNullIndex")
                }
                VisitError::PoisonedMutex => {
                    ::core::fmt::Formatter::write_str(f, "PoisonedMutex")
                }
                VisitError::FileLoadError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FileLoadError",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl Error for VisitError {}
    impl Display for VisitError {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self {
                Self::Io(io) => f.write_fmt(format_args!("io error: {0}", io)),
                Self::UnknownFieldType(type_index) => {
                    f.write_fmt(format_args!("unknown field type {0}", type_index))
                }
                Self::FieldDoesNotExist(name) => {
                    f.write_fmt(format_args!("field does not exists {0}", name))
                }
                Self::FieldAlreadyExists(name) => {
                    f.write_fmt(format_args!("field already exists {0}", name))
                }
                Self::RegionAlreadyExists(name) => {
                    f.write_fmt(format_args!("region already exists {0}", name))
                }
                Self::InvalidCurrentNode => {
                    f.write_fmt(format_args!("invalid current node"))
                }
                Self::FieldTypeDoesNotMatch => {
                    f.write_fmt(format_args!("field type does not match"))
                }
                Self::RegionDoesNotExist(name) => {
                    f.write_fmt(format_args!("region does not exists {0}", name))
                }
                Self::NoActiveNode => f.write_fmt(format_args!("no active node")),
                Self::NotSupportedFormat => {
                    f.write_fmt(format_args!("not supported format"))
                }
                Self::InvalidName => f.write_fmt(format_args!("invalid name")),
                Self::TypeMismatch => f.write_fmt(format_args!("type mismatch")),
                Self::RefCellAlreadyMutableBorrowed => {
                    f.write_fmt(format_args!("ref cell already mutable borrowed"))
                }
                Self::User(msg) => {
                    f.write_fmt(format_args!("user defined error: {0}", msg))
                }
                Self::UnexpectedRcNullIndex => {
                    f.write_fmt(format_args!("unexpected rc null index"))
                }
                Self::PoisonedMutex => {
                    f.write_fmt(format_args!("attempt to lock poisoned mutex"))
                }
                Self::FileLoadError(e) => {
                    f.write_fmt(format_args!("file load error: {0:?}", e))
                }
            }
        }
    }
    impl<T> From<std::sync::PoisonError<std::sync::MutexGuard<'_, T>>> for VisitError {
        fn from(_: std::sync::PoisonError<std::sync::MutexGuard<'_, T>>) -> Self {
            Self::PoisonedMutex
        }
    }
    impl<T> From<std::sync::PoisonError<&mut T>> for VisitError {
        fn from(_: std::sync::PoisonError<&mut T>) -> Self {
            Self::PoisonedMutex
        }
    }
    impl<T> From<std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>>
    for VisitError {
        fn from(_: std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>) -> Self {
            Self::PoisonedMutex
        }
    }
    impl From<std::io::Error> for VisitError {
        fn from(io_err: std::io::Error) -> Self {
            Self::Io(io_err)
        }
    }
    impl From<FromUtf8Error> for VisitError {
        fn from(_: FromUtf8Error) -> Self {
            Self::InvalidName
        }
    }
    impl From<String> for VisitError {
        fn from(s: String) -> Self {
            Self::User(s)
        }
    }
    impl From<FileLoadError> for VisitError {
        fn from(e: FileLoadError) -> Self {
            Self::FileLoadError(e)
        }
    }
    /// The result of a [Visit::visit] or of a Visitor encoding operation
    /// such as [Visitor::save_binary]. It has no value unless an error occurred.
    pub type VisitResult = Result<(), VisitError>;
    trait VisitableElementaryField {
        fn write(&self, file: &mut dyn Write) -> VisitResult;
        fn read(&mut self, file: &mut dyn Read) -> VisitResult;
    }
    impl VisitableElementaryField for f64 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_f64::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_f64::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for f32 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_f32::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_f32::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for u8 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_u8(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_u8()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for i8 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_i8(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_i8()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for u16 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_u16::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_u16::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for i16 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_i16::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_i16::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for u32 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_u32::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_u32::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for i32 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_i32::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_i32::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for u64 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_u64::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_u64::<LittleEndian>()?;
            Ok(())
        }
    }
    impl VisitableElementaryField for i64 {
        fn write(&self, file: &mut dyn Write) -> VisitResult {
            file.write_i64::<LittleEndian>(*self)?;
            Ok(())
        }
        fn read(&mut self, file: &mut dyn Read) -> VisitResult {
            *self = file.read_i64::<LittleEndian>()?;
            Ok(())
        }
    }
    impl Field {
        pub fn new(name: &str, kind: FieldKind) -> Self {
            Self {
                name: name.to_owned(),
                kind,
            }
        }
        fn save(field: &Field, file: &mut dyn Write) -> VisitResult {
            fn write_vec_n<T, const N: usize>(
                file: &mut dyn Write,
                type_id: u8,
                vec: &SVector<T, N>,
            ) -> VisitResult
            where
                T: VisitableElementaryField,
            {
                file.write_u8(type_id)?;
                for v in vec.iter() {
                    v.write(file)?;
                }
                Ok(())
            }
            let name = field.name.as_bytes();
            file.write_u32::<LittleEndian>(name.len() as u32)?;
            file.write_all(name)?;
            match &field.kind {
                FieldKind::U8(data) => {
                    file.write_u8(1)?;
                    file.write_u8(*data)?;
                }
                FieldKind::I8(data) => {
                    file.write_i8(2)?;
                    file.write_i8(*data)?;
                }
                FieldKind::U16(data) => {
                    file.write_u8(3)?;
                    file.write_u16::<LittleEndian>(*data)?;
                }
                FieldKind::I16(data) => {
                    file.write_u8(4)?;
                    file.write_i16::<LittleEndian>(*data)?;
                }
                FieldKind::U32(data) => {
                    file.write_u8(5)?;
                    file.write_u32::<LittleEndian>(*data)?;
                }
                FieldKind::I32(data) => {
                    file.write_u8(6)?;
                    file.write_i32::<LittleEndian>(*data)?;
                }
                FieldKind::U64(data) => {
                    file.write_u8(7)?;
                    file.write_u64::<LittleEndian>(*data)?;
                }
                FieldKind::I64(data) => {
                    file.write_u8(8)?;
                    file.write_i64::<LittleEndian>(*data)?;
                }
                FieldKind::F32(data) => {
                    file.write_u8(9)?;
                    file.write_f32::<LittleEndian>(*data)?;
                }
                FieldKind::F64(data) => {
                    file.write_u8(10)?;
                    file.write_f64::<LittleEndian>(*data)?;
                }
                FieldKind::Vector3F32(data) => {
                    write_vec_n(file, 11, data)?;
                }
                FieldKind::UnitQuaternion(data) => {
                    file.write_u8(12)?;
                    file.write_f32::<LittleEndian>(data.i)?;
                    file.write_f32::<LittleEndian>(data.j)?;
                    file.write_f32::<LittleEndian>(data.k)?;
                    file.write_f32::<LittleEndian>(data.w)?;
                }
                FieldKind::Matrix4(data) => {
                    file.write_u8(13)?;
                    for f in data.iter() {
                        file.write_f32::<LittleEndian>(*f)?;
                    }
                }
                FieldKind::BinaryBlob(data) => {
                    file.write_u8(14)?;
                    file.write_u32::<LittleEndian>(data.len() as u32)?;
                    file.write_all(data.as_slice())?;
                }
                FieldKind::Bool(data) => {
                    file.write_u8(15)?;
                    file.write_u8(u8::from(*data))?;
                }
                FieldKind::Matrix3(data) => {
                    file.write_u8(16)?;
                    for f in data.iter() {
                        file.write_f32::<LittleEndian>(*f)?;
                    }
                }
                FieldKind::Vector2F32(data) => {
                    write_vec_n(file, 17, data)?;
                }
                FieldKind::Vector4F32(data) => {
                    write_vec_n(file, 18, data)?;
                }
                FieldKind::Uuid(uuid) => {
                    file.write_u8(19)?;
                    file.write_all(uuid.as_bytes())?;
                }
                FieldKind::UnitComplex(c) => {
                    file.write_u8(20)?;
                    file.write_f32::<LittleEndian>(c.re)?;
                    file.write_f32::<LittleEndian>(c.im)?;
                }
                FieldKind::PodArray { type_id, element_size, bytes } => {
                    file.write_u8(21)?;
                    file.write_u8(*type_id)?;
                    file.write_u32::<LittleEndian>(*element_size)?;
                    file.write_u64::<LittleEndian>(bytes.len() as u64)?;
                    file.write_all(bytes)?;
                }
                FieldKind::Matrix2(data) => {
                    file.write_u8(22)?;
                    for f in data.iter() {
                        file.write_f32::<LittleEndian>(*f)?;
                    }
                }
                FieldKind::Vector2F64(data) => {
                    write_vec_n(file, 23, data)?;
                }
                FieldKind::Vector3F64(data) => {
                    write_vec_n(file, 24, data)?;
                }
                FieldKind::Vector4F64(data) => {
                    write_vec_n(file, 25, data)?;
                }
                FieldKind::Vector2I8(data) => {
                    write_vec_n(file, 26, data)?;
                }
                FieldKind::Vector3I8(data) => {
                    write_vec_n(file, 27, data)?;
                }
                FieldKind::Vector4I8(data) => {
                    write_vec_n(file, 28, data)?;
                }
                FieldKind::Vector2U8(data) => {
                    write_vec_n(file, 29, data)?;
                }
                FieldKind::Vector3U8(data) => {
                    write_vec_n(file, 30, data)?;
                }
                FieldKind::Vector4U8(data) => {
                    write_vec_n(file, 31, data)?;
                }
                FieldKind::Vector2I16(data) => {
                    write_vec_n(file, 32, data)?;
                }
                FieldKind::Vector3I16(data) => {
                    write_vec_n(file, 33, data)?;
                }
                FieldKind::Vector4I16(data) => {
                    write_vec_n(file, 34, data)?;
                }
                FieldKind::Vector2U16(data) => {
                    write_vec_n(file, 35, data)?;
                }
                FieldKind::Vector3U16(data) => {
                    write_vec_n(file, 36, data)?;
                }
                FieldKind::Vector4U16(data) => {
                    write_vec_n(file, 37, data)?;
                }
                FieldKind::Vector2I32(data) => {
                    write_vec_n(file, 38, data)?;
                }
                FieldKind::Vector3I32(data) => {
                    write_vec_n(file, 39, data)?;
                }
                FieldKind::Vector4I32(data) => {
                    write_vec_n(file, 40, data)?;
                }
                FieldKind::Vector2U32(data) => {
                    write_vec_n(file, 41, data)?;
                }
                FieldKind::Vector3U32(data) => {
                    write_vec_n(file, 42, data)?;
                }
                FieldKind::Vector4U32(data) => {
                    write_vec_n(file, 43, data)?;
                }
                FieldKind::Vector2I64(data) => {
                    write_vec_n(file, 44, data)?;
                }
                FieldKind::Vector3I64(data) => {
                    write_vec_n(file, 45, data)?;
                }
                FieldKind::Vector4I64(data) => {
                    write_vec_n(file, 46, data)?;
                }
                FieldKind::Vector2U64(data) => {
                    write_vec_n(file, 47, data)?;
                }
                FieldKind::Vector3U64(data) => {
                    write_vec_n(file, 48, data)?;
                }
                FieldKind::Vector4U64(data) => {
                    write_vec_n(file, 49, data)?;
                }
            }
            Ok(())
        }
        fn load(file: &mut dyn Read) -> Result<Field, VisitError> {
            fn read_vec_n<T, S, const N: usize>(
                file: &mut dyn Read,
            ) -> Result<Matrix<T, Const<N>, U1, S>, VisitError>
            where
                T: VisitableElementaryField + Scalar + Default,
                S: RawStorage<T, Const<N>> + RawStorageMut<T, Const<N>> + Default,
            {
                let mut vec = Matrix::<T, Const<N>, U1, S>::default();
                for v in vec.iter_mut() {
                    v.read(file)?;
                }
                Ok(vec)
            }
            let name_len = file.read_u32::<LittleEndian>()? as usize;
            let mut raw_name = ::alloc::vec::from_elem(Default::default(), name_len);
            file.read_exact(raw_name.as_mut_slice())?;
            let id = file.read_u8()?;
            Ok(
                Field::new(
                    String::from_utf8(raw_name)?.as_str(),
                    match id {
                        1 => FieldKind::U8(file.read_u8()?),
                        2 => FieldKind::I8(file.read_i8()?),
                        3 => FieldKind::U16(file.read_u16::<LittleEndian>()?),
                        4 => FieldKind::I16(file.read_i16::<LittleEndian>()?),
                        5 => FieldKind::U32(file.read_u32::<LittleEndian>()?),
                        6 => FieldKind::I32(file.read_i32::<LittleEndian>()?),
                        7 => FieldKind::U64(file.read_u64::<LittleEndian>()?),
                        8 => FieldKind::I64(file.read_i64::<LittleEndian>()?),
                        9 => FieldKind::F32(file.read_f32::<LittleEndian>()?),
                        10 => FieldKind::F64(file.read_f64::<LittleEndian>()?),
                        11 => {
                            FieldKind::Vector3F32({
                                let x = file.read_f32::<LittleEndian>()?;
                                let y = file.read_f32::<LittleEndian>()?;
                                let z = file.read_f32::<LittleEndian>()?;
                                Vector3::new(x, y, z)
                            })
                        }
                        12 => {
                            FieldKind::UnitQuaternion({
                                let x = file.read_f32::<LittleEndian>()?;
                                let y = file.read_f32::<LittleEndian>()?;
                                let z = file.read_f32::<LittleEndian>()?;
                                let w = file.read_f32::<LittleEndian>()?;
                                UnitQuaternion::new_normalize(Quaternion::new(w, x, y, z))
                            })
                        }
                        13 => {
                            FieldKind::Matrix4({
                                let mut f = [0.0f32; 16];
                                for n in &mut f {
                                    *n = file.read_f32::<LittleEndian>()?;
                                }
                                Matrix4::from_row_slice(&f)
                            })
                        }
                        14 => {
                            FieldKind::BinaryBlob({
                                let len = file.read_u32::<LittleEndian>()? as usize;
                                let mut vec = ::alloc::vec::from_elem(
                                    Default::default(),
                                    len,
                                );
                                file.read_exact(vec.as_mut_slice())?;
                                vec
                            })
                        }
                        15 => FieldKind::Bool(file.read_u8()? != 0),
                        16 => {
                            FieldKind::Matrix3({
                                let mut f = [0.0f32; 9];
                                for n in &mut f {
                                    *n = file.read_f32::<LittleEndian>()?;
                                }
                                Matrix3::from_row_slice(&f)
                            })
                        }
                        17 => {
                            FieldKind::Vector2F32({
                                let x = file.read_f32::<LittleEndian>()?;
                                let y = file.read_f32::<LittleEndian>()?;
                                Vector2::new(x, y)
                            })
                        }
                        18 => {
                            FieldKind::Vector4F32({
                                let x = file.read_f32::<LittleEndian>()?;
                                let y = file.read_f32::<LittleEndian>()?;
                                let z = file.read_f32::<LittleEndian>()?;
                                let w = file.read_f32::<LittleEndian>()?;
                                Vector4::new(x, y, z, w)
                            })
                        }
                        19 => {
                            FieldKind::Uuid({
                                let mut bytes = uuid::Bytes::default();
                                file.read_exact(&mut bytes)?;
                                Uuid::from_bytes(bytes)
                            })
                        }
                        20 => {
                            FieldKind::UnitComplex({
                                let re = file.read_f32::<LittleEndian>()?;
                                let im = file.read_f32::<LittleEndian>()?;
                                UnitComplex::from_complex(Complex::new(re, im))
                            })
                        }
                        21 => {
                            let type_id = file.read_u8()?;
                            let element_size = file.read_u32::<LittleEndian>()?;
                            let data_size = file.read_u64::<LittleEndian>()?;
                            let mut bytes = ::alloc::vec::from_elem(
                                0,
                                data_size as usize,
                            );
                            file.read_exact(&mut bytes)?;
                            FieldKind::PodArray {
                                type_id,
                                element_size,
                                bytes,
                            }
                        }
                        22 => {
                            FieldKind::Matrix2({
                                let mut f = [0.0f32; 3];
                                for n in &mut f {
                                    *n = file.read_f32::<LittleEndian>()?;
                                }
                                Matrix2::from_row_slice(&f)
                            })
                        }
                        23 => FieldKind::Vector2F64(read_vec_n(file)?),
                        24 => FieldKind::Vector3F64(read_vec_n(file)?),
                        25 => FieldKind::Vector4F64(read_vec_n(file)?),
                        26 => FieldKind::Vector2I8(read_vec_n(file)?),
                        27 => FieldKind::Vector3I8(read_vec_n(file)?),
                        28 => FieldKind::Vector4I8(read_vec_n(file)?),
                        29 => FieldKind::Vector2U8(read_vec_n(file)?),
                        30 => FieldKind::Vector3U8(read_vec_n(file)?),
                        31 => FieldKind::Vector4U8(read_vec_n(file)?),
                        32 => FieldKind::Vector2I16(read_vec_n(file)?),
                        33 => FieldKind::Vector3I16(read_vec_n(file)?),
                        34 => FieldKind::Vector4I16(read_vec_n(file)?),
                        35 => FieldKind::Vector2U16(read_vec_n(file)?),
                        36 => FieldKind::Vector3U16(read_vec_n(file)?),
                        37 => FieldKind::Vector4U16(read_vec_n(file)?),
                        38 => FieldKind::Vector2I32(read_vec_n(file)?),
                        39 => FieldKind::Vector3I32(read_vec_n(file)?),
                        40 => FieldKind::Vector4I32(read_vec_n(file)?),
                        41 => FieldKind::Vector2U32(read_vec_n(file)?),
                        42 => FieldKind::Vector3U32(read_vec_n(file)?),
                        43 => FieldKind::Vector4U32(read_vec_n(file)?),
                        44 => FieldKind::Vector2I64(read_vec_n(file)?),
                        45 => FieldKind::Vector3I64(read_vec_n(file)?),
                        46 => FieldKind::Vector4I64(read_vec_n(file)?),
                        47 => FieldKind::Vector2U64(read_vec_n(file)?),
                        48 => FieldKind::Vector3U64(read_vec_n(file)?),
                        49 => FieldKind::Vector4U64(read_vec_n(file)?),
                        _ => return Err(VisitError::UnknownFieldType(id)),
                    },
                ),
            )
        }
        fn as_string(&self) -> String {
            {
                let res = ::alloc::fmt::format(
                    format_args!("{0}{1}", self.name, self.kind.as_string()),
                );
                res
            }
        }
    }
    /// A node is a collection of [Fields](Field) that exists within a tree of nodes
    /// that allows a [Visitor] to store its data.
    /// Each node has a name, and may have a parent node and child nodes.
    pub struct VisitorNode {
        name: String,
        fields: Vec<Field>,
        parent: Handle<VisitorNode>,
        children: Vec<Handle<VisitorNode>>,
    }
    impl VisitorNode {
        fn new(name: &str, parent: Handle<VisitorNode>) -> Self {
            Self {
                name: name.to_owned(),
                fields: Vec::new(),
                parent,
                children: Vec::new(),
            }
        }
    }
    impl Default for VisitorNode {
        fn default() -> Self {
            Self {
                name: String::new(),
                fields: Vec::new(),
                parent: Handle::NONE,
                children: Vec::new(),
            }
        }
    }
    /// A RegionGuard is a [Visitor] that automatically leaves the current region
    /// when it is dropped.
    #[must_use = "the guard must be used"]
    pub struct RegionGuard<'a>(&'a mut Visitor);
    impl<'a> Deref for RegionGuard<'a> {
        type Target = Visitor;
        fn deref(&self) -> &Self::Target {
            self.0
        }
    }
    impl<'a> DerefMut for RegionGuard<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0
        }
    }
    impl<'a> Drop for RegionGuard<'a> {
        fn drop(&mut self) {
            self.0.leave_region().unwrap();
        }
    }
    /// A Blackboard is a mapping from TypeId to value that allows a [Visitor] to store
    /// a particular value for each registered type.
    pub struct Blackboard {
        items: FxHashMap<TypeId, Arc<dyn Any>>,
    }
    #[automatically_derived]
    impl ::core::default::Default for Blackboard {
        #[inline]
        fn default() -> Blackboard {
            Blackboard {
                items: ::core::default::Default::default(),
            }
        }
    }
    impl Blackboard {
        pub fn new() -> Self {
            Self { items: Default::default() }
        }
        pub fn register<T: Any>(&mut self, value: Arc<T>) {
            self.items.insert(TypeId::of::<T>(), value);
        }
        pub fn get<T: Any>(&self) -> Option<&T> {
            self.items.get(&TypeId::of::<T>()).and_then(|v| (**v).downcast_ref::<T>())
        }
        pub fn inner(&self) -> &FxHashMap<TypeId, Arc<dyn Any>> {
            &self.items
        }
        pub fn inner_mut(&mut self) -> &mut FxHashMap<TypeId, Arc<dyn Any>> {
            &mut self.items
        }
    }
    /// Flags that can be used to influence the behaviour of [Visit::visit] methods.
    pub struct VisitorFlags(
        <VisitorFlags as ::bitflags::__private::PublicFlags>::Internal,
    );
    impl VisitorFlags {
        /// No flags set, do nothing special.
        #[allow(deprecated, non_upper_case_globals)]
        pub const NONE: Self = Self::from_bits_retain(0);
        /// Tell [crate::variable::InheritableVariable::visit] to assume that it's
        /// [VariableFlags::MODIFIED](create::variable::VariableFlags::MODIFIED) is set,
        /// and therefore write its data. Otherwise, InheritableVariable has the special
        /// property of *not writing itself* when the `MODIFIED` flag is not set.
        #[allow(deprecated, non_upper_case_globals)]
        pub const SERIALIZE_EVERYTHING: Self = Self::from_bits_retain(1 << 1);
    }
    impl ::bitflags::Flags for VisitorFlags {
        const FLAGS: &'static [::bitflags::Flag<VisitorFlags>] = &[
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new("NONE", VisitorFlags::NONE)
            },
            {
                #[allow(deprecated, non_upper_case_globals)]
                ::bitflags::Flag::new(
                    "SERIALIZE_EVERYTHING",
                    VisitorFlags::SERIALIZE_EVERYTHING,
                )
            },
        ];
        type Bits = u32;
        fn bits(&self) -> u32 {
            VisitorFlags::bits(self)
        }
        fn from_bits_retain(bits: u32) -> VisitorFlags {
            VisitorFlags::from_bits_retain(bits)
        }
    }
    #[allow(
        dead_code,
        deprecated,
        unused_doc_comments,
        unused_attributes,
        unused_mut,
        unused_imports,
        non_upper_case_globals,
        clippy::assign_op_pattern,
        clippy::indexing_slicing,
        clippy::same_name_method,
        clippy::iter_without_into_iter,
    )]
    const _: () = {
        #[repr(transparent)]
        pub struct InternalBitFlags(u32);
        #[automatically_derived]
        impl ::core::clone::Clone for InternalBitFlags {
            #[inline]
            fn clone(&self) -> InternalBitFlags {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for InternalBitFlags {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for InternalBitFlags {
            #[inline]
            fn eq(&self, other: &InternalBitFlags) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for InternalBitFlags {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for InternalBitFlags {
            #[inline]
            fn partial_cmp(
                &self,
                other: &InternalBitFlags,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for InternalBitFlags {
            #[inline]
            fn cmp(&self, other: &InternalBitFlags) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for InternalBitFlags {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        impl ::bitflags::__private::PublicFlags for VisitorFlags {
            type Primitive = u32;
            type Internal = InternalBitFlags;
        }
        impl ::bitflags::__private::core::default::Default for InternalBitFlags {
            #[inline]
            fn default() -> Self {
                InternalBitFlags::empty()
            }
        }
        impl ::bitflags::__private::core::fmt::Debug for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                if self.is_empty() {
                    f.write_fmt(format_args!("{0:#x}", <u32 as ::bitflags::Bits>::EMPTY))
                } else {
                    ::bitflags::__private::core::fmt::Display::fmt(self, f)
                }
            }
        }
        impl ::bitflags::__private::core::fmt::Display for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter<'_>,
            ) -> ::bitflags::__private::core::fmt::Result {
                ::bitflags::parser::to_writer(&VisitorFlags(*self), f)
            }
        }
        impl ::bitflags::__private::core::str::FromStr for InternalBitFlags {
            type Err = ::bitflags::parser::ParseError;
            fn from_str(
                s: &str,
            ) -> ::bitflags::__private::core::result::Result<Self, Self::Err> {
                ::bitflags::parser::from_str::<VisitorFlags>(s).map(|flags| flags.0)
            }
        }
        impl ::bitflags::__private::core::convert::AsRef<u32> for InternalBitFlags {
            fn as_ref(&self) -> &u32 {
                &self.0
            }
        }
        impl ::bitflags::__private::core::convert::From<u32> for InternalBitFlags {
            fn from(bits: u32) -> Self {
                Self::from_bits_retain(bits)
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl InternalBitFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                { Self(<u32 as ::bitflags::Bits>::EMPTY) }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                {
                    let mut truncated = <u32 as ::bitflags::Bits>::EMPTY;
                    let mut i = 0;
                    {
                        {
                            let flag = <VisitorFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    {
                        {
                            let flag = <VisitorFlags as ::bitflags::Flags>::FLAGS[i]
                                .value()
                                .bits();
                            truncated = truncated | flag;
                            i += 1;
                        }
                    };
                    let _ = i;
                    Self::from_bits_retain(truncated)
                }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                let f = self;
                { f.0 }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u32,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    let truncated = Self::from_bits_truncate(bits).0;
                    if truncated == bits {
                        ::bitflags::__private::core::option::Option::Some(Self(bits))
                    } else {
                        ::bitflags::__private::core::option::Option::None
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                let bits = bits;
                { Self(bits & Self::all().bits()) }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                let bits = bits;
                { Self(bits) }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    {
                        if name == "NONE" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(VisitorFlags::NONE.bits()),
                            );
                        }
                    };
                    {
                        if name == "SERIALIZE_EVERYTHING" {
                            return ::bitflags::__private::core::option::Option::Some(
                                Self(VisitorFlags::SERIALIZE_EVERYTHING.bits()),
                            );
                        }
                    };
                    let _ = name;
                    ::bitflags::__private::core::option::Option::None
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                { f.bits() == <u32 as ::bitflags::Bits>::EMPTY }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                { Self::all().bits() | f.bits() == f.bits() }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.bits() & other.bits() != <u32 as ::bitflags::Bits>::EMPTY }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.bits() & other.bits() == other.bits() }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).union(other);
                }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).difference(other);
                }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                {
                    *f = Self::from_bits_retain(f.bits()).symmetric_difference(other);
                }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                {
                    if value {
                        f.insert(other);
                    } else {
                        f.remove(other);
                    }
                }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() & other.bits()) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() | other.bits()) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() & !other.bits()) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self::from_bits_retain(f.bits() ^ other.bits()) }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                { Self::from_bits_truncate(!f.bits()) }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for InternalBitFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for InternalBitFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: InternalBitFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for InternalBitFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for InternalBitFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for InternalBitFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for InternalBitFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for InternalBitFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for InternalBitFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for InternalBitFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<InternalBitFlags>
        for InternalBitFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl InternalBitFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<VisitorFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <VisitorFlags as ::bitflags::Flags>::FLAGS,
                    VisitorFlags::from_bits_retain(self.bits()),
                    VisitorFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<VisitorFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <VisitorFlags as ::bitflags::Flags>::FLAGS,
                    VisitorFlags::from_bits_retain(self.bits()),
                    VisitorFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for InternalBitFlags {
            type Item = VisitorFlags;
            type IntoIter = ::bitflags::iter::Iter<VisitorFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
        impl InternalBitFlags {
            /// Returns a mutable reference to the raw value of the flags currently stored.
            #[inline]
            pub fn bits_mut(&mut self) -> &mut u32 {
                &mut self.0
            }
        }
        #[allow(dead_code, deprecated, unused_attributes)]
        impl VisitorFlags {
            /// Get a flags value with all bits unset.
            #[inline]
            pub const fn empty() -> Self {
                { Self(InternalBitFlags::empty()) }
            }
            /// Get a flags value with all known bits set.
            #[inline]
            pub const fn all() -> Self {
                { Self(InternalBitFlags::all()) }
            }
            /// Get the underlying bits value.
            ///
            /// The returned value is exactly the bits set in this flags value.
            #[inline]
            pub const fn bits(&self) -> u32 {
                let f = self;
                { f.0.bits() }
            }
            /// Convert from a bits value.
            ///
            /// This method will return `None` if any unknown bits are set.
            #[inline]
            pub const fn from_bits(
                bits: u32,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let bits = bits;
                {
                    match InternalBitFlags::from_bits(bits) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        }
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
            }
            /// Convert from a bits value, unsetting any unknown bits.
            #[inline]
            pub const fn from_bits_truncate(bits: u32) -> Self {
                let bits = bits;
                { Self(InternalBitFlags::from_bits_truncate(bits)) }
            }
            /// Convert from a bits value exactly.
            #[inline]
            pub const fn from_bits_retain(bits: u32) -> Self {
                let bits = bits;
                { Self(InternalBitFlags::from_bits_retain(bits)) }
            }
            /// Get a flags value with the bits of a flag with the given name set.
            ///
            /// This method will return `None` if `name` is empty or doesn't
            /// correspond to any named flag.
            #[inline]
            pub fn from_name(
                name: &str,
            ) -> ::bitflags::__private::core::option::Option<Self> {
                let name = name;
                {
                    match InternalBitFlags::from_name(name) {
                        ::bitflags::__private::core::option::Option::Some(bits) => {
                            ::bitflags::__private::core::option::Option::Some(Self(bits))
                        }
                        ::bitflags::__private::core::option::Option::None => {
                            ::bitflags::__private::core::option::Option::None
                        }
                    }
                }
            }
            /// Whether all bits in this flags value are unset.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let f = self;
                { f.0.is_empty() }
            }
            /// Whether all known bits in this flags value are set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let f = self;
                { f.0.is_all() }
            }
            /// Whether any set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.0.intersects(other.0) }
            }
            /// Whether all set bits in a source flags value are also set in a target flags value.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let f = self;
                let other = other;
                { f.0.contains(other.0) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let f = self;
                let other = other;
                { f.0.insert(other.0) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `remove` won't truncate `other`, but the `!` operator will.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let f = self;
                let other = other;
                { f.0.remove(other.0) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let f = self;
                let other = other;
                { f.0.toggle(other.0) }
            }
            /// Call `insert` when `value` is `true` or `remove` when `value` is `false`.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let f = self;
                let other = other;
                let value = value;
                { f.0.set(other.0, value) }
            }
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.intersection(other.0)) }
            }
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.union(other.0)) }
            }
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.difference(other.0)) }
            }
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let f = self;
                let other = other;
                { Self(f.0.symmetric_difference(other.0)) }
            }
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let f = self;
                { Self(f.0.complement()) }
            }
        }
        impl ::bitflags::__private::core::fmt::Binary for VisitorFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Binary::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::Octal for VisitorFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::Octal::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::LowerHex for VisitorFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::LowerHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::fmt::UpperHex for VisitorFlags {
            fn fmt(
                &self,
                f: &mut ::bitflags::__private::core::fmt::Formatter,
            ) -> ::bitflags::__private::core::fmt::Result {
                let inner = self.0;
                ::bitflags::__private::core::fmt::UpperHex::fmt(&inner, f)
            }
        }
        impl ::bitflags::__private::core::ops::BitOr for VisitorFlags {
            type Output = Self;
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor(self, other: VisitorFlags) -> Self {
                self.union(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitOrAssign for VisitorFlags {
            /// The bitwise or (`|`) of the bits in two flags values.
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                self.insert(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitXor for VisitorFlags {
            type Output = Self;
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor(self, other: Self) -> Self {
                self.symmetric_difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitXorAssign for VisitorFlags {
            /// The bitwise exclusive-or (`^`) of the bits in two flags values.
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                self.toggle(other);
            }
        }
        impl ::bitflags::__private::core::ops::BitAnd for VisitorFlags {
            type Output = Self;
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }
        impl ::bitflags::__private::core::ops::BitAndAssign for VisitorFlags {
            /// The bitwise and (`&`) of the bits in two flags values.
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = Self::from_bits_retain(self.bits()).intersection(other);
            }
        }
        impl ::bitflags::__private::core::ops::Sub for VisitorFlags {
            type Output = Self;
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub(self, other: Self) -> Self {
                self.difference(other)
            }
        }
        impl ::bitflags::__private::core::ops::SubAssign for VisitorFlags {
            /// The intersection of a source flags value with the complement of a target flags value (`&!`).
            ///
            /// This method is not equivalent to `self & !other` when `other` has unknown bits set.
            /// `difference` won't truncate `other`, but the `!` operator will.
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                self.remove(other);
            }
        }
        impl ::bitflags::__private::core::ops::Not for VisitorFlags {
            type Output = Self;
            /// The bitwise negation (`!`) of the bits in a flags value, truncating the result.
            #[inline]
            fn not(self) -> Self {
                self.complement()
            }
        }
        impl ::bitflags::__private::core::iter::Extend<VisitorFlags> for VisitorFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn extend<T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>>(
                &mut self,
                iterator: T,
            ) {
                for item in iterator {
                    self.insert(item)
                }
            }
        }
        impl ::bitflags::__private::core::iter::FromIterator<VisitorFlags>
        for VisitorFlags {
            /// The bitwise or (`|`) of the bits in each flags value.
            fn from_iter<
                T: ::bitflags::__private::core::iter::IntoIterator<Item = Self>,
            >(iterator: T) -> Self {
                use ::bitflags::__private::core::iter::Extend;
                let mut result = Self::empty();
                result.extend(iterator);
                result
            }
        }
        impl VisitorFlags {
            /// Yield a set of contained flags values.
            ///
            /// Each yielded flags value will correspond to a defined named flag. Any unknown bits
            /// will be yielded together as a final flags value.
            #[inline]
            pub const fn iter(&self) -> ::bitflags::iter::Iter<VisitorFlags> {
                ::bitflags::iter::Iter::__private_const_new(
                    <VisitorFlags as ::bitflags::Flags>::FLAGS,
                    VisitorFlags::from_bits_retain(self.bits()),
                    VisitorFlags::from_bits_retain(self.bits()),
                )
            }
            /// Yield a set of contained named flags values.
            ///
            /// This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
            /// Any unknown bits, or bits not corresponding to a contained flag will not be yielded.
            #[inline]
            pub const fn iter_names(&self) -> ::bitflags::iter::IterNames<VisitorFlags> {
                ::bitflags::iter::IterNames::__private_const_new(
                    <VisitorFlags as ::bitflags::Flags>::FLAGS,
                    VisitorFlags::from_bits_retain(self.bits()),
                    VisitorFlags::from_bits_retain(self.bits()),
                )
            }
        }
        impl ::bitflags::__private::core::iter::IntoIterator for VisitorFlags {
            type Item = VisitorFlags;
            type IntoIter = ::bitflags::iter::Iter<VisitorFlags>;
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
    /// A collection of nodes that stores data that can be read or write values of types with the [Visit] trait.
    ///
    /// Instead of calling methods of the visitor in order to read or write the visitor's data, reading
    /// and writing happens in the [Visit::visit] method of a variable that will either store the read value
    /// or holds the value to be written.
    ///
    /// For example, `x.visit("MyValue", &mut visitor)` will do one of:
    ///
    /// 1. Take the value of `x` and store it in `visitor` under the name "MyValue", if `visitor.is_reading()` is false.
    /// 2. Read a value named "MyValue" from `visitor` and store it in `x`, if `visitor.is_reading()` is true.
    ///
    /// Whether the value of `x` gets written into `visitor` or overwitten with a value from `visitor` is determined
    /// by whether [Visitor::is_reading()] returns true or false.
    pub struct Visitor {
        nodes: Pool<VisitorNode>,
        rc_map: FxHashMap<u64, Rc<dyn Any>>,
        arc_map: FxHashMap<u64, Arc<dyn Any + Send + Sync>>,
        reading: bool,
        current_node: Handle<VisitorNode>,
        root: Handle<VisitorNode>,
        /// A place to store whatever objects may be needed to assist with reading and writing values.
        pub blackboard: Blackboard,
        /// Flags that can activate special behaviour in some Visit values, such as
        /// [crate::variable::InheritableVariable].
        pub flags: VisitorFlags,
    }
    /// Trait of types that can be read from a [Visitor] or written to a Visitor.
    pub trait Visit {
        /// Read or write this value, depending on whether [Visitor::is_reading()] is true or false.
        ///
        /// # In Write Mode
        ///
        /// The given name is a key to identify where this value will be stored in the visitor.
        /// Whether this name indicates a field or a region is determined by the value being visited.
        /// No two regions can exist with the same name as children of a single node,
        /// and no two fields can exist with the same name within a single node,
        /// but a region may share the same name as a field. If a name clash occurs, then an error
        /// is returned. Otherwise the value is written into the Visitor data at the given name.
        ///
        /// # In Read Mode
        ///
        /// The given name is a key to identify where this value should be found the visitor.
        /// Whether the name indicates a field or a region is determined by the the value being visited.
        /// If the field or region is not found with the given name
        /// then an error is returned. Otherwise the value being visited will be mutated
        /// to match the data in the visitor.
        ///
        /// # Buiding a Complex Value out of Multiple Fields
        ///
        /// If representing this value requires more than one field,
        /// [Visitor::enter_region] can be used to create a new node within the
        /// visitor with the given name, and the fields of this value can then read from
        /// or write to that node using the returned Visitor without risk of name
        /// clashes.
        ///
        /// See the documentation for [the Visit derive macro](fyrox_core_derive::Visit) for examples of how to
        /// implement Visit for some simple types.
        ///
        /// # Abnormal Implementations
        ///
        /// Types with special needs may choose to read and write in unusual ways. In addition to choosing
        /// whether they will store their data in a region or a field, a value might choose to do neither.
        /// A value may also choose to attempt to read its data in multiple ways so as to remain
        /// backwards-compatible with older versions where the format of data storage may be different.
        ///
        /// See [crate::variable::InheritableVariable::visit] for an example of an abnormal implementation of Visit.
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult;
    }
    impl Default for Visitor {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Visitor {
        /// Sequence of bytes that is automatically written at the start when a visitor
        /// is encoded into bytes. It is written by [Visitor::save_binary], [Visitor::save_binary_to_memory],
        /// and [Visitor::save_binary_to_vec].
        ///
        /// [Visitor::load_binary] will return an error if this sequence of bytes is not present at the beginning
        /// of the file, and [Visitor::load_from_memory] will return an error of these bytes are not at the beginning
        /// of the given slice.
        pub const MAGIC: &'static str = "RG3D";
        /// Creates a Visitor containing only a single node called "`__ROOT__`" which will be the
        /// current region of the visitor.
        pub fn new() -> Self {
            let mut nodes = Pool::new();
            let root = nodes.spawn(VisitorNode::new("__ROOT__", Handle::NONE));
            Self {
                nodes,
                rc_map: FxHashMap::default(),
                arc_map: FxHashMap::default(),
                reading: false,
                current_node: root,
                root,
                blackboard: Blackboard::new(),
                flags: VisitorFlags::NONE,
            }
        }
        fn find_field(&mut self, name: &str) -> Option<&mut Field> {
            self.nodes
                .borrow_mut(self.current_node)
                .fields
                .iter_mut()
                .find(|field| field.name == name)
        }
        /// True if this Visitor is changing the values that it visits.
        /// In other words `x.visit("MyValue", &mut visitor)` will result in `x` being mutated to match
        /// whatever value is stored in `visitor`.
        ///
        /// False if this visitor is copying and storing the values that it visits.
        /// In other words `x.visit("MyValue", &mut visitor)` will result in `x` being unchanged,
        /// but `visitor` will be mutated to store the value of `x` under the name "MyValue".
        pub fn is_reading(&self) -> bool {
            self.reading
        }
        fn current_node(&mut self) -> &mut VisitorNode {
            self.nodes.borrow_mut(self.current_node)
        }
        /// If [Visitor::is_reading], find a node with the given name that is a child
        /// of the current node, and return a Visitor for the found node. Return an error
        /// if no node with that name exists.
        ///
        /// If not reading, create a node with the given name as a chld of the current
        /// node, and return a visitor for the new node. Return an error if a node with
        /// that name already exists.
        pub fn enter_region(&mut self, name: &str) -> Result<RegionGuard, VisitError> {
            let node = self.nodes.borrow(self.current_node);
            if self.reading {
                let mut region = Handle::NONE;
                for child_handle in node.children.iter() {
                    let child = self.nodes.borrow(*child_handle);
                    if child.name == name {
                        region = *child_handle;
                        break;
                    }
                }
                if region.is_some() {
                    self.current_node = region;
                    Ok(RegionGuard(self))
                } else {
                    Err(VisitError::RegionDoesNotExist(name.to_owned()))
                }
            } else {
                for child_handle in node.children.iter() {
                    let child = self.nodes.borrow(*child_handle);
                    if child.name == name {
                        return Err(VisitError::RegionAlreadyExists(name.to_owned()));
                    }
                }
                let node_handle = self
                    .nodes
                    .spawn(VisitorNode::new(name, self.current_node));
                self.nodes.borrow_mut(self.current_node).children.push(node_handle);
                self.current_node = node_handle;
                Ok(RegionGuard(self))
            }
        }
        /// The name of the current region.
        /// This should never be None if the Visitor is operating normally,
        /// because there should be no way to leave the initial `__ROOT__` region.
        pub fn current_region(&self) -> Option<&str> {
            self.nodes.try_borrow(self.current_node).map(|n| n.name.as_str())
        }
        fn leave_region(&mut self) -> VisitResult {
            self.current_node = self.nodes.borrow(self.current_node).parent;
            if self.current_node.is_none() {
                Err(VisitError::NoActiveNode)
            } else {
                Ok(())
            }
        }
        fn print_node(
            &self,
            node_handle: Handle<VisitorNode>,
            nesting: usize,
            out_string: &mut String,
        ) {
            let offset = (0..nesting).map(|_| "\t").collect::<String>();
            let node = self.nodes.borrow(node_handle);
            *out_string
                += {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}{1}[Fields={2}, Children={3}]: ",
                            offset,
                            node.name,
                            node.fields.len(),
                            node.children.len(),
                        ),
                    );
                    res
                }
                    .as_str();
            for field in node.fields.iter() {
                *out_string += field.as_string().as_str();
            }
            *out_string += "\n";
            for child_handle in node.children.iter() {
                self.print_node(*child_handle, nesting + 1, out_string);
            }
        }
        /// Create a String containing all the data of this Visitor.
        /// The String is formatted to be human-readable with each node on its own line
        /// and tabs to indent child nodes.
        pub fn save_text(&self) -> String {
            let mut out_string = String::new();
            self.print_node(self.root, 0, &mut out_string);
            out_string
        }
        /// Write the data of this Visitor to the given writer.
        /// Begin by writing [Visitor::MAGIC].
        pub fn save_binary_to_memory<W: Write>(&self, mut writer: W) -> VisitResult {
            writer.write_all(Self::MAGIC.as_bytes())?;
            let mut stack = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([self.root]),
            );
            while let Some(node_handle) = stack.pop() {
                let node = self.nodes.borrow(node_handle);
                let name = node.name.as_bytes();
                writer.write_u32::<LittleEndian>(name.len() as u32)?;
                writer.write_all(name)?;
                writer.write_u32::<LittleEndian>(node.fields.len() as u32)?;
                for field in node.fields.iter() {
                    Field::save(field, &mut writer)?
                }
                writer.write_u32::<LittleEndian>(node.children.len() as u32)?;
                stack.extend_from_slice(&node.children);
            }
            Ok(())
        }
        /// Encode the data of this visitor into bytes and push the bytes
        /// into the given `Vec<u8>`.
        /// Begin by writing [Visitor::MAGIC].
        pub fn save_binary_to_vec(&self) -> Result<Vec<u8>, VisitError> {
            let mut writer = Cursor::new(Vec::new());
            self.save_binary_to_memory(&mut writer)?;
            Ok(writer.into_inner())
        }
        /// Create a file at the given path and write the data of this visitor
        /// into that file in a non-human-readable binary format so that the data
        /// can be reconstructed using [Visitor::load_binary].
        /// Begin by writing [Visitor::MAGIC].
        pub fn save_binary<P: AsRef<Path>>(&self, path: P) -> VisitResult {
            let writer = BufWriter::new(File::create(path)?);
            self.save_binary_to_memory(writer)
        }
        fn load_node_binary(
            &mut self,
            file: &mut dyn Read,
        ) -> Result<Handle<VisitorNode>, VisitError> {
            let name_len = file.read_u32::<LittleEndian>()? as usize;
            let mut raw_name = ::alloc::vec::from_elem(Default::default(), name_len);
            file.read_exact(raw_name.as_mut_slice())?;
            let mut node = VisitorNode {
                name: String::from_utf8(raw_name)?,
                ..VisitorNode::default()
            };
            let field_count = file.read_u32::<LittleEndian>()? as usize;
            for _ in 0..field_count {
                let field = Field::load(file)?;
                node.fields.push(field);
            }
            let child_count = file.read_u32::<LittleEndian>()? as usize;
            let mut children = Vec::with_capacity(child_count);
            for _ in 0..child_count {
                children.push(self.load_node_binary(file)?);
            }
            node.children.clone_from(&children);
            let handle = self.nodes.spawn(node);
            for child_handle in children.iter() {
                let child = self.nodes.borrow_mut(*child_handle);
                child.parent = handle;
            }
            Ok(handle)
        }
        /// Create a visitor by reading data from the file at the given path,
        /// assuming that the file was created using [Visitor::save_binary].
        /// Return a [VisitError::NotSupportedFormat] if [Visitor::MAGIC] is not the first bytes read from the file.
        pub async fn load_binary<P: AsRef<Path>>(path: P) -> Result<Self, VisitError> {
            Self::load_from_memory(&io::load_file(path).await?)
        }
        /// Create a visitor by decoding data from the given byte slice,
        /// assuming that the bytes are in the format that would be produced
        /// by [Visitor::save_binary_to_vec].
        /// Return a [VisitError::NotSupportedFormat] if [Visitor::MAGIC] is not the first bytes read from the slice.
        pub fn load_from_memory(data: &[u8]) -> Result<Self, VisitError> {
            let mut reader = Cursor::new(data);
            let mut magic: [u8; 4] = Default::default();
            reader.read_exact(&mut magic)?;
            if !magic.eq(Self::MAGIC.as_bytes()) {
                return Err(VisitError::NotSupportedFormat);
            }
            let mut visitor = Self {
                nodes: Pool::new(),
                rc_map: Default::default(),
                arc_map: Default::default(),
                reading: true,
                current_node: Handle::NONE,
                root: Handle::NONE,
                blackboard: Blackboard::new(),
                flags: VisitorFlags::NONE,
            };
            visitor.root = visitor.load_node_binary(&mut reader)?;
            visitor.current_node = visitor.root;
            Ok(visitor)
        }
    }
    impl<T> Visit for RefCell<T>
    where
        T: Visit + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            if let Ok(mut data) = self.try_borrow_mut() {
                data.visit(name, visitor)
            } else {
                Err(VisitError::RefCellAlreadyMutableBorrowed)
            }
        }
    }
    impl<T> Visit for Vec<T>
    where
        T: Default + Visit + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut len = self.len() as u32;
            len.visit("Length", &mut region)?;
            if region.reading {
                self.clear();
                for index in 0..len {
                    let region_name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", index));
                        res
                    };
                    let mut region = region.enter_region(region_name.as_str())?;
                    let mut object = T::default();
                    object.visit("ItemData", &mut region)?;
                    self.push(object);
                }
            } else {
                for (index, item) in self.iter_mut().enumerate() {
                    let region_name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", index));
                        res
                    };
                    let mut region = region.enter_region(region_name.as_str())?;
                    item.visit("ItemData", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl<T> Visit for Option<T>
    where
        T: Default + Visit + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut is_some = u8::from(self.is_some());
            is_some.visit("IsSome", &mut region)?;
            if is_some != 0 {
                if region.reading {
                    let mut value = T::default();
                    value.visit("Data", &mut region)?;
                    *self = Some(value);
                } else {
                    self.as_mut().unwrap().visit("Data", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl Visit for String {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut len = self.as_bytes().len() as u32;
            len.visit("Length", &mut region)?;
            let mut data = if region.reading {
                Vec::new()
            } else {
                Vec::from(self.as_bytes())
            };
            let mut proxy = BinaryBlob { vec: &mut data };
            proxy.visit("Data", &mut region)?;
            if region.reading {
                *self = String::from_utf8(data)?;
            }
            Ok(())
        }
    }
    impl Visit for PathBuf {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let portable_path = replace_slashes(&self);
            let bytes = if let Some(path_str) = portable_path.as_os_str().to_str() {
                path_str.as_bytes()
            } else {
                return Err(VisitError::InvalidName);
            };
            let mut len = bytes.len() as u32;
            len.visit("Length", &mut region)?;
            let mut data = if region.reading { Vec::new() } else { Vec::from(bytes) };
            let mut proxy = BinaryBlob { vec: &mut data };
            proxy.visit("Data", &mut region)?;
            if region.reading {
                *self = PathBuf::from(String::from_utf8(data)?);
            }
            Ok(())
        }
    }
    impl<T> Visit for Cell<T>
    where
        T: Copy + Clone + Visit + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut value = self.get();
            value.visit(name, visitor)?;
            if visitor.is_reading() {
                self.set(value);
            }
            Ok(())
        }
    }
    impl<T> Visit for Rc<T>
    where
        T: Visit + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            if region.reading {
                let mut raw = 0u64;
                raw.visit("Id", &mut region)?;
                if raw == 0 {
                    return Err(VisitError::UnexpectedRcNullIndex);
                }
                if let Some(ptr) = region.rc_map.get(&raw) {
                    if let Ok(res) = Rc::downcast::<T>(ptr.clone()) {
                        *self = res;
                    } else {
                        return Err(VisitError::TypeMismatch);
                    }
                } else {
                    region.rc_map.insert(raw, self.clone());
                    let raw = rc_to_raw(self);
                    unsafe { &mut *raw }.visit("RcData", &mut region)?;
                }
            } else {
                let raw = rc_to_raw(self);
                let mut index = raw as u64;
                index.visit("Id", &mut region)?;
                if let Entry::Vacant(entry) = region.rc_map.entry(index) {
                    entry.insert(self.clone());
                    unsafe { &mut *raw }.visit("RcData", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl<T> Visit for Mutex<T>
    where
        T: Visit + Send,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            self.get_mut()?.visit(name, visitor)
        }
    }
    impl<T> Visit for parking_lot::Mutex<T>
    where
        T: Visit + Send,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            self.get_mut().visit(name, visitor)
        }
    }
    impl<T> Visit for Box<T>
    where
        T: Visit,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            self.deref_mut().visit(name, visitor)
        }
    }
    impl<T> Visit for RwLock<T>
    where
        T: Visit + Send,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            self.write()?.visit(name, visitor)
        }
    }
    fn arc_to_raw<T>(arc: &Arc<T>) -> *mut T {
        &**arc as *const T as *mut T
    }
    fn rc_to_raw<T>(rc: &Rc<T>) -> *mut T {
        &**rc as *const T as *mut T
    }
    impl<T> Visit for Arc<T>
    where
        T: Visit + Send + Sync + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            if region.reading {
                let mut raw = 0u64;
                raw.visit("Id", &mut region)?;
                if raw == 0 {
                    return Err(VisitError::UnexpectedRcNullIndex);
                }
                if let Some(ptr) = &mut region.arc_map.get(&raw) {
                    if let Ok(res) = Arc::downcast::<T>(ptr.clone()) {
                        *self = res;
                    } else {
                        return Err(VisitError::TypeMismatch);
                    }
                } else {
                    region.arc_map.insert(raw, self.clone());
                    let raw = arc_to_raw(self);
                    unsafe { &mut *raw }.visit("ArcData", &mut region)?;
                }
            } else {
                let raw = arc_to_raw(self);
                let mut index = raw as u64;
                index.visit("Id", &mut region)?;
                if let Entry::Vacant(entry) = region.arc_map.entry(index) {
                    entry.insert(self.clone());
                    unsafe { &mut *raw }.visit("ArcData", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl<T> Visit for std::rc::Weak<T>
    where
        T: Default + Visit + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            if region.reading {
                let mut raw = 0u64;
                raw.visit("Id", &mut region)?;
                if raw != 0 {
                    if let Some(ptr) = &mut region.rc_map.get(&raw) {
                        if let Ok(res) = Rc::downcast::<T>(ptr.clone()) {
                            *self = Rc::downgrade(&res);
                        } else {
                            return Err(VisitError::TypeMismatch);
                        }
                    } else {
                        let rc = Rc::new(T::default());
                        region.rc_map.insert(raw, rc.clone());
                        let raw = rc_to_raw(&rc);
                        unsafe { &mut *raw }.visit("RcData", &mut region)?;
                        *self = Rc::downgrade(&rc);
                    }
                }
            } else if let Some(rc) = std::rc::Weak::upgrade(self) {
                let raw = rc_to_raw(&rc);
                let mut index = raw as u64;
                index.visit("Id", &mut region)?;
                if let Entry::Vacant(entry) = region.rc_map.entry(index) {
                    entry.insert(rc);
                    unsafe { &mut *raw }.visit("RcData", &mut region)?;
                }
            } else {
                let mut index = 0u64;
                index.visit("Id", &mut region)?;
            }
            Ok(())
        }
    }
    impl<T> Visit for std::sync::Weak<T>
    where
        T: Default + Visit + Send + Sync + 'static,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            if region.reading {
                let mut raw = 0u64;
                raw.visit("Id", &mut region)?;
                if raw != 0 {
                    if let Some(ptr) = region.arc_map.get(&raw) {
                        if let Ok(res) = Arc::downcast::<T>(ptr.clone()) {
                            *self = Arc::downgrade(&res);
                        } else {
                            return Err(VisitError::TypeMismatch);
                        }
                    } else {
                        let arc = Arc::new(T::default());
                        region.arc_map.insert(raw, arc.clone());
                        let raw = arc_to_raw(&arc);
                        unsafe { &mut *raw }.visit("ArcData", &mut region)?;
                        *self = Arc::downgrade(&arc);
                    }
                }
            } else if let Some(arc) = std::sync::Weak::upgrade(self) {
                let raw = arc_to_raw(&arc);
                let mut index = raw as u64;
                index.visit("Id", &mut region)?;
                if let Entry::Vacant(entry) = region.arc_map.entry(index) {
                    entry.insert(arc);
                    unsafe { &mut *raw }.visit("ArcData", &mut region)?;
                }
            } else {
                let mut index = 0u64;
                index.visit("Id", &mut region)?;
            }
            Ok(())
        }
    }
    impl<K, V, S> Visit for HashMap<K, V, S>
    where
        K: Visit + Default + Clone + Hash + Eq,
        V: Visit + Default,
        S: BuildHasher + Clone,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut count = self.len() as u32;
            count.visit("Count", &mut region)?;
            if region.is_reading() {
                self.clear();
                for i in 0..(count as usize) {
                    let name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", i));
                        res
                    };
                    let mut region = region.enter_region(name.as_str())?;
                    let mut key = K::default();
                    key.visit("Key", &mut region)?;
                    let mut value = V::default();
                    value.visit("Value", &mut region)?;
                    self.insert(key, value);
                }
            } else {
                for (i, (key, value)) in self.iter_mut().enumerate() {
                    let name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", i));
                        res
                    };
                    let mut region = region.enter_region(name.as_str())?;
                    let mut key = key.clone();
                    key.visit("Key", &mut region)?;
                    value.visit("Value", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl<K, S> Visit for HashSet<K, S>
    where
        K: Visit + Default + Clone + Hash + Eq,
        S: BuildHasher + Clone,
    {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut count = self.len() as u32;
            count.visit("Count", &mut region)?;
            if region.is_reading() {
                self.clear();
                for i in 0..(count as usize) {
                    let name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", i));
                        res
                    };
                    let mut region = region.enter_region(name.as_str())?;
                    let mut key = K::default();
                    key.visit("Key", &mut region)?;
                    self.insert(key);
                }
            } else {
                for (i, mut key) in self.clone().into_iter().enumerate() {
                    let name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", i));
                        res
                    };
                    let mut region = region.enter_region(name.as_str())?;
                    key.visit("Key", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl<T: Default + Visit, const SIZE: usize> Visit for [T; SIZE] {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut len = SIZE as u32;
            len.visit("Length", &mut region)?;
            if region.reading {
                if len > SIZE as u32 {
                    return VisitResult::Err(
                        VisitError::User({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Not enough space in static array, got {0}, needed {1}!",
                                    len,
                                    SIZE,
                                ),
                            );
                            res
                        }),
                    );
                }
                for index in 0..len {
                    let region_name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", index));
                        res
                    };
                    let mut region = region.enter_region(region_name.as_str())?;
                    let mut object = T::default();
                    object.visit("ItemData", &mut region)?;
                    self[index as usize] = object;
                }
            } else {
                for (index, item) in self.iter_mut().enumerate() {
                    let region_name = {
                        let res = ::alloc::fmt::format(format_args!("Item{0}", index));
                        res
                    };
                    let mut region = region.enter_region(region_name.as_str())?;
                    item.visit("ItemData", &mut region)?;
                }
            }
            Ok(())
        }
    }
    impl Visit for Duration {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            let mut secs: u64 = self.as_secs();
            let mut nanos: u32 = self.subsec_nanos();
            secs.visit("Secs", &mut region)?;
            nanos.visit("Nanos", &mut region)?;
            if region.is_reading() {
                *self = Duration::new(secs, nanos);
            }
            Ok(())
        }
    }
    impl Visit for char {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut bytes = *self as u32;
            bytes.visit(name, visitor)?;
            if visitor.is_reading() {
                *self = char::from_u32(bytes).unwrap();
            }
            Ok(())
        }
    }
    impl<T: Visit> Visit for Range<T> {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut region = visitor.enter_region(name)?;
            self.start.visit("Start", &mut region)?;
            self.end.visit("End", &mut region)?;
            Ok(())
        }
    }
    impl Visit for usize {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut this = *self as u64;
            this.visit(name, visitor)?;
            if visitor.is_reading() {
                *self = this as usize;
            }
            Ok(())
        }
    }
    impl Visit for isize {
        fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
            let mut this = *self as i64;
            this.visit(name, visitor)?;
            if visitor.is_reading() {
                *self = this as isize;
            }
            Ok(())
        }
    }
}
pub mod watcher {
    //! Resource watcher allows you to track changed resources and "tell" resource manager to reload
    //! them.
    use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
    use std::{
        path::Path, sync::mpsc::{channel, Receiver},
        time::Duration,
    };
    /// Resource watcher allows you to track changed resources and "tell" resource manager to reload
    /// them.
    pub struct FileSystemWatcher {
        #[allow(dead_code)]
        watcher: RecommendedWatcher,
        receiver: Receiver<notify::Result<Event>>,
    }
    impl FileSystemWatcher {
        /// Creates new resource watcher with a path to watch and notification delay.
        pub fn new<P: AsRef<Path>>(
            path: P,
            delay: Duration,
        ) -> Result<Self, notify::Error> {
            let (tx, rx) = channel();
            let mut watcher = RecommendedWatcher::new(
                tx,
                Config::default().with_poll_interval(delay),
            )?;
            watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
            Ok(Self { receiver: rx, watcher })
        }
        pub fn try_get_event(&self) -> Option<Event> {
            if let Ok(Ok(evt)) = self.receiver.try_recv() {
                return Some(evt);
            }
            None
        }
    }
}
pub use futures;
pub use instant;
pub use notify;
use std::marker::PhantomData;
pub use type_traits::prelude::*;
/// Utility function that replaces back slashes \ to forward /. Internally, it converts the input
/// path to string (lossy - see [`Path::to_string_lossy`]) and replaces the slashes in the string.
/// Finally, it converts the string to the PathBuf and returns it. This method is intended to be
/// used only for paths, that does not contain non-unicode characters.
pub fn replace_slashes<P: AsRef<Path>>(path: P) -> PathBuf {
    PathBuf::from(path.as_ref().to_string_lossy().to_string().replace('\\', "/"))
}
/// Appends specified extension to the path.
///
/// # Examples
///
/// ```rust
/// # use std::path::Path;
/// # use fyrox_core::append_extension;
/// let path = Path::new("foo.bar");
/// let new_path = append_extension(path, "baz");
/// assert_eq!(new_path, Path::new("foo.bar.baz"))
/// ```
#[must_use]
pub fn append_extension<P: AsRef<Path>, E: AsRef<str>>(
    path: P,
    additional_extension: E,
) -> PathBuf {
    let mut final_path = path.as_ref().to_path_buf();
    let new_extension = final_path
        .extension()
        .map(|e| {
            let mut ext = e.to_owned();
            ext.push(".");
            ext.push(additional_extension.as_ref());
            ext
        })
        .unwrap_or_else(|| OsString::from(additional_extension.as_ref()));
    final_path.set_extension(new_extension);
    final_path
}
pub struct BiDirHashMap<K, V> {
    forward_map: FxHashMap<K, V>,
    backward_map: FxHashMap<V, K>,
}
#[automatically_derived]
impl<K: ::core::clone::Clone, V: ::core::clone::Clone> ::core::clone::Clone
for BiDirHashMap<K, V> {
    #[inline]
    fn clone(&self) -> BiDirHashMap<K, V> {
        BiDirHashMap {
            forward_map: ::core::clone::Clone::clone(&self.forward_map),
            backward_map: ::core::clone::Clone::clone(&self.backward_map),
        }
    }
}
#[automatically_derived]
impl<K: ::core::fmt::Debug, V: ::core::fmt::Debug> ::core::fmt::Debug
for BiDirHashMap<K, V> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "BiDirHashMap",
            "forward_map",
            &self.forward_map,
            "backward_map",
            &&self.backward_map,
        )
    }
}
impl<K: Hash + Eq + Clone, V: Hash + Eq + Clone> BiDirHashMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let existing = self.forward_map.insert(key.clone(), value.clone());
        self.backward_map.insert(value, key);
        existing
    }
    pub fn remove_by_key(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.forward_map.remove(key) {
            self.backward_map.remove(&value);
            Some(value)
        } else {
            None
        }
    }
    pub fn contains_key<Q: ?Sized + Hash + Eq>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
    {
        self.forward_map.contains_key(key)
    }
    pub fn remove_by_value(&mut self, value: &V) -> Option<K> {
        if let Some(key) = self.backward_map.remove(value) {
            self.forward_map.remove(&key);
            Some(key)
        } else {
            None
        }
    }
    pub fn contains_value<Q: ?Sized + Hash + Eq>(&self, value: &Q) -> bool
    where
        V: Borrow<Q>,
    {
        self.backward_map.contains_key(value)
    }
    pub fn value_of(&self, node: &K) -> Option<&V> {
        self.forward_map.get(node)
    }
    pub fn key_of(&self, value: &V) -> Option<&K> {
        self.backward_map.get(value)
    }
    pub fn len(&self) -> usize {
        self.forward_map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.forward_map.is_empty()
    }
    pub fn clear(&mut self) {
        self.forward_map.clear();
        self.backward_map.clear();
    }
    pub fn forward_map(&self) -> &FxHashMap<K, V> {
        &self.forward_map
    }
    pub fn backward_map(&self) -> &FxHashMap<V, K> {
        &self.backward_map
    }
    pub fn into_inner(self) -> (FxHashMap<K, V>, FxHashMap<V, K>) {
        (self.forward_map, self.backward_map)
    }
}
impl<K, V> Default for BiDirHashMap<K, V> {
    fn default() -> Self {
        Self {
            forward_map: Default::default(),
            backward_map: Default::default(),
        }
    }
}
impl<K: Hash + Eq + Clone, V: Hash + Eq + Clone> From<FxHashMap<K, V>>
for BiDirHashMap<K, V> {
    fn from(forward_map: FxHashMap<K, V>) -> Self {
        let mut backward_map = FxHashMap::default();
        for (k, v) in forward_map.iter() {
            backward_map.insert(v.clone(), k.clone());
        }
        Self { forward_map, backward_map }
    }
}
impl<K, V> Visit for BiDirHashMap<K, V>
where
    K: Hash + Eq + Clone + Default + Visit,
    V: Hash + Eq + Clone + Default + Visit,
{
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        let mut region = visitor.enter_region(name)?;
        self.forward_map.visit("ForwardMap", &mut region)?;
        self.backward_map.visit("BackwardMap", &mut region)?;
        Ok(())
    }
}
impl<K, V> FromIterator<(K, V)> for BiDirHashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut hm = Self::default();
        for (k, v) in iter {
            hm.forward_map.insert(k.clone(), v.clone());
            hm.backward_map.insert(v, k);
        }
        hm
    }
}
#[inline]
pub fn hash_combine(lhs: u64, rhs: u64) -> u64 {
    lhs ^ (rhs.wrapping_add(0x9e3779b9).wrapping_add(lhs << 6).wrapping_add(lhs >> 2))
}
/// Strip working directory from file name. The function may fail for one main reason -
/// input path is not valid, does not exist, or there is some other issues with it.
pub fn make_relative_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, std::io::Error> {
    match path
        .as_ref()
        .canonicalize()?
        .strip_prefix(std::env::current_dir()?.canonicalize()?)
    {
        Ok(relative_path) => Ok(replace_slashes(relative_path)),
        Err(_) => {
            Err(
                std::io::Error::new(std::io::ErrorKind::Other, "unable to strip prefix!"),
            )
        }
    }
}
/// "Transmutes" array of any sized type to a slice of bytes.
pub fn array_as_u8_slice<T: Sized>(v: &[T]) -> &'_ [u8] {
    unsafe {
        std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v))
    }
}
/// "Transmutes" array of any sized type to a slice of some other type.
pub fn transmute_slice<T: Sized, U: Sized>(v: &[T]) -> &'_ [U] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const U,
            std::mem::size_of_val(v) / std::mem::size_of::<U>(),
        )
    }
}
/// "Transmutes" value of any sized type to a slice of bytes.
pub fn value_as_u8_slice<T: Sized>(v: &T) -> &'_ [u8] {
    unsafe {
        std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>())
    }
}
/// Takes a vector of trivially-copyable values and turns it into a vector of bytes.
pub fn transmute_vec_as_bytes<T: Copy>(vec: Vec<T>) -> Vec<u8> {
    unsafe {
        let mut vec = std::mem::ManuallyDrop::new(vec);
        Vec::from_raw_parts(
            vec.as_mut_ptr() as *mut u8,
            vec.len() * std::mem::size_of::<T>(),
            vec.capacity() * std::mem::size_of::<T>(),
        )
    }
}
/// Performs hashing of a sized value by interpreting it as raw memory.
pub fn hash_as_bytes<T: Sized, H: Hasher>(value: &T, hasher: &mut H) {
    hasher.write(value_as_u8_slice(value))
}
/// Compares two strings using case-insensitive comparison. This function does not allocate any
/// any memory and significantly faster than `a.to_lowercase() == b.to_lowercase()`.
pub fn cmp_strings_case_insensitive(a: impl AsRef<str>, b: impl AsRef<str>) -> bool {
    let a_ref = a.as_ref();
    let b_ref = b.as_ref();
    if a_ref.len() != b_ref.len() {
        return false;
    }
    a_ref
        .chars()
        .zip(b_ref.chars())
        .all(|(ca, cb)| ca.to_lowercase().eq(cb.to_lowercase()))
}
pub fn make_pretty_type_name(type_name: &str) -> &str {
    let mut colon_position = None;
    let mut byte_pos = 0;
    for c in type_name.chars() {
        byte_pos += c.len_utf8();
        if c == ':' {
            colon_position = Some(byte_pos);
        } else if c == '<' {
            break;
        }
    }
    if let Some(colon_position) = colon_position {
        type_name.split_at(colon_position).1
    } else {
        type_name
    }
}
#[repr(transparent)]
pub struct PhantomDataSendSync<T: ?Sized>(PhantomData<T>);
#[automatically_derived]
impl<T: ::core::fmt::Debug + ?Sized> ::core::fmt::Debug for PhantomDataSendSync<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(
            f,
            "PhantomDataSendSync",
            &&self.0,
        )
    }
}
unsafe impl<T: ?Sized> Send for PhantomDataSendSync<T> {}
unsafe impl<T: ?Sized> Sync for PhantomDataSendSync<T> {}
impl<T: ?Sized> Hash for PhantomDataSendSync<T> {
    #[inline]
    fn hash<H: Hasher>(&self, _: &mut H) {}
}
impl<T: ?Sized> PartialEq for PhantomDataSendSync<T> {
    fn eq(&self, _other: &PhantomDataSendSync<T>) -> bool {
        true
    }
}
impl<T: ?Sized> Eq for PhantomDataSendSync<T> {}
impl<T: ?Sized> PartialOrd for PhantomDataSendSync<T> {
    fn partial_cmp(&self, _other: &PhantomDataSendSync<T>) -> Option<cmp::Ordering> {
        Some(self.cmp(_other))
    }
}
impl<T: ?Sized> Ord for PhantomDataSendSync<T> {
    fn cmp(&self, _other: &PhantomDataSendSync<T>) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}
impl<T: ?Sized> Copy for PhantomDataSendSync<T> {}
impl<T: ?Sized> Clone for PhantomDataSendSync<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized> Default for PhantomDataSendSync<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
/// A trait for entities that have name.
pub trait NameProvider {
    /// Returns a reference to the name of the entity.
    fn name(&self) -> &str;
}
/// Tries to find an entity by its name in a series of entities produced by an iterator.
pub fn find_by_name_ref<'a, T, I, S, K>(mut iter: I, name: S) -> Option<(K, &'a T)>
where
    T: NameProvider,
    I: Iterator<Item = (K, &'a T)>,
    S: AsRef<str>,
{
    iter.find(|(_, value)| value.name() == name.as_ref())
}
/// Tries to find an entity by its name in a series of entities produced by an iterator.
pub fn find_by_name_mut<'a, T, I, S, K>(mut iter: I, name: S) -> Option<(K, &'a mut T)>
where
    T: NameProvider,
    I: Iterator<Item = (K, &'a mut T)>,
    S: AsRef<str>,
{
    iter.find(|(_, value)| value.name() == name.as_ref())
}

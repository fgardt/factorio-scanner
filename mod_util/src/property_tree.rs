/*
Parts of this were rewritten after taking some inspiration and copying some code from:
https://github.com/raiguard/fmm/blob/9744e812797f84f0728efc649e5365feb52d0c7b/src/dat.rs
For this reason, this file is licensed under the MIT license, as per the license of the above repository.

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{self, Cursor, Seek, SeekFrom},
};

use anyhow::{anyhow, Error, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PropertyTreeType {
    None = 0,
    Bool = 1,
    Number = 2,
    String = 3,
    List = 4,
    Dictionary = 5,
}

#[allow(clippy::upper_case_acronyms)]
type PTT = PropertyTreeType;

impl TryFrom<&u8> for PropertyTreeType {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Bool),
            2 => Ok(Self::Number),
            3 => Ok(Self::String),
            4 => Ok(Self::List),
            5 => Ok(Self::Dictionary),
            _ => Err(anyhow!("Invalid PropertyTreeType")),
        }
    }
}

impl TryFrom<u8> for PropertyTreeType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl From<&PropertyTree> for PropertyTreeType {
    fn from(value: &PropertyTree) -> Self {
        match value {
            PropertyTree::None => Self::None,
            PropertyTree::Bool(_) => Self::Bool,
            PropertyTree::Number(_) => Self::Number,
            PropertyTree::String(_) => Self::String,
            PropertyTree::List(_) => Self::List,
            PropertyTree::Dictionary(_) => Self::Dictionary,
        }
    }
}

impl From<PropertyTree> for PropertyTreeType {
    fn from(value: PropertyTree) -> Self {
        Self::from(&value)
    }
}

#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyTree {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<PropertyTree>),
    Dictionary(HashMap<String, PropertyTree>),
    // SignedInteger(i32),
    // UnsignedInteger(u32),
}

impl PropertyTree {
    pub fn load(reader: &mut Cursor<Vec<u8>>) -> Result<Self> {
        let pt_type: PTT = reader.read_u8()?.try_into()?;
        reader.seek(SeekFrom::Current(1))?; // skip any type flag

        let data = match pt_type {
            PTT::None => Self::None,
            PTT::Bool => Self::Bool(reader.read_bool()?),
            PTT::Number => Self::Number(reader.read_f64::<LittleEndian>()?),
            PTT::String => Self::String(reader.read_string()?),
            PTT::List => {
                let len = reader.read_u32::<LittleEndian>()?;
                let mut list = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    reader.read_string()?; // skip key
                    list.push(Self::load(reader)?);
                }

                Self::List(list)
            }
            PTT::Dictionary => {
                let len = reader.read_u32::<LittleEndian>()?;
                let mut dict = HashMap::with_capacity(len as usize);

                for _ in 0..len {
                    dict.insert(reader.read_string()?, Self::load(reader)?);
                }

                Self::Dictionary(dict)
            }
        };

        Ok(data)
    }

    pub fn write(&self, out: &mut Vec<u8>) -> Result<()> {
        let pt_type: PropertyTreeType = self.into();
        out.write_u8(pt_type as u8)?;
        out.write_u8(0)?; // any type flag, false

        match self {
            Self::None => {}
            Self::Bool(val) => out.write_bool(*val)?,
            Self::Number(val) => out.write_f64::<LittleEndian>(*val)?,
            Self::String(val) => out.write_string(val)?,
            Self::List(val) => {
                #[allow(clippy::cast_possible_truncation)]
                out.write_u32::<LittleEndian>(val.len() as u32)?;

                for val in val {
                    out.write_string("")?;
                    val.write(out)?;
                }
            }
            Self::Dictionary(val) => {
                #[allow(clippy::cast_possible_truncation)]
                out.write_u32::<LittleEndian>(val.len() as u32)?;

                for (key, value) in val {
                    out.write_string(key)?;
                    value.write(out)?;
                }
            }
        }

        Ok(())
    }

    pub const fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    pub const fn is_dict(&self) -> bool {
        matches!(self, Self::Dictionary(_))
    }

    pub const fn as_list(&self) -> Option<&Vec<Self>> {
        match self {
            Self::List(val) => Some(val),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::List(val) => Some(val),
            _ => None,
        }
    }

    pub const fn as_dict(&self) -> Option<&HashMap<String, Self>> {
        match self {
            Self::Dictionary(val) => Some(val),
            _ => None,
        }
    }

    pub fn as_dict_mut(&mut self) -> Option<&mut HashMap<String, Self>> {
        match self {
            Self::Dictionary(val) => Some(val),
            _ => None,
        }
    }

    pub fn get<K: Key>(&self, key: &K) -> Option<&Self> {
        key.index_into(self)
    }

    pub fn get_mut<K: Key>(&mut self, key: &K) -> Option<&mut Self> {
        key.index_into_mut(self)
    }
}

pub trait Key {
    fn index_into<'a>(&self, pt: &'a PropertyTree) -> Option<&'a PropertyTree>;
    fn index_into_mut<'a>(&self, pt: &'a mut PropertyTree) -> Option<&'a mut PropertyTree>;
}

impl Key for &str {
    fn index_into<'a>(&self, pt: &'a PropertyTree) -> Option<&'a PropertyTree> {
        match pt {
            PropertyTree::Dictionary(dict) => dict.get(*self),
            _ => None,
        }
    }

    fn index_into_mut<'a>(&self, pt: &'a mut PropertyTree) -> Option<&'a mut PropertyTree> {
        match pt {
            PropertyTree::Dictionary(dict) => dict.get_mut(*self),
            _ => None,
        }
    }
}

impl Key for usize {
    fn index_into<'a>(&self, pt: &'a PropertyTree) -> Option<&'a PropertyTree> {
        match pt {
            PropertyTree::List(list) => list.get(*self),
            _ => None,
        }
    }

    fn index_into_mut<'a>(&self, pt: &'a mut PropertyTree) -> Option<&'a mut PropertyTree> {
        match pt {
            PropertyTree::List(list) => list.get_mut(*self),
            _ => None,
        }
    }
}

pub trait Read: io::Read {
    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? == 1)
    }

    fn read_optimized_u32(&mut self) -> Result<u32> {
        let small = self.read_u8()?;
        if small == u8::MAX {
            Ok(self.read_u32::<LittleEndian>()?)
        } else {
            Ok(small.into())
        }
    }

    fn read_string(&mut self) -> Result<String> {
        if self.read_bool()? {
            Ok(String::new())
        } else {
            let len = self.read_optimized_u32()?;
            let mut buf = vec![0; len as usize];
            self.read_exact(&mut buf)?;

            Ok(String::from_utf8_lossy(&buf).to_string())
        }
    }
}

impl<R: io::Read + ?Sized> Read for R {}

pub trait Write: io::Write {
    fn write_bool(&mut self, val: bool) -> Result<()> {
        self.write_u8(u8::from(val))?;

        Ok(())
    }

    fn write_optimized_u32(&mut self, val: u32) -> Result<()> {
        if val < u32::from(u8::MAX) {
            #[allow(clippy::cast_possible_truncation)]
            self.write_u8(val as u8)?;
        } else {
            self.write_u8(u8::MAX)?;
            self.write_u32::<LittleEndian>(val)?;
        }

        Ok(())
    }

    fn write_string(&mut self, val: &str) -> Result<()> {
        if val.is_empty() {
            self.write_bool(true)?;
        } else {
            self.write_bool(false)?;

            #[allow(clippy::cast_possible_truncation)]
            self.write_optimized_u32(val.len() as u32)?;

            self.write_all(val.as_bytes())?;
        }

        Ok(())
    }
}

impl<W: io::Write + ?Sized> Write for W {}

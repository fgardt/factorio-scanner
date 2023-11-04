#![allow(dead_code)]

use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum PropertyTreeType {
    None = 0,
    Bool = 1,
    Number = 2,
    String = 3,
    List = 4,
    Dictionary = 5,
    // SignedInteger = 6,
    // UnsignedInteger = 7,
}

#[allow(clippy::upper_case_acronyms)]
type PTT = PropertyTreeType;

impl TryFrom<&u8> for PropertyTreeType {
    type Error = ();

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Bool),
            2 => Ok(Self::Number),
            3 => Ok(Self::String),
            4 => Ok(Self::List),
            5 => Ok(Self::Dictionary),
            // 6 => Ok(Self::SignedInteger),
            // 7 => Ok(Self::UnsignedInteger),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for PropertyTreeType {
    type Error = ();

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
            // PropertyTree::SignedInteger(_) => Self::SignedInteger,
            // PropertyTree::UnsignedInteger(_) => Self::UnsignedInteger,
        }
    }
}

impl From<PropertyTree> for PropertyTreeType {
    fn from(value: PropertyTree) -> Self {
        Self::from(&value)
    }
}

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
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_bytes_internal(bytes, &mut 0)
    }

    fn from_bytes_internal(bytes: &[u8], byte_idx: &mut usize) -> Option<Self> {
        if bytes.len() < 2 {
            return None;
        }

        let pt_type: PropertyTreeType = bytes.get(*byte_idx)?.try_into().ok()?;
        //let any_type: bool = bytes[*byte_idx + 1] == 1;

        *byte_idx += 2;

        let pt = match pt_type {
            PTT::None => Self::None,
            PTT::Bool => Self::Bool(Self::bool_from_bytes(bytes, byte_idx)?),
            PTT::Number => Self::Number(Self::number_from_bytes(bytes, byte_idx)?),
            PTT::String => Self::String(Self::string_from_bytes(bytes, byte_idx)?),
            PTT::List => Self::List(Self::list_from_bytes(bytes, byte_idx)?),
            PTT::Dictionary => Self::Dictionary(Self::dict_from_bytes(bytes, byte_idx)?),
            // PTT::SignedInteger => todo!(),
            // PTT::UnsignedInteger => Self::UnsignedInteger(Self::u32_from_bytes(bytes, byte_idx)?)
        };

        // if *byte_idx != bytes.len() {
        //     return None;
        // }

        Some(pt)
    }

    fn bool_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<bool> {
        let bool = *bytes.get(*byte_idx)? == 1;

        *byte_idx += 1;

        Some(bool)
    }

    fn number_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<f64> {
        let buff = [
            *bytes.get(*byte_idx)?,
            *bytes.get(*byte_idx + 1)?,
            *bytes.get(*byte_idx + 2)?,
            *bytes.get(*byte_idx + 3)?,
            *bytes.get(*byte_idx + 4)?,
            *bytes.get(*byte_idx + 5)?,
            *bytes.get(*byte_idx + 6)?,
            *bytes.get(*byte_idx + 7)?,
        ];
        let number = f64::from_le_bytes(buff);

        *byte_idx += 8;

        Some(number)
    }

    fn u32_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<u32> {
        let buff = [
            *bytes.get(*byte_idx)?,
            *bytes.get(*byte_idx + 1)?,
            *bytes.get(*byte_idx + 2)?,
            *bytes.get(*byte_idx + 3)?,
        ];
        let number = u32::from_le_bytes(buff);

        *byte_idx += 4;

        Some(number)
    }

    fn packed_u32_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<u32> {
        let small = *bytes.get(*byte_idx)?;
        *byte_idx += 1;

        if small == u8::MAX {
            Self::u32_from_bytes(bytes, byte_idx)
        } else {
            Some(u32::from(small))
        }
    }

    fn string_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<String> {
        let empty = *bytes.get(*byte_idx)? == 1;
        *byte_idx += 1;

        if empty {
            return Some(String::new());
        }

        let len = Self::packed_u32_from_bytes(bytes, byte_idx)? as usize;
        let mut buff = Vec::with_capacity(len);

        for _ in 0..len {
            buff.push(*bytes.get(*byte_idx)?);
            *byte_idx += 1;
        }

        String::from_utf8(buff).ok()
    }

    fn list_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<Vec<Self>> {
        let tmp_dict = Self::dict_from_bytes(bytes, byte_idx)?;
        Some(tmp_dict.values().cloned().collect())
    }

    fn dict_from_bytes(bytes: &[u8], byte_idx: &mut usize) -> Option<HashMap<String, Self>> {
        let len = Self::u32_from_bytes(bytes, byte_idx)? as usize; // packed u32 maybe?
        let mut dict = HashMap::with_capacity(len);

        for _ in 0..len {
            let key = Self::string_from_bytes(bytes, byte_idx)?;
            let value = Self::from_bytes_internal(bytes, byte_idx)?;

            dict.insert(key, value);
        }

        Some(dict)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buff: Vec<u8> = Vec::new();

        let pt_type: PropertyTreeType = self.into();
        buff.push(pt_type as u8);
        buff.push(0); // any type flag, false

        let data = match self {
            Self::None => vec![],
            Self::Bool(val) => vec![u8::from(*val)],
            Self::Number(val) => val.to_le_bytes().to_vec(),
            Self::String(val) => Self::string_to_bytes(val),
            Self::List(val) => Self::list_to_bytes(val),
            Self::Dictionary(val) => Self::dict_to_bytes(val),
        };

        buff.extend(data);

        buff
    }

    fn packed_u32_to_bytes(val: u32) -> Vec<u8> {
        if val < u32::from(u8::MAX) {
            #[allow(clippy::cast_possible_truncation)]
            let val = val as u8;
            vec![val]
        } else {
            let mut buff = Vec::with_capacity(5);
            buff.push(u8::MAX);
            buff.extend(val.to_le_bytes());
            buff
        }
    }

    fn string_to_bytes(string: &str) -> Vec<u8> {
        if string.is_empty() {
            return vec![1];
        }

        let mut buff: Vec<u8> = vec![0];

        #[allow(clippy::cast_possible_truncation)]
        let len = string.len() as u32;

        buff.extend(Self::packed_u32_to_bytes(len));
        buff.extend(string.bytes());

        buff
    }

    fn list_to_bytes(list: &[Self]) -> Vec<u8> {
        let mut buff: Vec<u8> = Vec::new();

        #[allow(clippy::cast_possible_truncation)]
        let len = list.len() as u32;

        buff.extend(len.to_le_bytes());

        for val in list {
            buff.extend(Self::string_to_bytes(""));
            buff.extend(val.to_bytes());
        }

        buff
    }

    fn dict_to_bytes(dict: &HashMap<String, Self>) -> Vec<u8> {
        let mut buff: Vec<u8> = Vec::new();

        #[allow(clippy::cast_possible_truncation)]
        let len = dict.len() as u32;

        buff.extend(len.to_le_bytes());

        for (key, value) in dict {
            buff.extend(Self::string_to_bytes(key));
            buff.extend(value.to_bytes());
        }

        buff
    }
}

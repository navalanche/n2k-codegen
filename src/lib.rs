extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate env_logger;

use serde::export::PhantomData;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct PGNDefinition {
    #[serde(rename = "PGN")]
    pub pgn: u32,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Complete")]
    pub complete: bool,
    #[serde(rename = "Length")]
    pub length: u32,
    #[serde(rename = "RepeatingFields", default)]
    pub repeating_fields: u32,
    #[serde(rename = "Fields")]
    pub fields: Vec<PGNDefinitionField>,
}


#[derive(Deserialize, Debug)]
struct PGNDefinitionField {
    #[serde(rename = "Id")]
    pub id: String,
}


#[derive(Deserialize, Debug)]
struct PGNsFile {
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "CreatorCode")]
    pub creator_code: String,
    #[serde(rename = "License")]
    pub license: String,
    #[serde(rename = "PGNs")]
    pub pgns: PGNS,
}

#[derive(Deserialize, Debug)]
pub struct PGNS {
    #[serde(rename = "PGNInfo")]
    pub pgn_infos: Vec<PGNInfo>,
}

#[derive(Deserialize, Debug)]
pub struct PGNInfo {
    #[serde(rename = "PGN")]
    pub pgn: StringVal<String>,
    #[serde(rename = "Id")]
    pub id: StringVal<String>,
    #[serde(rename = "Length")]
    pub length: StringVal<u32>,
    #[serde(rename = "Fields", default)]
    pub fields: Fields,
}


#[derive(Deserialize)]
pub struct StringVal<T> where T: std::fmt::Debug {
    #[serde(rename = "$value", default)]
    pub val: T,
    #[serde(skip)]
    phantom: PhantomData<T>,
}

impl<T> std::fmt::Debug for StringVal<T> where T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{:?}\"", self.val)
    }
}

impl<T> std::default::Default for StringVal<T> where T: std::fmt::Debug + std::default::Default {
    fn default() -> StringVal<T> {
        StringVal { val: T::default(), phantom: PhantomData }
    }
}

impl<T> std::ops::Deref for StringVal<T> where T: FromStr + std::fmt::Debug {
    type Target = T;
    fn deref(&self) -> &T {
        &self.val
    }
}

#[derive(Deserialize, Debug)]
pub struct Fields {
    #[serde(rename = "Field", default)]
    pub fields: Vec<Field>
}

impl std::default::Default for Fields {
    fn default() -> Fields {
        Fields { fields: vec!() }
    }
}


#[derive(Deserialize, Debug)]
pub struct Field {
    #[serde(rename = "Order")]
    pub order: StringVal<u32>,
    #[serde(rename = "Id")]
    pub id: StringVal<String>,
    #[serde(rename = "Name")]
    pub name: StringVal<String>,
    #[serde(rename = "BitLength")]
    pub bit_length: StringVal<u32>,
    #[serde(rename = "BitOffset", default)]
    pub bit_offset: StringVal<u32>,

}


pub fn n2k_codegen() {
    let my_str = include_str!("pgns.xml");
    let content: PGNsFile = serde_xml_rs::deserialize(my_str.as_bytes()).unwrap();
    eprintln!("content.Comment = {:#?}", content.comment);
    eprintln!("content.creator_code = {:#?}", content.creator_code);
    eprintln!("content.license = {:#?}", content.license);


    content.pgns.pgn_infos.iter()
        .filter(|info| *info.pgn == "59392")
        .for_each(|info| eprintln!("info = {:?}", info));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        env_logger::init().unwrap();
        n2k_codegen();
    }
}

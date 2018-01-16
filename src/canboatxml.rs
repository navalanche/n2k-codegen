use std;

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
pub struct PGNsFile {
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
    pub pgn: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Length")]
    pub length: String,
    #[serde(rename = "Fields", default)]
    pub fields: Fields,
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
    pub order: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "BitLength")]
    pub bit_length: String,
    #[serde(rename = "BitOffset", default)]
    pub bit_offset: String,
    #[serde(rename = "Type", default)]
    pub n2k_type: String,
    #[serde(rename = "Resolution", default)]
    pub resolution: String,
}

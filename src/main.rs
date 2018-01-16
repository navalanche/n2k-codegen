extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate env_logger;

use std::ops::Add;
use std::path::Path;
use std::fs::File;
use std::io::Write;

mod canboatxml;

use canboatxml::*;

pub fn n2k_codegen() {
    let dest_path = Path::new("../n2k-messages/src");

    let my_str = include_str!("pgns.xml");
    let content: PGNsFile = serde_xml_rs::deserialize(my_str.as_bytes()).unwrap();


    let lib_path = dest_path.join("lib.rs");
    let mut lib_file = File::create(&lib_path).unwrap();
    writeln!(lib_file,"mod generated;").unwrap();
    writeln!(lib_file,"mod types;").unwrap();

    let gen_lib_path = dest_path.join("generated/structs/mod.rs");
    let mut gen_lib_file = File::create(&gen_lib_path).unwrap();

    content.pgns.pgn_infos.iter()
        .filter(|info| info.pgn == "60928" || info.pgn == "59904")
        .for_each(|info| codegen(&mut lib_file, &mut gen_lib_file, &dest_path, info));
}

fn codegen(lib_file: &mut File, gen_lib_file: &mut File, path: &Path, pgninfo: &PGNInfo) {
    let struct_name = first_char_to_upper(&pgninfo.id);
    let module_name = snake_name(&pgninfo.id);
    writeln!(gen_lib_file, "pub mod {};", module_name).unwrap();
    writeln!(lib_file, "pub use generated::structs::{}::{};", module_name, struct_name).unwrap();

    let name = format!("generated/structs/{}.rs", &module_name);
    let current_file_path = path.join(&name);
    let mut message_file = File::create(&current_file_path).unwrap();


    writeln!(message_file, "use ::generated::enums::*;").unwrap();
    writeln!(message_file, "use ::types::*;").unwrap();
    writeln!(message_file, "pub struct {} {{", struct_name).unwrap();

    pgninfo.fields.fields.iter()
        .for_each(|field| codegen_field(&mut message_file, field));

    writeln!(message_file, "}}").unwrap();

    codegen_impl(&mut message_file, pgninfo);
}

fn codegen_field(message_file: &mut File, field: &Field) {
    let field_name = snake_name(&field.id);
    let length = field.bit_length.parse::<usize>().unwrap();
    let resolution = field.resolution.parse::<f32>().unwrap_or(0.0);
    let rust_type = decode_type(&field.id, &field.n2k_type, length, resolution);
    writeln!(message_file, "\t// {:?}", field).unwrap();
    if field_name != "reserved" {
        writeln!(message_file, "\tpub {}: {},", field_name, rust_type).unwrap();
    } else {
        writeln!(message_file, "\t// reserved field, length = {} bits", length).unwrap();
    }
}

fn codegen_impl(mut message_file: &mut File, pgninfo: &PGNInfo) {
    let struct_name = first_char_to_upper(&pgninfo.id);
    writeln!(message_file, "impl {} {{", struct_name).unwrap();
    codegen_decode(&mut message_file, pgninfo);
    codegen_encode(&mut message_file, pgninfo);
    writeln!(message_file, "}}").unwrap();
}

fn codegen_decode(mut message_file: &mut File, pgninfo: &PGNInfo) {
    let struct_name = first_char_to_upper(&pgninfo.id);
    writeln!(message_file, "\tpub fn decode(/* TODO */) -> {} {{", struct_name).unwrap();

    pgninfo.fields.fields.iter()
        .for_each(|field| codegen_field_decode(&mut message_file, field));

    writeln!(message_file, "\t\t{} {{", struct_name).unwrap();
    pgninfo.fields.fields.iter()
        .for_each(|field| {
            let field_name = snake_name(&field.id);
            if field_name != "reserved" {
                writeln!(message_file, "\t\t\t{},", field_name).unwrap();
            }
        });

    writeln!(message_file, "\t\t}}").unwrap();

    writeln!(message_file, "\t}}").unwrap();
}


fn codegen_field_decode(message_file: &mut File, field: &Field) {
    let field_name = snake_name(&field.id);
    let length = field.bit_length.parse::<usize>().unwrap();
    let resolution = field.resolution.parse::<f32>().unwrap_or(0.0);
    let rust_type = decode_type(&field.id, &field.n2k_type, length, resolution);

    if field_name != "reserved" {
        writeln!(message_file, "\t\tlet {} = {}::default(); // TODO", field_name, rust_type).unwrap();
    } else {
        writeln!(message_file, "\t\t// not decoding reserved field with {} bits", length).unwrap();
    }
}

fn codegen_encode(message_file: &mut File, _pgninfo: &PGNInfo){
    writeln!(message_file, "\tpub fn encode(&self) -> Vec<u8> {{").unwrap();
    writeln!(message_file, "\t\tunimplemented!();").unwrap();
    writeln!(message_file, "\t}}").unwrap();
}


fn decode_type(field_name: &str, n2k_type: &str, bit_length: usize, resolution: f32) -> String {
    match n2k_type {
        "Binary data" => {
            decode_int_type_for_bit_length(bit_length)
        }
        "Lookup table" => lookup_table_type(field_name),
        "Manufacturer code" => "u16".to_owned(),
        "Temperature" => "N2kTemperature".to_owned(),
        "ASCII text" => "String".to_owned(),
        "Date" => "N2kData".to_owned(),
        "Time" => "N2kTime".to_owned(),
        "Latitude" => "N2kLatitude".to_owned(),
        "Longitude" => "N2kLongitude".to_owned(),
        "Pressure" => "N2kPressure".to_owned(),
        "Pressure (hires)" => "N2kPressureHighRes".to_owned(),
        "Temperature (hires)" => "N2kTemperatureHighRes".to_owned(),
        "ASCII or UNICODE string starting with length and control byte" => "String".to_owned(),
        "ASCII string starting with length byte" => "String".to_owned(),
        "String with start/stop byte" => "String".to_owned(),
        "Bitfield" => "Bitfield".to_owned(),
        "IEEE Float" => decode_float_type_for_bit_length(bit_length),
        "Decimal encoded number" => decode_int_type_for_bit_length(bit_length),

        "" => {
            match bit_length {
                _a if _a > 16 => "u32".to_owned(),
                _a if _a >= 8 && _a < 17 => "u16".to_owned(),
                _a if _a < 8 => "u8".to_owned(),
                _ => "UNKNOWN".to_owned()
            }
        }
        "Integer" => {
            if resolution == 1.0 {
                decode_int_type_for_bit_length(bit_length)
            } else {
                eprintln!("resolution = {:#?}", resolution);
                eprintln!("bit_length = {:#?}", bit_length);
                unimplemented!()
            }
        }
        x @ _ => {
            eprintln!("x = {:#?}", x);

            "UNKOWN".to_owned()
        }
    }
}

fn lookup_table_type(_field_name: &str) -> String {
    "Dummy".to_owned()
}

fn decode_int_type_for_bit_length(bit_length: usize) -> String {
    match bit_length {
        _a if _a > 64 => "Vec<u8>",
        _a if _a > 32 && _a <= 64 => "u64",
        _a if _a > 16 && _a <= 32 => "u32",
        _a if _a > 8 && _a <= 16 => "u16",
        _a if _a > 1 && _a <= 8 => "u8",
        _a if _a == 1 => "bool",
        _ => "Vec<u8>"
    }.to_owned()
}

fn decode_float_type_for_bit_length(bit_length: usize) -> String {
    match bit_length {
        _a if _a > 16 && _a < 33 => "f32",
        _a if _a >= 8 && _a < 17 => "f16",
        _a if _a < 8 => "f8",
        _ => "Vec<u8>"
    }.to_owned()
}


fn first_char_to_upper(field_name: &str) -> String {
    let start = field_name.get(0..1).unwrap().to_ascii_uppercase();
    let end = field_name.get(1..).unwrap();

    String::from(start).add(end)
}

fn snake_name(field_name: &str) -> String {
    //everytime we encounter a capital letter, insert an _ and to_lower that letter
    let mut result: String = String::new();
    for (index, chr) in field_name.chars().enumerate() {
        if chr.is_uppercase() {
            let lower = chr.to_lowercase().to_string();
            if index != 0 {
                result.push('_');
            }
            result.push_str(&lower);
        } else {
            result.push(chr);
        }
    }
    result
}

pub fn main() {
    env_logger::init().unwrap();
    n2k_codegen();
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

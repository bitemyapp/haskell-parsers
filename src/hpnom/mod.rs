use nom::{alphanumeric, multispace, space};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::str;

pub struct Module {
    head: Option<ModuleHead>,
    pragmas: Vec<Pragma>,
    import_declarations: Vec<ImportDecl>,
    declarations: Vec<Decl>,
}

// (ModuleName l) (Maybe (WarningText l)) (Maybe (ExportSpecList l))

pub struct ModuleHead {
    name: ModuleName,
    warning_text: Option<WarningText>,
    export_spec_list: Option<ExportSpecList>,
}

pub struct Pragma(());
pub struct ImportDecl(());
pub struct Decl(());
pub struct WarningText(());
pub struct ExportSpecList(());

#[derive(Debug)]
pub struct ModuleName(Vec<String>);

named!(
  pub module<&str>,
  map_res!(
    delimited!(
      tag!("module "),
      take_while!(call!(|c| c != ' ' as u8)),
      tag!(" where")
    ),
    str::from_utf8
  )
);

named!(
    pub module_name<ModuleName>,
    map!(separated_list_complete!(char!('.'), alphanumeric),
         |module_vec|
         ModuleName(module_vec
                    .into_iter()
                    .map(|v| str::from_utf8(v).unwrap().to_string())
                    .collect())
    )
);

pub fn parse_file(filename: &str) -> Result<(), io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // println!("{}", contents);
    Ok (())
}

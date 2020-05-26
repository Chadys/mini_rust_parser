#![deny(missing_docs)]
//! This module define all struct needed to contains analysis results of file processing

use std::collections::HashMap;
use std::fmt;

/// Contains aggregated infos for a single type of json object
#[derive(Debug)]
pub struct TypeInfo {
    /// Number of objects of this type
    nb_object: u32,
    /// Total byte size of all the messages for this type
    byte_size: usize,
}

impl TypeInfo {
    /// Create a new empty TypeInfo
    pub fn new() -> TypeInfo {
        TypeInfo { nb_object: 0, byte_size: 0 }
    }

    /// Add info for a single object to that type
    pub fn add_object(&mut self, byte_size: usize) -> () {
        self.nb_object += 1;
        self.byte_size += byte_size;
    }
}

/// Wrapper around an [HashMap](std::collections::HashMap)
/// containing [TypeInfo]
/// for every type present in a single file.
/// Add report display capability
pub struct Analysis {
    data: HashMap<String, TypeInfo>,
}

impl Analysis {
    /// Simple wrapper of [HashMap::new](std::collections::HashMap#method.new)
    pub fn new() -> Analysis {
        Analysis { data: HashMap::new() }
    }

    /// data attribute getter
    pub fn get_data(&mut self) -> &mut HashMap<String, TypeInfo> {
        &mut self.data
    }
}

/// Manually implement Debug trait to add a pretty print functionality
/// to display the analysis report as a table
impl fmt::Debug for Analysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // without pretty print
        if !f.alternate() {
            return write!(f, "{:?}", self.data);
        }

        // if pretty print, display table
        let mut table = term_table::Table::new();
        table.max_column_width = 40;

        table.style = term_table::TableStyle::extended();
        // headers
        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new_with_alignment("Type", 1, term_table::table_cell::Alignment::Center),
            term_table::table_cell::TableCell::new_with_alignment("Number of objects", 1, term_table::table_cell::Alignment::Center),
            term_table::table_cell::TableCell::new_with_alignment("Total byte size", 1, term_table::table_cell::Alignment::Center)
        ]));

        for (json_type, type_info) in &self.data {
            table.add_row(term_table::row::Row::new(vec![
                term_table::table_cell::TableCell::new(format!("{}", json_type)),
                term_table::table_cell::TableCell::new_with_alignment(format!("{}", type_info.nb_object), 1, term_table::table_cell::Alignment::Right),
                term_table::table_cell::TableCell::new_with_alignment(format!("{}", type_info.byte_size), 1, term_table::table_cell::Alignment::Right)
            ]));
        }
        write!(f, "{}", table.render())
    }
}
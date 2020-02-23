use std::collections::HashMap;

// A Map relates "<SET> <NAME>" identifiers (like the ones we get from CDIF
// parsing as data::cdif::Line objects) to unique PrintCard IDs (from a
// data::Pool, for example).
//
pub type Map = HashMap<String, String>;

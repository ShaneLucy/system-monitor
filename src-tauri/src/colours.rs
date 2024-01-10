use lazy_static::lazy_static;

// Define a static array of Strings
lazy_static! {
    pub static ref COLOUR_HEX_CODES: [String; 11] = [
        String::from("rgb(150,206,180)"),
        String::from("rgb(14,154,167)"),
        String::from("rgb(61,194,171)"),
        String::from("rgb(246,205,97)"),
        String::from("rgb(254,138,113)"),
        String::from("rgb(254,200,193)"),
        String::from("rgb(42,183,202)"),
        String::from("rgb(255,111,105)"),
        String::from("rgb(230,230,234)"),
        String::from("rgb(136,216,176)"),
        String::from("rgb(74,78,77)"),
    ];
}

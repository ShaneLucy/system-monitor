use lazy_static::lazy_static;

// Define a static array of Strings
lazy_static! {
    pub static ref COLOUR_HEX_CODES: [String; 25] = [
        String::from("#FFD1DC"),
        String::from("#B0E0E6"),
        String::from("#E6E6FA"),
        String::from("#98FB98"),
        String::from("#FFDAB9"),
        String::from("#87CEEB"),
        String::from("#C5E2FF"),
        String::from("#C8A2C8"),
        String::from("#FFFFE0"),
        String::from("#89CFF0"),
        String::from("#F08080"),
        String::from("#00FFFF"),
        String::from("#FFFACD"),
        String::from("#D8BFD8"),
        String::from("#8FBC8F"),
        String::from("#FF6F61"),
        String::from("#40E0D0"),
        String::from("#FFC0CB"),
        String::from("#ACE1AF"),
        String::from("#FDBCB4"),
        String::from("#FFF0F5"),
        String::from("#FFBCD9"),
        String::from("#AFEEEE"),
        String::from("#F7E7CE"),
        String::from("#E5E4E2"),
    ];
}

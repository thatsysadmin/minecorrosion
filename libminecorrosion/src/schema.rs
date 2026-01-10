use diesel::table;

table! {
    minecorrosion_settings (key) {
        // id -> Integer,
        key -> Text,
        value -> Text,
    }
}

table! {
    minecorrosion_instances (id) {
        id -> Integer,
        demo_mode -> Bool,

    }
}
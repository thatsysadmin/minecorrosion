use diesel::table;

table! {
    minecorrosion_settings (id) {
        id -> Integer,
        database_version -> Integer,
    }
}

table! {
    minecorrosion_instances (id) {
        id -> Integer,
        demo_mode -> Bool,
        
    }
}
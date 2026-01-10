// @generated automatically by Diesel CLI.

diesel::table! {
    minecorrosion_settings (rowid) {
        rowid -> Integer,
        key -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    child_profiles (id) {
        id -> Integer,
        name -> Text,
        birthday -> Date,
        gender -> Bool,
        weight -> Nullable<Float>,
        height -> Nullable<Float>,
    }
}

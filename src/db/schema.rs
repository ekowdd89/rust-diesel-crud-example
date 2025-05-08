diesel::table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        content -> Text,
        published -> Bool,
    }
}
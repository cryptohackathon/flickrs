table! {
    attributes (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    images (id) {
        id -> Integer,
        path -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(attributes, images,);

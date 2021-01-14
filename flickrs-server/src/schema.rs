table! {
    attribute_groups (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    attributes (id) {
        id -> Integer,
        name -> Text,
        group_id -> Nullable<Integer>,
    }
}

table! {
    images (id) {
        id -> Integer,
        path -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    attribute_groups,
    attributes,
    images,
);

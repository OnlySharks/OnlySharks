table! {
    posts (id) {
        id -> Text,
        creatorid -> Text,
        date -> Timestamp,
        content -> Text,
        images -> Nullable<Array<Text>>,
        likes -> Int4,
    }
}

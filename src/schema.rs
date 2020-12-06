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

table! {
    users (id) {
        id -> Text,
        username -> Text,
        email -> Text,
        password -> Text,
        creationdate -> Timestamp,
        displayname -> Text,
        pronouns -> Text,
        description -> Text,
        birthday -> Date,
        followers -> Int4,
        posts -> Array<Text>,
        likedposts -> Array<Text>,
        following -> Array<Text>,
        authkey -> Text,
        pfp -> Text,
        banner -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);

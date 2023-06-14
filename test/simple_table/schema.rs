diesel::table! {
    todos (id) {
        id -> Int4,
        unsigned -> Unsigned<Integer>,
        optional_unsigned -> Nullable<Unsigned<Integer>>,
        text -> Text,
        completed -> Bool,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

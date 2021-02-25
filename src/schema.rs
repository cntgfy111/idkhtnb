table! {
    tasks (id) {
        id -> Int4,
        theme -> Int4,
        task_text -> Text,
        input -> Text,
        output -> Text,
    }
}

table! {
    themes (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(tasks -> themes (theme));

allow_tables_to_appear_in_same_query!(tasks, themes,);

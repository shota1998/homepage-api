table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    tmp_articles (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    tmp_articles,
);

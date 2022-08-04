table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    editing_articles (id) {
        id -> Int4,
        article_id -> Int4,
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

joinable!(editing_articles -> articles (article_id));

allow_tables_to_appear_in_same_query!(
    articles,
    editing_articles,
    tmp_articles,
);

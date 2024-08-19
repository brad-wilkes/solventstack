use diesel::table;

table! {
    users (id) {
        id -> Int4,
        author -> Varchar,
        genre -> Varchar,
        itemone -> Varchar,
        itemtwo -> Varchar,
        itemthree -> Varchar,
        itemfour -> Varchar,
        itemfive -> Varchar,
    }
}
table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    collectible_zone_membership (collectible_id, zone_id) {
        collectible_id -> Integer,
        zone_id -> Integer,
    }
}

table! {
    collectibles (id) {
        id -> Integer,
        name -> Text,
        short_name -> Nullable<Text>,
        description -> Nullable<Text>,
        category -> Integer,
        collected -> Bool,
    }
}

table! {
    zones (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

joinable!(collectible_zone_membership -> collectibles (collectible_id));
joinable!(collectible_zone_membership -> zones (zone_id));
joinable!(collectibles -> categories (category));

allow_tables_to_appear_in_same_query!(
    categories,
    collectible_zone_membership,
    collectibles,
    zones,
);

#[derive(Debug, Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    id: i32,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Collectible {
    id: i32,
    name: String,
    description: Option<String>,
    category: i32,
    collected: bool,
}

#[derive(Debug, Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct CollectibleZoneMembership {
    collectible_id: i32,
    zone_id: i32,
}

#[derive(Debug)]
pub struct Zone {
    id: u32,
    name: String
}

#[derive(Debug)]
pub struct Category {
    id: u32,
    name: String
}

#[derive(Debug)]
pub struct Collectible {
    id: u32,
    name: String,
    description: Option<String>,
    category: u32,
    zones: Vec<u32>
}

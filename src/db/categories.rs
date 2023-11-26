pub struct Category {
    pub id: usize,
    pub name: &'static str,
}

pub const DATA: [Category; 8] = [
    Category {
        id: 1,
        name: "Snakes",
    },
    Category {
        id: 2,
        name: "Mushrooms",
    },
    Category {
        id: 3,
        name: "Frogs",
    },
    Category {
        id: 4,
        name: "Birds",
    },
    Category {
        id: 5,
        name: "Fishes",
    },
    Category {
        id: 6,
        name: "Fruits",
    },
    Category {
        id: 7,
        name: "Miscellaneous",
    },
    Category {
        id: 8,
        name: "Medi Plants",
    },
];

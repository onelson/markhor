#[derive(Copy, Clone, PartialEq)]
pub struct Zone {
    pub id: usize,
    pub name: &'static str,
    pub description: &'static str,
}

pub const DATA: [Zone; 40] = [
    Zone {
        id: 1,
        name: "Dremuchij East",
        description: "This is the first area of Snake Eater"
    },
    Zone {
        id: 2,
        name: "Dremuchij North",
        description: "The second area of Snake Eater. This area has the very large drop at the beginning, where you cannot return from."
    },
    Zone {
        id: 3,
        name: "Dremuchij Swampland",
        description: "This is the area from the Viruous Mission that contains the death trap mud and crocs. You must back track to reach it in the Snake Eater Mission."
    },
    Zone {
        id: 4,
        name: "Dremuchij South",
        description: "Just like the swamp, you must back track. This is the area you start in the Viruous mission."
    },
    Zone {
        id: 5,
        name: "Dolinovodno",
        description: "The area with the long wooden bridge."
    },
    Zone {
        id: 6,
        name: "Rassvet",
        description: "The area where you first fight the GRU, where you meet Ocelot twice, and first recuse Sokolov. Get the Thermals here! This area changes... I couldn't seem to find any birds here during the Snake Eater Mission."
    },
    Zone {
        id: 7,
        name: "Chyornyj Prud",
        description: "The larger swamp infrested with fish and crocs."
    },
    Zone {
        id: 8,
        name: "Bolshaya Past South",
        description: "The first area with electric fences."
    },
    Zone {
        id: 9,
        name: "Bolshaya Past Base",
        description: "The area after the sotuh part with the building and helicopter."
    },
    Zone {
        id: 10,
        name: "Bolshaya Past Crevice",
        description: "The fight area against Ocelot."
    },
    Zone {
        id: 11,
        name: "Chyomaya Peschera Cave Branch",
        description: "The cave right after the Ocelot fight."
    },
    Zone {
        id: 12,
        name: "Chyomaya Peschera Cave",
        description: "The second cave and the area where you fight The Pain."
    },
    Zone {
        id: 13,
        name: "Chyomaya Peschera Cave Entrance",
        description: "The area after The Pain fight."
    },
    Zone {
        id: 14,
        name: "Ponizovje South",
        description: "This is the long river after the caves."
    },
    Zone {
        id: 15,
        name: "Ponizovje West",
        description: "The area west of the long river... this is the first spot you can get the SVD."
    },
    Zone {
        id: 16,
        name: "Ponizovje Warehouse: Exterior",
        description: "This is the area before the warehouse, the docks. It is the area where you can kill The End before he gets wheeled in the doors."
    },
    Zone {
        id: 17,
        name: "Ponizovje Warehouse",
        description: "Kind of hard to not remember... the Warehouse after the docks."
    },
    Zone {
        id: 18,
        name: "Graniny Gorki South",
        description: "The area where you fight The Fear."
    },
    Zone {
        id: 19,
        name: "Graniny Gorki Lab Exterior",
        description: "The area outside of the lab, it has an electric fence guarding it."
    },
    Zone {
        id: 20,
        name: "Graniny Gorky Lab 1F",
        description: "This is the top floor of the lab."
    },
    Zone {
        id: 21,
        name: "Graniny Gorky Lab B1",
        description: "The lower floor of the lab."
    },
    Zone {
        id: 22,
        name: "Svyatogornyj South",
        description: "This is the area right after leaving the warehouse though the newly unlocked door."
    },
    Zone {
        id: 23,
        name: "Svyatogornyj West",
        description: "This part is right after the South area... not really much else you can say."
    },
    Zone {
        id: 24,
        name: "Svyatogornyj East",
        description: "This area is mostly unheard of, as you must keep going right to reach it while in the West."
    },
    Zone {
        id: 25,
        name: "Sokrovenno South",
        description: "The first screen during The End fight."
    },
    Zone {
        id: 26,
        name: "Sokrovenno West",
        description: "Part of the area you fight The End."
    },
    Zone {
        id: 27,
        name: "Sokrovenno North",
        description: "Part of the area you fight The End."
    },
    Zone {
        id: 28,
        name: "Krasnogorje Tunnel",
        description: "The area where you fight The Ladder."
    },
    Zone {
        id: 29,
        name: "Krasnogorje Mountain Base",
        description: "The first part of the desert mountain. This part has no steep hills yet."
    },
    Zone {
        id: 30,
        name: "Krasnogorje Mountainside",
        description: "As the name suggests, this is the area where you will be moving up the mountain."
    },
    Zone {
        id: 31,
        name: "Krasnogorje Mountaintop",
        description: "The top of the moutain. This area doesn't have as much climbing, but has trenches."
    },
    Zone {
        id: 32,
        name: "Krasnogorje Mountaintop Ruins",
        description: "This is the little shack you meet up with Eva at."
    },
    Zone {
        id: 33,
        name: "Groznyj Grad Underground Tunnel",
        description: "This is the whole section where you fight The Fury."
    },
    Zone {
        id: 34,
        name: "Groznyj Grad 1F",
        description: "This is the holding cell. All the food from here is given to you if you wait."
    },
    Zone {
        id: 35,
        name: "Groznyj Grad Sewers",
        description: "If you didn't know where this was, it is before The Sorrow fight."
    },
    Zone {
        id: 36,
        name: "Tikhogomyj",
        description: "The forest area after The Sorrow."
    },
    Zone {
        id: 37,
        name: "Tikhogomyj: Behind Waterfall",
        description: "This is the area where you meet up with Eva and get your stuff back."
    },
    Zone {
        id: 38,
        name: "Zaozyorje South",
        description: "This is the first part while walking with Eva."
    },
    Zone {
        id: 39,
        name: "Zaozyorje North",
        description: "The second part with Eva and the last part of the game without a boss."
    },
    Zone {
        id: 40,
        name: "Rokovoj Bereg",
        description: "The Boss fight."
    }
];

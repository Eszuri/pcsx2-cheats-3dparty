pub struct Game {
    pub title: &'static str,
    pub serial: &'static str,
    pub region: &'static str,
    pub crc: &'static str,
}

pub const DATA_GAMES: &[Game] = &[
    Game {
        title: "Black",
        serial: "SLUS-21376",
        region: "NTSC-U",
        crc: "5C891FF1",
    },
    Game {
        title: "Def Jam - Fight for NY",
        serial: "SLUS-21004",
        region: "NTSC-U",
        crc: "4538436F",
    },
    Game {
        title: "Downhill Domination",
        serial: "SCUS-97177",
        region: "NTSC-U",
        crc: "5AE01D98",
    },
    Game {
        title: "Mortal Kombat - Shaolin Monks",
        serial: "SLUS-21087",
        region: "NTSC-U",
        crc: "455DD546",
    },
];

pub struct EE {
    pub title: &'static str,
    pub address: u32,
    pub value: u32,
}

pub struct Game {
    pub title: &'static str,
    pub serial: &'static str,
    pub region: &'static str,
    pub crc: &'static str,
    pub ee_ram: &'static [EE],
}

pub const DATA_GAMES: &[Game] = &[
    Game {
        title: "Black",
        serial: "SLUS-21376",
        region: "NTSC-U",
        crc: "5C891FF1",
        ee_ram: &[
            EE {
                title: "Health Player",
                address: 0x005A8DA8,
                value: 1144750080,
            },
            EE {
                title: "infinite Grenade",
                address: 0x001551B0,
                value: 00000001,
            },
            EE {
                title: "Increase Ammo",
                address: 0x0015515C,
                value: 6690849,
            },
            EE {
                title: "Complete All Level",
                address: 0x0048EFAC,
                value: 2097160,
            },
            EE {
                title: "Unlock All Level",
                address: 0x048EFC4,
                value: 2097160,
            },
        ],
    },
    Game {
        title: "Def Jam - Fight for NY",
        serial: "SLUS-21004",
        region: "NTSC-U",
        crc: "4538436F",
        ee_ram: &[EE {
            title: "Health Player",
            address: 0x005A8DA8,
            value: 0x4E6E6B28,
        }],
    },
    Game {
        title: "Downhill Domination",
        serial: "SCUS-97177",
        region: "NTSC-U",
        crc: "5AE01D98",
        ee_ram: &[EE {
            title: "Health Player",
            address: 0x005A8DA8,
            value: 0x4E6E6B28,
        }],
    },
    Game {
        title: "Mortal Kombat - Shaolin Monks",
        serial: "SLUS-21087",
        region: "NTSC-U",
        crc: "455DD546",
        ee_ram: &[EE {
            title: "Health Player",
            address: 0x005A8DA8,
            value: 0x4E6E6B28,
        }],
    },
];

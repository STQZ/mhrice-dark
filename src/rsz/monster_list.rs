use super::*;
use crate::rsz_struct;
use nalgebra_glm::*;
use serde::*;

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData.PartData",
        0x8999739b = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct PartData {
        pub part: i32, // snow.data.monsterList.PartType
        pub circle_size: i32,
        pub circle_pos: Vec2,
        pub em_meat: i32, // snow.enemy.EnemyDef.Meat
        pub em_meat_group_index: u32,
    }
}

rsz_struct! {
    #[rsz("snow.BitSetFlag`1<snow.data.monsterList.HabitatType>",
        0xd8e3d0dc = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct BitSetFlagHabitatType {
        flag: Vec<u32>
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.BossMonsterData",
        0xe2ca959a = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct BossMonsterData {
        pub em_type: EmTypes,
        pub family_type: i32, // snow.data.monsterList.FamilyType
        pub habitat_area: BitSetFlagHabitatType,
        pub is_limit_open_lv: bool,
        pub part_table_data: Vec<PartData>,
    }
}

rsz_struct! {
    #[rsz("snow.data.monsterList.MonsterListBossData",
        path = "data/Define/Common/HunterNote/MonsterListBossData.user",
        0x4a9edb4f = 0
    )]
    #[derive(Debug, Serialize)]
    pub struct MonsterListBossData {
        pub data_list: Vec<BossMonsterData>
    }
}

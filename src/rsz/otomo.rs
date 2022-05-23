use super::item::*;
use super::*;
use crate::rsz_enum;
use crate::rsz_struct;
use serde::*;

// snow.data.DataDef.OtArmorId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum OtArmorId {
        None = 0x20000000,
        AirouHead(u32) = 0x20100000..=0x2010FFFF,
        AirouChest(u32) = 0x20200000..=0x2020FFFF,
        DogHead(u32) = 0x20300000..=0x2030FFFF,
        DogChest(u32) = 0x20400000..=0x2040FFFF,
    }
}

impl OtArmorId {
    pub fn icon_index(self) -> u32 {
        match self {
            OtArmorId::None => 0,
            OtArmorId::AirouHead(_) => 13,
            OtArmorId::AirouChest(_) => 14,
            OtArmorId::DogHead(_) => 34,
            OtArmorId::DogChest(_) => 35,
        }
    }
}

// snow.data.DataDef.OtEquipSeriesId
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum OtEquipSeriesId {
        Airou(i32) = 0x00000000..=0x0000FFFF,
        Dog(i32) = 0x00010000..=0x0001FFFF,
    }
}

impl OtEquipSeriesId {
    pub fn to_tag(self) -> String {
        match self {
            OtEquipSeriesId::Airou(i) => format!("Airou_{i:03}"),
            OtEquipSeriesId::Dog(i) => format!("Dog_{i:03}"),
        }
    }
}

// not a rsz type
rsz_struct! {
    #[rsz()]
    #[derive(Debug, Serialize)]
    pub struct OtArmorBase {
        pub id: OtArmorId,
        pub sort_id: u32,
        pub series_id: OtEquipSeriesId,
        pub rare_type: RareTypes,
        pub model_id: u32, // snow.data.DataDef.OtEquipModelId
        pub def: i32,
        pub element_regist_list: [i32; 5],
        pub base_color_index: u32,
        pub sell_value: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtAirouArmorBaseUserData.Param",
        0x101e83d6 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtAirouArmorBaseUserDataParam {
        #[serde(flatten)]
        pub base: OtArmorBase,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtAirouArmorBaseUserData",
        0x63942732 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtAirouArmorBaseUserData {
        pub param: Vec<OtAirouArmorBaseUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.OtDogArmorBaseUserData.Param",
        0xa137ef7c = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtDogArmorBaseUserDataParam {
        #[serde(flatten)]
        pub base: OtArmorBase,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtDogArmorBaseUserData",
        0xc1c7f588 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtDogArmorBaseUserData {
        pub param: Vec<OtDogArmorBaseUserDataParam>
    }
}

rsz_struct! {
    #[rsz("snow.data.OtArmorProductUserData.Param",
        0x86abfedf = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtArmorProductUserDataParam {
        pub id: OtArmorId,
        pub item_list: Vec<ItemId>,
        pub item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtArmorProductUserData",
        0xa2529f96 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtArmorProductUserData {
        pub param: Vec<OtArmorProductUserDataParam>
    }
}

// snow.data.DataDef.OtWeaponId
rsz_enum! {
    #[rsz(u32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum OtWeaponId {
        None = 0x1C000000,
        Airou(u32) = 0x1C100000..=0x1C10FFFF,
        Dog(u32) = 0x1C200000..=0x1C20FFFF,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtWeaponProductUserData.Param",
        0x4410d939 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtWeaponProductUserDataParam {
        pub id: OtWeaponId,
        pub item_list: Vec<ItemId>,
        pub item_num: Vec<u32>,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtWeaponProductUserData",
        0x55f9b42b = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtWeaponProductUserData {
        pub param: Vec<OtWeaponProductUserDataParam>
    }
}

// snow.data.OtWeaponData.AtkTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum OtAtkTypes {
        Smash = 0,
        Blow = 1,
    }
}

// snow.data.DataDef.OtSpecializeTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum OtSpecializeTypes {
        Short = 0,
        Balance = 1,
        Long = 2,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtWeaponBaseUserData.Param",
        0x972af528 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtWeaponBaseUserDataParam {
        pub id: OtWeaponId,
        pub sort_id: u32,
        pub series_id: OtEquipSeriesId,
        pub rare_type: RareTypes,
        pub model_id: u32, // snow.data.DataDef.OtEquipModelId
        pub atk_type: OtAtkTypes,
        pub element_type: super::skill::ElementType,
        pub specilize_type: OtSpecializeTypes,
        pub def_bonus: i32,
        pub atk_val_list: [i32; 2],
        pub element_val_list: [u32; 2],
        pub critical_rate_list: [i32; 2],
        pub throw_model_color_index: u32,
        pub sell_value: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtWeaponBaseUserData",
        0x97da62e4 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtWeaponBaseUserData {
        pub param: Vec<OtWeaponBaseUserDataParam>
    }
}

// snow.data.OtEquipSeriesData.RankTypes
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum RankTypes {
        Lower = 0,
        Upper = 1,

    }
}

// snow.data.DataDef.EvaluationTypeFor3Argument
rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum EvaluationTypeFor3Argument {
        AndAnd = 0,
        OrOr = 1,
        AndOr = 2,
        OrAnd = 3,
    }
}

rsz_struct! {
    #[rsz("snow.data.OtEquipSeriesUserData.Param",
        0xe18c4847 = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtEquipSeriesUserDataParam {
        pub id: OtEquipSeriesId,
        pub rank: RankTypes,
        pub sort_id: u32,
        pub over_sort_id: u32,
        pub is_collaboration: i32,
        pub evaluation: EvaluationTypeFor3Argument,
        pub unlock_item: Vec<ItemId>,
        pub unlock_enemy: EmTypes,
        pub unlock_progress: i32, // snow.data.DataDef.UnlockProgressTypes
    }
}

rsz_struct! {
    #[rsz("snow.data.OtEquipSeriesUserData",
        0x6b5a7cf = 0,
    )]
    #[derive(Debug, Serialize)]
    pub struct OtEquipSeriesUserData {
        pub param: Vec<OtEquipSeriesUserDataParam>
    }
}
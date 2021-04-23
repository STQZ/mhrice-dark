use super::*;
use crate::{rsz_enum, rsz_struct};
use serde::*;

rsz_enum! {
    #[rsz(i32)]
    #[derive(Debug, Serialize)]
    pub enum SexualEquipableFlag {
        MaleOnly = 0,
        FemaleOnly = 1,
        Both = 2,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorBaseUserData.Param")]
    #[derive(Debug, Serialize)]
    pub struct ArmorBaseUserDataParam {
        pub pl_armor_id: u32,
        pub is_valid: bool,
        pub series: i32,
        pub sort_id: u32,
        pub model_id: u32,
        pub rare: u8, // 0 = rarity 1
        pub value: u32,
        pub buy_value: u32,
        pub sexual_equipable: SexualEquipableFlag,
        pub symbol_color1: bool,
        pub symbol_color2: bool,
        pub def_val: i32,
        pub fire_reg_val: i32,
        pub water_reg_val: i32,
        pub ice_reg_val: i32,
        pub thunder_reg_val: i32,
        pub dragon_reg_val: i32,
        pub buildup_table: i32, // snow.data.ArmorBuildupData.TableTypes
        pub buff_formula: i32, // snow.data.GameItemEnum.SeriesBufType
        pub decorations_num_list: Vec<u32>,
        pub skill_list: Vec<u8>, // snow.data.DataDef.PlEquipSkillId, 1 = ID_0
        pub skill_lv_list: Vec<i32>,
        pub id_after_ex_change: u32,
    }
}

rsz_struct! {
    #[rsz("snow.data.ArmorBaseUserData")]
    #[derive(Debug, Serialize)]
    pub struct ArmorBaseUserData {
        pub param: Vec<ArmorBaseUserDataParam>
    }
}

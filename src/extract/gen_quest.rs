use super::gen_item::*;
use super::gen_monster::*;
use super::gen_website::*;
use super::pedia::*;
use super::sink::*;
use crate::rsz::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

pub fn gen_quest_tag(quest: &Quest, is_target: bool) -> Box<div<String>> {
    let target_tag = if is_target {
        html!(<span class="tag is-primary">"Target"</span>)
    } else {
        html!(<span />)
    };
    html!(<div>
        <span class="tag">{text!("{:?}-{:?}", quest.param.enemy_level, quest.param.quest_level)}</span>
        {
            quest.is_dl.then(
                ||html!(<span class="tag">{text!("Event")}</span>)
            )
        }
        <a href={format!("/quest/{:06}.html", quest.param.quest_no)}>
        {quest.name.map_or(
            html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
            gen_multi_lang
        )}
        </a>
        {target_tag}
    </div>)
}

pub fn gen_quest_list(quests: &[Quest], output: &impl Sink) -> Result<()> {
    let mut quests_ordered: BTreeMap<_, BTreeMap<_, Vec<&Quest>>> = BTreeMap::new();
    for quest in quests {
        quests_ordered
            .entry(quest.param.enemy_level)
            .or_default()
            .entry(quest.param.quest_level)
            .or_default()
            .push(quest);
    }

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Quests - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">"Quests"</h1>
                {
                    quests_ordered.into_iter().map(|(enemy_level, quests)|{
                        html!(<section>
                         <h2 class="title">{text!("{:?}", enemy_level)}</h2>
                         <ul class="mh-list-quest">{
                            quests.into_iter().map(|(quest_level, quests)|{
                                html!(
                                    <li class="mh-list-quest">
                                        <h3 class="title">{text!("{:?}", quest_level)}</h3>
                                        <ul>{
                                            quests.into_iter().map(|quest|{
                                                let link = format!("/quest/{:06}.html", quest.param.quest_no);
                                                let name = quest.name.map_or(
                                                    html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
                                                    gen_multi_lang
                                                );
                                                let img = format!("/resources/questtype_{}.png",
                                                    quest.param.quest_type.icon_index());
                                                html!{<li>
                                                    <a href={link} class="mh-icon-text">
                                                    <img alt="Quest icon" src={img} class="mh-quest-icon"/>
                                                    {
                                                        quest.is_dl.then(
                                                            ||html!(<span class="tag">{text!("Event")}</span>)
                                                        )
                                                    }
                                                    {name}
                                                    </a>
                                                </li>}
                                            })
                                        }</ul>
                                    </li>
                                )
                            })
                        }</ul></section>)
                    })
                }
               </div> </main>
            </body>
        </html>
    );

    output
        .create_html("quest.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_quest_monster_data(
    enemy_param: Option<&SharedEnemyParam>,
    em_type: EmTypes,
    index: usize,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
) -> impl IntoIterator<Item = Box<td<String>>> {
    let enemy_param = if let Some(enemy_param) = enemy_param.as_ref() {
        enemy_param
    } else {
        return vec![html!(<td colspan=11>"[NO DATA]"</td>)];
    };

    let size = if let (Some(scale_tbl_i), Some(base_scale)) = (
        enemy_param.scale_tbl.get(index),
        enemy_param.scale.get(index),
    ) {
        if let (Some(size), Some(size_dist)) = (
            pedia_ex.sizes.get(&em_type),
            pedia_ex.size_dists.get(scale_tbl_i),
        ) {
            let mut small_chance = 0;
            let mut large_chance = 0;
            for sample in *size_dist {
                let scale = sample.scale * (*base_scale as f32) / 100.0;
                if scale <= size.small_boarder {
                    small_chance += sample.rate;
                }
                if scale >= size.king_boarder {
                    large_chance += sample.rate;
                }
            }

            let small = (small_chance != 0).then(|| {
                html!(<span class="tag">
                    <img alt="Small crown" src="/resources/small_crown.png" />
                    {text!("{}%", small_chance)}
                </span>)
            });

            let large = (large_chance != 0).then(|| {
                html!(<span class="tag">
                    <img alt="Large crown" src="/resources/king_crown.png" />
                    {text!("{}%", large_chance)}
                </span>)
            });

            html!(<span>{small}{large}</span>)
        } else {
            html!(<span>"-"</span>)
        }
    } else {
        html!(<span>"-"</span>)
    };

    let hp = enemy_param.vital_tbl.get(index).map_or_else(
        || "-".to_owned(),
        |v| {
            pedia
                .difficulty_rate
                .vital_rate_table_list
                .get(usize::from(*v))
                .map_or_else(|| format!("~ {}", v), |r| format!("x{}", r.vital_rate))
        },
    );
    let attack = enemy_param.attack_tbl.get(index).map_or_else(
        || "-".to_owned(),
        |v| {
            pedia
                .difficulty_rate
                .attack_rate_table_list
                .get(usize::from(*v))
                .map_or_else(|| format!("~ {}", v), |r| format!("x{}", r.attack_rate))
        },
    );
    let parts = enemy_param.parts_tbl.get(index).map_or_else(
        || "-".to_owned(),
        |v| {
            pedia
                .difficulty_rate
                .parts_rate_table_list
                .get(usize::from(*v))
                .map_or_else(
                    || format!("~ {}", v),
                    |r| format!("x{}", r.parts_vital_rate),
                )
        },
    );

    let defense;
    let element_a;
    let element_b;
    let stun;
    let exhaust;
    let ride;

    if let Some(v) = enemy_param.other_tbl.get(index) {
        if let Some(r) = pedia
            .difficulty_rate
            .other_rate_table_list
            .get(usize::from(*v))
        {
            defense = format!("x{}", r.defense_rate);
            element_a = format!("x{}", r.damage_element_rate_a);
            element_b = format!("x{}", r.damage_element_rate_b);
            stun = format!("x{}", r.stun_rate);
            exhaust = format!("x{}", r.tired_rate);
            ride = format!("x{}", r.marionette_rate);
        } else {
            let placeholder = format!("~ {}", v);
            defense = placeholder.clone();
            element_a = placeholder.clone();
            element_b = placeholder.clone();
            stun = placeholder.clone();
            exhaust = placeholder.clone();
            ride = placeholder;
        }
    } else {
        defense = "-".to_owned();
        element_a = "-".to_owned();
        element_b = "-".to_owned();
        stun = "-".to_owned();
        exhaust = "-".to_owned();
        ride = "-".to_owned();
    };

    let stamina = enemy_param
        .stamina_tbl
        .get(index)
        .map_or_else(|| "-".to_owned(), |v| format!("{}", v));

    vec![
        html!(<td>{size}</td>),
        html!(<td>{text!("{}", hp)}</td>),
        html!(<td>{text!("{}", attack)}</td>),
        html!(<td>{text!("{}", parts)}</td>),
        html!(<td>{text!("{}", defense)}</td>),
        html!(<td>{text!("{}", element_a)}</td>),
        html!(<td>{text!("{}", element_b)}</td>),
        html!(<td>{text!("{}", stun)}</td>),
        html!(<td>{text!("{}", exhaust)}</td>),
        html!(<td>{text!("{}", ride)}</td>),
        html!(<td>{text!("{}", stamina)}</td>),
    ]
}

fn gen_multi_factor(data: &MultiData) -> Box<div<String>> {
    html!(<div><ul class="mh-multi-factor">
        <li><span>"2: "</span><span>{text!("x{}", data.two)}</span></li>
        <li><span>"3: "</span><span>{text!("x{}", data.three)}</span></li>
        <li><span>"4: "</span><span>{text!("x{}", data.four)}</span></li>
    </ul></div>)
}

fn translate_rule(rule: LotRule) -> Box<span<String>> {
    let desc = match rule {
        LotRule::Random => "Get random amount",
        LotRule::RandomOut1 => "Get one",
        LotRule::RandomOut2 => "Get two",
        LotRule::RandomOut3 => "Get three",
        LotRule::FirstFix => "First one fixed",
    };
    html!(<span class="mh-lot-rule">{ text!("{}", desc) }</span>)
}

#[allow(clippy::vec_box)]
fn gen_quest_monster_multi_player_data(
    enemy_param: Option<&SharedEnemyParam>,
    index: usize,
    pedia: &Pedia,
) -> Vec<Box<td<String>>> {
    let no_data = || vec![html!(<td colspan=9>"[NO DATA]"</td>)];

    let enemy_param = if let Some(enemy_param) = enemy_param.as_ref() {
        enemy_param
    } else {
        return no_data();
    };

    let multi = if let Some(multi) = enemy_param.boss_multi.get(index) {
        *multi
    } else {
        return no_data();
    };

    let table = if let Some(table) = pedia
        .difficulty_rate
        .multi_rate_table_list
        .get(usize::from(multi))
    {
        &table.multi_data_list
    } else {
        return no_data();
    };

    table
        .iter()
        .map(|d| html!(<td>{gen_multi_factor(d)}</td>))
        .collect()
}

impl HyakuryuQuestData {
    fn display(&self) -> String {
        let mut list = vec![];
        if self.attr.contains(HyakuryuQuestAttr::FIX_WAVE_ORDER) {
            list.push("Fixed wave order")
        }
        if self.attr.contains(HyakuryuQuestAttr::LOT_HIGH_EX) {
            list.push("Red 7* reward")
        }
        if self.attr.contains(HyakuryuQuestAttr::LOT_TRUE_ED) {
            list.push("After true ending reward")
        }
        if self.attr.contains(HyakuryuQuestAttr::FINAL_BOSS_KILL) {
            list.push("Requires true ending")
        }
        if self.category == HyakuryuQuestCategory::Nushi {
            list.push("Has apex")
        }
        if self.is_village {
            list.push("Village")
        } else {
            list.push("Hub")
        }
        list.join(" | ")
    }
}

fn gen_quest(
    quest: &Quest,
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    mut output: impl Write,
    mut toc_sink: TocSink<'_>,
) -> Result<()> {
    if let Some(title) = quest.name {
        toc_sink.add(title);
    }

    let has_normal_em = quest
        .param
        .boss_em_type
        .iter()
        .any(|&em_type| em_type != EmTypes::Em(0));
    let img = format!(
        "/resources/questtype_{}.png",
        quest.param.quest_type.icon_index()
    );

    let target = quest
        .param
        .target_type
        .iter()
        .filter(|&&t| t != QuestTargetType::None)
        .map(|t| match t {
            QuestTargetType::ItemGet => "Collect".to_owned(),
            QuestTargetType::Hunting => "Hunt".to_owned(),
            QuestTargetType::Kill => "Slay".to_owned(),
            QuestTargetType::Capture => "Capture".to_owned(),
            QuestTargetType::AllMainEnemy => "Hunt all".to_owned(),
            QuestTargetType::EmTotal => "Hunt small monsters".to_owned(),
            QuestTargetType::FinalBarrierDefense => "Defend final barrier".to_owned(),
            QuestTargetType::FortLevelUp => "Level up fort".to_owned(),
            QuestTargetType::PlayerDown => "PlayerDown".to_owned(),
            QuestTargetType::FinalBoss => "Final boss".to_owned(),
            x => format!("{:?}", x),
        })
        .collect::<Vec<String>>()
        .join(", ");

    let requirement = quest
        .param
        .order_type
        .iter()
        .filter(|&&t| t != QuestOrderType::None)
        .map(|t| format!("{:?}", t))
        .collect::<Vec<String>>()
        .join(", ");

    let has_target_material = quest
        .param
        .tgt_item_id
        .iter()
        .any(|&item| item != ItemId::None);
    let target_material = has_target_material.then(|| {
        html!(<section class="section">
        <h2 class="title">"Target material"</h2>
        <ul>{
        quest.param.tgt_item_id.iter().zip(quest.param.tgt_num.iter())
            .filter(|&(&item, _)| item != ItemId::None)
            .map(|(&item, num)|{
            let item = if let Some(item) = pedia_ex.items.get(&item) {
                html!(<span>{gen_item_label(item)}</span>)
            } else {
                html!(<span>{text!("{:?}", item)}</span>)
            };
            html!(<li>
                {text!("{}x ", num)}
                {item}
            </li>)
        })
    }</ul>
        </section>)
    });

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Quest {:06}", quest.param.quest_no)}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container"> <div class="content">
                <div class="mh-title-icon">
                    <img alt="Quest icon" src={img} class="mh-quest-icon"/>
                </div>
                <h1 class="title">
                <span class="tag">{text!("{:?}-{:?}", quest.param.enemy_level, quest.param.quest_level)}</span>
                {
                    quest.is_dl.then(
                        ||html!(<span class="tag">{text!("Event")}</span>)
                    )
                }
                {
                    quest.name.map_or(
                        html!(<span>{text!("Quest {:06}", quest.param.quest_no)}</span>),
                        gen_multi_lang
                    )
                }</h1>
                <p><span>"Objective: "</span><span> {
                    quest.target.map_or(
                        html!(<span>"-"</span>),
                        gen_multi_lang
                    )
                }</span></p>
                <p><span>"From: "</span><span> {
                    quest.requester.map_or(
                        html!(<span>"-"</span>),
                        gen_multi_lang
                    )
                }</span></p>
                <p><span>"Detail: "</span></p>{
                    quest.detail.map_or(
                        html!(<div>"-"</div>),
                        |m|html!(<div><pre>{gen_multi_lang(m)}</pre></div>)
                    )
                }

                <section class="section">
                <h2 class="title">"Basic data"</h2>
                <div class="mh-kvlist">
                <p class="mh-kv"><span>"Map"</span>
                    <span>{ text!("{}", quest.param.map_no) }</span></p>
                <p class="mh-kv"><span>"Base time"</span>
                    <span>{ text!("{}", quest.param.base_time) }</span></p>
                <p class="mh-kv"><span>"Time variation"</span>
                    <span>{ text!("{}", quest.param.time_variation) }</span></p>
                <p class="mh-kv"><span>"Time limit"</span>
                    <span>{ text!("{}", quest.param.time_limit) }</span></p>
                <p class="mh-kv"><span>"Carts"</span>
                    <span>{ text!("{}", quest.param.quest_life) }</span></p>
                <p class="mh-kv"><span>"Requirement"</span>
                    <span>{ text!("{}", requirement) }</span></p>
                <p class="mh-kv"><span>"Target"</span>
                    <span>{ text!("{}", target) }</span></p>
                <p class="mh-kv"><span>"Reward money"</span>
                    <span>{ text!("{}", quest.param.rem_money) }</span></p>
                <p class="mh-kv"><span>"Reward village point"</span>
                    <span>{ text!("{}", quest.param.rem_village_point) }</span></p>
                <p class="mh-kv"><span>"Reward rank point"</span>
                    <span>{ text!("{}", quest.param.rem_rank_point) }</span></p>
                <p class="mh-kv"><span>"Is tutorial"</span>
                    <span>{ text!("{}", quest.param.is_tutorial) }</span></p>
                <p class="mh-kv"><span>"Auto match HR"</span>
                    <span>{ text!("{}", quest.param.auto_match_hr) }</span></p>
                </div>
                </section>

                { target_material }

                // TODO: monster spawn/swap behavior
                // TODO: supply_tbl
                // TODO: fence
                // TODO is_use_pillar

                { has_normal_em.then(||html!(<section class="section">
                <h2 class="title">"Monster stats"</h2>
                <table>
                    <thead><tr>
                        <th>"Monster"</th>
                        <th>"Size"</th>
                        <th>"HP"</th>
                        <th>"Attack"</th>
                        <th>"Parts"</th>
                        <th>"Defense"</th>
                        <th>"Element A"</th>
                        <th>"Element B"</th>
                        <th>"Stun"</th>
                        <th>"Exhaust"</th>
                        <th>"Ride"</th>
                        <th>"Stamina"</th>
                    </tr></thead>
                    <tbody> {
                        quest.param.boss_em_type.iter().copied().enumerate()
                        .filter(|&(_, em_type)|em_type != EmTypes::Em(0))
                        .map(|(i, em_type)|{
                            html!(<tr>
                                <td>{ gen_monster_tag(pedia, em_type, quest.param.has_target(em_type), false) }</td>
                                { gen_quest_monster_data(quest.enemy_param.as_ref().map(|p|&p.param),
                                    em_type, i, pedia, pedia_ex) }
                            </tr>)
                        })
                    } </tbody>
                </table>
                </section>))}

                { has_normal_em.then(||html!(<section class="section">
                <h2 class="title">"Multiplayer Factor"</h2>

                <table>
                    <thead><tr>
                        <th>"Monster"</th>
                        <th>"HP"</th>
                        <th>"Attack"</th>
                        <th>"Parts"</th>
                        <th>"Other parts"</th>
                        <th>"Multi parts"</th>
                        <th>"Defense"</th>
                        <th>"Element A"</th>
                        <th>"Element B"</th>
                        <th>"Stun"</th>
                        <th>"Exhaust"</th>
                        <th>"Ride"</th>
                        <th>"Monster to monster"</th>
                    </tr></thead>
                    <tbody> {
                        quest.param.boss_em_type.iter().copied().enumerate()
                        .filter(|&(_, em_type)|em_type != EmTypes::Em(0))
                        .map(|(i, em_type)|{
                            html!(<tr>
                                <td>{ gen_monster_tag(pedia, em_type, quest.param.has_target(em_type), false)}</td>
                                { gen_quest_monster_multi_player_data(
                                    quest.enemy_param.as_ref().map(|p|&p.param), i, pedia) }
                            </tr>)
                        })
                    } </tbody>
                </table>

                </section>)) }

                { quest.hyakuryu.map(|h| {
                    html!(<section class="section">
                    <h2 class="title">"Rampage data"</h2>
                    <div class="mh-kvlist">
                    <p class="mh-kv"><span>"Attribute"</span>
                        <span>{ text!("{}", h.display()) }</span></p>
                    <p class="mh-kv"><span>"Base time"</span>
                        <span>{ text!("{}", h.base_time) }</span></p>
                    <p class="mh-kv"><span>"Map block"</span>
                        <span>{ text!("{} - {}", h.start_block_no, h.end_block_no) }</span></p>
                    <p class="mh-kv"><span>"Magnamalo appears at wave"</span>
                        <span>{ text!("{}", h.extra_em_wave_no) }</span></p>
                    <p class="mh-kv"><span>"Magnamalo difficulty table"</span>
                        <span>{ text!("{}", h.extra_em_nando_tbl_no) }</span></p>
                    <p class="mh-kv"><span>"Apex order table"</span>
                        <span>{ text!("{}", h.nushi_order_tbl_no)}</span></p>
                    <p class="mh-kv"><span>"Siege weapon unlock table"</span>
                        <span>{ text!("{}", h.hm_unlock_tbl_no) }</span></p>
                    </div>
                    <div>"Tasks:"<ul>{
                        h.sub_target.iter().enumerate()
                        .filter(|(_, target)|**target != QuestTargetType::None)
                        .map(|(i, target)| {
                            let extra_target = (i == 5).then(
                                ||html!(<span>{
                                    text!(" (appears on wave {})", h.sub_target5_wave_no)}
                                </span>));
                            let s = match target {
                                QuestTargetType::HuntingMachine => "Install siege weapons",
                                QuestTargetType::DropItem => "Collect drops",
                                QuestTargetType::EmStun => "Stun monsters",
                                QuestTargetType::EmElement => "Apply element blight",
                                QuestTargetType::EmCondition => "Apply status",
                                QuestTargetType::EmCntWeapon => "Repel using weapon",
                                QuestTargetType::EmCntHmBallista  => "Repel using ballista",
                                QuestTargetType::EmCntHmCannon  => "Repel using cannon",
                                QuestTargetType::EmCntHmGatling => "Repel using gatling",
                                QuestTargetType::EmCntHmTrap => "Repel using bomb trap",
                                QuestTargetType::EmCntHmFlameThrower => "Repel using flamethrower",
                                QuestTargetType::EmCntHmNpc => "Repel by NPC",
                                QuestTargetType::EmCntHmDragnator => "Repel using dragonator",
                                QuestTargetType::ExtraEmRunaway => "Repel Magnamalo",
                                x => return html!(<li>{ text!("{}", *x as u8) }{extra_target}</li>)
                            };
                            html!(<li>{ text!("{}", s) }{extra_target}</li>)
                        })
                    }</ul></div>

                    <table>
                    <thead><tr>
                        <th>"Boss monster"</th>
                        <th>"Sub type"</th>
                        <th>"Boss scale table"</th>
                        <th>"Other monsters"</th>
                        <th>"Other scale table"</th>
                        <th>"Order table"</th>
                    </tr></thead>
                    <tbody> {
                        h.wave_data.iter()
                        .filter(|wave|wave.boss_em != EmTypes::Em(0))
                        .map(|wave| {
                            html!(<tr>
                                <td>{ gen_monster_tag(pedia, wave.boss_em, false, false) }</td>
                                <td>{text!("{}", wave.boss_sub_type)}</td>
                                <td>{text!("{}", wave.boss_em_nando_tbl_no)}</td>
                                <td><ul class="mh-rampage-em-list"> {
                                    wave.em_table.iter().filter(|&&em|em != EmTypes::Em(0))
                                    .map(|&em|html!(<li class="mh-rampage-em-list">
                                        { gen_monster_tag(pedia, em, false, true) }
                                    </li>))
                                } </ul></td>
                                <td>{text!("{}", wave.wave_em_nando_tbl_no)}</td>
                                <td>{text!("{}", wave.order_table_no)}</td>
                            </tr>)
                        })
                    } </tbody>
                    </table>

                    </section>)
                }) }

                <section class="section">
                <h2 class="title">"Rewards"</h2>
                { if let Some(reward) = &quest.reward {
                    html!(<div>
                    <p>{text!("Addtional target rewards: {}", reward.param.target_reward_add_num)}</p>
                    <p>{text!("Addtional quest rewards: {}", reward.param.common_material_add_num)}</p>
                    <p>"See monster's page for target rewards."</p>
                    <div class="mh-reward-tables">

                    { if let Some(common_material_reward) = &reward.common_material_reward {
                        html!(<div class="mh-reward-box">
                        <table>
                            <thead><tr>
                                <th>"Quest rewards"<br/>{translate_rule(common_material_reward.lot_rule)}</th>
                                <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &common_material_reward.item_id_list,
                                    &common_material_reward.num_list,
                                    &common_material_reward.probability_list)
                            } </tbody>
                        </table>
                        </div>)
                    } else {
                        html!(<div></div>)
                    }}

                    { if let Some(additional_target_reward) = reward.additional_target_reward {
                        html!(<div class="mh-reward-box">
                        <table>
                            <thead><tr>
                                <th>"Addtional target rewards"<br/>{translate_rule(additional_target_reward.lot_rule)}</th>
                                <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &additional_target_reward.item_id_list,
                                    &additional_target_reward.num_list,
                                    &additional_target_reward.probability_list)
                            } </tbody>
                        </table>
                        </div>)
                    } else {
                        html!(<div></div>)
                    }}

                    { reward.additional_quest_reward.iter().map(|additional_quest_reward| {
                        html!(<div class="mh-reward-box">
                        <table>
                            <thead><tr>
                                <th>"Addtional rewards"<br/>{translate_rule(additional_quest_reward.lot_rule)}</th>
                                <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &additional_quest_reward.item_id_list,
                                    &additional_quest_reward.num_list,
                                    &additional_quest_reward.probability_list)
                            } </tbody>
                        </table>
                        </div>)
                    })}

                    { if let Some(cloth_ticket) = &reward.cloth_ticket {
                        html!(<div class="mh-reward-box">
                        <table>
                            <thead><tr>
                                <th>"Outfit voucher"<br/>{translate_rule(cloth_ticket.lot_rule)}</th>
                                <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &cloth_ticket.item_id_list,
                                    &cloth_ticket.num_list,
                                    &cloth_ticket.probability_list)
                            } </tbody>
                        </table>
                        </div>)
                    } else {
                        html!(<div></div>)
                    }}

                    </div>
                    </div>)
                } else {
                    html!(<div>"No data"</div>)
                }}
                </section>
                </div> </div> </main>
            </body>
        </html>
    );

    output.write_all(doc.to_string().as_bytes())?;
    Ok(())
}

pub fn gen_quests(
    pedia: &Pedia,
    pedia_ex: &PediaEx<'_>,
    output: &impl Sink,
    toc: &mut Toc,
) -> Result<()> {
    let quest_path = output.sub_sink("quest")?;
    for quest in &pedia_ex.quests {
        let (path, toc_sink) =
            quest_path.create_html_with_toc(&format!("{:06}.html", quest.param.quest_no), toc)?;
        gen_quest(quest, pedia, pedia_ex, path, toc_sink)?
    }
    Ok(())
}

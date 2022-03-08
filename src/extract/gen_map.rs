use super::gen_common::*;
use super::gen_item::*;
use super::gen_website::*;
use super::pedia::*;
use super::prepare_map::*;
use super::sink::*;
use anyhow::Result;
use std::io::Write;
use typed_html::{dom::*, elements::*, html, text};

fn map_page(id: i32) -> String {
    format!("{id:02}.html")
}

fn gen_map_label(id: i32, pedia: &Pedia) -> Box<a<String>> {
    let link = format!("/map/{}", map_page(id));
    let name_name = format!("Stage_Name_{id:02}");
    let name = pedia.map_name.get_entry(&name_name);
    if let Some(name) = name {
        html!(<a href={link}>{ gen_multi_lang(name) }</a>)
    } else {
        html!(<a href={link}>{ text!("Map {:02}", id) }</a>)
    }
}

fn gen_map(
    id: i32,
    map: &GameMap,
    pedia: &Pedia,
    pedia_ex: &PediaEx,
    mut output: impl Write,
) -> Result<()> {
    let mut map_icons = vec![];
    let mut map_explains = vec![];
    for (i, pop) in map.pops.iter().enumerate() {
        let x = (pop.x + map.x_offset) / map.map_scale * 100.0;
        let y = (pop.y + map.y_offset) / map.map_scale * 100.0;

        let icon_inner: Box<dyn Fn() -> Box<div<String>>>;
        let explain_inner;
        match &pop.kind {
            MapPopKind::Item { behavior, relic } => {
                icon_inner = Box::new(|| {
                    let icon_path = format!("/resources/item/{:03}", behavior.pop_icon);
                    gen_colored_icon(behavior.pop_icon_color, &icon_path, &[])
                });

                if let Some(lot) = pedia_ex
                    .item_pop
                    .get(&(behavior.pop_id, id))
                    .or_else(|| pedia_ex.item_pop.get(&(behavior.pop_id, -1)))
                {
                    explain_inner = html!(<div class="mh-reward-tables">
                        <div class="mh-reward-box"><table>
                            <thead><tr>
                            <th>"Low rank material"</th>
                            <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &lot.lower_id,
                                    &lot.lower_num,
                                    &lot.lower_probability)
                            } </tbody>
                        </table></div>

                        <div class="mh-reward-box"><table>
                            <thead><tr>
                            <th>"High rank material"</th>
                            <th>"Probability"</th>
                            </tr></thead>
                            <tbody> {
                                gen_reward_table(pedia_ex,
                                    &lot.upper_id,
                                    &lot.upper_num,
                                    &lot.upper_probability)
                            } </tbody>
                        </table></div>
                    </div>);
                } else {
                    explain_inner = html!(<div>"No data"</div>)
                }
            }
        }
        let map_icon_id = format!("mh-map-icon-{i}");
        let map_explain_id = format!("mh-map-explain-{i}");
        let map_explain_event = format!("onShowMapExplain('{i}');");

        map_icons.push(html!(<div class="mh-map-pop" id={map_icon_id.as_str()}
            style={format!("left:{x}%;top:{y}%")} onclick={map_explain_event.as_str()}> {icon_inner()} </div>: String
        ));
        map_explains.push(html!(<div class="mh-hidden" id={map_explain_id.as_str()}>
            {icon_inner()}
            <p>{ text!("level: {}", pop.z) }</p>
            {explain_inner}
        </div>))
    }

    let name_name = format!("Stage_Name_{id:02}");
    let name = pedia.map_name.get_entry(&name_name);

    let title = if let Some(name) = name {
        gen_multi_lang(name)
    } else {
        html!(<span>{ text!("Map {:02}", id) }</span>)
    };

    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Map {:02}", id)}</title>
                { head_common() }
            </head>
            <body>
            { navbar() }
            <main><div class="container"> <div class="content">

            <h1 class="title">
            {title}
            </h1>

            <div class="columns">

            <div class="column is-two-thirds">
            <div class="mh-map-container">
            <div class="mh-map" id="mh-map">
            {(0..map.layer_count).map(|j| {
                let c = if j == 0 {
                    "mh-map-layer"
                } else {
                    "mh-map-layer mh-hidden"
                };
                let html_id = format!("mh-map-layer-{}", j);
                html!(
                    <img class={c} id={html_id.as_str()} src={format!("/resources/map{id:02}_{j}.png")}/>
                )
            })}
            { map_icons }
            </div>
            </div>
            </div>

            <div class="column">
            <div>
            <button class="button is-white" id="button-scale-down" onclick="scaleDownMap();" disabled=true>
                <span class="icon"><i class="fas fa-search-minus"></i></span>
            </button>
            <button class="button is-white" id="button-scale-up" onclick="scaleUpMap();">
                <span class="icon"><i class="fas fa-search-plus"></i></span>
            </button>
            {
                (map.layer_count > 1).then(||html!(
                    <button class="button is-white" onclick="switchMapLayer();">
                      <span class="icon"><i class="fas fa-layer-group"></i></span>
                      <span>"Change Layer"</span>
                    </button>: String))
            }
            </div>
            { map_explains }
            <div id="mh-map-explain-default">"Click an icon on the map to learn the detail."</div>
            </div>

            </div>

            </div></div></main>
            </body>
        </html>: String
    );

    output.write_all(doc.to_string().as_bytes())?;

    Ok(())
}

pub fn gen_maps(pedia: &Pedia, pedia_ex: &PediaEx, output: &impl Sink) -> Result<()> {
    let map_path = output.sub_sink("map")?;
    for (&id, map) in &pedia.maps {
        let path = map_path.create_html(&map_page(id))?;
        gen_map(id, map, pedia, pedia_ex, path)?
    }
    Ok(())
}

pub fn gen_map_list(pedia: &Pedia, output: &impl Sink) -> Result<()> {
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>{text!("Maps - MHRice")}</title>
                { head_common() }
            </head>
            <body>
                { navbar() }
                <main> <div class="container">
                <h1 class="title">"Map"</h1>
                <ul>
                {
                    pedia.maps.iter().map(|(&i, _)|{
                        html!(<li>
                            {gen_map_label(i, pedia)}
                        </li>)
                    })
                }
                </ul>
                </div></main>
            </body>
        </html>: String
    );
    output
        .create_html("map.html")?
        .write_all(doc.to_string().as_bytes())?;

    Ok(())
}

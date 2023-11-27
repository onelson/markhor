use crate::db::Zone;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub active_zone: Option<usize>,
}

// FIXME: original has a "none" item in the list to restore the collectibles to being unfiltered
#[function_component]
pub fn ZonePicker(props: &Props) -> Html {
    let zones = use_memo((), |_| crate::DATABASE.zones().copied().collect::<Vec<_>>());
    let items = zones
        .iter()
        .map(|zone| {
            html! {
                <li key={zone.id}>
                <Item active_zone={props.active_zone} item={*zone}/>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <ul>{items}</ul>
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    active_zone: Option<usize>,
    item: Zone,
}

#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    let active = if Some(props.item.id) == props.active_zone {
        "active"
    } else {
        ""
    };

    html! {
        <div class={classes!("mk", "item", active)}>
            {props.item.name}
        </div>
    }
}

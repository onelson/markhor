use crate::store::{Action, StoreContext};
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
                <Item active_zone={props.active_zone} label={zone.name} zone_id={zone.id}/>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <ul>
        <li><Item active_zone={props.active_zone} label={"None"} zone_id={None} /></li>
        {items}
        </ul>
    }
}

#[derive(PartialEq, Properties)]
pub struct ItemProps {
    active_zone: Option<usize>,
    label: &'static str,
    zone_id: Option<usize>,
}

#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    let store = use_context::<StoreContext>().expect("Store ctx");
    let active = props.zone_id == props.active_zone;

    let onclick = {
        let store = store.clone();
        let id = props.zone_id;
        Callback::from(move |_| store.dispatch(Action::SetZone(id)))
    };

    html! {
        <div class={classes!("mk", "item", if active {"active"} else {""})}>
            <label onclick={onclick}><input type="radio" name="active-zone" checked={active}/>{props.label}</label>
        </div>
    }
}

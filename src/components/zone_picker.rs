use crate::store::{Action, StoreContext};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub active_zone: Option<usize>,
}

#[function_component]
pub fn ZonePicker(props: &Props) -> Html {
    let store = use_context::<StoreContext>().expect("Store ctx");

    let onchange = {
        let store = store.clone();
        Callback::from(move |event: Event| {
            if let Some(target) = event.target_dyn_into::<HtmlSelectElement>() {
                let value = target.value().parse::<usize>().ok();
                store.dispatch(Action::SetZone(value))
            }
        })
    };

    let zones = use_memo((), |_| crate::DATABASE.zones().copied().collect::<Vec<_>>());
    let items = zones
        .iter()
        .map(|zone| html! { <Item key={zone.id} label={zone.name} zone_id={zone.id} active_zone={props.active_zone}/> })
        .collect::<Html>();

    html! {
        <div class="zone-picker">
            <select {onchange}>
                <Item label={"All"} zone_id={None} active_zone={props.active_zone}/>
                {items}
            </select>
        </div>
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
    let value = props.zone_id.map(|x| x.to_string()).unwrap_or_default();
    html! {<option {value} selected={props.active_zone == props.zone_id}>{props.label}</option>}
}

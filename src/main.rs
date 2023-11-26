use components::zone_picker::ZonePicker;
use yew::prelude::*;
mod components;
mod db;

#[function_component]
fn App() -> Html {
    let zones = use_memo((), |_| {
        let data = db::Database::default();
        data.zones().copied().collect::<Vec<_>>()
    });
    html! {
        <ZonePicker zones={zones} active_zone={None}/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

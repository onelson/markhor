use components::zone_picker::ZonePicker;
use yew::prelude::*;
mod components;
mod db;

const DATABASE: db::Database = db::Database::new();

#[function_component]
fn App() -> Html {
    html! {
        <ZonePicker active_zone={None}/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

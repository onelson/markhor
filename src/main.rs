use crate::store::StoreContext;
use components::collectible_list::CollectibleList;
use components::zone_picker::ZonePicker;
use store::Store;
use yew::prelude::*;

mod components;
mod db;
mod store;

const DATABASE: db::Database = db::Database::new();

#[function_component]
fn App() -> Html {
    let store = use_reducer(Store::default);

    html! {
        <ContextProvider<StoreContext> context={store.clone()}>
            <CollectibleList/>
            <ZonePicker active_zone={store.active_zone}/>
        </ContextProvider<StoreContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

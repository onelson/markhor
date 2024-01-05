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

    let collected_count = store.collected.len();
    let total_count = use_memo((), |_| DATABASE.collectibles().count());
    let counts = format!("({collected_count}/{total_count})");

    html! {
        <ContextProvider<StoreContext> context={store.clone()}>
            <ZonePicker active_zone={store.active_zone}/>
            <div class="total-count">{counts}</div>
            <CollectibleList/>
        </ContextProvider<StoreContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

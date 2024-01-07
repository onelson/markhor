use crate::store::StoreContext;
use components::collectible_list::CollectibleList;
use components::zone_picker::ZonePicker;
use gloo::storage::{LocalStorage, Storage};
use std::collections::BTreeSet;
use store::Store;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;

mod components;
mod db;
mod store;

const DATABASE: db::Database = db::Database::new();

#[function_component]
fn App() -> Html {
    const STORAGE_KEY: &str = "markhor.collected";

    let persisted = use_memo((), |_| {
        LocalStorage::get::<BTreeSet<usize>>(STORAGE_KEY).unwrap_or_default()
    });

    let store = use_reducer(|| Store::from(persisted.as_ref()));

    // N.b. this hook will fire when any field on store changes (including `active_zone`).
    // Using `store` as the dependency rather than `store.collected` saves us from having to clone the `BTreeSet` with
    // each render, however. Other workarounds are available, but this kludge is the least annoying.
    use_effect_with(store.clone(), |store| {
        LocalStorage::set(STORAGE_KEY, &store.collected).unwrap_throw();
    });

    let collected_count = store.collected.len();
    let total_count = use_memo((), |_| DATABASE.collectibles().count());
    let counts = format!("Progress: ({collected_count}/{total_count})");

    html! {
        <ContextProvider<StoreContext> context={store.clone()}>
            <div class="topbar">
                <div class="label">{"Filter by Zone:"}</div>
                <ZonePicker active_zone={store.active_zone}/>
                <div class="label">{counts}</div>
            </div>
            <CollectibleList/>
        </ContextProvider<StoreContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

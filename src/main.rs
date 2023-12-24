use crate::store::StoreContext;
use components::collectible_list::CollectibleList;
use components::zone_picker::ZonePicker;
use store::Store;
use yew::prelude::*;

mod components;
mod db;
mod store;

const DATABASE: db::Database = db::Database::new();

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum Tab {
    #[default]
    Collectibles,
    Zones,
}

#[function_component]
fn App() -> Html {
    let store = use_reducer(Store::default);
    let active_tab = use_state_eq(Tab::default);

    let tabs = html! {
      <ul>
        <li class={classes!(if *active_tab == Tab::Collectibles {"active"} else {""})}
            onclick={
                Callback::from({
                let active_tab = active_tab.clone();
                move |_| active_tab.set(Tab::Collectibles)})
            }
            >{"items"}</li>
        <li class={classes!(if *active_tab == Tab::Zones {"active"} else {""})}
            onclick={
                Callback::from({
                let active_tab = active_tab.clone();
                move |_| active_tab.set(Tab::Zones)})
            }>{"zones"}</li>
      </ul>
    };

    let content = if *active_tab == Tab::Collectibles {
        html! { <CollectibleList/> }
    } else {
        html! { <ZonePicker active_zone={store.active_zone}/> }
    };

    html! {
        <ContextProvider<StoreContext> context={store.clone()}>
        {tabs}
        {content}
        </ContextProvider<StoreContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

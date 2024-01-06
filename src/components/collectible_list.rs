use crate::db::Collectible;
use crate::store::{Action, StoreContext};
use std::collections::BTreeSet;
use std::rc::Rc;
use yew::prelude::*;

#[function_component]
pub fn CollectibleList() -> Html {
    let store = use_context::<StoreContext>().expect("Store ctx");
    let active_items = use_memo(store.active_zone, |active_zone| match active_zone {
        None => crate::DATABASE
            .collectibles()
            .map(|x| x.id)
            .collect::<BTreeSet<_>>(),
        Some(active_zone) => crate::DATABASE
            .collectibles_by_zone(*active_zone)
            .map(|x| x.id)
            .collect::<BTreeSet<_>>(),
    });

    let mut groups = crate::DATABASE
        .categories()
        .map(|cat| {
            let items = crate::DATABASE
                .collectibles_by_category(cat.id)
                .copied()
                .collect::<Vec<_>>();

            (cat.name, items)
        })
        .collect::<Vec<_>>();

    // Ordering the groups by name, but more importantly by "if any of the active items are not yet collected."
    // N.b. the first field in the key looks backwards, but remember `false` sorts **lower** than `true` so `false`
    // gives a higher priority!
    groups.sort_by_key(|x| {
        (
            !x.1.iter()
                .any(|i| active_items.contains(&i.id) && !store.collected.contains(&i.id)),
            x.0,
        )
    });

    let groups = groups
        .into_iter()
        .map(|(cat_name, items)| {
            html! {
                <CollectibleGroup
                    key={cat_name}
                    active_items={active_items.clone()}
                    label={cat_name}
                    {items}/>
            }
        })
        .collect::<Html>();

    html! {
        <div class="collectibles">{groups}</div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CollectibleGroupProps {
    active_items: Rc<BTreeSet<usize>>,
    label: &'static str,
    items: Vec<Collectible>,
}

#[function_component]
pub fn CollectibleGroup(props: &CollectibleGroupProps) -> Html {
    let store = use_context::<StoreContext>().expect("Store ctx");

    let collected_in_group: BTreeSet<usize> = {
        let item_ids = props
            .items
            .iter()
            .map(|x| x.id)
            .collect::<BTreeSet<usize>>();

        &store.collected & &item_ids
    };

    let collected_count = collected_in_group.len();
    let total_count = props.items.len();
    let counts = format!("({collected_count}/{total_count})");

    let items = props
        .items
        .iter()
        .map(|x| {
            let active = props.active_items.contains(&x.id);
            html! {
                <li key={x.id}>
                    <CollectibleItem {active} item={*x}/>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <div class="collectible-group">
            <h4 class="heading">{props.label}{" "}{counts}</h4>
            <ul class="list">
                {items}
            </ul>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct CollectibleItemProps {
    active: bool,
    item: Collectible,
}

#[function_component]
pub fn CollectibleItem(props: &CollectibleItemProps) -> Html {
    let store = use_context::<StoreContext>().expect("Store ctx");

    let id = props.item.id;
    let collected = store.collected.contains(&id);
    let onchange = Callback::from(move |e: Event| {
        e.prevent_default();
        e.stop_propagation();
        store.dispatch(Action::ToggleCollectible(id));
    });

    html! {
        <div class={classes!("mk", "item", if !props.active {"inactive"} else {""})}>
            <label>
                <input type="checkbox" checked={collected} {onchange}/>
                {props.item.name}
                {" "}
                <span>{props.item.short_name}</span>
            </label>
        </div>
    }
}

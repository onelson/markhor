use std::collections::BTreeSet;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    SetZone(Option<usize>),
    ToggleCollectible(usize),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Store {
    pub active_zone: Option<usize>,
    pub collected: BTreeSet<usize>,
}

impl Reducible for Store {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next = match action {
            Action::SetZone(zone) => Self {
                active_zone: zone,
                collected: self.collected.clone(),
            },
            Action::ToggleCollectible(x) => Self {
                active_zone: self.active_zone,
                collected: &self.collected ^ &BTreeSet::from([x]),
            },
        };
        next.into()
    }
}

pub type StoreContext = UseReducerHandle<Store>;

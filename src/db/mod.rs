pub use crate::db::categories::Category;
pub use crate::db::zones::Zone;
use collectible_zone_membership::CollectibleZoneMembership;
pub use collectibles::Collectible;

mod categories;
mod collectible_zone_membership;
mod collectibles;
mod zones;

pub struct Database<'a> {
    categories: &'a [Category],
    collectibles: &'a [Collectible],
    membership: &'a [CollectibleZoneMembership],
    zones: &'a [Zone],
}

impl<'a> Database<'a> {
    pub const fn new() -> Self {
        Self {
            categories: &categories::DATA,
            collectibles: &collectibles::DATA,
            membership: &collectible_zone_membership::DATA,
            zones: &zones::DATA,
        }
    }

    pub fn categories(&self) -> impl Iterator<Item = &Category> {
        self.categories.iter()
    }

    pub fn collectibles_by_category(
        &self,
        category_id: usize,
    ) -> impl Iterator<Item = &Collectible> {
        self.collectibles
            .iter()
            .filter(move |x| x.category_id == category_id)
    }

    pub fn collectible_by_id(&self, id: usize) -> &Collectible {
        // XXX: cheating the lookup. IDs start at 1, so the offset is knowable.
        debug_assert!(id > 0, "ids start at 1");
        &self.collectibles[id - 1]
    }

    pub fn collectibles_by_zone(&self, zone_id: usize) -> impl Iterator<Item = &Collectible> {
        self.membership.iter().filter_map(move |x| {
            if x.zone_id == zone_id {
                Some(self.collectible_by_id(x.collectible_id))
            } else {
                None
            }
        })
    }

    pub fn collectibles(&self) -> impl Iterator<Item = &Collectible> {
        self.collectibles.iter()
    }

    pub fn zones(&self) -> impl Iterator<Item = &Zone> {
        self.zones.iter()
    }
}

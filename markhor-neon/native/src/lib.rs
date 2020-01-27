#[macro_use]
extern crate neon;
use neon::prelude::*;
use serde::Serialize;

pub struct MarkhorDB {
    database_url: String,
    conn: markhor::Conn,
}

impl MarkhorDB {
    pub fn new(database_url: String) -> Self {
        let conn = markhor::get_conn(&database_url);
        markhor::init_db(&conn).expect("init db");
        Self { database_url, conn }
    }
}

declare_types! {
  pub class JsMarkhorDB for MarkhorDB {
    init(mut cx) {
      let database_url = cx.argument::<JsString>(0)?;
      let database_url = database_url.value();
      Ok(MarkhorDB::new(database_url) )
    }

    method getZones(mut cx) {
        let this = cx.this();
        let zones = {
            let guard = cx.lock();
            let db = this.borrow(&guard);
            markhor::get_zones(&db.conn).unwrap()
        };
        Ok(neon_serde::to_value(&mut cx, &zones)?)
    }

    method getCategories(mut cx) {
        let this = cx.this();
        let categories = {
            let guard = cx.lock();
            let db = this.borrow(&guard);
            markhor::get_categories(&db.conn).unwrap()
        };
        Ok(neon_serde::to_value(&mut cx, &categories)?)

    }

    method getCollectibles(mut cx) {
        let this = cx.this();
        let collectibles = {
            let guard = cx.lock();
            let db = this.borrow(&guard);
            markhor::get_collectibles(&db.conn).unwrap()
        };
        Ok(neon_serde::to_value(&mut cx, &collectibles)?)

    }

    method getMemberships(mut cx) {
        let this = cx.this();
        let memberships = {
            let guard = cx.lock();
            let db = this.borrow(&guard);
            markhor::get_memberships(&db.conn).unwrap()
        };
        Ok(neon_serde::to_value(&mut cx, &memberships)?)

    }

    method updateCollectibleCollected(mut cx) {
        let this = cx.this();
        let collectible_id = cx.argument::<JsNumber>(0)?.value() as i32;
        let is_collected = cx.argument::<JsBoolean>(1)?.value();

        let updated = {
            let guard = cx.lock();
            let db = this.borrow(&guard);
            markhor::update_collectible_collected(
                &db.conn,
                collectible_id,
                is_collected
            ).unwrap()
        };
        Ok(neon_serde::to_value(&mut cx, &updated)?)
    }
  }

}

register_module!(mut m, {
    m.export_class::<JsMarkhorDB>("MarkhorDB")?;
    Ok(())
});

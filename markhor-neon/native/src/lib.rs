use neon::prelude::*;
use neon::{class_definition, declare_types, impl_managed, register_module};

pub struct MarkhorDB {
    database_url: String,
    conn: markhor::Conn,
}

impl MarkhorDB {
    pub fn new(database_url: String) -> Self {
        let conn = markhor::get_conn(&database_url);
        Self { database_url, conn }
    }
}

declare_types! {
  pub class JsMarkhorDB for MarkhorDB {
    init(mut cx) {
      let database_url = cx.argument::<JsString>(0)?;
      let database_url = database_url.value();
      let conn = markhor::get_conn(&database_url);

      let _ = markhor::init_db(&conn).expect("init db");

      Ok(MarkhorDB::new(database_url))
    }

    method panic(_) {
      panic!("MarkhorDB.prototype.panic")
    }

    method get_zones(_) { unimplemented!() }
    method get_categories(_) { unimplemented!() }
    method get_collectibles(_) { unimplemented!() }
    method get_memberships(_) { unimplemented!() }
    method update_collectible_collected(_) { unimplemented!() }

  }
}

register_module!(mut m, { m.export_class::<JsMarkhorDB>("MarkhorDB") });

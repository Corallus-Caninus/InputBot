use crate::public::*;
use once_cell::sync::Lazy;
pub use std::{
    collections::hash_map::HashMap,
    sync::atomic::{AtomicPtr, Ordering},
    sync::Mutex,
    //import rwlock
    sync::{Arc, RwLock},
    thread::spawn,
};

pub enum Bind {
    NormalBind(BindHandler),
    BlockBind(BlockBindHandler),
    BlockableBind(BlockableBindHandler),
}

pub type BindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockBindHandler = Arc<dyn Fn() + Send + Sync + 'static>;
pub type BlockableBindHandler = Arc<dyn Fn() -> BlockInput + Send + Sync + 'static>;
pub type KeybdBindMap = HashMap<KeybdKey, Bind>;
pub type MouseBindMap = HashMap<MouseButton, Bind>;

// pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
//same as above but RWLock since only written on init
// pub static KEYBD_BINDS: Lazy<KeybdBindMap> = Lazy::new(|| KeybdBindMap::new());
//same as above but with mutable keybdbindmap
pub static KEYBD_BINDS: Lazy<Mutex<KeybdBindMap>> = Lazy::new(|| Mutex::new(KeybdBindMap::new()));
pub static KEYBD_RELEASE_BINDS: Lazy<Mutex<KeybdBindMap>> =
    Lazy::new(|| Mutex::new(KeybdBindMap::new()));

pub static MOUSE_BINDS: Lazy<Mutex<MouseBindMap>> = Lazy::new(|| Mutex::new(MouseBindMap::new()));

use fxhash::FxHashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VarId(u32);

impl VarId {
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Default)]
pub struct SymbolTable {
    names: Vec<String>,
    ids: FxHashMap<String, VarId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn intern(&mut self, name: impl AsRef<str>) -> VarId {
        let name = name.as_ref();
        if let Some(&id) = self.ids.get(name) {
            return id;
        }

        let id = VarId(self.names.len() as u32);
        let owned = name.to_owned();
        self.names.push(owned.clone());
        self.ids.insert(owned, id);
        id
    }

    pub fn resolve(&self, id: VarId) -> Option<&str> {
        self.names.get(id.index()).map(|name| name.as_str())
    }
}

pub struct Env<T> {
    values: FxHashMap<VarId, T>,
}

impl<T> Default for Env<T> {
    fn default() -> Self {
        Self {
            values: FxHashMap::default(),
        }
    }
}

impl<T> Env<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, symbol: VarId, value: T) {
        self.values.insert(symbol, value);
    }

    pub fn get(&self, symbol: VarId) -> Option<&T> {
        self.values.get(&symbol)
    }
}

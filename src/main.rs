use std::{
    collections::{BTreeSet, HashSet},
    fs,
};

use quartz_nbt::{
    io::{read_nbt, Flavor},
    NbtCompound, NbtList, NbtTag,
};
use stupid_utils::predule::{MapValue, MutableInit};
fn main() {
    println!("Hello, world!");
    let mut f = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open("level.dat")
        .unwrap();
    let (mut nbt, _) = read_nbt(&mut f, Flavor::GzCompressed).unwrap();

    let mut names = BTreeSet::new();

    visit_compound(&mut names, &nbt);

    let biomes = nbt
        .get_cpd_mut_unwarp("Data")
        .get_cpd_mut_unwarp("WorldGenSettings")
        .get_cpd_mut_unwarp("dimensions")
        .get_cpd_mut_unwarp("minecraft:overworld")
        .get_cpd_mut_unwarp("generator")
        .get_cpd_mut_unwarp("biome_source")
        .get_list_mut_unwarp("biomes");
    biomes.inner_mut().retain(|v| match v {
        NbtTag::Compound(c) => !c.get::<_, &str>("biome").unwrap().starts_with("terralith"),
        _ => true,
    });
    // dbg!(biomes);

    let mut f = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open("level_cleaned.dat")
        .unwrap();
    quartz_nbt::io::write_nbt(&mut f, None, &nbt, Flavor::GzCompressed).unwrap();
}
fn visit_compound(store: &mut BTreeSet<String>, nbt: &NbtCompound) {
    nbt.inner().iter().for_each(|(name, value)| {
        store.insert(name.clone());
        visit_tag(store, value);
    });
}

fn visit_tag(store: &mut BTreeSet<String>, nbt: &NbtTag) {
    match nbt {
        NbtTag::List(list) => {
            for t in list.iter() {
                visit_tag(store, t);
            }
        }
        NbtTag::Compound(c) => {
            visit_compound(store, c);
        }
        _ => {}
    }
}

trait NbtHelp {
    fn get_cpd_mut(&mut self, key: &str) -> Option<&mut NbtCompound>;
    fn get_cpd_mut_unwarp(&mut self, key: &str) -> &mut NbtCompound {
        self.get_cpd_mut(key).unwrap()
    }
    fn get_list_mut(&mut self, key: &str) -> Option<&mut NbtList>;
    fn get_list_mut_unwarp(&mut self, key: &str) -> &mut NbtList {
        self.get_list_mut(key).unwrap()
    }
}
impl NbtHelp for NbtCompound {
    fn get_cpd_mut(&mut self, key: &str) -> Option<&mut NbtCompound> {
        match self.inner_mut().get_mut(key)? {
            NbtTag::Compound(c) => Some(c),
            _ => None,
        }
    }

    fn get_list_mut(&mut self, key: &str) -> Option<&mut NbtList> {
        match self.inner_mut().get_mut(key)? {
            NbtTag::List(c) => Some(c),
            _ => None,
        }
    }
}

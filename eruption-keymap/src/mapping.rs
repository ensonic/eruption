/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.

    Copyright (c) 2019-2022, The Eruption Development Team
*/

use std::collections::BTreeSet;
use std::fmt::Display;
use std::ops::Add;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use serde_json_any_key::any_key_map;

//pub type Result<T> = std::result::Result<T, eyre::Error>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct KeyMappingTable {
    pub metadata: TableMetadata,

    #[serde(with = "any_key_map")]
    pub mappings: HashMap<Source, Action>,
}

#[allow(unused)]
impl KeyMappingTable {
    pub fn new() -> Self {
        Self {
            metadata: TableMetadata::default(),
            mappings: HashMap::new(),
        }
    }

    pub fn file_name(&self) -> &Path {
        &self.metadata.file_name
    }

    pub fn description(&self) -> &str {
        &self.metadata.description
    }

    pub fn set_file_name<P: AsRef<Path>>(&mut self, file_name: P) {
        self.metadata.file_name = file_name.as_ref().to_path_buf();
    }

    pub fn set_description(&mut self, description: &str) {
        self.metadata.description = description.to_string();
    }

    pub fn insert(&mut self, source: Source, action: Action) -> Option<Action> {
        self.mappings.insert(source, action)
    }

    pub fn remove(&mut self, source: &Source) -> Option<Action> {
        self.mappings.remove(source)
    }

    pub fn metadata(&self) -> &TableMetadata {
        &self.metadata
    }

    pub fn mappings(&self) -> &HashMap<Source, Action> {
        &self.mappings
    }

    pub fn metadata_mut(&mut self) -> &mut TableMetadata {
        &mut self.metadata
    }

    pub fn mappings_mut(&mut self) -> &mut HashMap<Source, Action> {
        &mut self.mappings
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct TableMetadata {
    pub file_name: PathBuf,
    pub description: String,
    pub creation_date: DateTime<Utc>,
}

#[allow(unused)]
impl TableMetadata {
    pub fn new(file_name: PathBuf, description: String, creation_date: DateTime<Utc>) -> Self {
        TableMetadata {
            file_name,
            description,
            creation_date,
        }
    }
}

impl Default for TableMetadata {
    fn default() -> TableMetadata {
        Self {
            file_name: PathBuf::from("default.keymap"),
            description: "<no description specified>".to_string(),
            creation_date: Utc::now(),
        }
    }
}

impl Display for TableMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "File: {}\nDescription: {}\nCreation date: {}",
            self.file_name.display(),
            self.description,
            self.creation_date
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub struct Source {
    pub event: Event,
    pub layers: LayerSet<usize>,
}

impl Source {
    pub fn new(event: Event) -> Self {
        let mut layers = BTreeSet::new();

        layers.insert(1);

        Self {
            event,
            layers: LayerSet(layers),
        }
    }

    #[allow(unused)]
    pub fn new_with_layers(event: Event, active_layers: &[usize]) -> Self {
        let mut layers = BTreeSet::new();

        layers.extend(active_layers.iter());

        Self {
            event,
            layers: LayerSet(layers),
        }
    }

    pub fn get_layers_mut(&mut self) -> &mut BTreeSet<usize> {
        &mut self.layers.0
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Source: event: {}, on layers: [ {}]",
            self.event, self.layers
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LayerSet<T>(pub BTreeSet<T>)
where
    T: Ord + PartialOrd;

impl<T> Display for LayerSet<T>
where
    T: Display + Ord + PartialOrd,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.0.iter() {
            f.write_str(&format!("{} ", value))?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Null,
    InjectKey(Key),
    Call(Macro),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Null => f.write_str("<No action>"),

            Action::InjectKey(key) => f.write_str(&format!("Key: {}", key)),
            Action::Call(call) => f.write_str(&format!("Call: {}", call)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub struct Key {
    pub key_index: usize,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.write_str(&format!("Key idx:{}", self.key_index))
        f.write_str(&format!("{}", self.key_index))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub struct Macro {
    pub function_name: String,
}

impl Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("function {}", self.function_name))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Null,

    HidKeyDown(Key),
    HidKeyUp(Key),
    HidMouseDown(Key),
    HidMouseUp(Key),

    EasyShiftKeyDown(Key),
    EasyShiftKeyUp(Key),
    EasyShiftMouseDown(Key),
    EasyShiftMouseUp(Key),
    EasyShiftMouseWheel(Direction),

    SimpleKeyDown(Key),
    SimpleKeyUp(Key),
    SimpleMouseDown(Key),
    SimpleMouseUp(Key),
    SimpleMouseWheel(Direction),
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Null => f.write_str("<No action>"),

            Event::HidKeyDown(key) => f.write_str(&format!("hid+key-down: {}", key)),
            Event::HidKeyUp(key) => f.write_str(&format!("hid+key-up: {}", key)),
            Event::HidMouseDown(key) => f.write_str(&format!("hid+mouse-down: {}", key)),
            Event::HidMouseUp(key) => f.write_str(&format!("hid+mouse-up: {}", key)),

            Event::EasyShiftKeyDown(key) => f.write_str(&format!("es+key-down: {}", key)),
            Event::EasyShiftKeyUp(key) => f.write_str(&format!("es+key-up: {}", key)),
            Event::EasyShiftMouseDown(key) => f.write_str(&format!("es+mouse-down: {}", key)),
            Event::EasyShiftMouseUp(key) => f.write_str(&format!("es+mouse-up: {}", key)),
            Event::EasyShiftMouseWheel(direction) => {
                f.write_str(&format!("es+mouse-wheel: {}", direction))
            }

            Event::SimpleKeyDown(key) => f.write_str(&format!("key-down: {}", key)),
            Event::SimpleKeyUp(key) => f.write_str(&format!("key-up: {}", key)),
            Event::SimpleMouseDown(key) => f.write_str(&format!("mouse-down: {}", key)),
            Event::SimpleMouseUp(key) => f.write_str(&format!("mouse-up: {}", key)),
            Event::SimpleMouseWheel(direction) => {
                f.write_str(&format!("mouse-wheel: {}", direction))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Direction::Up => f.write_str("Up"),
            Direction::Down => f.write_str("Down"),
            Direction::Left => f.write_str("Left"),
            Direction::Right => f.write_str("Right"),
        }
    }
}

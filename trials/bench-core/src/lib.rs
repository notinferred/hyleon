//! A stand-in for Hyleon's core at the end-case scale, for the T-02 comparison. It generates a
//! made-up building, checks it, applies commands and draws a sheet per page, so every UI stack is
//! measured doing the same work. It does no I/O and builds for wasm. T-03 builds the real core;
//! nothing here carries over.

use std::collections::HashMap;
use std::fmt::Write;

use serde::{Deserialize, Serialize};

/// The part every drag scenario moves: it has [`HUB_LINKS`] signal connections.
pub const HUB: u32 = 0;
pub const HUB_LINKS: u32 = 20;
/// The part every edit scenario re-addresses.
pub const EDITED: u32 = 1;

const DEVICES_PER_BUS: usize = 30;
const PAGE_COLUMNS: usize = 10;
const TILE_WIDTH: f32 = 1000.0;
const TILE_HEIGHT: f32 = 700.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Size {
    Panel,
    Building,
    /// For the capacity test: any number of parts (a multiple of 50), each with 4 connections and
    /// 50 parts to a page.
    Parts(u32),
}

impl Size {
    pub fn parse(name: &str) -> Option<Size> {
        match name {
            "panel" => Some(Size::Panel),
            "building" => Some(Size::Building),
            n => n
                .parse::<u32>()
                .ok()
                .map(|n| Size::Parts((n / 50).max(1) * 50)),
        }
    }

    pub fn name(self) -> String {
        match self {
            Size::Panel => "panel".to_string(),
            Size::Building => "building".to_string(),
            Size::Parts(n) => n.to_string(),
        }
    }

    pub fn parts(self) -> usize {
        match self {
            Size::Panel => 500,
            Size::Building => 5_000,
            Size::Parts(n) => n as usize,
        }
    }

    pub fn connections(self) -> usize {
        match self {
            Size::Panel => 2_000,
            Size::Building => 20_000,
            Size::Parts(n) => 4 * n as usize,
        }
    }

    pub fn pages(self) -> usize {
        match self {
            Size::Panel => 20,
            Size::Building => 100,
            Size::Parts(n) => n as usize / 50,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Feeder,
    Drive,
    Sensor,
    Valve,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Feeder => "Feeder",
            Kind::Drive => "Drive",
            Kind::Sensor => "Sensor",
            Kind::Valve => "Valve",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Part {
    pub id: u32,
    pub tag: String,
    pub kind: Kind,
    pub page: u16,
    /// Position on the whole-building diagram.
    pub x: f32,
    pub y: f32,
    pub bus: u32,
    pub address: u8,
    pub terminated: bool,
    /// The feeder part on the same page.
    pub feeder: u32,
    pub load_kw: f32,
    pub load_confirmed: bool,
    /// Only feeders have a limit.
    pub limit_kw: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Link {
    Power,
    Bus,
    Signal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    pub id: u32,
    pub from: u32,
    pub to: u32,
    pub link: Link,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub size: Size,
    pub pages: u16,
    pub parts: Vec<Part>,
    pub connections: Vec<Connection>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    /// Stable across reruns, so lists can be keyed by it.
    pub id: String,
    pub severity: Severity,
    pub what: String,
    pub place: String,
    pub why: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Move { part: u32, x: f32, y: f32 },
    SetAddress { part: u32, address: u8 },
}

/// What the diagram draws: every part and wire, without the rest of the model. It's what crosses
/// the bridge when the core runs outside the UI.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub pages: u16,
    pub width: f32,
    pub height: f32,
    pub parts: Vec<ScenePart>,
    pub wires: Vec<SceneWire>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScenePart {
    pub id: u32,
    pub tag: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneWire {
    pub id: u32,
    pub from: u32,
    pub to: u32,
}

/// One line of the parts-and-connections list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Row {
    pub key: String,
    pub tag: String,
    pub kind: String,
    pub page: u16,
    pub detail: String,
}

/// SplitMix64: small, fast and the same on every platform, so every stack gets identical data.
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
}

impl Project {
    /// A made-up building. The same size and seed always give the same project.
    pub fn generate(size: Size, seed: u64) -> Project {
        let mut random = Random(seed);
        let (parts_n, pages) = (size.parts(), size.pages());
        let per_page = parts_n / pages;
        let tiles = (pages as f32).sqrt().ceil() as usize;

        let mut parts: Vec<Part> = (0..parts_n)
            .map(|i| {
                let page = i / per_page;
                let local = i % per_page;
                let (tile_x, tile_y) = (page % tiles, page / tiles);
                let x = tile_x as f32 * TILE_WIDTH + 40.0 + (local % PAGE_COLUMNS) as f32 * 95.0;
                let y = tile_y as f32 * TILE_HEIGHT + 40.0 + (local / PAGE_COLUMNS) as f32 * 110.0;
                let kind = match (local, i % 3) {
                    (0, _) => Kind::Feeder,
                    (_, 0) => Kind::Drive,
                    (_, 1) => Kind::Sensor,
                    _ => Kind::Valve,
                };
                let bus = i / DEVICES_PER_BUS;
                let mut address = (i % DEVICES_PER_BUS) as u8 + 1;
                if i % 97 == 5 {
                    address -= 1; // the same address as the previous device on its bus
                }
                Part {
                    id: i as u32,
                    tag: format!("{}-{:04}", &kind.name()[..1], i + 1),
                    kind,
                    page: page as u16,
                    x,
                    y,
                    bus: bus as u32,
                    address,
                    terminated: i % DEVICES_PER_BUS == DEVICES_PER_BUS - 1 && bus % 7 != 3,
                    feeder: (page * per_page) as u32,
                    load_kw: 0.37 + random.unit() * 21.6,
                    load_confirmed: i % 5 != 0,
                    limit_kw: 0.0,
                }
            })
            .collect();

        // Each feeder's limit is near its page's load; every third page is overloaded.
        for page in 0..pages {
            let range = page * per_page..(page + 1) * per_page;
            let load: f32 = parts[range.clone()].iter().map(|p| p.load_kw).sum();
            parts[range.start].limit_kw = load * if page % 3 == 0 { 0.8 } else { 1.25 };
        }

        let mut connections = Vec::with_capacity(size.connections());
        let link = |from: usize, to: usize, kind: Link, connections: &mut Vec<Connection>| {
            let id = connections.len() as u32;
            connections.push(Connection {
                id,
                from: from as u32,
                to: to as u32,
                link: kind,
            });
        };
        for to in 1..=HUB_LINKS as usize {
            link(HUB as usize, to, Link::Signal, &mut connections);
        }
        for (i, part) in parts.iter().enumerate() {
            if part.kind != Kind::Feeder {
                link(part.feeder as usize, i, Link::Power, &mut connections);
            }
            if i % DEVICES_PER_BUS != 0 {
                link(i - 1, i, Link::Bus, &mut connections);
            }
        }
        while connections.len() < size.connections() {
            let page = random.below(pages);
            let from = page * per_page + random.below(per_page);
            let to = page * per_page + random.below(per_page);
            if from != to {
                link(from, to, Link::Signal, &mut connections);
            }
        }

        Project {
            size,
            pages: pages as u16,
            parts,
            connections,
        }
    }

    pub fn apply(&mut self, command: Command) {
        match command {
            Command::Move { part, x, y } => {
                if let Some(part) = self.parts.get_mut(part as usize) {
                    part.x = x;
                    part.y = y;
                }
            }
            Command::SetAddress { part, address } => {
                if let Some(part) = self.parts.get_mut(part as usize) {
                    part.address = address;
                }
            }
        }
    }

    /// Every check, over the whole model, as Hyleon reruns them on every change.
    pub fn check(&self) -> Vec<Finding> {
        let mut findings = Vec::new();

        let mut addresses: HashMap<(u32, u8), Vec<&Part>> = HashMap::new();
        let mut terminated: HashMap<u32, bool> = HashMap::new();
        let mut load: HashMap<u32, f32> = HashMap::new();
        for part in &self.parts {
            addresses
                .entry((part.bus, part.address))
                .or_default()
                .push(part);
            *terminated.entry(part.bus).or_default() |= part.terminated;
            *load.entry(part.feeder).or_default() += part.load_kw;
        }
        let mut duplicates: Vec<_> = addresses.into_iter().filter(|(_, p)| p.len() > 1).collect();
        duplicates.sort_by_key(|((bus, address), _)| (*bus, *address));
        for ((bus, address), parts) in duplicates {
            let tags: Vec<&str> = parts.iter().map(|p| p.tag.as_str()).collect();
            findings.push(Finding {
                id: format!("duplicate-{bus}-{address}"),
                severity: Severity::Error,
                what: format!("Bus address {address} is used {} times.", parts.len()),
                place: format!("Bus {bus}: {}", tags.join(", ")),
                why: "Each device on a Modbus line needs its own address.".to_string(),
            });
        }
        let mut unterminated: Vec<u32> = terminated
            .into_iter()
            .filter(|(_, t)| !t)
            .map(|(bus, _)| bus)
            .collect();
        unterminated.sort_unstable();
        for bus in unterminated {
            findings.push(Finding {
                id: format!("unterminated-{bus}"),
                severity: Severity::Warning,
                what: "The bus isn't terminated at its last device.".to_string(),
                place: format!("Bus {bus}"),
                why: "An unterminated RS-485 line reflects signals.".to_string(),
            });
        }
        for part in self.parts.iter().filter(|p| p.kind == Kind::Feeder) {
            let used = load.get(&part.id).copied().unwrap_or_default();
            if used > part.limit_kw {
                findings.push(Finding {
                    id: format!("overloaded-{}", part.id),
                    severity: Severity::Error,
                    what: format!(
                        "The feeder carries {used:.1} kW, over its {:.1} kW limit.",
                        part.limit_kw
                    ),
                    place: part.tag.clone(),
                    why: "A feeder's load can't exceed what its protection allows.".to_string(),
                });
            }
        }
        let mut wired = vec![false; self.parts.len()];
        for c in self.connections.iter().filter(|c| c.link == Link::Signal) {
            wired[c.from as usize] = true;
            wired[c.to as usize] = true;
        }
        for part in &self.parts {
            if !part.load_confirmed {
                findings.push(Finding {
                    id: format!("unconfirmed-{}", part.id),
                    severity: Severity::Warning,
                    what: format!("The load of {:.1} kW isn't confirmed.", part.load_kw),
                    place: part.tag.clone(),
                    why: "The value came from an agent and is marked inferred.".to_string(),
                });
            }
            if part.kind == Kind::Sensor && !wired[part.id as usize] {
                findings.push(Finding {
                    id: format!("unwired-{}", part.id),
                    severity: Severity::Info,
                    what: "The sensor's alarm output isn't wired.".to_string(),
                    place: part.tag.clone(),
                    why: "Every sensor reports an alarm to the controller.".to_string(),
                });
            }
        }
        findings
    }

    /// The sheet for one page, as SVG: the stand-in for the future `render` crate.
    pub fn sheet_svg(&self, page: u16) -> String {
        let per_page = self.parts.len() / self.pages as usize;
        let first = page as usize * per_page;
        let on_page = first..first + per_page;
        let local = |id: u32| {
            let k = id as usize - first;
            (15.0 + (k % 5) as f32 * 80.0, 25.0 + (k / 5) as f32 * 26.0)
        };
        let mut svg = String::with_capacity(32 * 1024);
        let _ = write!(
            svg,
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 420 297" role="img" aria-label="Sheet {} of {}"><rect width="420" height="297" fill="#fff"/><rect x="5" y="5" width="410" height="287" fill="none" stroke="#000" stroke-width="0.5"/><text x="300" y="288" font-size="5">Sheet {} of {}</text><g stroke="#000" stroke-width="0.25" font-size="3.4" font-family="sans-serif">"##,
            page + 1,
            self.pages,
            page + 1,
            self.pages
        );
        for c in &self.connections {
            if on_page.contains(&(c.from as usize)) && on_page.contains(&(c.to as usize)) {
                let ((x1, y1), (x2, y2)) = (local(c.from), local(c.to));
                let _ = write!(
                    svg,
                    r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}"/>"#,
                    x1 + 30.0,
                    y1 + 7.0,
                    x2 + 30.0,
                    y2 + 7.0
                );
            }
        }
        for part in &self.parts[on_page] {
            let (x, y) = local(part.id);
            let _ = write!(
                svg,
                r##"<g transform="translate({x:.1} {y:.1})"><rect width="60" height="14" fill="#fff"/><text x="2" y="5" stroke="none">{}</text><text x="2" y="11" stroke="none">{} @{}</text></g>"##,
                part.tag,
                part.kind.name(),
                part.address
            );
        }
        svg.push_str("</g></svg>");
        svg
    }

    /// The first `limit` rows matching `filter` (any case), and how many match in all.
    pub fn rows(&self, filter: &str, limit: usize) -> (Vec<Row>, usize) {
        let filter = filter.to_lowercase();
        let matches = |text: &str| filter.is_empty() || text.to_lowercase().contains(&filter);
        let mut rows = Vec::with_capacity(limit);
        let mut total = 0;
        for part in &self.parts {
            if matches(&part.tag) {
                total += 1;
                if rows.len() < limit {
                    rows.push(Row {
                        key: format!("p{}", part.id),
                        tag: part.tag.clone(),
                        kind: part.kind.name().to_string(),
                        page: part.page + 1,
                        detail: format!(
                            "bus {} @{}, {:.1} kW",
                            part.bus, part.address, part.load_kw
                        ),
                    });
                }
            }
        }
        for c in &self.connections {
            let tag = format!("W-{:05}", c.id + 1);
            if matches(&tag) {
                total += 1;
                if rows.len() < limit {
                    let (from, to) = (&self.parts[c.from as usize], &self.parts[c.to as usize]);
                    rows.push(Row {
                        key: format!("c{}", c.id),
                        tag,
                        kind: format!("{:?}", c.link),
                        page: from.page + 1,
                        detail: format!("{} to {}", from.tag, to.tag),
                    });
                }
            }
        }
        (rows, total)
    }

    /// The whole-building diagram's size: a tile per page, in a square grid.
    pub fn diagram_size(&self) -> (f32, f32) {
        let tiles = (self.pages as f32).sqrt().ceil();
        (tiles * TILE_WIDTH, tiles * TILE_HEIGHT)
    }

    pub fn scene(&self) -> Scene {
        let (width, height) = self.diagram_size();
        Scene {
            pages: self.pages,
            width,
            height,
            parts: self
                .parts
                .iter()
                .map(|p| ScenePart {
                    id: p.id,
                    tag: p.tag.clone(),
                    x: p.x,
                    y: p.y,
                })
                .collect(),
            wires: self
                .connections
                .iter()
                .map(|c| SceneWire {
                    id: c.id,
                    from: c.from,
                    to: c.to,
                })
                .collect(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("a project always serializes")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Project, String> {
        serde_json::from_slice(bytes).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_exact_and_repeatable() {
        for size in [Size::Panel, Size::Building] {
            let project = Project::generate(size, 1);
            assert_eq!(project.parts.len(), size.parts());
            assert_eq!(project.connections.len(), size.connections());
            assert_eq!(project, Project::generate(size, 1));
            let hub_links = project.connections.iter().filter(|c| c.from == HUB).count();
            assert!(hub_links >= HUB_LINKS as usize);
        }
    }

    #[test]
    fn findings_cover_every_check() {
        let findings = Project::generate(Size::Building, 1).check();
        for prefix in ["duplicate-", "unterminated-", "overloaded-", "unconfirmed-"] {
            assert!(
                findings.iter().any(|f| f.id.starts_with(prefix)),
                "{prefix}"
            );
        }
        assert!(findings.len() > 1_000, "{} findings", findings.len());
    }

    #[test]
    fn an_edit_changes_the_findings() {
        let mut project = Project::generate(Size::Panel, 1);
        let before = project.check().len();
        let taken = project.parts[EDITED as usize + 1].address;
        project.apply(Command::SetAddress {
            part: EDITED,
            address: taken,
        });
        assert_eq!(project.check().len(), before + 1);
    }

    #[test]
    fn sheets_rows_and_bytes_work() {
        let project = Project::generate(Size::Panel, 1);
        assert!(project.sheet_svg(3).contains("Sheet 4 of 20"));
        let (rows, total) = project.rows("w-0001", 500);
        assert_eq!(total, 10); // W-00010 to W-00019
        assert_eq!(rows.len(), 10);
        assert_eq!(Project::from_bytes(&project.to_bytes()).unwrap(), project);
    }
}

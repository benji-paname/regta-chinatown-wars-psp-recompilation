// SPDX-License-Identifier: GPL-3.0-only
//! Chinatown Wars PSP street furniture and dynamic props (cDynamicPropManager,
//! PropLibrary, cStreetFurniture, and cPropDynamics).
//! Exact binary structures from PAK resources 1866 (prop definitions) and
//! 1867 (prop kinds and physics parameters).
use crate::{
    actors::{forward, Particle},
    collision::Collision,
    iso::{u16le, u32le, Disc, Result},
    pak::Pak,
};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum PropShape {
    Box { offset: [f32; 3], half: [f32; 3] },
    Cylinder { offset: [f32; 3], radius: f32, height: f32 },
    Sphere { offset: [f32; 3], radius: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PropKind {
    pub hit_eff: u8,
    pub smash_eff: u8,
    pub health: u8,
    pub flags: u8,
    pub mass: f32,
    pub smash_force: f32,
    pub uproot_force: f32,
    pub planar: bool,
    pub bullet_damage: bool,
    pub melee_damage: bool,
    pub inflammable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PropDef {
    pub model: u16,
    pub broken: u16,
    pub kind: i8,
    pub shapes: Vec<PropShape>,
}

#[derive(Default)]
pub struct PropLibrary {
    pub defs: Vec<PropDef>,
    pub kinds: Vec<PropKind>,
    pub models: HashMap<u16, crate::model::Mesh>,
    pub textures: HashMap<u16, crate::texture::Texture>,
    pub animations: HashMap<u16, Vec<(u16, u16)>>,
}

impl PropLibrary {
    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        Self {
            defs: Vec::new(),
            kinds: Vec::new(),
            models: HashMap::new(),
            textures: HashMap::new(),
            animations: HashMap::new(),
        }
    }

    pub fn load(disc: &mut Disc, pak: &Pak) -> Result<Self> {
        let d1867 = pak.read(disc, 1867)?.0;
        let mut kinds = Vec::new();
        let mut at = 0;
        while at + 16 <= d1867.len() {
            let end = u16le(&d1867, at + 14)?;
            if end != 0xdead {
                break;
            }
            let hit_eff = d1867[at];
            let smash_eff = d1867[at + 1];
            let health = d1867[at + 2];
            let flags = d1867[at + 3];
            let bullet_damage = d1867[at + 3] != 0;
            let melee_damage = d1867[at + 4] != 0;
            let inflammable = d1867[at + 5] != 0;
            let planar = d1867[at + 7] != 0;
            let mass = u16le(&d1867, at + 8)? as i16 as f32 / 4096.0;
            let smash_force = u16le(&d1867, at + 10)? as i16 as f32 / 4096.0;
            let uproot_force = u16le(&d1867, at + 12)? as i16 as f32 / 4096.0;
            kinds.push(PropKind {
                hit_eff,
                smash_eff,
                health,
                flags,
                mass,
                smash_force,
                uproot_force,
                planar,
                bullet_damage,
                melee_damage,
                inflammable,
            });
            at += 16;
        }

        let d1866 = pak.read(disc, 1866)?.0;
        let mut defs = Vec::new();
        let mut o = 0;
        let shape_sizes = [0, 0x1c, 0x18, 0x14, 0x10, 0x10, 0x10];
        while o + 8 <= d1866.len() {
            let magic = u32le(&d1866, o)?;
            if magic == 0xdeadbeef {
                break;
            }
            let model = u16le(&d1866, o)?;
            let broken = u16le(&d1866, o + 2)?;
            let dead = u16le(&d1866, o + 4)?;
            if dead != 0xdead {
                break;
            }
            let kind = d1866[o + 6] as i8;
            let n_shapes = d1866[o + 7] as usize;
            o += 8;
            let mut shapes = Vec::with_capacity(n_shapes);
            for _ in 0..n_shapes {
                if o + 4 > d1866.len() {
                    break;
                }
                let stype = u32le(&d1866, o)? as usize;
                if stype < 1 || stype > 6 || o + shape_sizes[stype] > d1866.len() {
                    break;
                }
                let fx = |idx: usize| -> Result<f32> {
                    Ok(u32le(&d1866, o + 4 + idx * 4)? as i32 as f32 / 4096.0)
                };
                match stype {
                    1 => {
                        let offset = [fx(0)?, fx(1)?, fx(2)?];
                        let half = [fx(3)? * 0.5, fx(4)? * 0.5, fx(5)? * 0.5];
                        shapes.push(PropShape::Box { offset, half });
                    }
                    2 => {
                        let offset = [fx(0)?, fx(1)?, fx(2)?];
                        let radius = fx(3)?;
                        let height = fx(4)?;
                        shapes.push(PropShape::Cylinder {
                            offset,
                            radius,
                            height,
                        });
                    }
                    3 => {
                        let offset = [fx(0)?, fx(1)?, fx(2)?];
                        let radius = fx(3)?;
                        shapes.push(PropShape::Sphere { offset, radius });
                    }
                    _ => {}
                }
                o += shape_sizes[stype];
            }
            defs.push(PropDef {
                model,
                broken,
                kind,
                shapes,
            });
        }

        let mut models = HashMap::new();
        let mut textures = HashMap::new();
        let mut animations = HashMap::new();

        let mut load_model = |model_id: u16| {
            if model_id == 0 || model_id >= 0xFFFE || models.contains_key(&model_id) {
                return;
            }
            if let Ok((raw, _)) = pak.read(disc, model_id as usize) {
                if let Ok(m) = crate::model::parse(&raw) {
                    for &tid in &m.texture_ids {
                        if tid != 65535 && !textures.contains_key(&tid) {
                            if let Ok((traw, _)) = pak.read(disc, tid as usize) {
                                if let Ok(Some(anim)) = crate::texture::animation(&traw) {
                                    for &(frame, _) in &anim {
                                        if !textures.contains_key(&frame) {
                                            if let Ok((fraw, _)) = pak.read(disc, frame as usize) {
                                                if let Ok(tex) = crate::texture::decode(frame as usize, &fraw) {
                                                    textures.insert(frame, tex);
                                                }
                                            }
                                        }
                                    }
                                    animations.insert(tid, anim);
                                } else if let Ok(tex) = crate::texture::decode(tid as usize, &traw) {
                                    textures.insert(tid, tex);
                                }
                            }
                        }
                    }
                    models.insert(model_id, m);
                }
            }
        };

        for def in &defs {
            load_model(def.model);
            if def.broken < 0xFFFE {
                load_model(def.broken);
            }
        }

        Ok(Self {
            defs,
            kinds,
            models,
            textures,
            animations,
        })
    }

    pub fn kind_for(&self, prop_id: usize) -> Option<&PropKind> {
        let def = self.defs.get(prop_id)?;
        if def.kind >= 0 && (def.kind as usize) < self.kinds.len() {
            Some(&self.kinds[def.kind as usize])
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PropState {
    Standing,
    Tilted,
    Loose,
    Broken,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ActiveProp {
    pub cell_x: usize,
    pub cell_y: usize,
    pub prop_idx: usize,
    pub prop_id: u16,
    pub model_id: u16,
    pub broken_model_id: u16,
    pub p: [f32; 3],
    pub angle: f32,
    pub v: [f32; 3],
    pub roll_angle: f32,
    pub tumble_axis: [f32; 3],
    pub angular_v: f32,
    pub tilt: f32,
    pub tilt_speed: f32,
    pub tilt_axis: [f32; 2],
    pub state: PropState,
    pub collidable: bool,
    pub fountain: bool,
    pub fire_timer: f32,
    pub health: u8,
    pub mass: f32,
    pub smash_eff: u8,
    pub smash_force: f32,
    pub uproot_force: f32,
    pub planar: bool,
    pub bullet_damage: bool,
    pub melee_damage: bool,
    pub inflammable: bool,
    pub base_z: f32,
    pub sleep: bool,
}

pub struct PropsManager {
    pub library: Option<std::sync::Arc<PropLibrary>>,
    pub active: Vec<ActiveProp>,
    pub pending_smashes: Vec<([f32; 3], usize)>,
    pub pending_disables: Vec<(usize, usize, usize)>,
    pub pending_explosions: Vec<([f32; 3], crate::explosions::ExplosionType, f32)>,
    pub pending_fires: Vec<([f32; 3], f32, f32)>,
}

impl PropsManager {
    pub fn new() -> Self {
        Self {
            library: None,
            active: Vec::new(),
            pending_smashes: Vec::new(),
            pending_disables: Vec::new(),
            pending_explosions: Vec::new(),
            pending_fires: Vec::new(),
        }
    }

    pub fn with_library(library: std::sync::Arc<PropLibrary>) -> Self {
        Self {
            library: Some(library),
            active: Vec::new(),
            pending_smashes: Vec::new(),
            pending_disables: Vec::new(),
            pending_explosions: Vec::new(),
            pending_fires: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn init(&mut self, disc: &mut Disc, pak: &Pak) -> Result<()> {
        self.library = Some(std::sync::Arc::new(PropLibrary::load(disc, pak)?));
        Ok(())
    }

    pub fn drain_pending_smashes(&mut self) -> Vec<([f32; 3], usize)> {
        std::mem::take(&mut self.pending_smashes)
    }

    pub fn drain_pending_disables(&mut self) -> Vec<(usize, usize, usize)> {
        std::mem::take(&mut self.pending_disables)
    }

    pub fn drain_pending_explosions(&mut self) -> Vec<([f32; 3], crate::explosions::ExplosionType, f32)> {
        std::mem::take(&mut self.pending_explosions)
    }

    pub fn drain_pending_fires(&mut self) -> Vec<([f32; 3], f32, f32)> {
        std::mem::take(&mut self.pending_fires)
    }

    /// Synchronizes street furniture and props within the streaming bubble around focus.
    /// All city props (both static street furniture and destructibles) are loaded.
    pub fn sync_area(&mut self, collision: &Collision, focus: [f32; 3]) {
        let Some(lib) = &self.library else {
            return;
        };
        let Some((cx, cy)) = Collision::cell_xy(focus[0], focus[1]) else {
            return;
        };
        for x in cx.saturating_sub(2)..=(cx + 2).min(139) {
            for y in cy.saturating_sub(2)..=(cy + 2).min(99) {
                let cell = &collision.cells[x * 100 + y];
                for (idx, p) in cell.props.iter().enumerate() {
                    let prop_id = p.prop as usize;
                    if prop_id >= lib.defs.len() {
                        continue;
                    }
                    let def = &lib.defs[prop_id];
                    let kind = lib.kind_for(prop_id);
                    let (mass, smash_eff, smash_force, uproot_force, health, planar, bullet_damage, melee_damage, inflammable) =
                        if let Some(k) = kind {
                            (
                                k.mass,
                                k.smash_eff,
                                k.smash_force,
                                k.uproot_force,
                                k.health,
                                k.planar,
                                k.bullet_damage,
                                k.melee_damage,
                                k.inflammable,
                            )
                        } else {
                            (1.0, 0, -1.0, -1.0, 15, false, true, true, false)
                        };
                    if self
                        .active
                        .iter()
                        .any(|ap| ap.cell_x == x && ap.cell_y == y && ap.prop_idx == idx)
                    {
                        continue;
                    }
                    let angle = p.angle();
                    self.active.push(ActiveProp {
                        cell_x: x,
                        cell_y: y,
                        prop_idx: idx,
                        prop_id: p.prop,
                        model_id: def.model,
                        broken_model_id: def.broken,
                        p: p.p,
                        angle,
                        v: [0.0; 3],
                        roll_angle: 0.0,
                        tumble_axis: [1.0, 0.0, 0.0],
                        angular_v: 0.0,
                        tilt: 0.0,
                        tilt_speed: 0.0,
                        tilt_axis: [0.0, 0.0],
                        state: PropState::Standing,
                        collidable: true,
                        fountain: false,
                        fire_timer: 0.0,
                        health,
                        mass: if mass > 0.0 { mass } else { 1.0 },
                        smash_eff,
                        smash_force,
                        uproot_force,
                        planar,
                        bullet_damage,
                        melee_damage,
                        inflammable,
                        base_z: p.p[2],
                        sleep: true,
                    });
                }
            }
        }
        // Cleanup distant props beyond 130m (unless currently spewing a fountain or tumbling)
        self.active.retain(|ap| {
            ap.fountain || !ap.sleep || {
                let dx = ap.p[0] - focus[0];
                let dy = ap.p[1] - focus[1];
                dx * dx + dy * dy < 130.0 * 130.0
            }
        });
    }

    /// Resolves vehicle impact with street furniture (hydrants, bins, fences, etc.).
    #[allow(dead_code)]
    pub fn check_vehicle_impact(
        &mut self,
        car: &mut crate::actors::Car,
        dt: f32,
        particles: &mut Vec<Particle>,
    ) {
        let mut no_pickups = None;
        self.check_vehicle_impact_with_loot(car, dt, particles, &mut no_pickups);
    }

    /// Resolves vehicle impact with street furniture, spawning dropped cash or loot if available.
    pub fn check_vehicle_impact_with_loot(
        &mut self,
        car: &mut crate::actors::Car,
        _dt: f32,
        particles: &mut Vec<Particle>,
        pickups: &mut Option<&mut crate::weapons::PickupManager>,
    ) {
        let speed = car.speed.abs();
        if speed < 1.2 {
            return;
        }
        let cf = forward(car.angle);
        let cside = [cf[1], -cf[0]];
        let mass = car.spec().mass();
        let force = speed * mass * 0.11;

        let mut smash_trigger = None;
        for i in 0..self.active.len() {
            let prop = &mut self.active[i];
            if !prop.collidable && prop.state != PropState::Standing {
                continue;
            }
            let dx = prop.p[0] - car.p[0];
            let dy = prop.p[1] - car.p[1];
            let long_dist = dx * cf[0] + dy * cf[1];
            let lat_dist = (dx * cside[0] + dy * cside[1]).abs();

            if long_dist.abs() < car.h[1] + 0.65 && lat_dist < car.h[0] + 0.55 {
                let impact_dir = [cf[0] * car.speed.signum(), cf[1] * car.speed.signum()];
                let damage = (force / (prop.mass * 0.5 + 0.5)).clamp(1.0, 50.0) as u8;
                prop.health = prop.health.saturating_sub(damage);

                let breakable = prop.smash_force >= 0.0 || prop.uproot_force >= 0.0;
                if !breakable {
                    // Indestructible prop (solid street pole, tree trunk, concrete pillar)
                    car.speed *= 0.55;
                    continue;
                }

                let can_uproot = prop.uproot_force >= 0.0 && (force >= prop.uproot_force || speed >= 2.2);
                let can_smash = prop.smash_force >= 0.0 && (force >= prop.smash_force || speed >= 2.2);

                if can_uproot || can_smash || prop.health == 0 || prop.smash_eff > 0 {
                    let prop_mass = prop.mass;
                    smash_trigger = Some((i, impact_dir, prop_mass));
                    break;
                }
            }
        }
        if let Some((i, impact_dir, prop_mass)) = smash_trigger {
            self.apply_smash(i, impact_dir, speed, particles, pickups);
            car.speed *= (1.0 - 0.06 * prop_mass.min(2.5)).max(0.72);
            if prop_mass > 1.5 && speed > 10.0 {
                car.health = (car.health - 2.0).max(0.0);
            }
        }
    }

    /// Triggers authentic smash effect, debris particles, audio events and loot drops.
    pub fn apply_smash(
        &mut self,
        idx: usize,
        impact_dir: [f32; 2],
        impact_speed: f32,
        particles: &mut Vec<Particle>,
        pickups: &mut Option<&mut crate::weapons::PickupManager>,
    ) {
        let prop = &mut self.active[idx];
        if prop.state == PropState::Broken {
            return;
        }
        self.pending_smashes.push((prop.p, prop.smash_eff as usize));
        self.pending_disables.push((prop.cell_x, prop.cell_y, prop.prop_idx));

        match prop.smash_eff {
            7 => {
                // Lamp post / streetlight: authentic Chinatown Wars toppling physics
                // Base snaps and pole topples under gravity until lying flat on pavement.
                // Streetlights NEVER fly up into the air or perform barrel rolls.
                prop.tilt = 0.08;
                prop.tilt_speed = (impact_speed * 0.15 + 2.8).min(5.5);
                prop.tilt_axis = [-impact_dir[1], impact_dir[0]];
                prop.state = PropState::Tilted;
                prop.collidable = false;
                prop.sleep = false;
                for i in 0..12 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 2.5],
                            v: [
                                ((i * 41) % 11) as f32 * 0.4 - 2.0,
                                ((i * 67) % 11) as f32 * 0.4 - 2.0,
                                ((i * 19) % 7) as f32 * 0.5 + 1.0,
                            ],
                            life: 0.0,
                            max_life: 0.45,
                            size: 0.12,
                            grow: 0.1,
                            kind: 5, // electric sparks
                        });
                    }
                }
            }
            8 => {
                // Large streetlight / Traffic signal: tilts flat immediately (0x3C72 ~ 1.48 rad ~ 85 deg)
                prop.tilt = 1.48;
                prop.tilt_speed = 0.0;
                prop.tilt_axis = [-impact_dir[1], impact_dir[0]];
                prop.state = PropState::Tilted;
                prop.collidable = false;
                prop.sleep = true;
                for i in 0..12 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 2.8],
                            v: [
                                ((i * 41) % 11) as f32 * 0.4 - 2.0,
                                ((i * 67) % 11) as f32 * 0.4 - 2.0,
                                ((i * 19) % 7) as f32 * 0.5 + 1.0,
                            ],
                            life: 0.0,
                            max_life: 0.45,
                            size: 0.12,
                            grow: 0.1,
                            kind: 5, // electric sparks
                        });
                    }
                }
            }
            9 => {
                // Fire Hydrant (tilt 24 degrees, indefinite water geyser fountain)
                prop.tilt = 0.42;
                prop.tilt_axis = [-impact_dir[1], impact_dir[0]];
                prop.state = PropState::Tilted;
                prop.collidable = false;
                prop.fountain = true;
            }
            10 => {
                // Glass phone booth: shatters with glass fragments
                prop.state = PropState::Broken;
                prop.collidable = false;
                for i in 0..16 {
                    if particles.len() < 160 {
                        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                        particles.push(Particle {
                            p: [prop.p[0] + sign * 0.35, prop.p[1] + (i as f32 * 0.08 - 0.6), prop.p[2] + 1.2],
                            v: [
                                impact_dir[0] * 2.5 + ((i * 29) % 7) as f32 * 0.6 - 1.8,
                                impact_dir[1] * 2.5 + ((i * 47) % 7) as f32 * 0.6 - 1.8,
                                1.5 + (i as f32) * 0.25,
                            ],
                            life: 0.0,
                            max_life: 0.65,
                            size: 0.20,
                            grow: 0.15,
                            kind: 6, // glass shards
                        });
                    }
                }
            }
            41 => {
                // Bus stop shelter: wood/metal frame splinters + 6 glass bursts along shelter width
                prop.state = PropState::Broken;
                prop.collidable = false;
                for i in 0..12 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.8],
                            v: [
                                impact_dir[0] * 3.0 + ((i * 31) % 7) as f32 * 0.5 - 1.5,
                                impact_dir[1] * 3.0 + ((i * 53) % 7) as f32 * 0.5 - 1.5,
                                2.0 + (i as f32) * 0.3,
                            ],
                            life: 0.0,
                            max_life: 0.75,
                            size: 0.26,
                            grow: 0.3,
                            kind: 3, // wood frame splinters
                        });
                    }
                }
                for i in 0..16 {
                    if particles.len() < 160 {
                        let offset = (i as f32 - 8.0) * 0.15;
                        particles.push(Particle {
                            p: [prop.p[0] - impact_dir[1] * offset, prop.p[1] + impact_dir[0] * offset, prop.p[2] + 1.1],
                            v: [
                                impact_dir[0] * 2.8 + ((i * 41) % 5) as f32 * 0.5 - 1.0,
                                impact_dir[1] * 2.8 + ((i * 67) % 5) as f32 * 0.5 - 1.0,
                                1.8 + (i as f32) * 0.2,
                            ],
                            life: 0.0,
                            max_life: 0.70,
                            size: 0.22,
                            grow: 0.15,
                            kind: 6, // glass shards
                        });
                    }
                }
            }
            21 => {
                // Petrol pump: MASSIVE EXPLOSION + persistent ground fire!
                prop.state = PropState::Broken;
                prop.collidable = false;
                self.pending_explosions.push((prop.p, crate::explosions::ExplosionType::BigVehicle, 1.25));
                self.pending_fires.push((prop.p, 3.5, 30.0));
                for i in 0..12 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.5],
                            v: [
                                ((i * 37) % 9) as f32 * 0.6 - 2.4,
                                ((i * 59) % 9) as f32 * 0.6 - 2.4,
                                3.0 + (i as f32) * 0.4,
                            ],
                            life: 0.0,
                            max_life: 0.90,
                            size: 0.45,
                            grow: 1.2,
                            kind: 7, // fire flame
                        });
                    }
                }
            }
            23 | 30 => {
                // Hotdog cart: propane gas & fire blast!
                prop.state = PropState::Broken;
                prop.collidable = false;
                prop.fire_timer = 5.0;
                let is_hard = impact_speed >= 4.5;
                if is_hard {
                    self.pending_explosions.push((prop.p, crate::explosions::ExplosionType::Barrel, 0.9));
                    self.pending_fires.push((prop.p, 2.2, 8.0));
                } else {
                    self.pending_explosions.push((prop.p, crate::explosions::ExplosionType::Generic, 0.65));
                    self.pending_fires.push((prop.p, 1.5, 4.0));
                }
                for i in 0..10 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.6],
                            v: [
                                impact_dir[0] * 3.5 + ((i * 23) % 7) as f32 * 0.5 - 1.5,
                                impact_dir[1] * 3.5 + ((i * 43) % 7) as f32 * 0.5 - 1.5,
                                2.2 + (i as f32) * 0.35,
                            ],
                            life: 0.0,
                            max_life: 0.85,
                            size: 0.35,
                            grow: 0.8,
                            kind: 7, // flame jet
                        });
                    }
                }
            }
            28 => {
                // Propane cylinders / explosive barrels: blast detonation
                prop.state = PropState::Broken;
                prop.collidable = false;
                self.pending_explosions.push((prop.p, crate::explosions::ExplosionType::Barrel, 1.0));
                for i in 0..10 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.5],
                            v: [
                                ((i * 31) % 9) as f32 * 0.8 - 3.2,
                                ((i * 53) % 9) as f32 * 0.8 - 3.2,
                                2.5 + (i as f32) * 0.35,
                            ],
                            life: 0.0,
                            max_life: 0.70,
                            size: 0.40,
                            grow: 0.9,
                            kind: 7, // flame fireball
                        });
                    }
                }
            }
            15 | 25 | 31 => {
                // Street signs, parking signs, small posts: tilt 30 degrees (0.52 rad)
                prop.tilt = 0.52;
                prop.tilt_axis = [-impact_dir[1], impact_dir[0]];
                prop.state = PropState::Tilted;
                prop.collidable = false;
                prop.sleep = true;
            }
            38 => {
                // Medium street sign / pole: tilt 45 degrees
                prop.tilt = 0.78;
                prop.tilt_axis = [-impact_dir[1], impact_dir[0]];
                prop.state = PropState::Tilted;
                prop.collidable = false;
                prop.sleep = true;
            }
            13 | 37 => {
                // Rubbish bins (stationary): garbage debris burst
                prop.state = PropState::Broken;
                prop.collidable = false;
                for i in 0..8 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.4],
                            v: [
                                impact_dir[0] * 2.2 + ((i * 37) % 7) as f32 * 0.4 - 1.2,
                                impact_dir[1] * 2.2 + ((i * 53) % 7) as f32 * 0.4 - 1.2,
                                1.8 + (i as f32) * 0.3,
                            ],
                            life: 0.0,
                            max_life: 0.80,
                            size: 0.22,
                            grow: 0.3,
                            kind: 2, // garbage debris
                        });
                    }
                }
            }
            35 => {
                // Trash can / Dumpster: becomes Loose physical body!
                prop.state = PropState::Loose;
                prop.collidable = false;
                prop.sleep = false;
                prop.v = [
                    impact_dir[0] * (impact_speed * 0.65 + 1.5),
                    impact_dir[1] * (impact_speed * 0.65 + 1.5),
                    3.2 + (impact_speed * 0.15).min(2.5),
                ];
                prop.tumble_axis = [-impact_dir[1], impact_dir[0], 0.3];
                prop.angular_v = impact_speed * 0.8 + 3.0;
                // Emit garbage debris
                for i in 0..8 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.4],
                            v: [
                                impact_dir[0] * 2.8 + ((i * 37) % 7) as f32 * 0.4 - 1.2,
                                impact_dir[1] * 2.8 + ((i * 53) % 7) as f32 * 0.4 - 1.2,
                                2.0 + (i as f32) * 0.35,
                            ],
                            life: 0.0,
                            max_life: 0.85,
                            size: 0.24,
                            grow: 0.35,
                            kind: 2, // garbage debris
                        });
                    }
                }
            }
            16 | 26 | 27 => {
                // Wooden barrier / fence / palisade: shatters into wood splinters
                prop.state = PropState::Broken;
                prop.collidable = false;
                for i in 0..10 {
                    if particles.len() < 160 {
                        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.5],
                            v: [
                                impact_dir[0] * 3.2 - impact_dir[1] * sign * 1.6,
                                impact_dir[1] * 3.2 + impact_dir[0] * sign * 1.6,
                                2.5 + (i as f32) * 0.3,
                            ],
                            life: 0.0,
                            max_life: 0.70,
                            size: 0.28,
                            grow: 0.35,
                            kind: 3, // wood slat splinters
                        });
                    }
                }
            }
            14 => {
                // USPS Mailbox (blue street postbox): authentic Chinatown Wars loose physical body!
                // Launches forward, tumbles in 3D, bounces along pavement and comes to rest on its side.
                prop.state = PropState::Loose;
                prop.collidable = false;
                prop.sleep = false;
                prop.v = [
                    impact_dir[0] * (impact_speed * 0.65 + 1.8),
                    impact_dir[1] * (impact_speed * 0.65 + 1.8),
                    2.8 + (impact_speed * 0.12).min(2.2),
                ];
                prop.tumble_axis = [-impact_dir[1], impact_dir[0], 0.25];
                prop.angular_v = impact_speed * 0.8 + 4.0;
                // Emit letters and mail envelopes: alternating blue USPS mail (kind 8) and white envelopes (kind 4)
                for i in 0..12 {
                    if particles.len() < 160 {
                        let k = if i % 2 == 0 { 8 } else { 4 };
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.6],
                            v: [
                                impact_dir[0] * 2.2 + ((i * 31) % 7) as f32 * 0.4 - 1.2,
                                impact_dir[1] * 2.2 + ((i * 53) % 7) as f32 * 0.4 - 1.2,
                                2.0 + (i as f32) * 0.25,
                            ],
                            life: 0.0,
                            max_life: 1.1,
                            size: 0.22,
                            grow: 0.2,
                            kind: k,
                        });
                    }
                }
            }
            22 => {
                // Newspaper vending box: authentic Chinatown Wars loose physical body!
                // Launches forward, tumbles in 3D, bounces along pavement, leaves newsprint flutter and cash!
                prop.state = PropState::Loose;
                prop.collidable = false;
                prop.sleep = false;
                prop.v = [
                    impact_dir[0] * (impact_speed * 0.65 + 1.8),
                    impact_dir[1] * (impact_speed * 0.65 + 1.8),
                    2.8 + (impact_speed * 0.12).min(2.2),
                ];
                prop.tumble_axis = [-impact_dir[1], impact_dir[0], 0.25];
                prop.angular_v = impact_speed * 0.8 + 4.0;
                // Emit fluttering white newsprint pages (kind 4)
                for i in 0..12 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.6],
                            v: [
                                impact_dir[0] * 2.2 + ((i * 29) % 9) as f32 * 0.35 - 1.4,
                                impact_dir[1] * 2.2 + ((i * 47) % 9) as f32 * 0.35 - 1.4,
                                2.0 + (i as f32) * 0.25,
                            ],
                            life: 0.0,
                            max_life: 1.1,
                            size: 0.22,
                            grow: 0.2,
                            kind: 4, // fluttering newspaper pages
                        });
                    }
                }
                // Smashed newspaper dispenser drops authentic $25 cash
                if let Some(pm) = pickups {
                    pm.spawn_dropped(crate::weapons::PickupKind::Money(25), prop.p);
                }
            }
            42 => {
                // Wooden newspaper stand: wood splinters + paper flutter + cash
                prop.state = PropState::Broken;
                prop.collidable = false;
                for i in 0..6 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.5],
                            v: [
                                impact_dir[0] * 2.8 + ((i * 37) % 5) as f32 * 0.4 - 0.8,
                                impact_dir[1] * 2.8 + ((i * 59) % 5) as f32 * 0.4 - 0.8,
                                2.0 + (i as f32) * 0.3,
                            ],
                            life: 0.0,
                            max_life: 0.70,
                            size: 0.26,
                            grow: 0.3,
                            kind: 3, // wood splinters
                        });
                    }
                }
                for i in 0..8 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.6],
                            v: [
                                impact_dir[0] * 2.0 + ((i * 29) % 7) as f32 * 0.3 - 0.9,
                                impact_dir[1] * 2.0 + ((i * 47) % 7) as f32 * 0.3 - 0.9,
                                1.8 + (i as f32) * 0.25,
                            ],
                            life: 0.0,
                            max_life: 1.1,
                            size: 0.22,
                            grow: 0.2,
                            kind: 4, // paper flutter
                        });
                    }
                }
                if let Some(pm) = pickups {
                    pm.spawn_dropped(crate::weapons::PickupKind::Money(25), prop.p);
                }
            }
            17 => {
                // Parking meter: loose physical body, drops $40 cash
                prop.state = PropState::Loose;
                prop.collidable = false;
                prop.sleep = false;
                prop.v = [
                    impact_dir[0] * (impact_speed * 0.5 + 1.2),
                    impact_dir[1] * (impact_speed * 0.5 + 1.2),
                    2.8,
                ];
                prop.tumble_axis = [-impact_dir[1], impact_dir[0], 0.2];
                prop.angular_v = 4.0;
                if let Some(pm) = pickups {
                    pm.spawn_dropped(crate::weapons::PickupKind::Money(40), prop.p);
                }
            }
            11 | 12 | 20 | 24 | 29 | 32..=34 | 36 | 39 | 43..=55 => {
                // Wooden crates, fruit boxes, pallets, market stalls
                for i in 0..10 {
                    if particles.len() < 160 {
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.4],
                            v: [
                                impact_dir[0] * 3.0 + ((i * 31) % 7) as f32 * 0.5 - 1.5,
                                impact_dir[1] * 3.0 + ((i * 53) % 7) as f32 * 0.5 - 1.5,
                                2.2 + (i as f32) * 0.3,
                            ],
                            life: 0.0,
                            max_life: 0.75,
                            size: 0.26,
                            grow: 0.3,
                            kind: 3, // wood splinters
                        });
                    }
                }
                if prop.mass < 1.8 && prop.uproot_force >= 0.0 && prop.uproot_force < 18.0 {
                    // Light crate tumbles briefly
                    prop.state = PropState::Loose;
                    prop.collidable = false;
                    prop.sleep = false;
                    prop.v = [
                        impact_dir[0] * (impact_speed * 0.5 + 1.2),
                        impact_dir[1] * (impact_speed * 0.5 + 1.2),
                        2.6,
                    ];
                    prop.tumble_axis = [-impact_dir[1], impact_dir[0], 0.2];
                    prop.angular_v = 4.5;
                } else {
                    prop.state = PropState::Broken;
                    prop.collidable = false;
                }
            }
            _ => {
                // Parking meters or generic loose prop
                if prop.mass < 2.0 && prop.uproot_force >= 0.0 && prop.uproot_force < 20.0 {
                    prop.state = PropState::Loose;
                    prop.collidable = false;
                    prop.sleep = false;
                    prop.v = [
                        impact_dir[0] * (impact_speed * 0.5 + 1.2),
                        impact_dir[1] * (impact_speed * 0.5 + 1.2),
                        2.8,
                    ];
                    prop.tumble_axis = [-impact_dir[1], impact_dir[0], 0.2];
                    prop.angular_v = 4.0;
                    if prop.prop_id == 174 || prop.prop_id == 176 {
                        if let Some(pm) = pickups {
                            pm.spawn_dropped(crate::weapons::PickupKind::Money(40), prop.p);
                        }
                    }
                } else {
                    prop.state = PropState::Broken;
                    prop.collidable = false;
                }
            }
        }
    }

    /// Damages props in radius from bullet traces or melee attacks.
    pub fn damage_at(
        &mut self,
        pos: [f32; 3],
        radius: f32,
        damage: u8,
        force_dir: [f32; 3],
        particles: &mut Vec<Particle>,
        pickups: &mut Option<&mut crate::weapons::PickupManager>,
    ) {
        let mut smash_trigger = None;
        for i in 0..self.active.len() {
            let prop = &mut self.active[i];
            if prop.state != PropState::Standing {
                continue;
            }
            let dx = prop.p[0] - pos[0];
            let dy = prop.p[1] - pos[1];
            let dz = prop.p[2] - pos[2];
            if dx * dx + dy * dy + dz * dz <= radius * radius {
                let breakable = prop.smash_force >= 0.0 || prop.uproot_force >= 0.0 || prop.smash_eff > 0;
                if !breakable {
                    continue;
                }
                prop.health = prop.health.saturating_sub(damage);
                if prop.health == 0
                    || (prop.smash_force >= 0.0 && (damage as f32) >= prop.smash_force)
                    || prop.smash_eff == 10
                    || prop.smash_eff == 28
                {
                    let dir_2d = [force_dir[0], force_dir[1]];
                    let len = (dir_2d[0] * dir_2d[0] + dir_2d[1] * dir_2d[1]).sqrt().max(1e-4);
                    let dir_norm = [dir_2d[0] / len, dir_2d[1] / len];
                    smash_trigger = Some((i, dir_norm));
                    break;
                }
            }
        }
        if let Some((i, dir_norm)) = smash_trigger {
            self.apply_smash(i, dir_norm, 8.0, particles, pickups);
        }
    }

    /// Blasts and damages props in radius from explosions.
    pub fn explosion(
        &mut self,
        blast_pos: [f32; 3],
        force_radius: f32,
        damage_radius: f32,
        damage: f32,
        particles: &mut Vec<Particle>,
        pickups: &mut Option<&mut crate::weapons::PickupManager>,
    ) {
        let mut to_smash = Vec::new();
        for i in 0..self.active.len() {
            let prop = &mut self.active[i];
            if prop.state != PropState::Standing {
                continue;
            }
            let dx = prop.p[0] - blast_pos[0];
            let dy = prop.p[1] - blast_pos[1];
            let dz = prop.p[2] - blast_pos[2];
            let dist2 = dx * dx + dy * dy + dz * dz;
            if dist2 <= force_radius * force_radius {
                let dist = dist2.sqrt().max(0.1);
                let dir_2d = [dx / dist, dy / dist];
                let damage_fraction = (1.0 - (dist / damage_radius).min(1.0)) * damage;
                prop.health = prop.health.saturating_sub(damage_fraction as u8);
                if prop.health == 0 || dist < force_radius * 0.75 {
                    if prop.smash_force >= 0.0 || prop.uproot_force >= 0.0 || prop.smash_eff > 0 {
                        to_smash.push((i, dir_2d));
                    }
                }
            }
        }
        for (i, dir_2d) in to_smash {
            self.apply_smash(i, dir_2d, 15.0, particles, pickups);
        }
    }

    /// Pushes a walking player out from solid standing street furniture.
    pub fn push_out_player(&self, p: &mut [f32; 3], radius: f32) -> bool {
        let mut pushed = false;
        for prop in &self.active {
            if !prop.collidable || prop.state != PropState::Standing {
                continue;
            }
            let dx = p[0] - prop.p[0];
            let dy = p[1] - prop.p[1];
            let d2 = dx * dx + dy * dy;
            let r = radius + 0.35;
            if d2 < r * r && d2 > 1e-6 {
                let d = d2.sqrt();
                let pen = r - d;
                p[0] += (dx / d) * pen;
                p[1] += (dy / d) * pen;
                pushed = true;
            }
        }
        pushed
    }

    /// Updates dynamic props physics, tumble integration, falling streetlights, and water hydrant geysers.
    pub fn update(&mut self, dt: f32, particles: &mut Vec<Particle>) {
        for prop in &mut self.active {
            // Indefinite water geyser fountain for smashed hydrants
            if prop.fountain {
                if particles.len() < 160 {
                    let rx = ((particles.len() * 37) % 100) as f32 / 200.0 - 0.25;
                    let ry = ((particles.len() * 53) % 100) as f32 / 200.0 - 0.25;
                    particles.push(Particle {
                        p: [prop.p[0] + rx, prop.p[1] + ry, prop.p[2] + 0.2],
                        v: [rx * 2.5, ry * 2.5, 7.5],
                        life: 0.0,
                        max_life: 0.85,
                        size: 0.26,
                        grow: 1.9,
                        kind: 1, // water spray geyser
                    });
                }
            }
            // Burning prop fire emitter (hotdog carts, petrol pump fires)
            if prop.fire_timer > 0.0 {
                prop.fire_timer = (prop.fire_timer - dt).max(0.0);
                if particles.len() < 160 && (particles.len() % 2 == 0) {
                    particles.push(Particle {
                        p: [
                            prop.p[0] + (prop.fire_timer * 11.0).sin() * 0.25,
                            prop.p[1] + (prop.fire_timer * 17.0).cos() * 0.25,
                            prop.p[2] + 0.3,
                        ],
                        v: [
                            (prop.fire_timer * 7.0).sin() * 0.4,
                            (prop.fire_timer * 13.0).cos() * 0.4,
                            1.8 + (prop.fire_timer * 3.0).sin().abs() * 1.2,
                        ],
                        life: 0.0,
                        max_life: 0.55,
                        size: 0.30,
                        grow: 0.6,
                        kind: 7, // flame
                    });
                }
            }
            // Gravitational topple for damaged streetlights
            if prop.state == PropState::Tilted && !prop.sleep {
                if prop.smash_eff == 7 {
                    prop.tilt_speed += 12.0 * dt;
                    prop.tilt += prop.tilt_speed * dt;
                    if prop.tilt >= 1.54 {
                        prop.tilt = 1.54; // flat on pavement (~88 degrees)
                        prop.tilt_speed = 0.0;
                        prop.sleep = true;
                        self.pending_smashes.push((prop.p, 56)); // Sound 56: ground impact clatter
                        for i in 0..6 {
                            if particles.len() < 160 {
                                particles.push(Particle {
                                    p: [prop.p[0], prop.p[1], prop.p[2] + 0.1],
                                    v: [
                                        ((i * 31) % 7) as f32 * 0.4 - 1.2,
                                        ((i * 47) % 7) as f32 * 0.4 - 1.2,
                                        0.8 + ((i * 13) % 5) as f32 * 0.3,
                                    ],
                                    life: 0.0,
                                    max_life: 0.35,
                                    size: 0.15,
                                    grow: 0.1,
                                    kind: 5, // ground impact sparks
                                });
                            }
                        }
                    }
                }
            }
            // Loose tumbling props (bins, crates, parking meters, uprooted poles)
            if prop.state == PropState::Loose && !prop.sleep {
                prop.p[0] += prop.v[0] * dt;
                prop.p[1] += prop.v[1] * dt;
                prop.p[2] += prop.v[2] * dt;
                prop.v[2] -= 9.8 * dt; // gravity
                prop.v[0] *= (1.0 - 1.2 * dt).max(0.0);
                prop.v[1] *= (1.0 - 1.2 * dt).max(0.0);

                prop.roll_angle += prop.angular_v * dt;
                prop.angular_v *= (1.0 - 1.5 * dt).max(0.0);

                if prop.p[2] <= prop.base_z {
                    prop.p[2] = prop.base_z;
                    prop.v[2] = -prop.v[2] * 0.38;
                    prop.v[0] *= 0.75;
                    prop.v[1] *= 0.75;
                    if prop.smash_eff == 35 && particles.len() < 160 {
                        // Debris puff on bounce
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.2],
                            v: [prop.v[0] * 0.5, prop.v[1] * 0.5, 1.2],
                            life: 0.0,
                            max_life: 0.5,
                            size: 0.20,
                            grow: 0.2,
                            kind: 2, // garbage debris
                        });
                    }
                    if (prop.smash_eff == 14 || prop.smash_eff == 22) && particles.len() < 160 {
                        // Mail / newspaper flutter puff on ground bounce
                        let k = if prop.smash_eff == 14 { 8 } else { 4 };
                        particles.push(Particle {
                            p: [prop.p[0], prop.p[1], prop.p[2] + 0.2],
                            v: [prop.v[0] * 0.4, prop.v[1] * 0.4, 1.2],
                            life: 0.0,
                            max_life: 0.5,
                            size: 0.18,
                            grow: 0.15,
                            kind: k,
                        });
                    }
                    if prop.v[2].abs() < 0.35 && (prop.v[0] * prop.v[0] + prop.v[1] * prop.v[1]) < 0.25 {
                        prop.v = [0.0; 3];
                        prop.angular_v = 0.0;
                        prop.sleep = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actors::make_vehicle;

    fn test_prop(prop_id: u16, model_id: u16, p: [f32; 3], smash_eff: u8, mass: f32) -> ActiveProp {
        ActiveProp {
            cell_x: 10,
            cell_y: 10,
            prop_idx: 0,
            prop_id,
            model_id,
            broken_model_id: 0xFFFE,
            p,
            angle: 0.0,
            v: [0.0; 3],
            roll_angle: 0.0,
            tumble_axis: [1.0, 0.0, 0.0],
            angular_v: 0.0,
            tilt: 0.0,
            tilt_speed: 0.0,
            tilt_axis: [0.0, 0.0],
            state: PropState::Standing,
            collidable: true,
            fountain: false,
            fire_timer: 0.0,
            health: 20,
            mass,
            smash_eff,
            smash_force: 10.0,
            uproot_force: 15.0,
            planar: false,
            bullet_damage: true,
            melee_damage: true,
            inflammable: false,
            base_z: p[2],
            sleep: true,
        }
    }

    #[test]
    fn authentic_psp_props_hydrant_geyser_and_trash_bins() {
        let mut manager = PropsManager::new();
        // Hydrant (effect 9)
        let mut hydrant = test_prop(175, 2195, [100.0, 100.0, 0.0], 9, 1.5);
        hydrant.smash_force = 15.0;
        hydrant.uproot_force = 30.0;
        manager.active.push(hydrant);

        // Trash bin (effect 35)
        let mut bin = test_prop(173, 2193, [200.0, 200.0, 0.0], 35, 0.8);
        bin.prop_idx = 1;
        bin.health = 10;
        bin.smash_force = 8.0;
        bin.uproot_force = 12.0;
        manager.active.push(bin);

        let mut particles = Vec::new();
        let mut car = make_vehicle([100.0, 98.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car.speed = 10.0;
        car.h = [0.9, 2.1, 0.7];

        // Impact with hydrant
        manager.check_vehicle_impact(&mut car, 0.016, &mut particles);
        let hydrant = &manager.active[0];
        assert_eq!(hydrant.state, PropState::Tilted, "Hydrant tilts on impact");
        assert!(hydrant.fountain, "Hydrant enables fountain geyser");
        assert!(!hydrant.collidable, "Hydrant snaps solid collision");
        assert!((hydrant.tilt - 0.42).abs() < 1e-4, "Hydrant tilts ~24 degrees");
        assert!(car.speed < 10.0, "Car slows down on impact");

        // Hydrant geyser emission on update
        manager.update(0.016, &mut particles);
        assert!(!particles.is_empty(), "Hydrant produces water spray particles");
        assert_eq!(particles[0].kind, 1, "Water spray particle kind is 1");
        assert_eq!(particles[0].v[2], 7.5, "Water geyser shoots upward at 7.5 u/s");

        // Impact with trash bin
        let mut car2 = make_vehicle([200.0, 198.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car2.speed = 12.0;
        car2.h = [0.9, 2.1, 0.7];
        let p_count_before = particles.len();
        manager.check_vehicle_impact(&mut car2, 0.016, &mut particles);
        let bin = &manager.active[1];
        assert_eq!(bin.state, PropState::Loose, "Trash bin becomes loose body");
        assert!(!bin.collidable, "Bin collision disabled while tumbling");
        assert!(bin.v[2] > 2.0, "Bin launched upward");
        assert!(particles.len() > p_count_before, "Garbage debris particles emitted");
        assert!(particles.iter().any(|p| p.kind == 2), "Garbage debris particle kind is 2");

        // Tumble integration
        let bin_p_before = manager.active[1].p;
        manager.update(0.1, &mut particles);
        assert_ne!(manager.active[1].p, bin_p_before, "Loose bin translates in update");
    }

    #[test]
    fn authentic_newsstand_and_parking_meter_cash_drops() {
        let mut manager = PropsManager::new();
        // 1. Newsstand kiosk (effect 42)
        let mut newsstand = test_prop(180, 2200, [300.0, 300.0, 0.0], 42, 1.2);
        newsstand.health = 15;
        manager.active.push(newsstand);

        // 2. Newspaper vending box (effect 22)
        let mut newsbox = test_prop(181, 2051, [320.0, 300.0, 0.0], 22, 1.0);
        newsbox.prop_idx = 1;
        manager.active.push(newsbox);

        // 3. USPS Mailbox (effect 14)
        let mut mailbox = test_prop(182, 2022, [340.0, 300.0, 0.0], 14, 0.8);
        mailbox.prop_idx = 2;
        manager.active.push(mailbox);

        // 4. Parking meter (effect 17)
        let mut meter = test_prop(183, 2218, [360.0, 300.0, 0.0], 17, 0.5);
        meter.prop_idx = 3;
        manager.active.push(meter);

        let mut pickups = crate::weapons::PickupManager::new();
        let mut particles = Vec::new();
        let mut car = make_vehicle([300.0, 298.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car.speed = 10.0;
        car.h = [0.9, 2.1, 0.7];

        // Impact newsstand kiosk (effect 42) -> Broken, drops $25 cash, wood & paper particles
        manager.check_vehicle_impact_with_loot(&mut car, 0.016, &mut particles, &mut Some(&mut pickups));
        assert_eq!(manager.active[0].state, PropState::Broken);
        assert!(!manager.active[0].collidable);
        assert!(particles.iter().any(|p| p.kind == 4), "Newsstand releases paper flutter particles");
        assert!(particles.iter().any(|p| p.kind == 3), "Newsstand releases wood splinters");
        assert!(pickups.pickups.iter().any(|p| matches!(p.kind, crate::weapons::PickupKind::Money(25))), "Newsstand drops $25 cash");

        // Impact newspaper vending box (effect 22) -> Loose physical tumbling body, drops $25 cash, paper particles
        let mut car2 = make_vehicle([320.0, 298.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car2.speed = 10.0;
        car2.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact_with_loot(&mut car2, 0.016, &mut particles, &mut Some(&mut pickups));
        assert_eq!(manager.active[1].state, PropState::Loose, "Newspaper dispenser becomes loose body");
        assert!(!manager.active[1].collidable);
        assert!(manager.active[1].v[2] > 2.0, "Newspaper dispenser launches vertically in 3D");
        assert!(manager.active[1].angular_v > 0.0, "Newspaper dispenser tumbles with angular velocity");
        assert_eq!(pickups.pickups.iter().filter(|p| matches!(p.kind, crate::weapons::PickupKind::Money(25))).count(), 2, "Newspaper box drops $25 cash");

        // Impact USPS mailbox (effect 14) -> Loose physical tumbling body, blue (kind 8) and white (kind 4) mail envelopes
        let mut car3 = make_vehicle([340.0, 298.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car3.speed = 10.0;
        car3.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact_with_loot(&mut car3, 0.016, &mut particles, &mut Some(&mut pickups));
        assert_eq!(manager.active[2].state, PropState::Loose, "USPS Mailbox becomes loose body");
        assert!(!manager.active[2].collidable);
        assert!(manager.active[2].v[2] > 2.0, "USPS Mailbox launches vertically in 3D");
        assert!(manager.active[2].angular_v > 0.0, "USPS Mailbox tumbles with angular velocity");
        assert!(particles.iter().any(|p| p.kind == 8), "Mailbox releases blue USPS mail envelopes (kind 8)");

        // Impact parking meter (effect 17) -> Loose physical body, drops $40 cash
        let mut car4 = make_vehicle([360.0, 298.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car4.speed = 10.0;
        car4.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact_with_loot(&mut car4, 0.016, &mut particles, &mut Some(&mut pickups));
        assert_eq!(manager.active[3].state, PropState::Loose, "Parking meter becomes loose body");
        assert!(pickups.pickups.iter().any(|p| matches!(p.kind, crate::weapons::PickupKind::Money(40))), "Parking meter drops $40 cash");

        // Physics integration test for loose newspaper dispenser & mailbox
        let pos_before = manager.active[1].p;
        for _ in 0..15 {
            manager.update(0.016, &mut particles);
        }
        assert_ne!(manager.active[1].p, pos_before, "Newspaper box integrates velocity");
        assert!(manager.active[1].roll_angle > 0.0, "Newspaper box rolls");
    }

    #[test]
    fn street_lamp_tilt_electric_sparks_and_player_push_out() {
        let mut manager = PropsManager::new();
        // Lamp post (effect 7)
        let mut lamp = test_prop(190, 2210, [400.0, 400.0, 0.0], 7, 2.5);
        lamp.health = 25;
        lamp.smash_force = 18.0;
        lamp.uproot_force = 25.0;
        manager.active.push(lamp);

        // Player walking pushes out from solid standing lamp post
        let mut player_p = [400.3, 400.0, 0.0];
        let pushed = manager.push_out_player(&mut player_p, 0.45);
        assert!(pushed, "Solid standing lamp post blocks walking player");
        assert!(player_p[0] > 400.3, "Player pushed out from lamp post");

        // Car rams lamp post at high speed (20 m/s) -> progressive topple initiated without barrel rolls or flight
        let mut particles = Vec::new();
        let mut car = make_vehicle([400.0, 398.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car.speed = 20.0;
        car.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact(&mut car, 0.016, &mut particles);
        assert_eq!(manager.active[0].state, PropState::Tilted, "Lamp post tilts on impact even at high speed");
        assert_eq!(manager.active[0].v, [0.0; 3], "Lamp post stays anchored to base without aerial flight");
        assert_eq!(manager.active[0].angular_v, 0.0, "Lamp post does not perform barrel rolls");
        assert!(particles.iter().any(|p| p.kind == 5), "Electric sparks generated from damaged lamp post");
        assert!(!manager.active[0].collidable, "Lamp post unanchors collision");

        // Gravitational topple progresses over time until landing flat on ground (>= 1.54 rad)
        let tilt_initial = manager.active[0].tilt;
        for _ in 0..30 {
            manager.update(0.016, &mut particles);
        }
        assert!(manager.active[0].tilt > tilt_initial, "Lamp post accelerates downward");
        for _ in 0..60 {
            manager.update(0.016, &mut particles);
        }
        assert!((manager.active[0].tilt - 1.54).abs() < 1e-4, "Lamp post lies flat on pavement (~88 degrees)");
        assert!(manager.active[0].sleep, "Fallen lamp post rests asleep on pavement");
        assert!(manager.drain_pending_smashes().iter().any(|&(_, eff)| eff == 56), "Impact sound 56 triggered on pavement landing");

        // Pending disables recorded
        let disables = manager.drain_pending_disables();
        assert_eq!(disables, vec![(10, 10, 0)]);

        // Explosion blast damages nearby props
        let mut exp_particles = Vec::new();
        let mut pickups = crate::weapons::PickupManager::new();
        let mut pickups_opt = Some(&mut pickups);
        manager.explosion([400.0, 401.0, 0.0], 5.0, 3.5, 40.0, &mut exp_particles, &mut pickups_opt);
        assert!(!manager.active[0].collidable, "Explosion disables collidability");
    }

    #[test]
    fn authentic_glass_phone_booth_and_bus_shelter_shattering() {
        let mut manager = PropsManager::new();
        // Phone booth (effect 10)
        manager.active.push(test_prop(201, 2301, [500.0, 500.0, 0.0], 10, 1.8));
        // Bus shelter (effect 41)
        let mut shelter = test_prop(202, 2302, [520.0, 500.0, 0.0], 41, 3.0);
        shelter.prop_idx = 1;
        manager.active.push(shelter);

        let mut particles = Vec::new();
        let mut car = make_vehicle([500.0, 498.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car.speed = 10.0;
        car.h = [0.9, 2.1, 0.7];

        // Impact with phone booth
        manager.check_vehicle_impact(&mut car, 0.016, &mut particles);
        assert_eq!(manager.active[0].state, PropState::Broken, "Phone booth shatters");
        assert!(!manager.active[0].collidable, "Collision removed on shatter");
        assert!(particles.iter().any(|p| p.kind == 6), "Glass shards particle emitted (kind 6)");

        // Impact with bus shelter
        let mut car2 = make_vehicle([520.0, 498.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car2.speed = 10.0;
        car2.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact(&mut car2, 0.016, &mut particles);
        assert_eq!(manager.active[1].state, PropState::Broken, "Bus shelter shatters");
        assert!(particles.iter().any(|p| p.kind == 3), "Wood frame splinters emitted (kind 3)");
        assert!(particles.iter().any(|p| p.kind == 6), "Glass panels burst into shards (kind 6)");
    }

    #[test]
    fn authentic_petrol_pump_and_barrel_explosions() {
        let mut manager = PropsManager::new();
        // Petrol pump (effect 21)
        manager.active.push(test_prop(210, 2400, [600.0, 600.0, 0.0], 21, 2.5));
        // Explosive barrel (effect 28)
        let mut barrel = test_prop(211, 2401, [620.0, 600.0, 0.0], 28, 1.2);
        barrel.prop_idx = 1;
        manager.active.push(barrel);
        // Hotdog cart (effect 23)
        let mut hotdog = test_prop(212, 2402, [640.0, 600.0, 0.0], 23, 1.4);
        hotdog.prop_idx = 2;
        manager.active.push(hotdog);

        let mut particles = Vec::new();
        let mut car = make_vehicle([600.0, 598.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car.speed = 12.0;
        car.h = [0.9, 2.1, 0.7];

        // Impact with petrol pump
        manager.check_vehicle_impact(&mut car, 0.016, &mut particles);
        let exps = manager.drain_pending_explosions();
        let fires = manager.drain_pending_fires();
        assert_eq!(exps.len(), 1, "Petrol pump queues explosion");
        assert_eq!(exps[0].1, crate::explosions::ExplosionType::BigVehicle, "Petrol pump triggers BigVehicle explosion");
        assert_eq!(fires.len(), 1, "Petrol pump leaves persistent petrol fire");
        assert!(fires[0].2 >= 20.0, "Petrol fire burns for long duration");
        assert!(particles.iter().any(|p| p.kind == 7), "Fire flames emitted from pump burst");

        // Impact with explosive barrel
        let mut car2 = make_vehicle([620.0, 598.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car2.speed = 10.0;
        car2.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact(&mut car2, 0.016, &mut particles);
        let exps2 = manager.drain_pending_explosions();
        assert_eq!(exps2.len(), 1, "Barrel queues explosion");
        assert_eq!(exps2[0].1, crate::explosions::ExplosionType::Barrel, "Barrel triggers Barrel explosion");

        // Impact with hotdog cart
        let mut car3 = make_vehicle([640.0, 598.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car3.speed = 8.0;
        car3.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact(&mut car3, 0.016, &mut particles);
        assert!(manager.active[2].fire_timer > 0.0, "Hotdog cart catches fire");
        let exps3 = manager.drain_pending_explosions();
        assert!(!exps3.is_empty(), "Hotdog cart triggers gas explosion");
    }

    #[test]
    fn authentic_wooden_crate_fence_and_sign_physics() {
        let mut manager = PropsManager::new();
        // Fence (effect 26)
        manager.active.push(test_prop(220, 2500, [700.0, 700.0, 0.0], 26, 1.0));
        // Street sign (effect 15)
        let mut sign = test_prop(221, 2501, [720.0, 700.0, 0.0], 15, 0.5);
        sign.prop_idx = 1;
        manager.active.push(sign);
        // Traffic signal (effect 8)
        let mut signal = test_prop(222, 2502, [740.0, 700.0, 0.0], 8, 2.0);
        signal.prop_idx = 2;
        manager.active.push(signal);

        let mut particles = Vec::new();
        let mut car = make_vehicle([700.0, 698.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car.speed = 8.0;
        car.h = [0.9, 2.1, 0.7];

        // Impact with fence
        manager.check_vehicle_impact(&mut car, 0.016, &mut particles);
        assert_eq!(manager.active[0].state, PropState::Broken, "Wooden fence shatters");
        assert!(particles.iter().any(|p| p.kind == 3), "Wood slat splinters emitted");

        // Impact with sign
        let mut car2 = make_vehicle([720.0, 698.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car2.speed = 6.0;
        car2.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact(&mut car2, 0.016, &mut particles);
        assert_eq!(manager.active[1].state, PropState::Tilted, "Sign tilts on impact");
        assert!((manager.active[1].tilt - 0.52).abs() < 1e-4, "Sign tilts 30 degrees (0.52 rad)");

        // Impact with traffic signal
        let mut car3 = make_vehicle([740.0, 698.8, 0.0], 0.0, 0, 0, 0, 0, false, false);
        car3.speed = 8.0;
        car3.h = [0.9, 2.1, 0.7];
        manager.check_vehicle_impact(&mut car3, 0.016, &mut particles);
        assert_eq!(manager.active[2].state, PropState::Tilted, "Traffic signal tilts flat");
        assert!((manager.active[2].tilt - 1.48).abs() < 1e-4, "Traffic signal tilts flat (1.48 rad)");
    }

    #[test]
    fn prop_gunfire_and_melee_damage() {
        let mut manager = PropsManager::new();
        // Phone booth (effect 10)
        manager.active.push(test_prop(230, 2600, [800.0, 800.0, 0.0], 10, 1.5));
        let mut particles = Vec::new();
        let mut pickups = crate::weapons::PickupManager::new();
        let mut pickups_opt = Some(&mut pickups);

        // Gunfire hits phone booth
        manager.damage_at([800.2, 800.1, 0.5], 1.5, 30, [1.0, 0.0, 0.0], &mut particles, &mut pickups_opt);
        assert_eq!(manager.active[0].state, PropState::Broken, "Gunfire shatters phone booth");
        assert!(!manager.active[0].collidable);
        assert!(particles.iter().any(|p| p.kind == 6), "Glass fragments emitted from bullet strike");
    }
}

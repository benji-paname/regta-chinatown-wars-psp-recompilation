// SPDX-License-Identifier: GPL-3.0-only
//! Reproducible, renderer-free driving diagnostics using the user's PSP data.
use crate::{actors::{make_vehicle, step_car, DriveInput}, collision::{Cell, Collision}, gps::{distance, Roads}, iso::{Disc, Result}, pak::Pak, wad::Wad};
use serde_json::json;
use std::path::Path;

pub fn run(disc: &mut Disc, pak: &Pak, wad: &Wad, output: &Path) -> Result<()> {
    crate::handling::load_table(&pak.read(disc, 29)?.0)?;
    let flat = Collision { cells: (0..14000).map(|_| Cell { ground: vec![0x55; 400], ..Cell::default() }).collect() };
    let mut traces = Vec::new();
    let mut failures = Vec::new();
    for spec in crate::vehicle::SPECS.iter().filter(|s| s.is_land_vehicle()) {
        let mut car = make_vehicle([0.; 3], 0., 0, 0, spec.info_id, 0, false, false);
        let mut samples = Vec::new();
        let mut max_speed = 0f32;
        let mut max_yaw = 0f32;
        for tick in 0..900 {
            let input = match tick {
                0..120 => DriveInput::controls(1., 0., 0., false),
                120..180 => DriveInput::controls(1., 0., 0.7, false),
                180..240 => DriveInput::controls(0., 0., 0., false),
                240..300 => DriveInput::controls(0., 1., 0., false),
                300..360 => DriveInput::controls(0., 1., -0.5, false),
                360..540 => DriveInput::controls(1., 0., 0., false),
                540..570 => DriveInput::controls(0., 0., 1., true),
                570..660 => DriveInput::controls(0., 0., 0., true),
                660..720 => DriveInput::controls(1., 1., 0., false),
                _ => DriveInput::controls(1., 0., 0., false),
            };
            step_car(&mut car, &flat, input, 1. / 30.);
            max_speed = max_speed.max(car.speed.hypot(car.lat_vel));
            max_yaw = max_yaw.max(car.yaw_rate.abs());
            if !car.p.iter().all(|v| v.is_finite()) || ![car.speed,car.lat_vel,car.yaw_rate,car.rpm].iter().all(|v| v.is_finite()) || max_speed > 200. || max_yaw > 20. {
                failures.push(json!({"vehicle": spec.name, "tick":tick,"speed":max_speed,"yaw":max_yaw}));
                break;
            }
            if tick % 15 == 0 { samples.push(json!({"tick":tick,"p":car.p,"heading":car.angle,"speed":car.speed,"lateral":car.lat_vel,"yaw":car.yaw_rate,"gear":car.gear,"rpm":car.rpm})); }
        }
        traces.push(json!({"id":spec.info_id,"name":spec.name,"max_speed":max_speed,"max_yaw":max_yaw,"samples":samples}));
    }
    let collision = Collision::parse(&wad.read(disc, crate::wad::name_hash("world.bin"))?)?;
    let roads = Roads::parse(&wad.read(disc, crate::wad::name_hash("ai.bin"))?)?;
    let mut routes = Vec::new();
    for (index, (a, b)) in roads.edges().enumerate().step_by(19) {
        let (lane, wobble) = roads.select_lane(a,b,index as u32);
        let next = roads.next_node(a,b,lane,index as u32).unwrap_or(b);
        let (nl,nw) = roads.convert_lane(a,b,next,lane,index as u32);
        let along = distance(roads.nodes[a].p,roads.nodes[b].p)*0.25;
        let (p,angle,_,_) = roads.rail_pose(a,b,next,along,lane,wobble,nl,nw);
        let Some(p) = crate::actors::rail_position_for(&collision,p,angle,0) else { continue; };
        let mut car = make_vehicle(p,angle,b,a,0,0,false,false);
        car.lane=lane;car.wobble=wobble;car.next_target=next;car.next_lane=nl;car.next_wobble=nw;car.rail_dist=along;
        let mut travelled = 0.;
        let mut stalled = 0;
        let mut max_step = 0f32;
        let mut lights = crate::vehicle::TrafficLightManager::new();
        for tick in 0..1800 {
            let old=car.p;
            crate::traffic::step_rail(&mut car,&roads,&collision,&lights,f32::INFINITY,false,1./30.,index as u32 + tick);
            lights.update(1./30.);
            let moved=distance(old,car.p);
            travelled+=moved;max_step=max_step.max(moved);
            if moved < 0.001 { stalled+=1; }
            if !car.on_rail { break; }
        }
        if max_step > 1. { failures.push(json!({"route":[a,b],"jump":max_step})); }
        routes.push(json!({"from":a,"to":b,"travelled":travelled,"max_step":max_step,"stopped_ticks":stalled,"world_obstruction":!car.on_rail,
            "ended_from":car.previous,"ended_to":car.target,"ended_next":car.next_target,"position":car.p,"rail_distance":car.rail_dist}));
    }
    std::fs::create_dir_all(output)?;
    let report=json!({"schema":1,"source":"PSP ULES01347 1.01 PAK29, ai.bin and world.bin; Android grounded algorithm adaptation","psp_parity_verified":false,"vehicles":traces,"routes":routes,"failures":failures});
    std::fs::write(output.join("driving-audit.json"),serde_json::to_vec_pretty(&report)?)?;
    println!("Driving audit: {} vehicles, {} real-map routes, {} failures. {}",traces.len(),routes.len(),failures.len(),output.join("driving-audit.json").display());
    if !failures.is_empty() { return Err("Driving audit reported invalid trajectories".into()); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn real_map_repaired_routes_have_zero_world_obstructions() {
        let mut iso_path = Path::new("work/Chinatown-Wars-PSP.iso");
        let alt = Path::new("../../work/Chinatown-Wars-PSP.iso");
        if !iso_path.exists() && alt.exists() {
            iso_path = alt;
        }
        if !iso_path.exists() {
            return;
        }
        let mut disc = Disc::open(iso_path).unwrap();
        let wad = Wad::open(&mut disc).unwrap();
        let roads = Roads::parse(&wad.read(&mut disc, crate::wad::name_hash("ai.bin")).unwrap()).unwrap();
        let collision = Collision::parse(&wad.read(&mut disc, crate::wad::name_hash("world.bin")).unwrap()).unwrap();

        // 1. Collinear junction with lateral shift (197 -> 142 -> 215)
        {
            let (lane, wobble) = roads.select_lane(197, 142, 0);
            let next = roads.next_node(197, 142, lane, 0).unwrap_or(142);
            let (nl, nw) = roads.convert_lane(197, 142, next, lane, 0);
            let along = distance(roads.nodes[197].p, roads.nodes[142].p) * 0.25;
            let (p, angle, _, _) = roads.rail_pose(197, 142, next, along, lane, wobble, nl, nw);
            let mut car = make_vehicle(p, angle, 142, 197, 0, 0, false, false);
            car.lane = lane; car.wobble = wobble; car.next_target = next; car.next_lane = nl; car.next_wobble = nw; car.rail_dist = along;
            let mut lights = crate::vehicle::TrafficLightManager::new();
            for tick in 0..1200 {
                crate::traffic::step_rail(&mut car, &roads, &collision, &lights, f32::INFINITY, false, 1. / 30., tick);
                lights.update(1. / 30.);
                assert!(car.on_rail, "Route 197->142 lost rail at tick {}", tick);
            }
            assert!(car.p[1] > 1350., "Route 197->142 progressed past node 142");
        }

        // 2. Curb decorative cylinder bypass on two-way street (443 -> 428 -> ... -> 431 -> 432)
        {
            let (lane, wobble) = roads.select_lane(443, 428, 19);
            let next = roads.next_node(443, 428, lane, 19).unwrap_or(428);
            let (nl, nw) = roads.convert_lane(443, 428, next, lane, 19);
            let along = distance(roads.nodes[443].p, roads.nodes[428].p) * 0.25;
            let (p, angle, _, _) = roads.rail_pose(443, 428, next, along, lane, wobble, nl, nw);
            let mut car = make_vehicle(p, angle, 428, 443, 0, 0, false, false);
            car.lane = lane; car.wobble = wobble; car.next_target = next; car.next_lane = nl; car.next_wobble = nw; car.rail_dist = along;
            let mut lights = crate::vehicle::TrafficLightManager::new();
            for tick in 0..1800 {
                crate::traffic::step_rail(&mut car, &roads, &collision, &lights, f32::INFINITY, false, 1. / 30., 19 + tick);
                lights.update(1. / 30.);
                assert!(car.on_rail, "Route 443->428 lost rail at tick {}", tick);
            }
        }

        // 3. Acute angle with negative parameter intercept (626 -> 1439 -> 1467)
        {
            let (lane, wobble) = roads.select_lane(626, 1439, 0);
            let next = roads.next_node(626, 1439, lane, 0).unwrap_or(1439);
            let (nl, nw) = roads.convert_lane(626, 1439, next, lane, 0);
            let along = distance(roads.nodes[626].p, roads.nodes[1439].p) * 0.25;
            let (p, angle, _, _) = roads.rail_pose(626, 1439, next, along, lane, wobble, nl, nw);
            let mut car = make_vehicle(p, angle, 1439, 626, 0, 0, false, false);
            car.lane = lane; car.wobble = wobble; car.next_target = next; car.next_lane = nl; car.next_wobble = nw; car.rail_dist = along;
            let mut lights = crate::vehicle::TrafficLightManager::new();
            for tick in 0..1200 {
                crate::traffic::step_rail(&mut car, &roads, &collision, &lights, f32::INFINITY, false, 1. / 30., tick);
                lights.update(1. / 30.);
                assert!(car.on_rail, "Route 626->1439 lost rail at tick {}", tick);
            }
            assert!(car.p[0] < -1030., "Route 626->1439 progressed West past node 1439");
        }
    }
}

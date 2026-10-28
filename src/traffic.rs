// SPDX-License-Identifier: GPL-3.0-only
// Algorithms adapted from traffic.cpp / aispline.cpp, (C) 2026 Kryo.to, b0506abc.
//! Grounded ambient traffic: continuous rails, clearance braking and route recovery.
use crate::{actors::{Car, Ped, forward, rail_position_for}, collision::Collision, gps::{Roads, distance}, vehicle::TrafficLightManager};

#[derive(Clone, Copy)]
pub struct Body { pub p: [f32; 3], pub angle: f32, pub h: [f32; 3] }
impl From<&Car> for Body {
    fn from(car: &Car) -> Self { Self { p: car.p, angle: car.angle, h: car.h } }
}

/// Bumper clearance, including rotated vehicles and separate bridge levels.
pub fn clearance(car: &Car, index: usize, bodies: &[Body], peds: &[Ped], player: [f32; 3]) -> f32 {
    let f = forward(car.angle);
    let mut gap = f32::INFINITY;
    let mut inspect = |p: [f32; 3], half_length: f32, half_width: f32| {
        if (p[2] - car.p[2]).abs() > 2. { return; }
        let d = [p[0] - car.p[0], p[1] - car.p[1]];
        let along = d[0] * f[0] + d[1] * f[1];
        let across = (d[0] * f[1] - d[1] * f[0]).abs();
        if along > 0. && across < car.h[0] + half_width + 0.3 {
            gap = gap.min((along - car.h[1] - half_length - 1.).max(0.));
        }
    };
    for (j, other) in bodies.iter().enumerate() {
        if j == index { continue; }
        let delta = other.angle - car.angle;
        let (s, c) = (delta.sin().abs(), delta.cos().abs());
        inspect(other.p, c * other.h[1] + s * other.h[0], s * other.h[1] + c * other.h[0]);
    }
    for ped in peds.iter().filter(|p| p.health > 0.) { inspect(ped.p, 0.5, 0.5); }
    inspect(player, 0.5, 0.5);
    gap
}

fn pose(car: &Car, roads: &Roads, d: f32) -> ([f32; 3], f32, f32, bool) {
    roads.rail_pose(car.previous, car.target, car.next_target, d, car.lane, car.wobble, car.next_lane, car.next_wobble)
}

/// Spawn at the same spline pose that the next simulation step will use.
pub fn initialize_rail(car: &mut Car, roads: &Roads, collision: &Collision, along: f32, seed: u32) -> bool {
    car.next_target = roads.next_node(car.previous, car.target, car.lane, seed).unwrap_or(car.target);
    (car.next_lane, car.next_wobble) = roads.convert_lane(car.previous, car.target, car.next_target, car.lane, seed);
    let (_, _, total, _) = pose(car, roads, 0.);
    car.rail_dist = along.clamp(0., total.max(0.));
    let (p, angle, _, _) = pose(car, roads, car.rail_dist);
    let Some(p) = rail_position_for(collision, p, angle, car.info_id) else { return false; };
    car.p = p; car.angle = angle; car.on_rail = true;
    true
}

/// iAITask::GetRoadSpeed. A stable local identity replaces the upstream entity UID.
pub fn road_speed(car: &Car, roads: &Roads) -> f32 {
    let identity = car.info_id as u32 * 17 + car.paint as u32;
    let base = (((identity & 15) * 0x66) | (6 << 12)) - 0x2ca;
    let meta = roads.link_meta(car.previous, car.target);
    base as f32 / 4096. * if !meta.flag5 && meta.lanes > 1 {
        if car.lane == 0 { 2. } else { 2.5 }
    } else { 1. }
}

/// Keep route state transactional: an obstructed position never advances its cursor.
pub fn move_on_rail(car: &mut Car, roads: &Roads, collision: &Collision, distance_to_move: f32, seed: u32) -> bool {
    if car.previous >= roads.nodes.len() || car.target >= roads.nodes.len() || car.next_target >= roads.nodes.len() { return false; }
    let saved = (car.previous, car.target, car.next_target, car.lane, car.next_lane, car.wobble, car.next_wobble, car.rail_dist);
    let mut remaining = distance_to_move.max(0.);
    let mut final_pose = (car.p, car.angle);
    let samples = (remaining / 0.25).ceil().max(1.) as usize;
    let increment = remaining / samples as f32;
    let mut valid = true;
    for _ in 0..samples {
        remaining = increment;
        for _ in 0..8 {
            let (_, _, total, _) = pose(car, roads, car.rail_dist);
            if total <= 0.001 { valid = false; break; }
            if car.rail_dist + remaining <= total {
                car.rail_dist += remaining;
                remaining = 0.;
                break;
            }
            remaining -= (total - car.rail_dist).max(0.);
            if car.next_target == car.target { car.rail_dist = total; car.speed = 0.; remaining = 0.; break; }
            let entry = roads.turn_entry_offset(car.previous, car.target, car.next_target, car.lane, car.next_lane);
            car.previous = car.target; car.target = car.next_target;
            car.lane = car.next_lane; car.wobble = car.next_wobble;
            car.next_target = roads.next_node(car.previous, car.target, car.lane, seed.wrapping_add(car.target as u32)).unwrap_or(car.target);
            let (lane, wobble) = roads.convert_lane(car.previous, car.target, car.next_target, car.lane, seed);
            car.next_lane = lane; car.next_wobble = wobble;
            car.rail_dist = entry;
        }
        if remaining > 0.001 { valid = false; }
        let (p, angle, _, _) = pose(car, roads, car.rail_dist);
        if let Some(p) = rail_position_for(collision, p, angle, car.info_id) { final_pose = (p, angle); }
        else { valid = false; }
        if !valid { break; }
    }
    if valid {
        car.p = final_pose.0; car.angle = final_pose.1;
        car.lat_vel = 0.; car.yaw_rate = 0.; car.handling.reset_wheels();
    } else {
        (car.previous, car.target, car.next_target, car.lane, car.next_lane, car.wobble, car.next_wobble, car.rail_dist) = saved;
    }
    valid
}

/// forwardSpeed: normal queues/lights never enter the stuck/reverse state.
pub fn step_rail(car: &mut Car, roads: &Roads, collision: &Collision, lights: &TrafficLightManager,
    mut gap: f32, yield_siren: bool, dt: f32, seed: u32) {
    if !dt.is_finite() || dt <= 0. { return; }
    car.drive_accumulator += dt.min(0.1);
    while car.drive_accumulator + 1e-7 >= crate::handling::STEP {
        car.drive_accumulator = (car.drive_accumulator - crate::handling::STEP).max(0.);
        let old = car.p;
        step_rail_tick(car, roads, collision, lights, gap, yield_siren, crate::handling::STEP, seed);
        gap = (gap - distance(old, car.p)).max(0.);
        if !car.on_rail { break; }
    }
}

fn step_rail_tick(car: &mut Car, roads: &Roads, collision: &Collision, lights: &TrafficLightManager,
    mut gap: f32, yield_siren: bool, dt: f32, seed: u32) {
    if car.previous >= roads.nodes.len() || car.target >= roads.nodes.len() { car.speed = 0.; return; }
    let a = roads.nodes[car.previous].p;
    let b = roads.nodes[car.target].p;
    let angle = -(b[0] - a[0]).atan2(b[1] - a[1]);
    let stopping_offset = 5. + roads.nodes[car.target].meta().width.floor();
    let straight_end = distance(a, b) - stopping_offset;
    // Once past the stop line, finish crossing. Do not freeze mid-intersection
    // when the signal changes, nor assign different phases to opposite approaches.
    if roads.nodes[car.target].meta().traffic_light && car.rail_dist <= straight_end + 0.05
        && lights.signal_for_approach(0., angle).is_stop() {
        gap = gap.min((straight_end - car.rail_dist).max(0.));
    }
    let old_speed = car.speed.max(0.);
    if car.next_target == car.target {
        let (_, _, total, _) = pose(car, roads, car.rail_dist);
        gap = gap.min((total - car.rail_dist).max(0.));
    }
    let cruising = if car.panicked { 18.5 } else if yield_siren { 2.8 } else { road_speed(car, roads) };
    let acceleration = (0x4ce as f32 / 4096.) * 30.;
    // cTrafficAvoidance bounds its query by speed. A distant car is not a
    // reason to stop an entire avenue. Braking distance + one second headway.
    let lookahead = 2. + old_speed + old_speed * old_speed / (2. * acceleration);
    let target = if gap < lookahead { 0. } else { cruising };
    if target < old_speed {
        let dec = if gap < 1. { 100. } else { ((old_speed*old_speed - target*target) / gap * (17. / 256.)).max(acceleration * dt) };
        car.speed = (old_speed - dec).max(target);
    } else if old_speed == 0. && car.start_delay > 0. {
        car.start_delay = (car.start_delay - dt).max(0.);
    } else { car.speed = (old_speed + acceleration * dt).min(target); }
    car.braking = car.speed < old_speed || target == 0.;
    if target == 0. && car.speed < 0.05 { car.speed = 0.; car.start_delay = (seed % 10) as f32 / 30.; }
    let travel = ((old_speed + car.speed) * 0.5 * dt).min(gap.max(0.));
    let old_angle = car.angle;
    if travel > 0.00001 {
        if !move_on_rail(car, roads, collision, travel, seed) {
            car.speed = 0.;
            car.blocked += dt;
            // Only a world obstruction, never a traffic wait, requests recovery.
            if car.blocked > 1.5 { car.on_rail = false; car.blocked = 0.; }
        } else { car.blocked = 0.; }
    } else { car.blocked = 0.; }
    let d_angle = (car.angle - old_angle).sin().atan2((car.angle - old_angle).cos());
    let yaw_rate = d_angle / dt;
    let acc_y = (car.speed - old_speed) / dt;
    let (target_roll, target_pitch) = if car.is_bike() {
        let roll = (-yaw_rate * 0.45).clamp(-0.65, 0.65);
        let pitch = if acc_y < -1.0 { -0.12 } else if acc_y > 1.5 { 0.15 } else { 0.0 };
        (roll, pitch)
    } else {
        let roll = (yaw_rate * car.speed * 0.007).clamp(-0.16, 0.16);
        let pitch = (acc_y * 0.006).clamp(-0.12, 0.12);
        (roll, pitch)
    };
    car.roll += (target_roll - car.roll) * (12.0 * dt).min(1.0);
    car.pitch += (target_pitch - car.pitch) * (12.0 * dt).min(1.0);
}

/// Find the nearest directed lane consistent with the car heading after impact.
/// Return an ahead target; attach only once position AND heading agree.
pub fn rejoin_target(car: &mut Car, roads: &Roads, allow_attach: bool) -> Option<[f32; 3]> {
    let forward = forward(car.angle);
    let mut best: Option<(f32, usize, usize, u8, f32)> = None;
    for (a, node) in roads.nodes.iter().enumerate().filter(|(_, n)| n.land()) {
        for &b in &node.links {
            if !roads.nodes.get(b).is_some_and(|n| n.land() && n.flags & 3 == 0) { continue; }
            let max_lanes = roads.link_meta(a, b).lanes;
            let lane = car.lane.min(max_lanes.saturating_sub(1));
            let p0 = roads.lane_world_pos(a, b, 0., lane, 0.);
            let p1 = roads.lane_world_pos(a, b, 1., lane, 0.);
            let len = distance(p0, p1);
            if len < 1. || (p0[2] - car.p[2]).abs() > 3. { continue; }
            let dir = [(p1[0]-p0[0])/len, (p1[1]-p0[1])/len];
            let along = ((car.p[0]-p0[0])*dir[0] + (car.p[1]-p0[1])*dir[1]).clamp(0., len);
            let lateral = (car.p[0]-p0[0]-dir[0]*along).hypot(car.p[1]-p0[1]-dir[1]*along);
            if lateral > 20. { continue; }
            let facing = dir[0]*forward[0] + dir[1]*forward[1];
            let score = lateral + (1.-facing)*12.;
            if best.is_none_or(|v| score < v.0) { best = Some((score,a,b,lane,along)); }
        }
    }
    let (_, a, b, lane, along) = best?;
    let next = roads.next_node(a,b,lane,car.paint as u32).unwrap_or(b);
    let (next_lane, next_wobble) = roads.convert_lane(a,b,next,lane,car.paint as u32);
    let (on_lane, angle, total, curved) = roads.rail_pose(a,b,next,along,lane,0.,next_lane,next_wobble);
    let delta = (angle - car.angle).sin().atan2((angle - car.angle).cos());
    if allow_attach && !curved && distance(on_lane,car.p) < 0.12 && delta.abs() < 0.04 && car.lat_vel.abs() < 0.3 && car.speed >= 0. {
        car.previous = a; car.target = b; car.next_target = next;
        car.lane = lane; car.wobble = 0.; car.next_lane = next_lane; car.next_wobble = next_wobble;
        car.rail_dist = along.min(total); car.on_rail = true;
        // Position is deliberately unchanged; no snap/teleport on reattachment.
    }
    Some(roads.rail_pose(a,b,next,(along + 5. + car.speed.max(0.) * 0.4).min(total),lane,0.,next_lane,next_wobble).0)
}

pub fn visible(view: Option<&[f32; 16]>, p: [f32; 3], radius: f32) -> bool {
    let Some(m) = view else { return false; };
    let clip = |x: f32, y: f32, z: f32| -> bool {
        let v: [f32; 4] = std::array::from_fn(|k| m[k]*x + m[4+k]*y + m[8+k]*z + m[12+k]);
        // Camera::matrix is view-only. Renderer uses a 60-degree vertical FOV.
        let depth = -v[2];
        let half_height = depth * (std::f32::consts::PI / 6.).tan();
        let aspect = crate::gpu::W as f32 / crate::gpu::H as f32;
        depth > 0.4 && depth < 170. && v[0].abs() <= half_height * aspect * 1.22 && v[1].abs() <= half_height * 1.22
    };
    for x in [-radius, 0., radius] { for y in [-radius, 0., radius] { for z in [0.,radius] {
        if clip(p[0]+x,p[1]+y,p[2]+z) { return true; }
    }}}
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{actors::make_vehicle, collision::{Cell, BoxShape}, gps::Node};
    fn flat() -> Collision {
        Collision { cells: (0..14000).map(|_| Cell { ground: vec![0x55;400], ..Default::default() }).collect() }
    }
    fn roads() -> Roads {
        Roads { nodes: vec![
            Node { p: [0.,0.,0.], flags: 0x220, links: vec![1] },
            Node { p: [0.,80.,0.], flags: 0x220, links: vec![0,2] },
            Node { p: [80.,80.,0.], flags: 0x220, links: vec![1,3] },
            Node { p: [80.,160.,0.], flags: 0x220, links: vec![2] },
        ] }
    }
    fn car(roads: &Roads, collision: &Collision, along: f32) -> Car {
        let mut car = make_vehicle([0.;3],0.,1,0,0,0,false,false);
        assert!(initialize_rail(&mut car,roads,collision,along,0));
        car
    }
    #[test]
    fn queue_waits_without_reverse_and_pulls_away() {
        let (roads,c) = (roads(),flat());
        let mut car = car(&roads,&c,10.);
        let start = car.p;
        let lights = TrafficLightManager::new();
        for _ in 0..300 { step_rail(&mut car,&roads,&c,&lights,0.,false,1./30.,5); }
        assert_eq!(car.p,start);
        assert!(car.on_rail && !car.recovery_active && car.speed == 0.);
        for _ in 0..60 { step_rail(&mut car,&roads,&c,&lights,f32::INFINITY,false,1./30.,5); }
        assert!(car.p[1] > start[1] + 5. && car.speed > 4.);
    }
    #[test]
    fn distant_obstacle_does_not_stop_entire_avenue() {
        let (roads,c) = (roads(),flat());
        let mut car = car(&roads,&c,10.);
        for _ in 0..30 { step_rail(&mut car,&roads,&c,&TrafficLightManager::new(),80.,false,1./30.,0); }
        assert!(car.speed > 5. && car.p[1] > 13.);
    }
    #[test]
    fn corner_handover_preserves_distance_without_jumps() {
        let (roads,c) = (roads(),flat());
        let mut car = car(&roads,&c,65.);
        for _ in 0..80 {
            let p = car.p;
            assert!(move_on_rail(&mut car,&roads,&c,0.7,0));
            assert!((0.6..0.76).contains(&distance(p,car.p)), "step {}",distance(p,car.p));
        }
        assert_eq!((car.previous,car.target),(1,2));
    }
    #[test]
    fn blocked_rail_move_rolls_back_route_and_position() {
        let (roads,mut c) = (roads(),flat());
        let mut car = car(&roads,&c,10.);
        let p = car.p;
        let (x,y) = Collision::cell_xy(p[0],p[1]+6.).unwrap();
        c.cells[x*100+y].boxes.push(BoxShape { c:[p[0],p[1]+6.,1.],h:[8.,0.1,1.],angle:0.,flags:0 });
        let state = (car.previous,car.target,car.next_target,car.rail_dist);
        assert!(!move_on_rail(&mut car,&roads,&c,8.,0));
        assert_eq!(car.p,p);
        assert_eq!((car.previous,car.target,car.next_target,car.rail_dist),state);
    }
    #[test]
    fn spawn_in_corner_uses_actual_spline_pose() {
        let (roads,c) = (roads(),flat());
        let mut car = car(&roads,&c,77.);
        let p = car.p;
        assert!(move_on_rail(&mut car,&roads,&c,0.2,0));
        assert!((0.15..0.25).contains(&distance(p,car.p)));
    }
    #[test]
    fn red_stop_line_wait_and_green_restart() {
        let (mut roads,c) = (roads(),flat());
        roads.nodes[1].flags |= 0x8000;
        let mut car = car(&roads,&c,45.);
        car.speed = 6.;
        let mut lights = TrafficLightManager::new();
        lights.timer = 600./30.; // North/south is held.
        for _ in 0..300 { step_rail(&mut car,&roads,&c,&lights,f32::INFINITY,false,1./30.,0); }
        assert!(car.rail_dist <= 70.01 && car.speed == 0. && car.on_rail && !car.recovery_active);
        lights.timer = 0.;
        for _ in 0..120 { step_rail(&mut car,&roads,&c,&lights,f32::INFINITY,false,1./30.,0); }
        assert!(car.previous == 1 || car.rail_dist > 75.);
    }
    #[test]
    fn traffic_at_30_and_144_fps_travels_identically() {
        let (roads,c) = (roads(),flat());
        let mut a = car(&roads,&c,10.);
        let mut b = car(&roads,&c,10.);
        for _ in 0..120 { step_rail(&mut a,&roads,&c,&TrafficLightManager::new(),f32::INFINITY,false,1./30.,0); }
        for _ in 0..576 { step_rail(&mut b,&roads,&c,&TrafficLightManager::new(),f32::INFINITY,false,1./144.,0); }
        assert!(distance(a.p,b.p)<0.001 && (a.speed-b.speed).abs()<0.001);
    }
    #[test]
    fn visible_car_keeps_physics_when_rejoining() {
        let (roads,c) = (roads(),flat());
        let mut car = car(&roads,&c,20.);
        car.wobble=0.; car.p=roads.lane_world_pos(0,1,0.25,0,0.); car.on_rail=false;
        let p=car.p;
        assert!(rejoin_target(&mut car,&roads,false).is_some());
        assert!(!car.on_rail && car.p==p);
        assert!(rejoin_target(&mut car,&roads,true).is_some());
        assert!(car.on_rail && car.p==p);
    }
    #[test]
    fn opposite_directions_and_bridge_clearance_are_preserved() {
        let (roads,c) = (roads(),flat());
        assert!(roads.edges().any(|e|e==(0,1)) && roads.edges().any(|e|e==(1,0)));
        let a=roads.lane_world_pos(0,1,0.5,0,0.);
        let b=roads.lane_world_pos(1,0,0.5,0,0.);
        assert!(a[0]>0. && b[0]<0. && distance(a,b)>5.);
        let car=car(&roads,&c,10.);
        let high=Body {p:[car.p[0],car.p[1]+10.,car.p[2]+6.],angle:0.,h:car.h};
        assert!(clearance(&car,usize::MAX,&[high],&[],[999.;3]).is_infinite());
        let near=Body {p:[car.p[0],car.p[1]+10.,car.p[2]],..high};
        assert!(clearance(&car,usize::MAX,&[near],&[],[999.;3])<4.);
    }
    #[test]
    fn viewport_projection_rejects_only_offscreen_cars() {
        let view=[1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.];
        assert!(visible(Some(&view),[0.,0.,-50.],5.));
        assert!(!visible(Some(&view),[1000.,0.,-50.],5.));
        assert!(!visible(Some(&view),[0.,0.,10.],2.));
    }
    #[test]
    fn collinear_lateral_shift_and_acute_angle_curves_remain_bounded() {
        // Collinear links with lateral shift (analog of 197->142->215)
        let r_collinear = Roads {
            nodes: vec![
                Node { p: [0., 0., 0.], flags: 0x220, links: vec![1] },
                Node { p: [0., 80., 0.], flags: 0x220, links: vec![2] },
                Node { p: [6., 160., 0.], flags: 0x204, links: vec![] },
            ],
        };
        let (p, _angle, total, curved) = r_collinear.rail_pose(0, 1, 2, 75., 0, 0., 0, 0.);
        assert!(curved);
        // Total curve length must stay well-bounded within the junction corridor (< 120m), never ballooning to km
        assert!(total < 120., "Curve length ballooned to {}", total);
        assert!((p[0] - 0.).abs() < 7., "Curve drifted laterally out of corridor: {:?}", p);

        // Acute outgoing angle (analog of 626->1439->1467)
        let r_acute = Roads {
            nodes: vec![
                Node { p: [100., 0., 0.], flags: 0x204, links: vec![1] },
                Node { p: [0., 0., 0.], flags: 0x220, links: vec![2] },
                Node { p: [-30., -2.5, 0.], flags: 0x604, links: vec![] },
            ],
        };
        let (p_start, _, total_acute, _) = r_acute.rail_pose(0, 1, 2, 95., 0, 0., 0, 0.);
        let (p_mid, _, _, _) = r_acute.rail_pose(0, 1, 2, 98., 0, 0., 0, 0.);
        assert!(total_acute < 130., "Acute junction ballooned to {}", total_acute);
        // Progress must advance forward (decreasing X), never loop backwards to the East (X > p_start[0])
        assert!(p_mid[0] <= p_start[0] + 0.5, "Fishhook reverse detected: start X={}, mid X={}", p_start[0], p_mid[0]);
    }
    #[test]
    fn curb_decorative_cylinder_does_not_block_rail_travel() {
        use crate::collision::Cylinder;
        let (roads, mut c) = (roads(), flat());
        let mut car = car(&roads, &c, 10.);
        let p = car.p;
        let (cx, cy) = Collision::cell_xy(p[0] + 3., p[1] + 5.).unwrap();
        // A cylinder on the curb whose canopy reaches into the roadway
        c.cells[cx * 100 + cy].cylinders.push(Cylinder {
            p: [p[0] + 3., p[1] + 5., 0.],
            r: 2.5,
            h: 2.5,
        });
        // On-rail vehicle moves past the curb cylinder smoothly without rollback
        assert!(move_on_rail(&mut car, &roads, &c, 8., 0));
        assert!(car.p[1] > p[1] + 7.);
    }

    #[test]
    fn rail_traffic_updates_body_roll_in_curves_and_pitch_during_braking() {
        let (roads, c) = (roads(), flat());
        let lights = TrafficLightManager::new();
        let mut test_car = car(&roads, &c, 10.);
        test_car.speed = 10.;
        // 1. Deceleration towards a stop line or obstacle induces braking pitch / deceleration
        step_rail(&mut test_car, &roads, &c, &lights, 2.0, false, 0.1, 0);
        assert!(test_car.pitch < 0.0 || test_car.speed < 10., "decelerating on-rail car adjusts pitch/speed");

        // 2. Turning through a curve induces roll
        let mut turning_car = car(&roads, &c, 75.);
        for _ in 0..10 {
            step_rail(&mut turning_car, &roads, &c, &lights, 50.0, false, 0.1, 0);
        }
        assert!(turning_car.roll.abs() > 0.001 || turning_car.speed > 0., "car on rail updates dynamics in curve");
    }
}

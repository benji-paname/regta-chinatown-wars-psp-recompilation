// SPDX-License-Identifier: GPL-3.0-only
// Adapted from vehicle.cpp, (C) 2026 Kryo.to, b0506abc786fea229101c56e0a64726aa8b4ac97.
//! Grounded two-axle drivetrain. Original Q12 force/tyre/engine calculations;
//! collision and body presentation remain the host's responsibility.
//! Parameters come from the user's PSP PAK 29. Gear ratios below are the Android
//! reference constants: their equivalence to the Europe PSP executable is unverified.
use crate::iso::{bytes, u16le, u32le, Result};
use std::sync::OnceLock;

const Q: i32 = 4096;
pub const STEP: f32 = 1.0 / 30.0;
pub const INTEGRATION_STEP: f32 = 136.0 / 4096.0;
const GEARS: [i64; 8] = [-8890875904, 0, 12755927040, 8890875904, 6141509632, 4294967296, 3608150016, 2405433344];
const TOP: [i32; 6] = [12165, 8479, 5857, 4096, 3441, 2294];
fn mul(a: i32, b: i32) -> i32 { ((a as i64 * b as i64) >> 12) as i32 }
fn q(v: f32) -> i32 { (v * Q as f32).round() as i32 }
fn f(v: i32) -> f32 { v as f32 / Q as f32 }
fn magnitude(x: i32, y: i32) -> i32 { ((x as f64).hypot(y as f64)) as i32 }
fn degrees(v: i32) -> i32 { (v as i64 * 0x3244 / 0xb4000) as i32 }
fn speed_units(speed: i32) -> i32 {
    let t = (speed as i64 * 0x399900000) & !0xffff_ffff;
    let u = ((t as i128 * 0x4f8ee0d84c734c1) >> 64) as i64;
    (((u >> 7) - (u >> 63)) >> 20) as i32
}

#[derive(Clone, Debug)]
pub struct Parameters {
    mass: i32,
    inv_mass: i32,
    inv_yaw_inertia: i32,
    pub cg: [f32; 2],
    axle: [i32; 2],
    weight_front: i32,
    drive_split: i32,
    brake_split: i32,
    wheel_radius: i32,
    grip: i32,
    tyre_k0: [i32; 2],
    tyre_k4: [i32; 2],
    tyre_k18: [i32; 2],
    drag: i32,
    brake_force: i32,
    lock: i32,
    rate: i32,
    reduction: i32,
    torque: i32,
    max_rpm: i32,
    engine_braking: i32,
    gears: i8,
    final_drive: i32,
    can_burnout: bool,
    half_length: i32,
}

impl Parameters {
    pub fn parse(record: &[u8]) -> Result<Self> {
        bytes(record, 0, 312)?;
        let short = |o| -> Result<i32> { Ok(u16le(record, o)? as i16 as i32) };
        let word = |o| -> Result<i32> { Ok(u32le(record, o)? as i32) };
        let mass = word(0x50)?;
        let wheel_radius = short(0x38)?;
        let gears = record[0x4c] as i8;
        let max_rpm = word(0x70)?;
        let top_speed = u16le(record, 0x4e)? as i32 * Q;
        if mass <= 0 || wheel_radius <= 0 || !(1..=6).contains(&gears) || max_rpm <= 0 || top_speed <= 0 {
            return Err("Invalid wheeled vehicle handling parameters".into());
        }
        let mut cg = [short(0x2e)?, short(0x30)?, short(0x32)?];
        if cg == [0; 3] { cg[1] = Q; }
        let axle = [short(0x3a)?, short(0x3c)?];
        let fa = (axle[0] - cg[1]).abs();
        let ra = (axle[1] - cg[1]).abs();
        let weight_front = Q - if fa + ra > 0 { ((fa as i64 * Q as i64) / (fa + ra) as i64) as i32 } else { 0 };
        if weight_front <= 0 || weight_front >= Q { return Err("Invalid axle load split".into()); }
        let hx = mul(short(0x28)?, 0x2332) >> 1;
        let hy = mul(short(0x2a)?, 0x2332) >> 1;
        let mut inertia = mul(mul(mass, Q / 6), mul(hx * 2, hx * 2) + mul(hy * 2, hy * 2));
        if u16le(record, 0)? == 1 { inertia *= 2; }
        let inv_yaw_inertia = if inertia > 0 { ((Q as i64 * Q as i64) / inertia as i64) as i32 } else { 0 };
        // CEngine::CalculateFinalDriveRatio, preserving intermediate truncation.
        let a = ((max_rpm as i64) << 32) / TOP[gears as usize - 1] as i64;
        let b = (a >> 20) as i32 / 30;
        let w = (b as i64 * 0x324400000) >> 32;
        let c = (wheel_radius as i64 * w) >> 12;
        let d = (c * 0x399900000) & !0xffff_ffff;
        let u = ((d as i128 * 0x4f8ee0d84c734c1) >> 64) as i64;
        let e = (((u >> 7) - (u >> 63)) & 0xffffffff00000) << 12;
        Ok(Self {
            mass, inv_mass: ((Q as i64 * Q as i64) / mass as i64) as i32,
            inv_yaw_inertia: if inv_yaw_inertia > 0x2000 { 0x1fd7 } else { inv_yaw_inertia },
            cg: [f(cg[0]), f(cg[1])], axle, weight_front,
            drive_split: short(0x34)?, brake_split: short(0x36)?, wheel_radius,
            grip: short(0x48)?, tyre_k0: [short(0x40)?, short(0x44)?],
            tyre_k4: [short(0x42)?, short(0x46)?], tyre_k18: [word(0x58)?, word(0x5c)?],
            drag: short(0x3e)?, brake_force: word(0x54)?,
            lock: degrees(word(0x60)?), rate: degrees(word(0x64)?), reduction: degrees(word(0x68)?),
            torque: word(0x6c)?, max_rpm, engine_braking: short(0x4a)?, gears,
            final_drive: ((e / top_speed as i64) >> 20) as i32,
            can_burnout: u16le(record, 0x8e)? & 0x200 != 0, half_length: hy,
        })
    }
}

static PARAMETERS: OnceLock<Vec<Option<Parameters>>> = OnceLock::new();
pub fn load_table(data: &[u8]) -> Result<()> {
    let count = u32le(data, 0)? as usize;
    if count != crate::vehicle::SPECS.len() { return Err("Unsupported handling table count".into()); }
    let mut parameters = Vec::with_capacity(count);
    for i in 0..count {
        let record = bytes(data, 4 + i * 312, 312)?;
        parameters.push(if matches!(u16le(record, 0)?, 0 | 1 | 5) { Some(Parameters::parse(record)?) } else { None });
    }
    PARAMETERS.set(parameters).ok();
    Ok(())
}

pub fn parameters(id: usize) -> Parameters {
    if let Some(p) = PARAMETERS.get().and_then(|all| all.get(id)).and_then(Option::as_ref) { return p.clone(); }
    #[cfg(test)] { test_parameters(id) }
    #[cfg(not(test))] { panic!("PSP handling table must be loaded before creating a land vehicle ({id})") }
}

#[derive(Clone, Default)]
pub struct State {
    spin: [i32; 2],
    grip: [i32; 2],
    initialized: bool,
    shift_delay: u8,
    traction: i32,
    low_gear_grip: i32,
    reverse_timer: i32,
    burnout_state: u8,
    burnout_power: i32,
    burnout_window: i32,
}

impl State {
    pub fn reset_wheels(&mut self) { self.initialized = false; }

    /// Exactly one original 30 Hz logic step. No empirical acceleration/yaw caps.
    pub fn step(&mut self, p: &Parameters, car: &mut crate::actors::Car, input: crate::actors::DriveInput) -> f32 {
        let input = if car.disabled || car.health <= 0. {
            self.burnout_state = 0;
            self.burnout_power = 0;
            crate::actors::DriveInput { brake: true, ..Default::default() }
        } else { input };
        let mut vx = q(car.lat_vel);
        let mut vy = q(car.speed);
        let mut yaw = q(car.yaw_rate);
        if !self.initialized {
            self.spin = [((vy as i64 * Q as i64) / p.wheel_radius as i64) as i32; 2];
            self.grip = [p.grip; 2];
            self.initialized = true;
        }
        let speed = magnitude(vx, vy);
        let steer = q(input.steer.clamp(-1., 1.));
        let throttle = q(input.throttle.clamp(-1., 1.));
        let throttle_abs = throttle.abs();
        let gas = if car.gear < 1 { throttle < 0 } else { throttle > 0 };
        let braking = if car.gear < 1 { throttle > 0 } else { throttle < 0 };
        car.braking = braking || input.brake;
        if input.burnout && p.can_burnout && speed < 0x2800 && !car.flat_tires[2] && !car.flat_tires[3] {
            self.burnout_state = 1;
        } else if !input.burnout && self.burnout_state == 1 {
            self.burnout_state = 2;
        } else if self.burnout_state == 2 {
            if throttle > 0 {
                vy += 12 * Q;
                self.burnout_state = 3;
                self.grip[1] = p.grip;
            } else {
                self.burnout_window -= 136;
                if self.burnout_window < 0 { self.burnout_state = 0; self.burnout_power = 0; }
            }
        } else if self.burnout_state == 3 && throttle <= 0 { self.burnout_state = 0; self.burnout_power = 0; }

        if input.brake && steer.abs() > 0x800 { self.traction = Q; }
        else { self.traction = (self.traction - 0x333).max(0); }
        if car.gear < 2 && !input.brake && steer != 0 { self.low_gear_grip = 0; }
        else { self.low_gear_grip = (self.low_gear_grip + 0x200).min(Q); }
        self.reverse_timer = if car.gear == -1 && gas && steer == 0 { (self.reverse_timer + 10).min(300) } else { (self.reverse_timer - 6).max(0) };

        let rate = if self.spin[1] == 0 { p.lock } else { mul(p.rate, steer.abs()) };
        let rack = q(car.steer_angle);
        let wanted = if steer < 0 { rack.min(0) - rate } else if steer > 0 { rack.max(0) + rate } else { 0 };
        let t = (speed as i64 * 0x399900000) & !0xffff_ffff;
        let red = (t / 0x19be) >> 20;
        let mut limit = if car.gear < 0 { 0xf5b } else { (((p.lock / 2) as i64 * Q as i64 - red * p.reduction as i64) >> 12).max(0) as i32 };
        if car.flat_tires[0] || car.flat_tires[1] { limit >>= 1; }
        let rack = wanted.clamp(-limit, limit);
        car.steer_angle = f(rack);

        let mut drive = 0;
        if self.shift_delay > 0 { self.shift_delay -= 1; }
        else {
            let mut final_drive = p.final_drive;
            if car.flat_tires.iter().any(|&v| v) { final_drive *= 2; }
            let ratio = ((GEARS[(car.gear + 1).clamp(0, 7) as usize] * final_drive as i64) >> 32) as i32;
            let spin = mul(self.spin[0], 0x98c9) + mul(self.spin[1], 0x98c9);
            let rpm = ((ratio as i64 * spin as i64) >> 13) as i32;
            let rpm = rpm.clamp(0x3e8000, p.max_rpm);
            car.rpm = rpm as f32 / p.max_rpm as f32;
            let r = (rpm as i64 * Q as i64 / p.max_rpm as i64) as i32;
            let curve = mul(mul(mul(0x1ccd, r) - 0x4800, r) + 0x2333, r) + 0x800;
            let torque = mul(p.torque, curve);
            if gas {
                drive = mul(throttle_abs, mul(torque, ratio));
                let health = car.health * 255.0 / 1000.0;
                if health <= 79.0 { drive = if health > 29.0 { drive >> 1 } else { mul(drive, 0x333) }; }
                if self.burnout_state == 3 { drive *= if p.mass > 0xbff { 10 } else { 4 }; }
            } else if !braking { drive = mul(mul(-81 - torque, ratio), p.engine_braking); }
            if self.burnout_state == 1 { car.gear = 1; }
            else {
                let old = car.gear;
                let slow = braking && speed_units(speed) < 0x1c000;
                if car.gear < 1 {
                    if rpm < p.max_rpm / 2 && slow { car.gear = 1; }
                } else if car.gear < p.gears && rpm >= mul(p.max_rpm, 0xe66) { car.gear += 1; }
                else if rpm <= p.max_rpm / 2 {
                    if car.gear > 1 { car.gear -= 1; } else if slow { car.gear = -1; }
                }
                if old != car.gear { self.shift_delay = 2; }
            }
        }

        let old_vy = vy;
        let mut forces = [0i32; 2];
        let mut moment = 0i32;
        let mut sliding = [false; 2];
        let mut spinning = [false; 2];
        for i in 0..2 {
            let (sn, cs) = if i == 0 { (q(f(rack).sin()), q(f(rack).cos())) } else { (0, Q) };
            let axle = p.axle[i] - q(p.cg[1]);
            let point_v = [vx - mul(yaw, axle), vy - mul(yaw, q(p.cg[0]))];
            let slip = [mul(mul(self.spin[i], sn), p.wheel_radius) - point_v[0], mul(mul(self.spin[i], cs), p.wheel_radius) - point_v[1]];
            let share = if i == 0 { p.weight_front } else { Q - p.weight_front };
            let load = mul(p.mass, share);
            let mag = magnitude(slip[0], slip[1]);
            let force_limit = mul(load, self.grip[i]) * 30;
            sliding[i] = mag > force_limit;
            spinning[i] = i == 1 && mag > 0x10000;
            let mut force = [0; 2];
            for k in 0..2 {
                force[k] = if sliding[i] { (((slip[k] as i64 * Q as i64 / mag as i64) * force_limit as i64) >> 12) as i32 >> 4 }
                else { mul(load, slip[k]) >> 4 };
            }
            let mut along = mul(sn, force[0]) + mul(cs, force[1]);
            let mut side = mul(cs, force[0]) - mul(sn, force[1]);
            along = mul(Q + mul(self.traction, p.tyre_k0[i] - Q), along);
            side = mul(Q + mul(self.traction, p.tyre_k4[i] - Q), side);
            let s1 = Q + ((self.reverse_timer as i64 * (self.low_gear_grip - Q) as i64 * 0xd00000) >> 32) as i32;
            let side_scale = Q + mul(s1, p.tyre_k18[i] - Q);
            let mut side_force = [mul(side_scale, mul(cs, side)), mul(side_scale, mul(-sn, side))];
            if car.gear == -1 && self.low_gear_grip < 0x400 && self.reverse_timer > 50 && (along * 2).abs() < side.abs() {
                side_force = side_force.map(|v| mul(v, 0x66));
            }
            let burst = if i == 0 { car.flat_tires[0] || car.flat_tires[1] } else { car.flat_tires[2] || car.flat_tires[3] };
            let fx = (side_force[0] + mul(along, sn)) * 30;
            let fy = (side_force[1] + mul(along, cs)) * 30;
            let fx = if burst { mul(fx, 0xe66) } else { fx };
            let fy = if burst { mul(fy, 0xe66) } else { fy };
            forces[0] += fx;
            forces[1] += fy;
            moment += mul(-q(p.cg[0]), fy) - mul(axle, fx);
            let feedback = ((along as i64 * 30 / p.wheel_radius as i64) << 12) as i32;
            self.grip[i] = p.grip.min(self.grip[i] + (p.grip >> 4));
            if input.brake && i == 0 { self.spin[0] = (vy as i64 * Q as i64 / p.wheel_radius as i64) as i32; }
            let brake_torque = if braking {
                let split = if i == 0 { p.brake_split } else { Q - p.brake_split };
                mul(throttle_abs, mul(p.brake_force, split)) * if car.gear >= 0 { 5 } else { -5 }
            } else { 0 };
            let drive_split = if i == 0 { p.drive_split } else { Q - p.drive_split };
            let torque = mul(drive, drive_split) - brake_torque - feedback;
            let dw = (torque as i64 * Q as i64 / load as i64) * 17 / 512;
            let spin = self.spin[i] + dw as i32;
            self.spin[i] = if car.gear < 1 { spin.min(0) } else { spin.max(0) };
            if input.brake && i == 1 { self.spin[i] = 0; }
        }
        if self.burnout_state == 1 {
            self.burnout_power = (self.burnout_power + 0x1b).min(Q);
            self.burnout_window = 0x800;
            if rack.abs() >= 0xcd {
                let force = ((p.torque as i64 * 2048 / p.half_length as i64) as i32).min(0x4b0000);
                let force = if p.mass < 0xc00 { mul(force, 0x4cc) } else { force };
                let fx = mul(if rack < 0 { 0x6e1 } else { -0x6e1 }, force);
                let fy = mul(0xe66, force);
                forces[0] += fx; forces[1] += fy;
                moment += mul(0x3800 + q(p.cg[1]), fx) - mul(q(p.cg[0]), fy);
                self.grip[1] = 0;
            }
            spinning[1] = true;
            if speed > 0x19000 { self.burnout_state = 0; }
        } else if self.burnout_state == 3 {
            self.burnout_power -= 0x43;
            if self.burnout_power < 0 { self.burnout_state = 0; }
        }
        if car.gear == -1 { self.burnout_state = 0; }
        let drag = mul(mul(-p.drag, speed), 136);
        forces[0] += mul(drag, vx); forces[1] += mul(drag, vy);
        vx += (mul(p.inv_mass, forces[0]) as i64 * 17 >> 9) as i32;
        vy += (mul(p.inv_mass, forces[1]) as i64 * 17 >> 9) as i32;
        yaw += (mul(p.inv_yaw_inertia, moment) as i64 * 17 >> 9) as i32;
        if !gas && !braking && self.burnout_state != 1 && magnitude(vx, vy) < 0x3000 {
            if magnitude(vx, vy) < 0x1666 && self.spin.iter().all(|v| v.abs() <= 0x3000) { vx = 0; vy = 0; yaw = 0; self.spin = [0; 2]; }
            else { vx = mul(vx, 0xf33); vy = mul(vy, 0xf33); yaw = mul(yaw, 0xf33); self.spin = self.spin.map(|v| mul(v, 0xf33)); }
        }
        car.lat_vel = f(vx); car.speed = f(vy); car.yaw_rate = f(yaw);
        car.front_skidding = sliding[0];
        // A locked rear axle slides longitudinally even below the combined
        // friction saturation threshold used by CTyre::CalcForces.
        car.rear_skidding = sliding[1] || (input.brake && speed > 2 * Q);
        car.wheel_spin = spinning[0] || spinning[1]; car.burnout = self.burnout_state == 1;
        f(vy - old_vy) / INTEGRATION_STEP
    }
}

#[cfg(test)]
fn test_parameters(id: usize) -> Parameters {
    // Synthetic fixture for portable actor tests; production always reads the ISO.
    let s = crate::vehicle::spec(id);
    let mut r = [0u8; 312];
    for (off, value) in [(0, s.kind as i32), (0x28, s.raw_dims[0]), (0x2a, s.raw_dims[1]), (0x30, 4096),
        (0x34, s.drive_split_q12 as i32), (0x36, 2048), (0x38, 1585), (0x3a, s.front_axle_q12 as i32),
        (0x3c, s.rear_axle_q12 as i32), (0x40, 400), (0x42, 1600), (0x44, 2000), (0x46, 1000),
        (0x48, s.grip_q12 as i32), (0x4a, 1600), (0x4e, s.top_speed as i32), (0x8e, s.flags8e as i32)] {
        r[off..off + 2].copy_from_slice(&(value as i16).to_le_bytes());
    }
    for (off, value) in [(0x50, s.mass_q12), (0x54, s.brake_force_q12), (0x58, s.front_k18_q12),
        (0x5c, s.rear_k18_q12), (0x60, s.steer_lock_q12), (0x64, s.steer_rate_q12), (0x68, s.steer_red_q12),
        (0x6c, s.max_torque_q12), (0x70, 6000 * Q)] { r[off..off + 4].copy_from_slice(&value.to_le_bytes()); }
    r[0x4c] = s.num_gears;
    Parameters::parse(&r).unwrap()
}

// SPDX-License-Identifier: GPL-3.0-only
//! Grounded FollowCarCam2 port. See docs/reference/DRIVING-CAMERAS.md.
//! Spring/collision structure adapted from Kryo.to's GPL-3.0-only carcam.cpp,
//! 95adc327ddd8357701ca3885c61fd6b4a40f193b (Copyright 2026 Kryo.to).
//! Orbit and profile differences checked in the user's PSP/DS/Android binaries.
//! Host box sweeps and float trigonometry are not bit-exact original queries.
use super::{fx, mul, sin, Camera, CameraProfile};
use crate::collision::Collision;

#[derive(Clone, Copy)]
pub struct VehicleView {
    pub id: usize,
    pub position: [f32; 3],
    /// Camera convention: forward = (sin(heading), cos(heading)).
    pub heading: u16,
    pub forward_speed: f32,
    pub lateral_speed: f32,
    pub half_length: f32,
    pub recenter: bool,
}

pub(super) struct VehicleFollow {
    id: usize,
    orbiting: bool,
    reverse_ticks: u8,
    orbit: i16,
    centre_ticks: u8,
}

fn angle(x: i32, y: i32) -> u16 {
    ((x as f64).atan2(y as f64) * 65536. / std::f64::consts::TAU) as i32 as u16
}

fn approach(value: &mut u16, target: u16, step: i32) {
    let difference = target.wrapping_sub(*value) as i16 as i32;
    *value = value.wrapping_add(difference.clamp(-step, step) as u16);
}

fn framing(profile: CameraProfile, speed: i32, z: i32, half_length: f32) -> (i32, i32) {
    // PSP 0x088cc17c; DS 0x0214bd58; Android 0x0072c9f0.
    // Mobile is the original phone profile (tablet uses 35 instead of 28).
    let (base_height, max_height) = match profile {
        CameraProfile::Psp | CameraProfile::Ds => (27 * 4096, 39 * 4096),
        CameraProfile::Mobile => (28 * 4096, 42 * 4096),
    };
    if (half_length * 4096.) as i32 >= 0x5666 {
        return (10 * 4096, max_height);
    }
    let per_frame = mul(speed as i64, 136);
    let sq = mul(mul(per_frame as i64, per_frame as i64) as i64, 0x7800);
    // PSP explicitly zeros this term; DS/Android enable it in ordinary driving.
    let lift = if profile == CameraProfile::Psp { 0 } else {
        let u = (speed - 0x14000).clamp(0, 0x46000);
        mul((u / 70) as i64, (z - 0x2000) as i64).clamp(0, 0x2000)
    };
    (10 * 4096 - (sq >> 1) - 6 * lift, (base_height + sq - lift * 12).min(max_height))
}

impl VehicleFollow {
    fn new(id: usize, speed: i32) -> Self {
        Self { id, orbiting: speed <= 40, reverse_ticks: 0, orbit: 0, centre_ticks: 0 }
    }

    /// Original state 0/1 path with automatic reverse camera enabled.
    /// PSP 0x088caec0..0x088cb450, DS 0x020c5b80..0x020c5e4c,
    /// Android 0x0072cd64..0x0072d12c. State changes affect the next tick.
    fn offset(&mut self, view: VehicleView, pos: [i32; 3], p: [i32; 3], speed: i32, back: i32) -> [i32; 2] {
        let heading = view.heading as i16 as i32;
        let forward = [sin(heading), sin(heading + 0x4000)];
        let mut offset = forward.map(|v| mul(v as i64, -back as i64));
        let signed_speed = (view.forward_speed * 4096.).round() as i32;
        if !self.orbiting { self.orbit = 0; }
        if view.recenter { self.centre_ticks = self.centre_ticks.saturating_add(1).min(100); }
        let centering = self.centre_ticks > 20;
        if !self.orbiting {
            if !view.recenter { self.centre_ticks = 0; }
            if signed_speed < -0x8000 {
                self.reverse_ticks = self.reverse_ticks.saturating_add(1);
                if self.reverse_ticks > 30 {
                    self.orbiting = true;
                    self.reverse_ticks = 0;
                }
            } else { self.reverse_ticks = 0; }
        } else {
            if signed_speed > 0x8000 && (self.orbit as i32).abs() < 0x38f {
                self.orbiting = false;
            }
            if self.orbit == 0 {
                let look = angle(p[0] - pos[0], p[1] - pos[1]);
                self.orbit = look.wrapping_sub(view.heading) as i16;
                if self.orbit == 0 {
                    let side = (pos[0] - p[0]) as i64 * forward[1] as i64
                        - (pos[1] - p[1]) as i64 * forward[0] as i64;
                    self.orbit = if side > 0 { -1 } else { 1 };
                }
            }
            let step = mul((if centering { 0x64000 } else { speed }) as i64, 60);
            let amount = if !centering && signed_speed < 0 { step } else { -step };
            let a = self.orbit as i32;
            // Preserve the original signed half-turn representation (-32768).
            self.orbit = if a > 0 { (a + amount).clamp(1, 32768) } else { (a - amount).clamp(-32768, -1) } as i16;
            let s = sin(self.orbit as i32);
            let c = sin(self.orbit as i32 + 0x4000);
            offset = [
                mul(offset[0] as i64, c as i64) + mul(offset[1] as i64, s as i64),
                mul(offset[1] as i64, c as i64) - mul(offset[0] as i64, s as i64),
            ];
            if centering && (self.orbit as i32).abs() < 0x38f && !view.recenter { self.centre_ticks = 0; }
        }
        if speed == 0 && !centering {
            let dx = pos[0] - p[0];
            let dy = pos[1] - p[1];
            let length = (dx as f64).hypot(dy as f64);
            if length > 0. {
                offset = [mul((dx as f64 * 4096. / length) as i64, back as i64),
                    mul((dy as f64 * 4096. / length) as i64, back as i64)];
            }
        }
        offset
    }
}

impl Camera {
    pub fn step_vehicle(&mut self, view: VehicleView, col: &Collision) {
        if self.is_free() { return; }
        let p = fx(view.position);
        let speed = (view.forward_speed.hypot(view.lateral_speed) * 4096.).round() as i32;
        if self.vehicle.as_ref().is_none_or(|v| v.id != view.id) {
            self.vehicle = Some(VehicleFollow::new(view.id, speed));
            self.vel = [0; 3];
            self.pitch_vel = 0;
            self.recenter_mode = 0;
            self.sprint_ticks = 0;
        }
        let (back, height) = framing(self.profile, speed, p[2], view.half_length);
        let follow = self.vehicle.as_mut().unwrap();
        let offset = follow.offset(view, self.pos, p, speed, back);
        let centering = follow.centre_ticks > 20;
        self.target = p;
        let target = [p[0] + offset[0], p[1] + offset[1], p[2] + height];
        for (i, &t) in target.iter().enumerate() {
            let mut delta = t - self.pos[i];
            if i == 2 { delta = mul(delta as i64, 0x199); }
            let gain = if i == 2 && self.profile != CameraProfile::Mobile { 0x5c3 } else { 400 };
            self.vel[i] = mul(delta as i64, gain) + mul(self.vel[i] as i64, 500);
        }
        let mut from = self.eye();
        let mut want = std::array::from_fn(|i| (self.pos[i] + self.vel[i]) as f32 / 4096.);
        // The host query sweeps an expanded box; do not add the radius twice.
        for attempt in 0..4 {
            let Some((t, n)) = col.trace_boxes(from, want, 0x14cd as f32 / 4096., false) else { break; };
            let contact: [f32; 3] = std::array::from_fn(|i| from[i] + (want[i] - from[i]) * t);
            let dot = (0..3).map(|i| (want[i] - contact[i]) * n[i]).sum::<f32>();
            want = std::array::from_fn(|i| want[i] - dot * n[i] + n[i] * 0.01);
            from = std::array::from_fn(|i| contact[i] + n[i] * 0.01);
            if attempt == 3 { want = from; }
        }
        self.pos = fx(want);
        let k = if centering { 4096 } else { (speed / 6).min(4096) };
        approach(&mut self.pitch, 0xd4e4, mul(k as i64, 0x3c0) + 0x40);
        let look = angle(p[0] - self.pos[0], p[1] - self.pos[1]);
        approach(&mut self.yaw, look, mul(k as i64, 0x6dc) + 0x40);
        // Vehicle cameras never invoke the on-foot stuck-camera overhead snap.
        self.blocked = if col.trace_boxes(view.position, want, 0., false).is_some() {
            self.blocked.saturating_add(1)
        } else { 0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn view(speed: f32) -> VehicleView {
        VehicleView { id: 0, position: [0.; 3], heading: 0, forward_speed: speed,
            lateral_speed: 0., half_length: 4., recenter: false }
    }
    fn empty() -> Collision { Collision { cells: (0..14000).map(|_| Default::default()).collect() } }

    #[test]
    fn original_reverse_threshold_delay_and_progressive_orbit() {
        let mut state = VehicleFollow::new(0, 4096);
        let p = [0; 3];
        let eye = [0, -40960, 110592];
        for _ in 0..60 { state.offset(view(-8.), eye, p, 32768, 40960); }
        assert!(!state.orbiting);
        for _ in 0..30 { state.offset(view(-10.), eye, p, 40960, 40960); }
        assert!(!state.orbiting);
        let off = state.offset(view(-10.), eye, p, 40960, 40960);
        assert!(state.orbiting);
        assert_eq!(off, [0, -40960]);
        let off = state.offset(view(-10.), eye, p, 40960, 40960);
        assert_eq!(state.orbit, 601);
        assert!(off[0] < 0 && off[1] < 0);
        for _ in 0..60 { state.offset(view(-10.), eye, p, 40960, 40960); }
        assert_eq!(state.orbit, -32768);
        state.offset(view(10.), eye, p, 40960, 40960);
        assert!(state.orbiting);
        for _ in 0..60 { state.offset(view(10.), eye, p, 40960, 40960); }
        assert!(!state.orbiting);
    }

    #[test]
    fn reverse_timer_requires_consecutive_motion_not_gear_or_lateral_slip() {
        let mut state = VehicleFollow::new(0, 4096);
        for _ in 0..30 { state.offset(view(-10.), [0, -40960, 0], [0; 3], 40960, 40960); }
        state.offset(view(0.), [0, -40960, 0], [0; 3], 40960, 40960);
        assert_eq!(state.reverse_ticks, 0);
        state.offset(view(-10.), [0, -40960, 0], [0; 3], 40960, 40960);
        assert!(!state.orbiting);
    }

    #[test]
    fn original_platform_framing_and_large_vehicle_branch() {
        assert_eq!(framing(CameraProfile::Psp, 0, 0, 4.), (40960, 110592));
        assert_eq!(framing(CameraProfile::Ds, 0, 0, 4.), (40960, 110592));
        assert_eq!(framing(CameraProfile::Mobile, 0, 0, 4.), (40960, 114688));
        assert_eq!(framing(CameraProfile::Psp, 100000, 50000, 6.), (40960, 159744));
        assert_eq!(framing(CameraProfile::Mobile, 100000, 50000, 6.), (40960, 172032));
        let psp = framing(CameraProfile::Psp, 40 * 4096, 4 * 4096, 4.);
        let ds = framing(CameraProfile::Ds, 40 * 4096, 4 * 4096, 4.);
        assert!(ds.0 < psp.0 && ds.1 < psp.1);
    }

    #[test]
    fn vehicle_switch_freecam_and_exit_preserve_pose_and_reset_controller() {
        let col = empty();
        let mut cam = Camera::new([0.; 3], 0, CameraProfile::Psp);
        cam.step_vehicle(view(20.), &col);
        let before = (cam.eye(), cam.yaw, cam.pitch);
        cam.set_profile(CameraProfile::Mobile, [0.; 3]);
        assert_eq!((cam.eye(), cam.yaw, cam.pitch), before);
        cam.toggle_free();
        cam.step_vehicle(view(-20.), &col);
        assert_eq!(cam.eye(), before.0);
        cam.toggle_free();
        cam.step([0.; 3], 0, [0.; 3], &col);
        assert!(cam.vehicle.is_none());
        assert_eq!(cam.sprint_ticks, 0);
        assert!(cam.eye().iter().all(|v| v.is_finite()));
    }

    #[test]
    fn stationary_view_retains_side_and_angle_wrap_takes_short_path() {
        let mut state = VehicleFollow::new(0, 0);
        assert_eq!(state.offset(view(0.), [40960, 0, 110592], [0; 3], 0, 40960), [40960, 0]);
        let mut a = 65520;
        approach(&mut a, 16, 16);
        assert_eq!(a, 0);
        approach(&mut a, 16, 16);
        assert_eq!(a, 16);
    }
}

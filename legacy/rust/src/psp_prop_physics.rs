// SPDX-License-Identifier: GPL-3.0-only
//! Staged PSP Europe physical core. Test-only until the coupled integrator,
//! contacts and stop rules are recovered; never mixed with the host bounce.
//! Verified against original instructions using tools/research/psp_prop_oracle.py.
#![cfg(test)]

fn mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) >> 12) as i32
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
struct Motion {
    velocity: [i32; 3],
    angular: [i32; 3],
    acceleration: [i32; 3],
    force: [i32; 3],
    torque: [i32; 3],
}

impl Motion {
    /// 0x089b5960..0x089b5b0c: cPhysical base force integration.
    /// Inverse inertia is a packed signed 16-bit matrix in this PSP build.
    /// 0x8934888 loads columns at byte offsets (0,6,12), (2,8,14), (4,10,16).
    fn recalc(&mut self, mass: i32, disabled: bool, inverse_mass: i32, matrix: [i16; 9]) {
        if mass == 0 || disabled { return; }
        if self.torque != [0; 3] {
            let acceleration: [i32; 3] = std::array::from_fn(|row| {
                ((0..3).map(|col| self.torque[col] as i64 * matrix[col*3+row] as i64)
                    .sum::<i64>() >> 12) as i32
            });
            self.torque = [0; 3];
            for (v, a) in self.angular.iter_mut().zip(acceleration) {
                *v = v.wrapping_add(mul(a, 0x88));
            }
        }
        if self.force != [0; 3] {
            self.acceleration = self.force.map(|f| mul(f, inverse_mass));
            self.force = [0; 3];
            for (v, a) in self.velocity.iter_mut().zip(self.acceleration) {
                *v = v.wrapping_add(mul(a, 0x88));
            }
        }
    }
}

#[test]
fn psp_signed_integration_and_accumulator_consumption() {
    let mut state = Motion { velocity: [0;3], angular: [0;3], acceleration: [0;3],
        force: [4096,-4097,123456], torque: [8192,1,-1] };
    state.recalc(4096, false, 4096, [4096,0,0,0,4096,0,0,0,4096]);
    // Results from the original Europe executable, not the Android adaptation.
    assert_eq!(state.velocity, [136,-137,4099]);
    assert_eq!(state.angular, [272,0,-1]);
    assert_eq!(state.force, [0;3]);
    assert_eq!(state.torque, [0;3]);
    let once = state.clone();
    state.recalc(4096, false, 4096, [4096,0,0,0,4096,0,0,0,4096]);
    assert_eq!(state, once, "A consumed force cannot accelerate the object twice");
}

#[test]
fn psp_disabled_or_zero_mass_preserves_pending_forces() {
    let initial = Motion { velocity: [1,2,3], angular: [4,5,6], acceleration: [7,8,9],
        force: [1000;3], torque: [-1000;3] };
    for (mass, disabled) in [(0,false),(4096,true)] {
        let mut state = initial.clone();
        state.recalc(mass, disabled, 4096, [4096;9]);
        assert_eq!(state, initial);
    }
}

#[test]
fn original_psp_kinematics_differential_when_private_fixture_present() {
    #[derive(serde::Deserialize)]
    struct Case {
        mass: i32, inverse_mass: i32, disabled: bool, matrix: [i16;9],
        #[serde(flatten)] input: Motion,
        expected: Motion,
    }
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/psp-prop-oracle.json");
    let Ok(bytes) = std::fs::read(path) else {
        eprintln!("Private PSP differential fixtures unavailable; run psp_prop_oracle.py");
        return;
    };
    let cases: Vec<Case> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(cases.len(), 512);
    for (index, case) in cases.into_iter().enumerate() {
        let mut actual = case.input;
        actual.recalc(case.mass, case.disabled, case.inverse_mass, case.matrix);
        assert_eq!(actual, case.expected, "original PSP case {index}");
    }
}

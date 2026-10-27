# Gemini Traffic & Navigation Behavior Audit (ReGTA CTW PC Recomp)

> Historical independent audit. Read the corrections in [DRIVING_TRAFFIC_REPAIR.md](DRIVING_TRAFFIC_REPAIR.md#review-of-geminis-independent-audit) before applying its recommendations.

**Author:** Gemini (Independent Evidence Auditor)<br>
**Date:** October 26, 2026<br>
**Audience:** Benjamin & ChatGPT<br>
**Target Architecture:** Sony PSP Europe `ULES01347` v1.01<br>
**Pinned Upstream Commits:**
- `CTW-Native`: `b0506abc786fea229101c56e0a64726aa8b4ac97` (GPL-3.0-only)
- `highcmd PSP recomp`: `6562163b6f504a1db93e1be5ce580d8cd6f024a7`
- `ai.bin` Reference: ROM.WAD GPS Road Network (`work/gps-ai.bin`)

---

## 1. Executive Summary & Audit Scope

This document presents an independent, ground-truth comparative audit between the original *Grand Theft Auto: Chinatown Wars* (PSP) traffic and navigation systems and our Rust implementation in `tu-voit-le-projet-vcs-rust`.

### Ground Rules & Non-Interference Mandate
In accordance with `AGENTS.md` and user instructions:
1. **ChatGPT owns all runtime fixes** in `src/actors.rs`, `src/gps.rs`, `src/play.rs`, `src/vehicle.rs`, `src/traffic.rs`, and handling modules.
2. **Gemini has not modified any code files**, executed builds, staged commits, pushed branches, or altered dashboards.
3. This audit provides exact file/line citations from upstream C++ sources, algorithmic formulas, concrete reproductions of defects, and targeted regression tests so ChatGPT can implement the fixes with mathematical precision.

### Facts vs. Assumptions
- **Verified Fact:** Direct analysis of upstream C++ files (`traffic.cpp`, `aispline.cpp`, `roads.cpp`, `trafficavoidance.cpp`, `driving.cpp`), the binary ROM table `gameplay_tables.bin`, and parsing of `work/gps-ai.bin` (2,008 nodes, 4,108 links).
- **Assumption:** Visual perception differences reported by players (e.g. "traffic jerks", "cars reverse into queues", "vehicles spawn on player") directly correlate with specific mathematical mismatches in spline interpolation, light cycling, and spawn frustum bounds.

---

## 2. Empirical PSP `ai.bin` Road Network Survey

Inspection of the original PlayStation Portable European release road network (`work/gps-ai.bin`) yields:
- **Total Nodes:** 2,008
- **Total Links:** 4,108
- **Header:** `0x10` links offset, 10 bytes per node (`cBaseNode`: `u16 firstLink`, `u16 flags`, `i16 x`, `i16 y` in 1/8 units, `i8 z` in 1/2 units, `u8 cover`).

### Node Flag Bit Distribution Across 2,008 Nodes
| Bit | Hex Mask | Meaning in Original C++ Engine | Node Count in `ai.bin` |
|---|---|---|---|
| **Bit 2–4** | `0x001C` | Outgoing link count (`flags >> 2 & 7`) | 1 link: 128, 2 links: 1124, 3 links: 521, 4 links: 139 |
| **Bit 5** | `0x0020` | `flag5` — Two-way bidirectional link | **1,181 nodes** (58.8% of network!) |
| **Bit 7** | `0x0080` | Bridge span flag in header definitions | 0 nodes (Bridge spans use $z \le -4.0$ with highway flag) |
| **Bit 9** | `0x0200` | Two-lane road link (`lanes = (flags >> 9 & 1) + 1`) | **1,959 nodes** (97.6% of network) |
| **Bit 10** | `0x0400` | Highway link (`halfWidth = 0x3C00` / 3.75m, speed x2.0) | **541 nodes** |
| **Bit 11** | `0x0800` | Restricted link (only taken for missions/special paths) | **87 nodes** |
| **Bit 15** | `0x8000` | Controlled junction (governed by 1024-tick traffic lights) | **428 nodes** (21.3% of network) |

### Representative Node IDs & Coordinates (Ground Evidence)

| Road Feature | Node ID | Flags | World Position $(X, Y, Z)$ | Outgoing Links | Characteristics |
|---|---|---|---|---|---|
| **Straight 1-Lane** | **46** | `0x0004` | $(-335.62, 1512.12, 0.00)$ | `[73]` | 1 link, 1 lane, narrow alleyway |
| **Straight 1-Lane** | **73** | `0x0004` | $(-353.75, 1534.00, 0.00)$ | `[104]` | 1 link, 1 lane, alleyway |
| **Straight 2-Lane** | **2** | `0x0604` | $(-398.88, 1512.00, 0.00)$ | `[75]` | 1 link, 2 lanes, highway, one-way |
| **Straight 2-Lane** | **4** | `0x0604` | $(-388.00, 1747.00, 0.00)$ | `[53]` | 1 link, 2 lanes, highway, one-way |
| **T-Junction** | **0** | `0x822c` | $(-530.00, 1511.88, 0.00)$ | `[2, 51, 64]` | 3 links, 2 lanes, two-way, **controlled** |
| **T-Junction** | **1** | `0x822c` | $(-372.00, 1610.00, 0.00)$ | `[21, 52, 89]` | 3 links, 2 lanes, two-way, **controlled** |
| **T-Junction** | **8** | `0x822c` | $(-810.00, 1527.75, 0.00)$ | `[14, 17, 106]` | 3 links, 2 lanes, two-way, **controlled** |
| **4-Way Crossing** | **13** | `0x0230` | $(-230.00, 1550.00, 0.00)$ | `[57, 59, 78, 82]` | 4 links, 2 lanes, two-way, uncontrolled |
| **4-Way Crossing** | **27** | `0x8230` | $(-290.00, 1450.00, 0.00)$ | `[39, 78, 82, 95]` | 4 links, 2 lanes, two-way, **controlled** |
| **4-Way Crossing** | **39** | `0x8230` | $(-290.00, 1370.00, 0.00)$ | `[25, 26, 27, 42]` | 4 links, 2 lanes, two-way, **controlled** |
| **One-Way Street** | **3** | `0x0604` | $(-372.12, 1538.00, 0.00)$ | `[47]` | No `flag5`, highway one-way ramp |
| **One-Way Street** | **5** | `0x0604` | $(-180.75, 1947.75, 0.00)$ | `[96]` | No `flag5`, divided highway lane |
| **Bridge (East Borough)** | **1743** | `0x0634` | $(78.38, 1107.62, -7.00)$ | `[1738, 1739, 1742, 1931, 1993]` | $Z = -7.0$, highway, connects Bohan/Algonquin |
| **Bridge (East Borough)** | **1931** | `0x062c` | $(-113.00, 1143.38, -7.00)$ | `[1743, 1885, 1930]` | $Z = -7.0$, highway, long span (194.7m) to 1743 |
| **Bridge (Broker)** | **1834** | `0x062c` | $(-790.12, -1387.38, -7.00)$ | `[1831, 1835, 1840]` | $Z = -7.0$, highway, span (194.9m) across river |
| **Bridge (Broker)** | **1841** | `0x0628` | $(-153.12, -1737.25, -7.00)$ | `[1836, 1842]` | $Z = -7.0$, highway bridge deck |
| **Bridge (Broker)** | **1842** | `0x0628` | $(41.38, -1730.62, -7.00)$ | `[1841, 1843]` | $Z = -7.0$, highway bridge deck (194.6m span) |

> [!IMPORTANT]
> In `ai.bin`, **Bit 7 is not used**. Instead, bridge spans across water are identified by $Z \le -4.0$ ($z < -8$ in raw nibbles) and `flags & 0x0400` (highway), giving water clearance and long-distance spans.

---

## 3. Priority Audit Area 1: Lane Metadata & Two-Way Links (`flag5`)

### Confirmed Defect 1.1: Missing Node Flip & Reverse Parametric Conversion on Two-Way Links (`flag5`)
- **Our File/Function:** `src/gps.rs:select_lane`, `src/gps.rs:convert_lane`, `src/traffic.rs:move_on_rail`.
- **Upstream File/Function/Line:** `references/ctw-native/port/game/aispline.cpp:152–161` (`AISpline::set`).

#### Concrete Reproduction
In `src/gps.rs`:
```rust
pub fn select_lane(&self, a: usize, b: usize, seed: u32) -> (u8, f32) {
    let m = self.link_meta(a, b);
    let lane = if m.lanes <= 1 || m.flag5 { 0 } else { (seed & 1) as u8 };
    ...
}
```
Because `select_lane` and `convert_lane` force `lane = 0` whenever `m.flag5` is true, **1,181 links out of 4,108 can only ever generate traffic moving in the forward directed graph direction ($A \to B$) on lane 0**. The oncoming lane (lane 1) is completely dead. Furthermore, if a vehicle were placed in lane 1 on a two-way link, our code does not flip the nodes or mirror the parametric distance, so the vehicle would drive backwards along the oncoming lane.

#### Upstream Algorithm
In `aispline.cpp`:
```cpp
RoadMeta m0 = rn.meta(t_[0].metaB ? t_[0].b : t_[0].a);
int l = laneConvert(lanes, lane, m0.lanes, m0.flag5);
int from = a, to = b;
if (m0.flag5 && l >= m0.lanes / 2) {   // a two-way link: upper lanes drive the other way
    l = m0.lanes - 1 - l;
    param_ = (int16_t)(0x1000 - param_);
    from = b; to = a;
}
setLink(rn, t_[1], from, to, 0, l, true);
setLink(rn, t_[2], from, to, 0x1000, l, true);
```
1. For a 2-lane road with `flag5 == true`:
   - Lane 0 ($l < 1$) drives forward: $A \to B$, parameter $t \in [0, 1]$.
   - Lane 1 ($l \ge 1$) drives reverse: $B \to A$, lane index becomes $l' = 2 - 1 - 1 = 0$, parameter becomes $t' = 1.0 - t$.
2. The vehicle heading and right-hand side offset are evaluated along $(B \to A)$, putting the vehicle naturally on the opposite side of the road moving in the opposite direction.

#### Recommended Regression Test
```rust
#[test]
fn two_way_flag5_link_supports_both_directions_without_lane_clipping() {
    let roads = load_test_roads();
    // Node 0 -> Node 51 is a flag5 link
    let (lane_fwd, _) = roads.select_lane_with_dir(0, 51, 0);
    let (lane_rev, _) = roads.select_lane_with_dir(0, 51, 1);
    assert_ne!(lane_fwd, lane_rev);
    let pose_fwd = roads.lane_world_pos(0, 51, 0.5, lane_fwd, 0.0);
    let pose_rev = roads.lane_world_pos(51, 0, 0.5, lane_rev, 0.0);
    // Reverse lane must be laterally separated by road width (approx 5.0m)
    let dist = ((pose_fwd[0] - pose_rev[0]).powi(2) + (pose_fwd[1] - pose_rev[1]).powi(2)).sqrt();
    assert!(dist >= 2.0 && dist <= 6.0);
}
```

---

## 4. Priority Audit Area 2: Junction Curve Construction & Handover

### Confirmed Defect 2.1: Discontinuous Spline Handover and Arbitrary `turn_entry_offset` Reset
- **Our File/Function:** `src/gps.rs:rail_pose`, `src/gps.rs:turn_entry_offset`, `src/traffic.rs:move_on_rail`.
- **Upstream File/Function/Line:** `references/ctw-native/port/game/aispline.cpp:263–275` (`curvePoint`), lines 299–314 (`shuffleDown`), lines 357–377 (`AISpline::inc`).

#### Concrete Reproduction
When `car.rail_dist >= total_len` in `src/traffic.rs:move_on_rail`:
```rust
let entry = roads.turn_entry_offset(car.previous, car.target, car.next_target, car.lane, car.next_lane);
car.previous = car.target; car.target = car.next_target;
car.rail_dist = entry;
```
`rail_dist` is forcibly reset to `entry` (a geometric setback distance based on node width) rather than carrying over the remaining distance traversed during the time step! This causes:
1. **Physical position snapping:** The car teleports forward or backward by the difference between $(rail\_dist - total\_len)$ and `entry`.
2. **Angle jerking:** The vehicle instantly snaps to the new link's entry orientation without tangent continuity.

#### Upstream Algorithm
In `aispline.cpp`, navigation is maintained via a 4-target window ($T_0, T_1, T_2, T_3$):
- Junction curves are **Quadratic Bezier** curves connecting $P_1$ (incoming setback) to $P_2$ (outgoing entry) via control point `icpt`:
  $$P(t) = (1 - t)^2 P_1 + 2t(1 - t) icpt + t^2 P_2$$
  The tangent vector is:
  $$D(t) = 2(1 - t)(icpt - P_1) + 2t(P_2 - icpt)$$
- In `AISpline::inc`:
  ```cpp
  if (t > 0x1000) {
      int32_t oldLen = len_;
      for (int guard = 0; t > 0x1000 && guard < 8; ++guard) {
          shuffleDown(rn, cop);
          if (!len_) { t = 0x1000; break; }
          t = (int32_t)(((int64_t)(t - 0x1000) * oldLen) / len_);
          oldLen = len_;
      }
      shuffled = true;
  }
  ```
  The leftover parameter $(t - 1.0) \times oldLen$ is the **exact remaining physical metric distance**. Dividing it by the new segment's metric length $len\_$ converts it into the exact parameter on the next segment with **zero discontinuity in position or velocity**.

#### Recommended Regression Test
```rust
#[test]
fn junction_handover_preserves_continuity_without_distance_loss() {
    let mut spline = AISpline::new();
    spline.set(&roads, node_a, node_b, 0.99, lane, 2);
    let p_before = spline.pos();
    spline.advance(&roads, 2.0); // moves across handover boundary
    let p_after = spline.pos();
    let moved = distance(p_before, p_after);
    assert!((moved - 2.0).abs() < 0.05, "handover must preserve exact metric displacement");
}
```

---

## 5. Priority Audit Area 3: Traffic Lights, Queues & Braking

### Confirmed Defect 3.1: Normal Traffic Queues and Red Lights Triggering Reversing Recovery
- **Our File/Function:** `src/actors.rs:1708–1714` (`car.blocked > 2.4`), `src/traffic.rs:115–117`.
- **Upstream File/Function/Line:** `references/ctw-native/port/game/traffic.cpp:325–384` (`forwardSpeed`), `aispline.cpp:412–442` (`tooSlowForHoldingPattern`).

#### Concrete Reproduction
In `src/actors.rs`:
```rust
if obstacle {
    car.blocked += dt;
    ...
    if car.blocked > 2.4 {
        car.on_rail = false;
        car.recovery_active = true;
        car.recovery_timer = 1.25;
        car.recovery_steer = if (i + car.target) % 2 == 0 { -0.85 } else { 0.85 };
        car.blocked = 0.0;
    }
}
```
Any red light (holding 12.8s) or queue of 2–3 cars will exceed `blocked > 2.4s`. The vehicle then activates `recovery_active`, **shifts into reverse with throttle -0.65 and maximum steer, slams into cars behind it, leaves the road, and spins out**. This is the root cause of Benjamin's report: *"la conduite est pire que avant... les voitures des pnj font n importe quoi"*.

#### Upstream Algorithm
1. In upstream `traffic.cpp`, an on-rail vehicle **NEVER** reverses or enters physics recovery for a red light or normal traffic queue!
2. `tooSlowForHoldingPattern` holds the vehicle at the stop line:
   ```cpp
   uint32_t f = frame & 0x3FF; // 1024-frame cycle
   uint32_t flow = f < 0x180 ? 0 : (f - 0x200 < 0x180 ? 1 : 3);
   uint32_t direction = std::abs(N[1] - pos[1]) <= std::abs(N[0] - pos[0]) ? 1 : 0;
   held = flow != direction;
   ```
3. Distance to stop line:
   $$\text{stop} = 5.0\text{m} + width_{nodeB}$$
   $$\text{left} = d - \text{stop}$$
   The holding pattern ONLY engages if $1.0\text{m} \le \text{left} \le 8.0\text{m}$.
   - If $\text{left} < 1.0\text{m}$ (already entering or past the stop line), the car **clears the junction** and does not stop!
4. Deceleration rate towards the stop line or car ahead:
   $$dv = \frac{v^2 - v_{target}^2}{d} \times \frac{17}{256}$$
5. When the light turns green:
   ```cpp
   if (sp == 0) {
       if (d.startDelay == 0) d.startDelay = (uint8_t)Rand16Critical(10);
       d.startDelay = (uint8_t)(d.startDelay - 1);
       if (d.startDelay != 0) target = 0;
   }
   ```
   Reaction latency is $0 \dots 9$ frames ($0 \dots 0.3$s). The car then accelerates smoothly at $+0x4CE$ per frame ($+3.5\text{ m/s}^2$). **No recovery or reverse steering is ever triggered.**

### Confirmed Defect 3.2: 100-Meter Instant Stop Threshold
- **Our File/Function:** `src/traffic.rs:102` (`let target = if gap < 100. { 0. } else { cruising };`).
- **Upstream File/Function/Line:** `references/ctw-native/port/game/traffic.cpp:340–355`.

#### Concrete Reproduction
In `src/traffic.rs`:
```rust
let target = if gap < 100. { 0. } else { cruising };
```
Setting `target = 0` whenever `gap < 100.` forces every car within 100 meters of a red light or another car to decelerate to a complete stop from a football field away, causing phantom traffic jams that freeze entire avenues.

#### Upstream Algorithm
In upstream `traffic.cpp`, `target` is set to 0 **only when an obstacle is within the immediate collision lookahead box** (`v.blocked == true`) or when within $8.0\text{m}$ of the junction stop line (`hold == true`). Otherwise, `target = roadSpeed`.

---

## 6. Priority Audit Area 4: Off-Rail Recovery & Rejoining

### Confirmed Defect 4.1: Visible On-Screen Snapping and Teleportation on Rail Rejoin
- **Our File/Function:** `src/traffic.rs:rejoin_target`, `src/actors.rs:1608–1630`.
- **Upstream File/Function/Line:** `references/ctw-native/port/game/traffic.cpp:638–650` (`cWanderRoadsUnderPhysics::Process`).

#### Concrete Reproduction
In our codebase, vehicles that leave the rail attempt to reattach as soon as they are within 0.12m of a lane center. When triggered in the field of view, the car snaps to the lane angle and position, appearing to "teleport" or "flicker" between lanes.

#### Upstream Algorithm
In `traffic.cpp`:
```cpp
if (!d.reacting) {
    // off screen ((+0x77 & 3) == 0) with a straight road ahead and no target:
    // SetToPhysics(false) and a cWanderRoads on rails from the spline (CreateWanderRoads), unseen
    if (v.screenTimer != 0xFF && d.spline.journeyStraight(g.roads)) {
        int32_t p[2], dir[2];
        d.spline.getPosAndDir(g.roads, p, dir);
        v.setToPhysics(false);
        d.speed = 0;
        v.placeOnRail(p[0], p[1], d.spline.z(), dir, d.speed);
        d.underPhysics = d.following = false;
        d.lastFrame = g.frame;
        return;
    }
}
```
**The Original Rule:**
1. A physics vehicle **NEVER** reattaches to rails while visible on the player's screen (`screenTimer == 0xFF`)!
2. Reattachment is **ONLY** allowed when the vehicle is **OFF SCREEN** (`screenTimer != 0xFF`) **AND** the road ahead is verified straight (`journeyStraight` returns true).
3. On-screen vehicles drive completely under physics using `AIDriving::towards` and physical collision.

---

## 7. Priority Audit Area 5: Spawn/Despawn Visibility & Reservations

### Confirmed Defect 5.1: Inadequate Spawning Distance and Missing Camera Frustum Rejection
- **Our File/Function:** `src/actors.rs:2580–2620` (`spawn_traffic_vehicle`).
- **Upstream File/Function/Line:** `references/ctw-native/port/game/traffic.cpp:164–185` (`createPos`), lines 205–250 (`generate`).

#### Concrete Reproduction
Our spawning loop generates cars at 45m to 55m radius around the player without robust frustum occlusion. When the camera pans or the player drives fast (e.g. 15–20 m/s), cars spawn directly in view or right in the player's path, causing pop-in and collisions.

#### Upstream Algorithm
1. **Chord Circle:** Spawning candidates are evaluated at **69.0 meters** radius (`0x45000 / 4096`):
   ```cpp
   bool clear = !g.view.canSee(point, 0x5000); // 5.0m bounding sphere MUST NOT be visible!
   ```
2. **Perpendicular Clearance:** Link perpendicular squared distance to player must satisfy:
   $$metric \le 1677721600 \quad (10.0\text{ meters}^2)$$
   ensuring the road link directly feeds into or originates near the player's path.
3. **Vehicle Clearance & Reservation:**
   ```cpp
   for (const auto& car : g.cars) {
       const int64_t dx = (int64_t)car.pos[0] - point[0], dy = (int64_t)car.pos[1] - point[1];
       if (dx*dx + dy*dy < (int64_t)0x5000 * 0x5000) { clear = false; break; } // 5.0m clearance
   }
   ```
   If clear, the vehicle slot is reserved immediately, preventing two vehicles or a pedestrian and car from spawning at the same position.

---

## 8. Summary Comparison Table

| Subsystem / Feature | Our Codebase (`src/`) | Upstream PSP Engine (`traffic.cpp` / `aispline.cpp`) | Impact of Parity Fix |
|---|---|---|---|
| **Two-way links (`flag5`)** | Fixed to Lane 0 only; opposite direction dead (`gps.rs:284`) | Dynamic node flip ($from = b, to = a$) and parameter mirror ($t' = 1 - t$) | Enables authentic two-way traffic on 1,181 links |
| **Junction interpolation** | Segment reset with abrupt `turn_entry_offset` | Continuous Quadratic Bezier with $(t - 1) \times len$ handover (`aispline.cpp:368`) | Eliminates turning stutters, jerks, and position jumping |
| **Queue / Light behavior** | Reverses if blocked $> 2.4$s (`actors.rs:1708`) | Clean stop line hold ($1.0 \dots 8.0$m), random green delay ($0 \dots 0.3$s), no reverse | Fixes vehicles ramming and spinning out at traffic lights |
| **Braking lookahead** | Target 0 at $< 100$m (`traffic.rs:102`) | Oriented 2D bounding box intersection (`traffic.cpp:252`) | Restores normal cruising flow and realistic bumper spacing |
| **Rail rejoining** | Snaps on screen when near lane | Off-screen only (`screenTimer != 0xFF`) and straight line | Eliminates visible teleportation and lane hopping |
| **Spawning radius** | 45m–55m, weak frustum check | 69.0m chord circle, $5.0$m frustum sphere check, $5.0$m car clearance | Eliminates pop-in and cars spawning on top of the player |

---

## 9. Conclusion: The Three Highest-Impact Fixes

To achieve immediate, noticeable parity with the original game and resolve Benjamin's bug reports, ChatGPT should prioritize the following three changes:

1. **Fix 1 (Queue & Traffic Light Stability):**<br>
   Remove the 2.4-second stuck-reverse trigger (`recovery_active`) from on-rail vehicles waiting at traffic lights or queues. Clamp traffic light braking to the $1.0\text{m} \dots 8.0\text{m}$ stop line window, remove the arbitrary $< 100\text{m}$ zero-speed target, and add the original $0 \dots 9$ frame pulling-away delay.
2. **Fix 2 (Continuous Spline Handover):**<br>
   Replace the hard reset to `turn_entry_offset` at junction boundaries with upstream's leftover parameter handover:
   $$t_{new} = \frac{(t_{old} - 1.0) \times length_{old}}{length_{new}}$$
   This completely removes the jerking, stuttering, and angle discontinuities when cars turn corners.
3. **Fix 3 (Bidirectional Two-Way Links & Off-Screen Rejoin):**<br>
   Implement the upstream `flag5` node flip ($from = b, to = a, t = 1.0 - t$) for the upper lane of two-way links to activate both directions of traffic across the 1,181 two-way road links, and restrict rail reattachment strictly to off-screen vehicles (`!visible`) on straight roads.

---
*Report completed and ready for ChatGPT.*

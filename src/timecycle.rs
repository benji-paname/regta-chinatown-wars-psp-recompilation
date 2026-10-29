// SPDX-License-Identifier: GPL-3.0-only
//! PSP ULES01347 timecycle.dat: 8 weather blocks, 55 channels, 24 hours.
//! Init/fill/gamma/interpolation recovered at 08894020/0889424c/08a8eb90.
//! GL lighting integration remains partial; see PSP-VISUAL-PARITY.md.
use crate::iso::Result;
#[derive(Clone)]
pub struct TimeCycle { tables: Vec<[u8;24]> }
impl TimeCycle {
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len()!=8*55*24*4 { return Err("Invalid PSP timecycle.dat size".into()); }
        let mut tables=Vec::new();
        for (i, row) in data.chunks_exact(96).enumerate() {
            let input=std::array::from_fn(|h| i32::from_le_bytes(row[h*4..h*4+4].try_into().unwrap()));
            let t=i%55;
            let angles=matches!(t,3|4|8|9|34|35);
            let rgb=matches!(t,0..=2|5..=7|10..=18|20..=22|26..=31|36..=44|49..=54);
            let mut out=fill(input,angles);
            if rgb { for v in &mut out { *v=((*v as f32/255.).powf(f32::from_bits(0x3f19999a))*255.) as u8; } }
            tables.push(out);
        }
        Ok(Self{tables})
    }
    pub fn sample(&self, channel: usize, current: usize, next: usize, time: u32) -> f32 {
        let time=time%(24<<12); let hour=(time>>12) as usize;
        let a=self.tables[(current%8)*55+channel][hour] as i32;
        let b=self.tables[(next%8)*55+channel][(hour+1)%24] as i32;
        let mut d=b-a;
        if matches!(channel,3|4|8|9|34|35) { if d>=128 {d-=256;} else if d<=-128 {d+=256;} }
        a as f32 + (d as f32/2048.)*((time&4095)>>1) as f32
    }
    pub fn rgb(&self, channel: usize, current: usize, next: usize, time: u32) -> [f32;4] {
        let mut c=[1.;4]; for (k,v) in c[..3].iter_mut().enumerate() {*v=self.sample(channel+k,current,next,time)/255.;} c
    }
    pub fn sun(&self, current: usize,next: usize,time:u32)->[f32;4] {
        // Direction convention from the sibling renderer; PSP GE binding still under review.
        let a=self.sample(3,current,next,time)*std::f32::consts::TAU/256.;
        let b=self.sample(4,current,next,time)*std::f32::consts::TAU/256.;
        [-b.sin()*a.sin(),-b.sin()*a.cos(),b.cos(),0.]
    }
}
fn fill(input:[i32;24],angle:bool)->[u8;24] {
    let mut out=input.map(|v|(v>>12) as u8);
    let Some(first)=input.iter().position(|&v|v != -4096) else {return out};
    let mut prev=first;
    loop {
        let mut next=(prev+1)%24;
        while input[next]==-4096 {next=(next+1)%24;}
        let distance=if next>prev {next-prev} else {next+24-prev};
        let mut diff=input[next]-input[prev];
        if angle {if diff>0x80000 {diff-=0x100000;} else if diff < -0x80000 {diff+=0x100000;}}
        let step=diff/distance as i32;
        for k in 1..distance {out[(prev+k)%24]=((input[prev]+step*k as i32)>>12) as u8;}
        prev=next; if prev==first {break;}
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn midnight_and_angle_fill() {
        let mut a=[-4096;24];a[22]=250<<12;a[2]=10<<12;
        assert_eq!(fill(a,true)[0],2);
        assert_eq!(fill(a,false)[0],130);
        assert_eq!(fill([-4096;24],false),[255;24]);
    }
    #[test] fn psp_size_gamma_and_fraction() {
        assert!(TimeCycle::parse(&vec![0;29184]).is_err());
        let mut data=vec![0;42240];
        for row in data.chunks_exact_mut(4) {row.copy_from_slice(&(255i32<<12).to_le_bytes());}
        let t=TimeCycle::parse(&data).unwrap();assert_eq!(t.rgb(13,0,0,0),[1.;4]);
        let mut t=t;t.tables[0][0]=0;t.tables[0][1]=200;
        assert_eq!(t.sample(0,0,0,2048),100.);
        assert_eq!(t.sample(0,0,0,2049),100.);
    }
}

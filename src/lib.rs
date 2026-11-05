#[derive(Debug, Clone)]
pub struct Keyframe { pub time_ms: u64, pub value: f64 }

#[derive(Debug, Default)]
pub struct AnimationTrack { pub name: String, keyframes: Vec<Keyframe> }
impl AnimationTrack {
    pub fn new(name: impl Into<String>) -> Self { Self { name: name.into(), keyframes: Vec::new() } }
    pub fn insert_keyframe(&mut self, time_ms: u64, value: f64) {
        self.keyframes.push(Keyframe { time_ms, value });
        self.keyframes.sort_by_key(|k| k.time_ms);
    }
    pub fn duration_ms(&self) -> u64 { self.keyframes.last().map(|k| k.time_ms).unwrap_or(0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_track() {
        let mut t = AnimationTrack::new("pos");
        t.insert_keyframe(100, 1.0);
        assert_eq!(t.duration_ms(), 100);
    }
}

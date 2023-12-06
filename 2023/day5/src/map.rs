use std::{ops::Range, str::FromStr};

struct MapLine {
    target_delta: i64,
    source_range: Range<i64>,
}

impl MapLine {
    fn try_apply(&self, source: i64) -> Option<i64> {
        self.source_range
            .contains(&source)
            .then(|| source + self.target_delta)
    }

    fn shift(&self, source: Range<i64>) -> Range<i64> {
        source.start + self.target_delta..source.end + self.target_delta
    }

    fn apply_range(&self, source: Range<i64>) -> (Vec<Range<i64>>, Option<Range<i64>>) {
        if source.is_empty() {
            return (Vec::new(), None);
        }
        let Range { start, end } = self.source_range;
        let overlaps_start = self.source_range.contains(&source.start);
        let overlaps_end = self.source_range.contains(&(source.end - 1));
        if overlaps_start && overlaps_end {
            // This line encompasses source.
            (Vec::new(), Some(self.shift(source)))
        } else if overlaps_start {
            // This line overlaps only the start of source.
            (vec![end..source.end], Some(self.shift(source.start..end)))
        } else if overlaps_end {
            // This line overlaps only the end of source.
            (
                vec![source.start..start],
                Some(self.shift(start..source.end)),
            )
        } else if source.contains(&start) && source.contains(&end) {
            // This line is encompassed by source.
            (
                vec![source.start..start, end..source.end],
                Some(self.shift(start..end)),
            )
        } else {
            // This line does not overlap source.
            (vec![source], None)
        }
    }

    fn apply_ranges(&self, sources: Vec<Range<i64>>) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
        let (mut mapped_sources, mut targets) = (Vec::new(), Vec::new());
        for source in sources {
            let (new_sources, target) = self.apply_range(source);
            mapped_sources.extend(new_sources);
            targets.extend(target);
        }
        (mapped_sources, targets) // TODO: Coalesce?
    }
}

impl FromStr for MapLine {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.splitn(3, ' ');
        let target_start: i64 = words
            .next()
            .ok_or("expected start of target range")?
            .parse()
            .or(Err("target range start should be a number"))?;
        let source_start: i64 = words
            .next()
            .ok_or("expected start of source range")?
            .parse()
            .or(Err("source range start should be a number"))?;
        let range_len: i64 = words
            .next()
            .ok_or("expected range length")?
            .parse()
            .or(Err("range length should be a number"))?;
        assert!(words.next().is_none());
        Ok(MapLine {
            target_delta: target_start - source_start,
            source_range: (source_start..source_start + range_len),
        })
    }
}

pub struct Map {
    lines: Vec<MapLine>,
}

impl Map {
    pub fn apply(&self, source: i64) -> i64 {
        for line in &self.lines {
            if let Some(target) = line.try_apply(source) {
                return target;
            }
        }
        source
    }

    pub fn apply_ranges(&self, mut sources: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut targets = Vec::new();
        for line in &self.lines {
            let (new_sources, new_targets) = line.apply_ranges(sources);
            sources = new_sources;
            targets.extend(new_targets);
        }
        targets.extend(sources.into_iter());
        targets
    }
}

impl FromStr for Map {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Result<Vec<MapLine>, _> = s.lines().skip(1).map(|line| line.parse()).collect();
        Ok(Map { lines: lines? })
    }
}

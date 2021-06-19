const SPARK: &'static [char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

pub fn graph(values: &[f32]) -> String {
    let mut min = f32::MAX;
    let mut max = 0f32;

    for &i in values.iter() {
        if i > max {
            max = i
        }
        if i < min {
            min = i
        }
    }

    let ratio = if max - min == 0.0 {
        1.0
    } else {
        (SPARK.len() - 1) as f32 / (max - min)
    };

    values.iter()
        .map(|n| ((n - min) * ratio).floor() as usize)
        .filter_map(|n| SPARK.get(n))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::spark::graph;

    #[test]
    fn simple() {
        assert_eq!("", graph(&[]));
        assert_eq!("▁", graph(&[1.0]));
        assert_eq!("▁▁", graph(&[1.0, 1.0]));
    }

    #[test]
    fn adv() {
        assert_eq!("▁▅▃▆▃█", graph(&[1.0, 1.3, 1.2, 1.4, 1.2, 1.5]));
        assert_eq!("▁█▄", graph(&[-1.0, 1.0, 0.0]));
    }
}
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

    let ratio = if max == min {
        1.0
    } else {
        (SPARK.len() - 1) as f32 / (max - min)
    };

    values.iter()
        .map(|n| ((n - min) * ratio).round() as usize)
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
        assert_eq!("▁▁▁▁", graph(&[1.0, 1.0, 1.0, 1.0]));
    }

    #[test]
    fn adv() {
        assert_eq!("▁▅▄▇▄█", graph(&[1.0, 1.3, 1.2, 1.4, 1.2, 1.5]));
        assert_eq!("▁█▅", graph(&[-1.0, 1.0, 0.0]));
    }

    #[test]
    fn it_graphs_argv_data() {
        assert_eq!("▁▂█▅▂", graph(&[1.0, 5.0, 22.0, 13.0, 5.0]));
    }

    #[test]
    fn it_charts_pipe_data() {
        assert_eq!("▁▂▄▅▃█", graph(&[0.0, 30.0, 55.0, 80.0, 33.0, 150.0]));
    }

    #[test]
    fn it_charts_100_lt_300() {
        assert_eq!("▁▁▁▁▃▁▁▁▂█", graph(&[1.0, 2.0, 3.0, 4.0, 100.0, 5.0, 10.0, 20.0, 50.0, 300.0]));
    }

    #[test]
    fn it_charts_50_lt_100() {
        assert_eq!("▁▄█", graph(&[1.0, 50.0, 100.0]));
        assert_eq!("▁▄█", graph(&[0.1, 5.0, 10.0]));
    }

    #[test]
    fn it_charts_4_lt_8() {
        assert_eq!("▁▃█", graph(&[2.0, 4.0, 8.0]));
    }
}
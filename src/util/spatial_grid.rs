use std::array;
use std::simd::cmp::SimdPartialOrd;
use std::simd::{Mask, Simd, i64x8};

struct GridCell<const Dimensions: usize> {
    ids: Vec<usize>,
    positions: [Vec<i64>; Dimensions],
}

impl<const Dimensions: usize> GridCell<Dimensions> {
    pub fn add_point(&mut self, id: usize, pos: &[i64; Dimensions]) {
        self.ids.push(id);
        for (d, vec) in self.positions.iter_mut().enumerate() {
            vec.push(pos[d]);
        }
    }

    pub fn has_points_in_bbox(&self, min: &[i64; Dimensions], max: &[i64; Dimensions]) -> bool {
        let num_items = self.ids.len();

        let min_simds: Vec<i64x8> = min.iter().map(|&m| Simd::splat(m)).collect();
        let max_simds: Vec<i64x8> = max.iter().map(|&m| Simd::splat(m)).collect();

        for i in (0..num_items).step_by(8) {
            if i + 8 > num_items {
                for j in i..num_items {
                    if self
                        .positions
                        .iter()
                        .enumerate()
                        .all(|(i, p)| min[i] < p[j] && max[i] > p[j])
                    {
                        return true;
                    }
                }
            } else {
                let mut test_mask = Mask::splat(true);
                for d in 0..Dimensions {
                    let dv = Simd::from_slice(&self.positions[d][i..i + 8]);
                    test_mask = test_mask & dv.simd_ge(min_simds[d]) & dv.simd_le(max_simds[d]);
                }
                if test_mask.any() {
                    return true;
                }
            }
        }

        false
    }
}

pub struct SpatialGrid<const Dimensions: usize, const Splits: usize> {
    cells: Vec<GridCell<Dimensions>>,
    min: [i64; Dimensions],
    max: [i64; Dimensions],
}

impl<const Dimensions: usize, const Splits: usize> SpatialGrid<Dimensions, Splits> {
    pub fn new(min: &[i64; Dimensions], max: &[i64; Dimensions]) -> Self {
        let num_cells = Splits.pow(Dimensions as u32);
        let mut cells = Vec::with_capacity(num_cells);

        for i in 0..num_cells {
            let positions: [Vec<i64>; Dimensions] = (0..Dimensions)
                .map(|_| Vec::<i64>::new())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            cells.push(GridCell {
                ids: Vec::new(),
                positions,
            });
        }

        Self {
            cells,
            min: min.clone(),
            max: max.clone(),
        }
    }

    pub fn add_point(&mut self, id: usize, pos: &[i64; Dimensions]) {
        let idx = self.get_cell_idx(pos).expect("Point is out of bounds");
        let cell = &mut self.cells[idx];

        cell.add_point(id, pos);
    }

    fn get_dimensional_idx(&self, pos: &[i64; Dimensions]) -> Option<[usize; Dimensions]> {
        for (d, &p) in pos.iter().enumerate() {
            if self.min[d] > p || self.max[d] < p {
                return None;
            }
        }

        Some(array::from_fn(|i| {
            ((Splits as i64 * (pos[i] - self.min[i])) / self.max[i]) as usize
        }))
    }

    fn flatten_idx(&self, idx: &[usize; Dimensions]) -> usize {
        idx.iter().cloned().reduce(|a, v| (a * Splits + v)).unwrap()
    }

    fn get_cell_idx(&self, pos: &[i64; Dimensions]) -> Option<usize> {
        let dim_idx = self.get_dimensional_idx(pos)?;

        Some(self.flatten_idx(&dim_idx))
    }

    fn clamp(&self, pos: &[i64; Dimensions]) -> [i64; Dimensions] {
        array::from_fn(|i| pos[i].clamp(self.min[i], self.max[i]))
    }

    pub fn bbox_contains_point(&self, min: &[i64; Dimensions], max: &[i64; Dimensions]) -> bool {
        let min = self.clamp(min);
        let max = self.clamp(max);

        let min_idx = self.get_dimensional_idx(&min).expect("Cell out of bounds");
        let max_idx = self.get_dimensional_idx(&max).expect("Cell out of bounds");
        let mut current = min_idx;

        while current[Dimensions - 1] <= max_idx[Dimensions - 1] {
            let flat_idx = self.flatten_idx(&current);
            let cell = &self.cells[flat_idx];

            if cell.has_points_in_bbox(&min, &max) {
                return true;
            }

            for d in 0..Dimensions {
                current[d] += 1;
                if current[d] > max_idx[d] && d < (Dimensions - 1) {
                    current[d] = min_idx[d];
                } else {
                    break
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_simd() {
        let mut grid = SpatialGrid::<3, 2>::new(&[0, 0, 0], &[10, 10, 10]);
        grid.add_point(0, &[1, 1, 1]);
        
        assert!(grid.bbox_contains_point(&[0, 0, 0], &[2, 2, 2]));
        assert!(!grid.bbox_contains_point(&[2, 2, 2], &[4, 4, 4]));
    }

    #[test]
    fn test_simd() {
        let mut grid = SpatialGrid::<3, 2>::new(&[0, 0, 0], &[10, 10, 10]);
        
        for i in 0..32 {
            grid.add_point(i, &[1, 1, 1]);
        }

        assert!(grid.bbox_contains_point(&[0, 0, 0], &[2, 2, 2]));
        assert!(!grid.bbox_contains_point(&[2, 2, 2], &[4, 4, 4]));
    }
}
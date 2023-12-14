
use std::{fs::File, path::Path};
use std::io::{BufRead, BufReader};
use std::cmp::max;

struct Forest<const SIZE: usize> {
    trees: [[u8; SIZE]; SIZE],
}

impl<const SIZE: usize> Forest<SIZE> {
    
    pub fn load<P: AsRef<Path>>(filename: P) -> Self {
        let mut forest = [[0u8; SIZE]; SIZE];
        let mut row = 0usize;
        let file = File::open(filename).unwrap();
        for line in BufReader::new(file).lines() {
            let mut col = 0usize;
            for c in line.unwrap().bytes() {
                forest[row][col] = c - 48;
                col += 1;
            }
            row += 1;
        }
        Self{trees: forest}
    }

    fn is_tree_visible(&self, tree_x: usize, tree_y: usize) -> bool {
        assert!((tree_x < SIZE) && (tree_y < SIZE));

        if (tree_x == 0) || (tree_x == SIZE - 1) || (tree_y == 0) || (tree_y == SIZE - 1) { return true; }

        let tree_size = self.trees[tree_x][tree_y];

        if (0..tree_x).all(|x| self.trees[x][tree_y] < tree_size) { return true };          // left
        if ((tree_x + 1)..SIZE).all(|x| self.trees[x][tree_y] < tree_size) { return true }; // right
        if (0..tree_y).all(|y| self.trees[tree_x][y] < tree_size) { return true };          // top
        if ((tree_y + 1)..SIZE).all(|y| self.trees[tree_x][y] < tree_size) { return true }; // bottom
        false
    }

    pub fn count_visible_trees(&self) -> u32 {
        let mut visible_trees = (SIZE * 2 + (SIZE - 2) * 2) as u32;
        for x in 1..SIZE-1 {
            for y in 1..SIZE-1 {
                if self.is_tree_visible(x, y) { visible_trees += 1; }
            }
        }
        visible_trees
    }

    pub fn calculate_scenic_score(&self, tree_x: usize, tree_y: usize) -> u32 {
        assert!((tree_x < SIZE) && (tree_y < SIZE));
        
        if (tree_x == 0) || (tree_x == SIZE - 1) || (tree_y == 0) || (tree_y == SIZE - 1) { return 0; }

        let tree_size = self.trees[tree_y][tree_x];

        let scenic_score = self.count_visible_trees_up(tree_x, tree_y, tree_size)
            * self.count_visible_trees_left(tree_x, tree_y, tree_size)
            * self.count_visible_trees_right(tree_x, tree_y, tree_size)
            * self.count_visible_trees_down(tree_x, tree_y, tree_size);

        scenic_score
    }

    fn count_visible_trees_up(&self, tree_x: usize, tree_y: usize, tree_size: u8) -> u32 {
        match (0..tree_y).rev().position(|y| self.trees[y][tree_x] >= tree_size) {
            None => tree_y as u32,
            Some(y) => y as u32 + 1,
        }
    }

    fn count_visible_trees_left(&self, tree_x: usize, tree_y: usize, tree_size: u8) -> u32 {
        match (0..tree_x).rev().position(|x| self.trees[tree_y][x] >= tree_size) {
            None => tree_x as u32,
            Some(pos) => pos as u32 + 1,
        }
    }

    fn count_visible_trees_right(&self, tree_x: usize, tree_y: usize, tree_size: u8) -> u32 {
        match (tree_x+1..SIZE).position(|x| self.trees[tree_y][x] >= tree_size) {
            None => (SIZE - tree_x - 1) as u32,
            Some(pos) => pos as u32 + 1,
        }
    }

    fn count_visible_trees_down(&self, tree_x: usize, tree_y: usize, tree_size: u8) -> u32 {
        match (tree_y+1..SIZE).position(|y| self.trees[y][tree_x] >= tree_size) {
            None => (SIZE - tree_y -1) as u32,
            Some(pos) => pos as u32 + 1,
        }
    }

    pub fn find_max_scenic_score(&self) -> u32 {
        let mut max_scenic_score = 0u32;
        for y in 1..SIZE-1 {
            for x in 1..SIZE-1 {
                max_scenic_score = max(max_scenic_score, self.calculate_scenic_score(x, y))
            }
        }
        max_scenic_score
    }
}

pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let forest = Forest::<99>::load(filename);
    forest.count_visible_trees()
}

pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let forest = Forest::<99>::load(filename);
    forest.find_max_scenic_score()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        let forest = super::Forest::<5>::load("test_input/day08.txt");
        assert_eq!(forest.count_visible_trees(), 21);
    }

    #[test]
    fn test_part2() {
        let forest = super::Forest::<5>::load("test_input/day08.txt");
        assert_eq!(forest.find_max_scenic_score(), 8);
    }

    #[test]
    fn test_calculate_scenic_score() {
        let forest = super::Forest::<5>::load("test_input/day08.txt");
        assert_eq!(forest.count_visible_trees_up(2, 1, 5), 1);
        assert_eq!(forest.count_visible_trees_left(2, 1, 5), 1);
        assert_eq!(forest.count_visible_trees_right(2, 1, 5), 2);
        assert_eq!(forest.count_visible_trees_down(2, 1, 5), 2);
        assert_eq!(forest.calculate_scenic_score(2, 1), 4);
    }

    #[test]
    fn test_calculate_scenic_score_2() {
        let forest = super::Forest::<5>::load("test_input/day08.txt");
        assert_eq!(forest.count_visible_trees_up(2, 3, 5), 2);
        assert_eq!(forest.count_visible_trees_left(2, 3, 5), 2);
        assert_eq!(forest.count_visible_trees_right(2, 3, 5), 2);
        assert_eq!(forest.count_visible_trees_down(2, 3, 5), 1);
        assert_eq!(forest.calculate_scenic_score(2, 3), 8);
    }

}



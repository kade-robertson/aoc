use common::{Problem, Solution};
/// \--- Day 8: Treetop Tree House ---
/// ----------
///
/// The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a [tree house](https://en.wikipedia.org/wiki/Tree_house).
///
/// First, determine whether there is enough tree cover here to keep a tree
/// house *hidden*. To do this, you need to count the number of trees that are
/// *visible from outside the grid* when looking directly along a row or column.
///
/// The Elves have already launched a [quadcopter](https://en.wikipedia.org/wiki/Quadcopter) to generate a map with the height of each tree (your puzzle input). For example:
///
/// ```
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// ```
///
/// Each tree is represented as a single digit whose value is its height, where
/// `0` is the shortest and `9` is the tallest.
///
/// A tree is *visible* if all of the other trees between it and an edge of the
/// grid are *shorter* than it. Only consider trees in the same row or column;
/// that is, only look up, down, left, or right from any given tree.
///
/// All of the trees around the edge of the grid are *visible* - since they are
/// already on the edge, there are no trees to block the view. In this example,
/// that only leaves the *interior nine trees* to consider:
///
/// * The top-left `5` is *visible* from the left and top. (It isn't visible
///   from the right or bottom since other trees of height `5` are in the way.)
/// * The top-middle `5` is *visible* from the top and right.
/// * The top-right `1` is not visible from any direction; for it to be visible,
///   there would need to only be trees of height *0* between it and an edge.
/// * The left-middle `5` is *visible*, but only from the right.
/// * The center `3` is not visible from any direction; for it to be visible,
///   there would need to be only trees of at most height `2` between it and an
///   edge.
/// * The right-middle `3` is *visible* from the right.
/// * In the bottom row, the middle `5` is *visible*, but the `3` and `4` are
///   not.
///
/// With 16 trees visible on the edge and another 5 visible in the interior, a
/// total of `*21*` trees are visible in this arrangement.
///
/// Consider your map; *how many trees are visible from outside the grid?*
///
/// \--- Part Two ---
/// ----------
///
/// Content with the amount of tree cover available, the Elves just need to know
/// the best spot to build their tree house: they would like to be able to see a
/// lot of *trees*.
///
/// To measure the viewing distance from a given tree, look up, down, left, and
/// right from that tree; stop if you reach an edge or at the first tree that is
/// the same height or taller than the tree under consideration. (If a tree is
/// right on the edge, at least one of its viewing distances will be zero.)
///
/// The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large [eaves](https://en.wikipedia.org/wiki/Eaves) to keep it dry, so they wouldn't be able to see higher than the tree house anyway.
///
/// In the example above, consider the middle `5` in the second row:
///
/// ```
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// ```
///
/// * Looking up, its view is not blocked; it can see `*1*` tree (of height
///   `3`).
/// * Looking left, its view is blocked immediately; it can see only `*1*` tree
///   (of height `5`, right next to it).
/// * Looking right, its view is not blocked; it can see `*2*` trees.
/// * Looking down, its view is blocked eventually; it can see `*2*` trees (one
///   of height `3`, then the tree of height `5` that blocks its view).
///
/// A tree's *scenic score* is found by *multiplying together* its viewing
/// distance in each of the four directions. For this tree, this is `*4*` (found
/// by multiplying `1 * 1 * 2 * 2`).
///
/// However, you can do even better: consider the tree of height `5` in the
/// middle of the fourth row:
///
/// ```
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// ```
///
/// * Looking up, its view is blocked at `*2*` trees (by another tree with a
///   height of `5`).
/// * Looking left, its view is not blocked; it can see `*2*` trees.
/// * Looking down, its view is also not blocked; it can see `*1*` tree.
/// * Looking right, its view is blocked at `*2*` trees (by a massive tree of
///   height `9`).
///
/// This tree's scenic score is `*8*` (`2 * 2 * 1 * 2`); this is the ideal spot
/// for the tree house.
///
/// Consider each tree on your map. *What is the highest scenic score possible
/// for any tree?*
static PROBLEM_INPUT: &str = include_str!("input.txt");
pub struct Day08;

mod square_grid;
use square_grid::SquareGrid;

struct Forest {
    trees: SquareGrid<u8>,
}

impl Forest {
    pub fn new(grid_size: usize) -> Self {
        Self { trees: SquareGrid::new(grid_size) }
    }

    pub fn add_trees(&mut self, heights: &mut dyn Iterator<Item = u8>) {
        self.trees.extend(heights);
    }

    pub fn left_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 0..new_grid.size {
            for col in 1..new_grid.size {
                let previous = *self.trees.get(row, col - 1);
                let new_previous = *new_grid.get(row, col - 1);
                new_grid.set(row, col, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn right_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 0..new_grid.size {
            for col in 2..new_grid.size + 1 {
                let col_rev = new_grid.size - col;
                let previous = *self.trees.get(row, col_rev + 1);
                let new_previous = *new_grid.get(row, col_rev + 1);
                new_grid.set(row, col_rev, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn top_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 1..new_grid.size {
            for col in 0..new_grid.size {
                let previous = *self.trees.get(row - 1, col);
                let new_previous = *new_grid.get(row - 1, col);
                new_grid.set(row, col, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn bottom_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 2..new_grid.size + 1 {
            let row_rev = new_grid.size - row;
            for col in 0..new_grid.size {
                let previous = *self.trees.get(row_rev + 1, col);
                let new_previous = *new_grid.get(row_rev + 1, col);
                new_grid.set(row_rev, col, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn find_visible(&self) -> u32 {
        let exterior_total = (self.trees.size * 4) - 4;

        let left_heights = self.left_max_height();
        let right_heights = self.right_max_height();
        let top_heights = self.top_max_height();
        let bottom_heights = self.bottom_max_height();

        (exterior_total
            + self
                .trees
                .iter_no_border()
                .filter(|(h, row, col)| {
                    top_heights.get(*row, *col) < h
                        || bottom_heights.get(*row, *col) < h
                        || left_heights.get(*row, *col) < h
                        || right_heights.get(*row, *col) < h
                })
                .count()) as u32
    }

    pub fn best_scenic_score(&self) -> u32 {
        let end_of_grid = self.trees.size - 1;

        self.trees
            .iter_no_border()
            .map(|(h, row, col)| {
                let up_collision = (0usize..row)
                    .filter(|i| self.trees.get(*i, col) >= h)
                    .next_back()
                    .map_or(row, |t| row - t);

                let down_collision = (row + 1..self.trees.size)
                    .find(|i| self.trees.get(*i, col) >= h)
                    .map_or(end_of_grid - row, |t| t - row);

                let left_collision = (0usize..col)
                    .filter(|i| self.trees.get(row, *i) >= h)
                    .next_back()
                    .map_or(col, |t| col - t);

                let right_collision = (col + 1..self.trees.size)
                    .find(|i| self.trees.get(row, *i) >= h)
                    .map_or(end_of_grid - col, |t| t - col);

                up_collision * down_collision * left_collision * right_collision
            })
            .max()
            .unwrap() as u32
    }
}

impl Day08 {
    fn parse(&self, data: &str) -> Forest {
        let grid_size = data.lines().next().unwrap().trim().len();
        let mut forest = Forest::new(grid_size);

        forest
            .add_trees(&mut data.chars().filter(|c| !c.is_whitespace()).map(|c| (c as u8) - b'0'));

        forest
    }

    fn solve_actual(&self, forest: &Forest) -> u32 {
        forest.find_visible()
    }

    fn solve_actual_part2(&self, forest: &Forest) -> u32 {
        forest.best_scenic_score()
    }
}

impl Problem for Day08 {
    fn day(&self) -> u8 {
        8u8
    }
    fn name(&self) -> &str {
        "Day 8: Treetop Tree House"
    }
    fn solve(&self) -> Solution {
        Solution::U32(self.solve_actual(&self.parse(PROBLEM_INPUT)))
    }
    fn solve_part2(&self) -> Solution {
        Solution::U32(self.solve_actual_part2(&self.parse(PROBLEM_INPUT)))
    }
}

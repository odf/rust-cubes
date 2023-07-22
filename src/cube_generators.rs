use crate::backtrack::BackTrackIterator;
use crate::backtrack::BackTracking;


type Position = [i32; 3];
type Direction = [i32; 3];
type Symmetry = [[i32; 3]; 3];


static DIRECTIONS: [Direction; 6] = [
    [-1,  0,  0],
    [ 1,  0,  0],
    [ 0,  1,  0],
    [ 0, -1,  0],
    [ 0,  0,  1],
    [ 0,  0, -1],
];


static SYMMETRIES: [Symmetry; 24] = [
    [[ 1,  0,  0], [ 0,  1,  0], [ 0,  0,  1]],
    [[ 1,  0,  0], [ 0, -1,  0], [ 0,  0, -1]],
    [[-1,  0,  0], [ 0,  1,  0], [ 0,  0, -1]],
    [[-1,  0,  0], [ 0, -1,  0], [ 0,  0,  1]],

    [[ 1,  0,  0], [ 0,  0,  1], [ 0, -1,  0]],
    [[ 1,  0,  0], [ 0,  0, -1], [ 0,  1,  0]],
    [[-1,  0,  0], [ 0,  0,  1], [ 0,  1,  0]],
    [[-1,  0,  0], [ 0,  0, -1], [ 0, -1,  0]],

    [[ 0,  1,  0], [ 0,  0,  1], [ 1,  0,  0]],
    [[ 0,  1,  0], [ 0,  0, -1], [-1,  0,  0]],
    [[ 0, -1,  0], [ 0,  0,  1], [-1,  0,  0]],
    [[ 0, -1,  0], [ 0,  0, -1], [ 1,  0,  0]],

    [[ 0,  1,  0], [ 1,  0,  0], [ 0,  0, -1]],
    [[ 0,  1,  0], [-1,  0,  0], [ 0,  0,  1]],
    [[ 0, -1,  0], [ 1,  0,  0], [ 0,  0,  1]],
    [[ 0, -1,  0], [-1,  0,  0], [ 0,  0, -1]],

    [[ 0,  0,  1], [ 1,  0,  0], [ 0,  1,  0]],
    [[ 0,  0,  1], [-1,  0,  0], [ 0, -1,  0]],
    [[ 0,  0, -1], [ 1,  0,  0], [ 0, -1,  0]],
    [[ 0,  0, -1], [-1,  0,  0], [ 0,  1,  0]],

    [[ 0,  0,  1], [ 0,  1,  0], [-1,  0,  0]],
    [[ 0,  0,  1], [ 0, -1,  0], [ 1,  0,  0]],
    [[ 0,  0, -1], [ 0,  1,  0], [ 1,  0,  0]],
    [[ 0,  0, -1], [ 0, -1,  0], [-1,  0,  0]],
];


type Code = Vec<[usize; 2]>;
type Shape = Vec<Position>;


struct CubeBackTracking {
    max_size: usize,
}


impl BackTracking for CubeBackTracking {
    type State = (Code, Shape);
    type Item = Vec<Position>;

    fn root(&self) -> Self::State {
        (vec![], vec![[0, 0, 0]])
    }

    fn extract(&self, state: &Self::State) -> Option<Self::Item> {
        let (code, shape) = state;
        if code.len() == self.max_size - 1 {
            Some(shape.clone())
        } else {
            None
        }
    }

    fn children(&self, state: &Self::State) -> Vec<Self::State> {
        let (code, shape) = state;

        if code.len() >= self.max_size - 1 {
            vec![]
        } else {
            let start = if let Some(c) = code.last() { c[0] } else { 0 };
            let mut result = vec![];

            for i in start..shape.len() {
                let p = shape[i];

                for (j, d) in DIRECTIONS.iter().enumerate() {
                    let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

                    if !shape.contains(&q) {
                        let mut new_shape = shape.clone();
                        new_shape.push(q);

                        let mut new_code = code.clone();
                        new_code.push([i, j]);

                        if is_canonical(&new_shape, &new_code) {
                            result.push((new_code, new_shape));
                        }
                    }
                }
            }

            result
        }
    }
}


#[inline(never)]
fn is_canonical(shape: &Shape, code: &Code) -> bool {
    let mut seen = Vec::with_capacity(shape.len());

    for sym in SYMMETRIES {
        let dirs = map_directions(&DIRECTIONS, sym);
        for &p in shape {
            if compare_encoding(&mut seen, &shape, &dirs, p, &code) < 0 {
                return false;
            }
        }
    }

    true
}


#[inline(never)]
fn map_directions(dirs: &[Direction], sym: Symmetry) -> Vec<Direction> {
    let mut dirs_out = vec![];

    for d in dirs {
        dirs_out.push([
            d[0] * sym[0][0] + d[1] * sym[1][0] + d[2] * sym[2][0],
            d[0] * sym[0][1] + d[1] * sym[1][1] + d[2] * sym[2][1],
            d[0] * sym[0][2] + d[1] * sym[1][2] + d[2] * sym[2][2],
        ]);
    }

    dirs_out
}


#[inline(never)]
fn compare_encoding(
    seen: &mut Vec<Position>,
    shape: &Shape, dirs: &[Direction], start: Position, code: &Code
)
    -> i32
{
    seen.clear();
    seen.push(start);

    let mut n = 0;
    let mut k = 0;

    while n < seen.len() {
        let p = seen[n];

        for j in 0..6 {
            let d = dirs[j];
            let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

            if shape.contains(&q) && !seen.contains(&q) {
                let c = [n, j];

                if c < code[k] {
                    return -1;
                } else if c > code[k] {
                    return 1;
                } else {
                    seen.push(q);
                    k += 1;
                }
            }
        }
        n += 1;
    }

    0
}


pub struct Cubes(BackTrackIterator<CubeBackTracking>);

impl Cubes {
    pub fn new(n: usize) -> Cubes {
        Cubes(BackTrackIterator::new(CubeBackTracking { max_size: n }))
    }
}

impl Iterator for Cubes {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

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


fn map_shape(shape: &Shape, sym: Symmetry) -> Shape {
    let mut new_shape = vec![];

    for p in shape {
        new_shape.push([
            p[0] * sym[0][0] + p[1] * sym[1][0] + p[2] * sym[2][0],
            p[0] * sym[0][1] + p[1] * sym[1][1] + p[2] * sym[2][1],
            p[0] * sym[0][2] + p[1] * sym[1][2] + p[2] * sym[2][2],
        ]);
    }

    new_shape
}


fn decode(code: &Code) -> Shape {
    let mut cubes = Vec::from([[0, 0, 0]]);

    for [from, dir] in code {
        let p = cubes[*from];
        let d = DIRECTIONS[*dir];
        let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

        assert!(!cubes.contains(&q));
        cubes.push(q);
    }

    cubes
}


fn encode(shape: &Shape, start: Position) -> Code {
    let mut cubes = Vec::from([start]);
    let mut code: Code = vec![];

    let mut n = 0;

    while n < cubes.len() {
        let p = cubes[n];

        for (j, d) in DIRECTIONS.iter().enumerate() {
            let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

            if shape.contains(&q) && !cubes.contains(&q) {
                cubes.push(q);
                code.push([n, j]);
            }
        }
        n += 1;
    }

    code
}


struct CubeBackTracking {
    max_size: usize,
}


impl BackTracking for CubeBackTracking {
    type State = Code;
    type Item = Vec<Position>;

    fn root(&self) -> Code {
        vec![]
    }

    fn extract(&self, code: &Code) -> Option<Self::Item> {
        if code.len() == self.max_size - 1 {
            Some(decode(&code))
        } else {
            None
        }
    }

    fn children(&self, code: &Code) -> Vec<Code> {
        let cubes = decode(code);

        if code.len() >= self.max_size - 1 {
            vec![]
        } else {
            let start = if let Some(c) = code.last() { c[0] } else { 0 };
            let mut result = vec![];

            for i in start..cubes.len() {
                let p = cubes[i];

                for (j, d) in DIRECTIONS.iter().enumerate() {
                    let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

                    if !cubes.contains(&q) {
                        let mut new_shape = cubes.clone();
                        new_shape.push(q);

                        let mut new_code = code.clone();
                        new_code.push([i, j]);

                        if is_canonical(&new_shape, &new_code) {
                            result.push(new_code);
                        }
                    }
                }
            }

            result
        }
    }
}


fn is_canonical(shape: &Shape, code: &Code) -> bool {
    for sym in SYMMETRIES {
        let mapped = map_shape(shape, sym);
        for &p in &mapped {
            let c = encode(&mapped, p);
            if c < *code {
                return false;
            }
        }
    }

    true
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


#[test]
fn test_decode() {
    assert_eq!(
        decode(&vec![]),
        Vec::from([[0, 0, 0]])
    );

    assert_eq!(
        decode(&vec![[0, 1]]),
        Vec::from([[0, 0, 0], [1, 0, 0]])
    );

    assert_eq!(
        decode(&vec![[0, 0], [0, 1]]),
        Vec::from([[0, 0, 0], [-1, 0, 0], [1, 0, 0]])
    );

    assert_eq!(
        decode(&vec![[0, 1], [1, 1]]),
        Vec::from([[0, 0, 0], [1, 0, 0], [2, 0, 0]])
    );
}


#[test]
fn test_encode() {
    assert_eq!(
        encode(&vec![[0, 0, 0]], [0, 0, 0]),
        Vec::<[usize; 2]>::new()
    );

    assert_eq!(
        encode(&vec![[0, 0, 0], [1, 0, 0]], [0, 0, 0]),
        vec![[0, 1]]
    );

    assert_eq!(
        encode(&vec![[0, 0, 0], [1, 0, 0], [-1, 0, 0]], [0, 0, 0]),
        vec![[0, 0], [0, 1]]
    );

    assert_eq!(
        encode(&vec![[0, 0, 0], [1, 0, 0], [2, 0, 0]], [0, 0, 0]),
        vec![[0, 1], [1, 1]]
    );
}

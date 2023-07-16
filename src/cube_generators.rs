use std::collections::HashMap;
use std::collections::HashSet;

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


type Code = Vec<[usize; 6]>;
type Shape = HashSet<Position>;


fn map_shape(shape: &Shape, sym: Symmetry) -> Shape {
    let mut new_shape = HashSet::new();

    for p in shape {
        new_shape.insert([
            p[0] * sym[0][0] + p[1] * sym[1][0] + p[2] * sym[2][0],
            p[0] * sym[0][1] + p[1] * sym[1][1] + p[2] * sym[2][1],
            p[0] * sym[0][2] + p[1] * sym[1][2] + p[2] * sym[2][2],
        ]);
    }

    new_shape
}


fn decode(code: &Code) -> Vec<Position> {
    let mut shape = HashSet::from([[0, 0, 0]]);
    let mut cubes = Vec::from([[0, 0, 0]]);

    for (i, line) in code.iter().enumerate() {
        let p = cubes[i];

        for (j, d) in DIRECTIONS.iter().enumerate() {
            let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

            if line[j] > 0 {
                if !shape.contains(&q) {
                    assert_eq!(line[j], shape.len() + 1);
                    shape.insert(q);
                    cubes.push(q);
                } else {
                    assert_eq!(q, cubes[line[j] - 1]);
                }
            } else {
                assert!(!shape.contains(&q))
            }
        }
    }

    cubes
}


fn encode(shape: &Shape, start: Position) -> Code {
    let mut index = HashMap::from([(start, 1)]);
    let mut cubes = Vec::from([start]);
    let mut code: Code = vec![];

    while code.len() < cubes.len() {
        let p = cubes[code.len()];
        let mut c = [0; 6];

        for (j, d) in DIRECTIONS.iter().enumerate() {
            let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

            if shape.contains(&q) {
                if let Some(&k) = index.get(&q) {
                    c[j] = k;
                } else {
                    cubes.push(q);
                    c[j] = cubes.len();
                    index.insert(q, cubes.len());
                }
            }
        }

        code.push(c);
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
        vec![[0; 6]]
    }

    fn extract(&self, code: &Code) -> Option<Self::Item> {
        if code.len() == self.max_size {
            Some(decode(&code))
        } else {
            None
        }
    }

    fn children(&self, code: &Code) -> Vec<Code> {
        let cubes = decode(code);
        let shape: HashSet<_> = cubes.iter().cloned().collect();
        let index: HashMap<_, _> = cubes.iter().enumerate()
            .map(|(i, p)| (p, i + 1))
            .collect();

        let mut result = vec![];

        for (i, p) in cubes.iter().enumerate() {
            for (j, d) in DIRECTIONS.iter().enumerate() {
                let q = [p[0] + d[0], p[1] + d[1], p[2] + d[2]];

                if !shape.contains(&q) {
                    let mut new_shape = shape.clone();
                    new_shape.insert(q);

                    let mut new_code = code.clone();
                    new_code[i][j] = code.len() + 1;

                    let mut c = [0; 6];
                    for (k, e) in DIRECTIONS.iter().enumerate() {
                        let r = [q[0] + e[0], q[1] + e[1], q[2] + e[2]];
                        if let Some(&nu) = index.get(&r) {
                            c[k] = nu;
                        }
                    }
                    new_code.push(c);

                    if is_canonical(&new_shape, &new_code) {
                        result.push(new_code);
                    }
                }
            }
        }

        result
    }
}


fn is_canonical(shape: &HashSet<Position>, code: &Code) -> bool {
    todo!()
}


#[test]
fn test_decode() {
    assert_eq!(
        decode(&vec![
            [0, 0, 0, 0, 0, 0]
        ]),
        Vec::from([
            [0, 0, 0]
        ])
    );

    assert_eq!(
        decode(&vec![
            [0, 2, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0],
        ]),
        Vec::from([
            [0, 0, 0], [1, 0, 0],
        ])
    );

    assert_eq!(
        decode(&vec![
            [2, 3, 0, 0, 0, 0], [0, 1, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0],
        ]),
        Vec::from([
            [0,  0,  0], [-1,  0,  0], [1,  0,  0],
        ])
    );

    assert_eq!(
        decode(&vec![
            [0, 2, 0, 0, 0, 0], [1, 3, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0],
        ]),
        Vec::from([
            [ 0,  0,  0], [ 1,  0,  0], [ 2,  0,  0],
        ])
    );
}


#[test]
fn test_encode() {
    assert_eq!(
        encode(
            &HashSet::from([
                [0, 0, 0]
            ]),
            [0, 0, 0]
        ),
        vec![
            [0, 0, 0, 0, 0, 0]
        ]
    );

    assert_eq!(
        encode(
            &HashSet::from([
                [0, 0, 0], [1, 0, 0],
            ]),
            [0, 0, 0]
        ),
        vec![
            [0, 2, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0],
        ]
    );

    assert_eq!(
        encode(
            &HashSet::from([
                [ 0,  0,  0], [ 1,  0,  0], [-1,  0,  0],
            ]),
            [0, 0, 0]
        ),
        vec![
            [2, 3, 0, 0, 0, 0], [0, 1, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0],
        ]
    );

    assert_eq!(
        encode(
            &HashSet::from([
                [ 0,  0,  0], [ 1,  0,  0], [ 2,  0,  0],
            ]),
            [0, 0, 0]
        ),
        vec![
            [0, 2, 0, 0, 0, 0], [1, 3, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0],
        ]
    );
}

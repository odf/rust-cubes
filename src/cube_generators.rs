use std::collections::HashMap;
use std::collections::HashSet;

use crate::backtrack::BackTrackIterator;
use crate::backtrack::BackTracking;


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
type Shape = HashSet<[i32; 3]>;


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


fn decode(code: &Code) -> Shape {
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

    shape
}


fn encode(shape: &Shape, start: [i32; 3]) -> Code {
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


struct CubeGenState {
    code: Vec<usize>,
}


struct CubeBackTracking {
    max_size: usize,
}


#[test]
fn test_decode() {
    assert_eq!(
        decode(&vec![
            [0, 0, 0, 0, 0, 0]
        ]),
        HashSet::from([
            [0, 0, 0]
        ])
    );

    assert_eq!(
        decode(&vec![
            [0, 2, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0],
        ]),
        HashSet::from([
            [0, 0, 0], [1, 0, 0],
        ])
    );

    assert_eq!(
        decode(&vec![
            [2, 3, 0, 0, 0, 0], [0, 1, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0],
        ]),
        HashSet::from([
            [ 0,  0,  0], [ 1,  0,  0], [-1,  0,  0],
        ])
    );

    assert_eq!(
        decode(&vec![
            [0, 2, 0, 0, 0, 0], [1, 3, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0],
        ]),
        HashSet::from([
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

#[derive(Copy,Clone)]

/*
A tree cell becomes a fire cell only if there are at least 4⋅size^−1nearby fire cells.

Any tree cell which catches on fire transfers its associated integer directly.

A fire cell always decrements its associated integer by 1, unless it is 1, at which point it becomes smouldering.

Smouldering and ground cells do not change.
*/
enum ForestState{
    Tree(u8),
    Ground,
    Fire(u8),
    Smouldering,
}
//[nw, n, ne
//  w  c   e
//  sw s   se  ]
type Neighborhood = [ForestState; 9];

const NEIGHBORHOOD: [(i64,i64);9] = [
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
    (-1,  0),
    ( 0,  0),
    ( 1,  0),
    (-1, -1),
    ( 0, -1),
    ( 1, -1)

];

const LENGTH : usize = 30;

const WIDTH : usize = 15;
//shoulnt the struct Forest be consisted of Neighborhood types???
struct Forest([[ForestState; LENGTH]; WIDTH]);

fn local_transition(n : Neighborhood) -> ForestState {
    match n[4] {
        Tree(size) => {
            if n.iter().filter(
                |x| match x {Fire(_)=> true, _=> false}
            ).count() >= 4.0/(size as f32) {Fire(size)}
            else{Tree(size)}
        }
        Fire(1) => Smouldering,
        Fire(size) => Fire(size-1),

    }

}


fn universal_transition(forest : Forest) -> Forest {
    forest.into_iter().map(local_transition).collect()
}


fn main() {
    
}


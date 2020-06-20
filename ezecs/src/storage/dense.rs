use crate::entity::EntInd;

/// if n is the number of entities with the desired
/// component, dense storage is O(1) insert, O(n)
/// iteration and O(1) clear.
///
/// T generic is the data in this storage, and K
/// generic is K::MAX_VALUE == max entities with
/// component T.
///
/// this storage is the fastest but takes the most
/// memory, but can be optimized by limiting the #
/// of bytes the K generic takes. Most cases don't
/// need millions or billions of entities with
/// component T.
///
/// this storage is most performant for components
/// owned by 25-75% of your entities. TODO find non
/// theoritical numbers
pub struct DenseStorage<T, K: Into<usize>>
{
    dense:  Vec<T>, // stores component data contiguously
    sparse: Vec<K>, // vec of length # entities in world,
                    // sparse[ent.id] points to dense ent
                    // data.

    id: Vec<EntInd>,// same size as dense, maps dense index
                    // to entity index. only used in join
                    // and iter entities operations.
}
#[derive(Debug, Clone)]
pub struct Node<K, V> 
where 
    K: Clone,
    V: Clone,
{
    pub key: K,
    pub value: V,
    pub prev: Option<K>,
    pub next: Option<K>,
}

impl<K, V> Node<K, V> 
where 
    K: Clone,
    V: Clone,
{
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}
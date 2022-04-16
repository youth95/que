pub mod terrains;


#[derive(Debug, Deserialize, Serialize)]
pub struct Weight<T>(pub T, pub f64);

use serde::{Deserialize, Serialize};

use crate::rng::RAND;
#[derive(Debug, Deserialize, Serialize)]
pub struct Pool<T>(Vec<Weight<T>>);

impl<T> Pool<T> {
    pub fn from_items(items: Vec<Weight<T>>) -> Pool<T> {
        let mut pool = Pool::<T>::new();
        for item in items {
            pool.push(item);
        }
        pool
    }

    pub fn from_items_with_average_probability(items: Vec<T>) -> Pool<T> {
        let mut pool = Pool::<T>::new();
        for item in items {
            pool.push(Weight(item, 1.));
        }
        pool
    }

    pub fn new() -> Pool<T> {
        let list = Vec::<Weight<T>>::new();
        Pool(list)
    }

    pub fn push(&mut self, item: Weight<T>) {
        self.0.push(item);
    }

    fn random(&self) -> f64 {
        let (_, value) = RAND.lock().unwrap().random();
        value
    }

    fn fetch(&self) -> &Weight<T> {
        let mut count = 0f64;
        for w in self.0.iter() {
            count += w.1;
        }
        let p = self.random() * count;
        let mut grand = 0.;
        for w in self.0.iter() {
            if p < w.1 + grand && p >= grand {
                return w;
            } else {
                grand += w.1;
            }
        }
        return &self.0[self.0.len() - 1];
    }

    pub fn fetch_item(&self) -> &T {
        &self.fetch().0
    }

    pub fn get_probability_list(&self) -> Vec<f64> {
        let mut count = 0f64;
        for w in self.0.iter() {
            count += w.1;
        }
        self.0.iter().map(|node| node.1 / count).collect::<Vec<_>>()
    }
}


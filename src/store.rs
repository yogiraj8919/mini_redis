// Store module for mini_redis

use std::{clone, collections::HashMap, sync::Arc};


use tokio::sync::RwLock;


#[derive(Clone)]
pub struct Store{
    inner:Arc<RwLock<HashMap<String,String>>>
}


impl Store {
     pub fn new() -> Self{
        Store{
            inner : Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn set(&self,key:String,value:String){
        let mut map  = self.inner.write().await;
        map.insert(key,value);
    }

    pub async fn get(&self,key:&str) -> Option<String>{
        let mut map = self.inner.read().await;
        map.get(key).cloned()
    }

    pub async fn del(&self, key:&str) -> bool{
        let mut map = self.inner.write().await;
        map.remove(key).is_some()
    }

}
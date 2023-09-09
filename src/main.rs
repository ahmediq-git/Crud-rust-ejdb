use ejdb::bson;
use ejdb::bson::oid::ObjectId;
use ejdb::query::{Q, QH};
use ejdb::Collection;
use ejdb::Database;
use ejdb::Result;
use std::time::{Duration, Instant};

const N: i64 = 1000000;

struct DB {
    db: Database,
}
impl DB {
    fn new(name: String) -> DB {
        let db = Database::open(name).unwrap();
        DB { db }
    }

    fn create_index(&self, collectionname: String, key: String) {
        let collection = self.db.collection(collectionname).unwrap();

        collection.index(key).number().set().unwrap();
    }

    fn addone(&self, collectionname: String, value: i64) -> ObjectId {
        // get collection/create collection if it does not exist
        let coll = self.db.collection(collectionname).unwrap();

        // create bson doc with given entries
        let d = bson! {
            "user_id" => value,
            "count" => 10,
            "age"=>40000
        };

        // saves the collection and returns the id of the doc
        let id = coll.save(&d).unwrap();

        id
    }

    fn find_one(&self, collectionname: String, id: i64) {
        // this function finds one record with the given id
        let coll = self.db.collection(collectionname).unwrap();

        let query = Q.field("user_id").eq(id);
        // println!("{:?}", query);

        let _ = coll.query(&query, QH.max(1)).find_one().unwrap();
    }

    fn find_one_by_objectid(&self, collectionname: String, id: ObjectId) {
        // this function finds one record with the given id
        let coll = self.db.collection(collectionname).unwrap();

        let query = Q.field("_id").eq(id);
        // println!("{:?}", query);

        let _ = coll.query(&query, QH.max(1)).find_one().unwrap();
    }
}

fn main() {
    let db = DB::new("test.db".to_string());
    let mut ids: Vec<ObjectId> = Vec::new();

    // db.create_index("user_idx".to_string(), "user_id".to_string());

    // // start timer
    let now = Instant::now();

    for i in 0..N {
        let id = db.addone("user_idx".to_string(), i);

        ids.push(id);

        // if i % 1000 == 0 {
        //     println!("{} records added", i);
        // }
    }
    let elapsed = now.elapsed();

    println!("Time taken to add {} records: {:?}", N, elapsed);
    println!("Time taken to add 1 record: {:?}", elapsed / N as u32);
    println!("Records per second: {}", N as f64 / elapsed.as_secs_f64());

    // // start timer
    let now = Instant::now();

    for i in 0..N {
        let id = ids.get(i as usize).unwrap();

        db.find_one_by_objectid("user_idx".to_string(), id.clone());
        // if i % 1000 == 0 {
        //     println!("{} records searched", i);
        // }
    }

    // for i in 0..N {
    //     db.find_one("user_idx".to_string(), i);
    //     // if i % 1000 == 0 {
    //     //     println!("{} records searched", i);
    //     // }
    // }

    let elapsed = now.elapsed();

    println!("Time taken to search {} records: {:?}", N, elapsed);
    println!("Time taken to search 1 record: {:?}", elapsed / N as u32);
    println!("Records per second: {}", N as f64 / elapsed.as_secs_f64());

    // println!("");
    // println!("Testing without index");

    // // do the same without index
    // // start timer
    // let now = Instant::now();

    // for i in 0..N {
    //     db.addone("user_standard".to_string(), i);
    //     // if i % 1000 == 0 {
    //     //     println!("{} records added", i);
    //     // }
    // }
    // let elapsed = now.elapsed();

    // println!("Time taken to add {} records: {:?}", N, elapsed);
    // println!("Time taken to add 1 record: {:?}", elapsed / N as u32);
    // println!("Records per second: {}", N as f64 / elapsed.as_secs_f64());

    // // start timer
    // let now = Instant::now();
    // for i in 0..N {
    //     db.find_one("user_standard".to_string(), i);
    //     // if i % 1000 == 0 {
    //     //     println!("{} records searched", i);
    //     // }
    // }

    // let elapsed = now.elapsed();

    // println!("Time taken to search {} records: {:?}", N, elapsed);
    // println!("Time taken to search 1 record: {:?}", elapsed / N as u32);
    // println!("Records per second: {}", N as f64 / elapsed.as_secs_f64());
}

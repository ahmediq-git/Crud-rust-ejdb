use ejdb::Collection;
use ejdb::Database;
use ejdb::bson;
use ejdb::bson::oid::ObjectId;
use ejdb::query::{Q, QH};
use ejdb::Result;

//creates collection
fn createCollection(db: &Database, collection: String){
    let coll = db.collection(collection).unwrap();
}

//adds one record
fn addone(db: &Database, collectionname: String, value: String)-> ObjectId {
    // get collection/create collection if it does not exist
    let coll = db.collection(collectionname).unwrap();

    // create bson doc with given entries
    let mut d = bson! {
        "name" => value,
        "count" => 10,
        "age"=>40000
    };

    // saves the collection and returns the id of the doc
    let inserted_id = coll.save(&d).unwrap();

    // adds a new field in an uncommited bson document
    //d.insert("name", inserted_id.clone());

    // saves after the new change
    // let inserted_id= coll.save(&d).unwrap();
    // println!("{}", d);

    // load is used to retrieve according to object id
    // let d2 = coll.load(&inserted_id).unwrap().unwrap();

    //println!("{}", d2);
    // assert_eq!(d, d2);
    return inserted_id
}

// adds multiple entries
fn addmultiple(db: &Database){
    let coll = db.collection("some_collection").unwrap();
    coll.save_all(&[
        bson!{ "name" => "Foo1", "count" => 123 },
        bson!{ "name" => "Bar1", "items" => [4, 5, 6] }
    ]).unwrap();
}

// searches for particular name
fn search(db: &Database, collectionname: String){
    let coll = db.collection(collectionname).unwrap();
    // n is number of records

    let allnames = ["Foo", "Foo1", "Bar1"];
    let items = coll.query(Q.field("name").contained_in(allnames), QH.max(300))
        .find().unwrap();
    // `items` is an iterator which contains at maximum 12 records whose `name`
    // field is either "foo", "bar" or "baz"

    let items: Result<Vec<bson::Document>> = items.collect();  // collect them into a vector


    // prints all entries
    for item in items.unwrap() {
        println!("{:?}", item);
    }

    // print database meta data

    // let meta=db.get_metadata().unwrap();
    // println!("{:?}",meta);
}   

// just drops the collection
fn deleteCollection(db: &Database, name: String){
    db.drop_collection(name, true);
}

//deletes key in collection
fn deleteKey(db: &Database, collectionname: String, key: String, inserted_id: ObjectId){
    let coll = db.collection(collectionname).unwrap();
    let mut doc = coll.load(&inserted_id).unwrap().unwrap();
    doc.remove(&key);
    coll.save(&doc).unwrap();
}

// changes count value of where name specified
fn update(db: &Database, collectionname: String, name: String){
    let coll = db.collection(collectionname).unwrap();
    coll.query(Q.field("name").eq(name).set("count", 308329), QH.empty()).update().unwrap();
}

fn main() {
    //if db exists, it opens it. If db does not exist, it creates said db
    let db = Database::open("test.db").unwrap();
    println!("Add one");
    let id= addone(&db, "some_collection".to_string(), "Foo".to_string());
    println!("Add Multiple");
    addmultiple(&db);
    println!("Search all");
    search(&db, "some_collection".to_string());
    println!("Update Foo's count");
    update(&db, "some_collection".to_string(), "Foo".to_string());
    println!("Delete key");
    deleteKey(&db, "some_collection".to_string(), "age".to_string(), id);
    println!("Search again");
    search(&db, "some_collection".to_string());

    //deleteCollection(&db, "some_collection".to_string());
}   

extern crate nearest_neighbor;

use bson::doc;
use futures::stream::StreamExt;
use mongodb::{options::ClientOptions, Client};
use nearest_neighbor::*;
use std::env;
use std::error::Error;

// async fn upload_dataset<T: Serialize>(dataset_collection: Collection<T>) {
//   let test_dataset: Vec<Datum<Point2d>> = create_dataset(1000);

//   dataset_collection.insert_many(test_dataset, None).await?;
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  loadenv::load().ok();

  let mongodb_connect_string = match env::var("MONGODB_CONNECTION_STRING") {
    Ok(val) => val,
    Err(_) => panic!("MONGODB_CONNECTION_STRING not set"),
  };

  let mut client_options = ClientOptions::parse(mongodb_connect_string.as_str()).await?;

  client_options.app_name = Some("KNN".to_string());

  let client = Client::with_options(client_options)?;

  let db = client.database("knn_demo");

  let dataset_collection = db.collection::<Datum<Point2d>>("dataset");

  let query_point = Point2d::new_random();

  let aggregation = [
    doc! {
      "$addFields": {
        "distance": {
          "$sqrt": {
            "$add": [
              {
                "$pow": [
                  {
                    "$subtract": [
                      "$point.x",
                      query_point.x
                    ]
                  },
                  2
                ],
                "$pow": [
                  {
                    "$subtract": [
                      "$point.y",
                      query_point.y
                    ]
                  },
                  2
                ]
              }
            ]
          }
        }
      }
    },
    doc! {
      "$sort": {
        "distance": 1
      }
    },
    doc! {
      "$limit": 13
    },
    doc! {
      "$group": {
        "_id": "$class",
        "sum": {
          "$sum": 1,
        }
      }
    },
    doc! {
      "$sort": {
        "sum": -1
      }
    },
    doc! {
      "$limit": 1
    },
  ];

  let cursor: mongodb::Cursor<bson::Document> =
    dataset_collection.aggregate(aggregation, None).await?;

  let result: Vec<mongodb::error::Result<_>> = cursor.collect().await;

  for res in result {
    match res {
      Ok(doc) => println!("{:?}", doc),
      Err(err) => println!("{:?}", err),
    }
  }

  Ok(())
}

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::region::Region;
use s3::request::ResponseDataStream;
use s3::BucketConfiguration;
use serde::Serialize;
use tokio::io::AsyncRead;

use crate::config::Config;

#[derive(Debug, Serialize)]
pub struct StoredFile {
    pub key: String,
    pub name: String,
    pub size: u64,
    pub last_modified: String,
}

pub struct Storage {
    bucket: Box<Bucket>,
}

impl Storage {
    pub async fn new(config: &Config) -> Result<Self, S3Error> {
        let region = Region::Custom {
            region: config.region.clone(),
            endpoint: config.endpoint.clone(),
        };
        let credentials = Credentials::new(
            Some(&config.access_key),
            Some(&config.secret_key),
            None,
            None,
            None,
        )?;
        let bucket = Bucket::new(&config.bucket, region.clone(), credentials.clone())?
            .with_path_style();

        if !bucket.exists().await? {
            Bucket::create_with_path_style(
                &config.bucket,
                region,
                credentials,
                BucketConfiguration::default(),
            )
            .await?;
        }

        Ok(Self { bucket })
    }

    pub async fn list(&self) -> Result<Vec<StoredFile>, S3Error> {
        let mut files = Vec::new();
        for page in self.bucket.list(String::new(), None).await? {
            for object in page.contents {
                files.push(StoredFile {
                    name: display_name(&object.key),
                    key: object.key,
                    size: object.size,
                    last_modified: object.last_modified,
                });
            }
        }
        files.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
        Ok(files)
    }

    pub async fn upload<R>(&self, key: &str, reader: &mut R) -> Result<usize, S3Error>
    where
        R: AsyncRead + Unpin + ?Sized,
    {
        let response = self.bucket.put_object_stream(reader, key).await?;
        Ok(response.uploaded_bytes())
    }

    pub async fn download(&self, key: &str) -> Result<ResponseDataStream, S3Error> {
        self.bucket.get_object_stream(key).await
    }

    pub async fn delete(&self, key: &str) -> Result<(), S3Error> {
        self.bucket.delete_object(key).await?;
        Ok(())
    }
}

pub fn display_name(key: &str) -> String {
    key.split_once('_')
        .map(|(_, name)| name.to_string())
        .unwrap_or_else(|| key.to_string())
}

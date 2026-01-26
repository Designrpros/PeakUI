#![allow(dead_code)]
use crate::brain::Error;

use sipper::{sipper, Straw};
#[cfg(feature = "native")]
use tokio::fs;
#[cfg(feature = "native")]
use tokio::io::{self, AsyncWriteExt};

use std::path::Path;
#[cfg(feature = "native")]
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub struct Progress {
    pub total: Option<u64>,
    pub downloaded: u64,
    pub speed: u64,
}

impl Progress {
    pub fn percent(self) -> Option<(u64, u32)> {
        let total = self.total?;

        Some((
            total,
            (self.downloaded as f32 / total as f32 * 100.0).round() as u32,
        ))
    }
}

pub fn download_file<'a>(
    url: impl AsRef<str> + Send + 'a,
    destination: impl AsRef<Path> + Send + 'a,
) -> impl Straw<(), Progress, Error> + 'a {
    sipper(
        move |#[cfg_attr(not(feature = "native"), allow(unused_mut))] mut progress| async move {
            #[cfg(feature = "native")]
            {
                let url = url.as_ref();
                let destination = destination.as_ref();
                let mut file = io::BufWriter::new(fs::File::create(destination).await?);

                // For now, we use HttpClient which reads the whole body.
                // In a future update, we can add streaming to HttpClient.
                let response = crate::http::HttpClient::get(url).await?;
                let body = response.bytes();
                let total = Some(body.len() as u64);
                let start = Instant::now();

                progress
                    .send(Progress {
                        total,
                        downloaded: body.len() as u64,
                        speed: (body.len() as f32 / start.elapsed().as_secs_f32()) as u64,
                    })
                    .await;

                file.write_all(body).await?;
                file.flush().await?;

                Ok(())
            }

            #[cfg(not(feature = "native"))]
            {
                let _ = (url, destination, progress);
                Err(Error::WasmError(
                    "File download not supported on this platform without the native feature"
                        .to_string(),
                ))
            }
        },
    )
}

use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::sync::watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filename = "data.txt";
    let (tx, mut rx) = watch::channel(false);
    tokio::spawn(watch_file_change(filename, tx));
    loop {
        let _ = rx.changed().await;
        if let Ok(contents) = read_file(filename).await {
            println!("File contents: {}", contents);
        }
    }
}

async fn watch_file_change(filename: &str, tx: watch::Sender<bool>) {
    let path = PathBuf::from(filename);
    let mut last_modified = None;
    loop {
        if let Ok(metadata) = path.metadata() {
            let modified = metadata.modified().unwrap();
            // println!("File modified: {:?}", modified);
            if last_modified != Some(modified) {
                last_modified = Some(modified);
                let _ = tx.send(true);
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(filename).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

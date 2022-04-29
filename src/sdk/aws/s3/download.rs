// let dir = Path::new(".test-data/downloads/");
// let key = "videos/ski-02.mp4";
// async fn download_file(client: &Client, bucket_name: &str, key: &str, dir: &Path) -> Result<()> {
// 	// VALIDATE
// 	if !dir.is_dir() {
// 		bail!("Path {} is not a directory", dir.display());
// 	}

// 	// create file path and parent dir(s)
// 	let file_path = dir.join(key);
// 	let parent_dir = file_path
// 		.parent()
// 		.ok_or_else(|| anyhow!("Invalid parent dir for {:?}", file_path))?;
// 	if !parent_dir.exists() {
// 		create_dir_all(parent_dir)?;
// 	}

// 	// BUILD - aws request
// 	let req = client.get_object().bucket(bucket_name).key(key);

// 	// EXECUTE
// 	let res = req.send().await?;

// 	// STREAM result to file
// 	let mut data: ByteStream = res.body;
// 	let file = File::create(&file_path)?;
// 	let mut buf_writer = BufWriter::new(file);
// 	while let Some(bytes) = data.try_next().await? {
// 		buf_writer.write(&bytes)?;
// 	}
// 	buf_writer.flush()?;

// 	Ok(())
// }
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    let args:Vec<String>=env::args().collect();
    let p=args.get(1).unwrap();
    // 获取当前工作目录
    let dir = std::path::Path::new(p);
    
    // 读取当前目录下的所有文件
    let mut files = fs::read_dir(dir)?
        .filter(|res| res.is_ok())
.filter(|entry| { let path = entry.clone();
   let path=path .as_ref().unwrap().path(); path.extension().and_then(|ext| ext.to_str()) == Some("mp4") }) 
        .map(|res| res.unwrap().path())
        .collect::<Vec<PathBuf>>();
    
    // 根据文件的创建时间对文件进行排序
    files.sort_by(|a, b| {
        let metadata_a = fs::metadata(a).unwrap();
        let metadata_b = fs::metadata(b).unwrap();
        let created_at_a = metadata_a.created().unwrap();
        let created_at_b = metadata_b.created().unwrap();
        created_at_a.cmp(&created_at_b)
    });
    
    // 重命名文件
    for (index, file) in files.iter().enumerate() {
        let new_name = format!("{}.mp4", index + 1);
        fs::rename(file, file.with_file_name(new_name))?;
    }
    
    Ok(())
}

 
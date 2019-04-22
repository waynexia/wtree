use std::env;

pub struct Setting{
    pub is_all : bool,
    pub root : String,
}

pub fn parase_parameter() -> std::io::Result<Setting> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut setting = Setting{
        is_all : true,
        root : "./".to_string(),
    };
    for i in args.iter() {
        match i.as_ref(){
            "-a" => setting.is_all = true,
            _ => setting.is_all = setting.is_all,
        }
    }

    Ok(setting)
}

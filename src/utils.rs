use std::{env, path::PathBuf};

pub fn extract_directory_from_env()->Option<PathBuf>{
    let args:Vec<String>=env::args().collect();

    let mut directory=None;

    for i in 0..args.len()-1{
        if args[i]=="--directory"{
            directory=Some(PathBuf::from(&args[i+1]));
            break;
    }
}
   directory
}
use clap::{load_yaml, App};
use verify_https_site_cert::download;
use verify_https_site_cert::download::CertDownloader;
use std::fs::File;
use std::io::Write;

fn main(){
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();
    if let Some(ref matches) = matches.subcommand_matches("download"){
        let fqdn = matches.value_of("fqdn").unwrap();
        let path = matches.value_of("path").unwrap();
        run_download(fqdn, path);
    }
}

fn run_download(fqdn:&str, path:&str){
    let downloader = download::new();
    let result = downloader.download_cert(fqdn).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&result).unwrap();
}

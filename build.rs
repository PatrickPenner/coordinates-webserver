fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/wrapper.cpp")
        .flag_if_supported("-I/usr/include/rdkit")
        .flag_if_supported("-std=c++17")
        .compile("coordinates-webserver");
    println!("cargo:rustc-link-lib=RDKitSmilesParse");
    println!("cargo:rustc-link-lib=RDKitRDGeneral");
    println!("cargo:rustc-link-lib=RDKitGraphMol");
    println!("cargo:rustc-link-lib=RDKitFileParsers");
    println!("cargo:rustc-link-lib=RDKitDistGeomHelpers");
}

pub mod app_config {
    // the playground metadata folder
    pub const PLAYGROUND_DIR: &str=  ".nix-playground-rs";
    // the filename of original checked out package name in the PLAYGROUND_DIR folder, like
    // .nix-playground/pkg_name
    pub const PKG_NAME: &str = "pkg_name";
    // the filename of JSON dump from nix der file in the PLAYGROUND_DIR folder, like
    // .nix-playground/drv.json
    pub const DRV_JSON_FILE: &str = "drv.json";
    // the filename of link to nix package in the PLAYGROUND_DIR folder, like
    // .nix-playground/pkg
    pub const PKG_LINK : &str = "pkg";
    // the filename of link to nix source code in the PLAYGROUND_DIR folder, like
    // .nix-playground/src
    pub const SRC_LINK: &str = "src";
    // the filename of link to nix build output in the PLAYGROUND_DIR folder, like
    // .nix-playground/result
    pub const RESULT_LINK : &str = "result";
    // the filename of link to checked out folder in PLAYGROUND_DIR folder, like
    // .nix-playground/checkout
    pub const CHECKOUT_LINK: &str = "checkout";
    // the filename of generated patch file in PLAYGROUND_DIR folder, like
    // .nix-playground/checkout.patch
    pub const PATCH_FILE: &str = "checkout.patch";
    // the filename of package patch files folder in PLAYGROUND_DIR folder, like
    // .nix-playground/pkg-patches
    pub const PKG_PATCHES_DIR: &str = "pkg-patches";
    pub const CHECKOUT_GIT_AUTHOR_NAME: &str = "Sudhanshu-Bharti";
    pub const CHECKOUT_GIT_AUTHOR_EMAIL : &str= "sudhanshubharti.dev@gmail.com";
    // the default checkout folder name
    pub const DEFAULT_CHECKOUT_DIR: &str = "checkout";
    // the default flake to use if not provided
    pub const DEFAULT_FLAKE: &str = "nixpkgs";
}

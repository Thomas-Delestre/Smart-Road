fn main() {
    println!("cargo:rustc-link-search=native=C:/libs/SDL2/x86_64-w64-mingw32/bin");
    println!("cargo:rustc-link-search=native=C:/libs/SDL2_ttf");
    
    println!("cargo:rustc-link-search=native=C:/libs/SDL2_image");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=sdl2_ttf");
    println!("cargo:rustc-link-lib=sdl2_image")
}
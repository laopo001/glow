fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use gl_generator::{Api, Fallbacks, Profile, Registry};
        use std::env;
        use std::fs::File;
        use std::path::Path;
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest = Path::new(&out_dir);
        let mut file = File::create(&dest.join("opengl_bindings.rs")).unwrap();
        Registry::new(
            Api::Gles2,
            (3, 3),
            Profile::Core,
            Fallbacks::All,
            [

            ],
        )
        .write_bindings(gl_generator::StructGenerator, &mut file)
        .unwrap();
    }
}

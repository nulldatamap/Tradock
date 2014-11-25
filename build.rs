
use std::io::Command;
use std::os;

fn main() {
  let out_dir = os::getenv( "OUT_DIR" ).unwrap();

  Command::new( "g++" ).args( &[ "src/consoleinterface.cpp"
                               , "-c", "-std=c++11", "-m64"
                               , "-fPIC", "-o" ] )
                     .arg( format!( "{}/consoleinterface.o", out_dir ) )
                     .status().unwrap();
  Command::new( "ar" ).args( &[ "crus", "libconsoleinterface.a"
                              , "consoleinterface.o" ] )
                    .cwd( &Path::new( &out_dir ) )
                    .status().unwrap();
  println!( "cargo:rustc-flags=-L {} -l consoleinterface:static -l stdc++:static", out_dir );
}

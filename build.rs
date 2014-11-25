use std::io::{Command, IoError};
use std::io::stdio::stderr;
use std::io::process::ProcessOutput;
use std::os;

fn compile_cpp_code( out_dir : &str ) -> Result<ProcessOutput, IoError> {
  Command::new( "g++" ).args( &[ "src/consoleinterface.cpp"
                               , "-c", "-std=c++11", "-m64"
                               , "-fPIC", "-o" ] )
                       .arg( format!( "{}/consoleinterface.o", out_dir ) )
                       .output()
}

fn generate_archive( out_dir : &str ) -> Result<ProcessOutput, IoError> {
  Command::new( "ar" ).args( &[ "crus", "libconsoleinterface.a"
                              , "consoleinterface.o" ] )
                      .cwd( &Path::new( &out_dir ) )
                      .output()
}

fn report_failure( pf : Result<ProcessOutput, IoError> ) {
  let po = pf.unwrap();
  if !po.status.success() {
    print!( "{}", String::from_utf8( po.output ).unwrap() );
    stderr().write_str( String::from_utf8( po.error ).unwrap().as_slice() );
    panic!( "Failed build phase." );
  }
}

fn main() {
  let out_dir = os::getenv( "OUT_DIR" ).unwrap();
  let libraries = "-l consoleinterface:static -l stdc++:static";
  report_failure( compile_cpp_code( out_dir.as_slice() ) );
  report_failure( generate_archive( out_dir.as_slice() ) );
  println!( "cargo:rustc-flags=-L {} {}", out_dir, libraries );
}

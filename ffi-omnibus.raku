use NativeCall; 

constant $n-path = './ffi-omnibus/target/debug/foo';

sub str2rust(Str is encoded('utf8')) is native($n-path) {*};
str2rust("hey");

sub str2raku() returns Pointer[Str] is native($n-path) {*};
say str2raku.deref;

sub add (int32, int32) returns int32 is native($n-path) { * };  
say add(1, 2); 


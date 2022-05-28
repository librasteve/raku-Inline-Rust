use NativeCall; 

constant $n-path = './dan/target/debug/dan';

sub str2rust(Str is encoded('utf8')) is native($n-path) {*};
str2rust("hey");

sub str2raku() returns Pointer[Str] is native($n-path) {*};
say str2raku.deref;

sub df_read_csv(Str is encoded('utf8')) is native($n-path) {*};
df_read_csv("./dan/src/iris.csv");

#`[
sub df_ret_csv(Str is encoded('utf8')) returns Pointer is native($n-path) {*};
my $dfp = df_ret_csv("./dan/src/iris.csv");
dd $dfp;

sub df_head(Pointer) is native($n-path) {*};
df_head($dfp);
#]

sub add (int32, int32) returns int32 is native($n-path) { * };  
say add(1, 2); 


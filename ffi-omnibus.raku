use NativeCall; 

constant $n-path = './ffi-omnibus/target/debug/foo';

## Rust FFI Omnibus: Integers
## http://jakegoulding.com/rust-ffi-omnibus/integers/

sub addition(int32, int32) returns int32 is native($n-path) { * }

say addition(1, 2); 

## Rust FFI Omnibus: String Arguments 
## http://jakegoulding.com/rust-ffi-omnibus/string-arguments/

sub how_many_characters(Str is encoded('utf8')) returns int32 is native($n-path) { * }

say how_many_characters("göes to élevên");

## Rust FFI Omnibus: String Return Values 
## http://jakegoulding.com/rust-ffi-omnibus/string-return-values/

sub theme_song_generate(uint8) returns Pointer[Str] is encoded('utf8') is native($n-path) { * }
sub theme_song_free(Pointer[Str]) is native($n-path) { * }

my \song = theme_song_generate(5);
say song.deref;
theme_song_free(song);

## Rust FFI Omnibus: Slice Arguments
## http://jakegoulding.com/rust-ffi-omnibus/slice-arguments/

sub sum_of_even(CArray[uint32], size_t) returns uint32 is native($n-path) { * }

my @numbers := CArray[uint32].new;
@numbers[$++] = $_ for 1..6; 

say sum_of_even( @numbers, @numbers.elems );

## Rust FFI Omnibus: Tuples 
## http:##jakegoulding.com/rust-ffi-omnibus/tuples/

#[ get segfault
class Tuple is repr('CStruct') {
    has uint32 $.x;
    has uint32 $.y;
}
sub flip_things_around(Tuple) returns Tuple is native($n-path) { * }

my \initial = Tuple.new( x => 10, y => 20 );
my \result  = flip_things_around(initial);
say result.x, result.y;
#]

## Rust FFI Omnibus: Objects
## http:##jakegoulding.com/rust-ffi-omnibus/objects/

class ZipCodeDatabase is repr('CPointer') {
    sub zip_code_database_new() returns ZipCodeDatabase is native($n-path) { * }
    sub zip_code_database_free(ZipCodeDatabase)         is native($n-path) { * }
    sub zip_code_database_populate(ZipCodeDatabase)     is native($n-path) { * }
    sub zip_code_database_population_of(ZipCodeDatabase, Str is encoded('utf8'))
                                         returns uint32 is native($n-path) { * }

    method new {
        zip_code_database_new
    }

    submethod DESTROY {        # Free data when the object is garbage collected.
        zip_code_database_free(self);
    }

    method populate {
        zip_code_database_populate(self)
    }

    method population_of( Str \zip ) {
        zip_code_database_population_of(self, zip);
    }
}

my \database = ZipCodeDatabase.new;
database.populate;

my \pop1 = database.population_of('90210');
my \pop2 = database.population_of('20500');
say pop1 - pop2;


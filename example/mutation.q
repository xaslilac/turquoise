
-- I wonder if there should be ---- "module documentation" comment type, that would be
-- required to be at the very beginning of a file.

-- Also, actually, how would global documentation like that work if the languages
-- compilation unit is an entire directory, and not a single file. Order might get weird.

--- This is a documentation comment, because it starts with three hyphens instead of just
--- two. It'll come up in the automatically generated documentation for you code when you
--- run `take docs`.
type Person:
	name: String;
	age: Number;;
type Person:
	; name : String
	; age  : Number
;

impl Person
	impl #Constructor
		func constructor(&mut self) -> Self
			.name = "";
			.age = 0;
			self ;;;;

let me = Person { name = "Kayla"; age = 42068; };
let me = Person:
	name = "Kayla";
	age = 42068;;
let me = Person:
	; name = "Kayla"
	; age = 42068
;

mut me.age += 1;;

mutation
	me.age += 1;
	me.name += "!";;

if condition:
	do_thing_1;
	do_thing_2;
	do_thing_3;;

if (
	condition
):
	do_thing_1;
	do_thing_2;
	do_thing_3;;

when thing
	x -> do_thing_1;
	y -> do_thing_2;
	z -> do_thing_3;;

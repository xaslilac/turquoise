fn hello::string name::string:
	"Hello {name}!";;

fn hello_to::string overload name::string:
	hello name;;

fn hello_to::string overload friend::Friend:
	hello friend.name;;

fn hello_eventually::string async id::string:
	hello db.get_name(id).await;;

fn hello_forever::string channel (name::string):
	for ...:
		yield hello name;;;

fn hello_forever::string async channel (id::string):
	let name = db.get_name(id).await;
	for ...:
		yield hello name;;;

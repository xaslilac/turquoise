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

fn hello_to::string name::T where T::Nameable
	hello name.name();;

trait Nameable:
	fn name::string self;;

impl Nameable for string:
	fn name::string self:
		self;;

impl Nameable for Friend:
	fn name::string self:
		self.name;;

fn get_user_name::Option{string} async overload id::string:
	db.get_name(id).await;;

fn get_user_name::string async overload user::User:
	db.get_name(user.id).await.unwrap_or(user.name);

fn get_user_name::string user::User;

type MapToUser = fn::string ::User;

let reacthor = import_lib("reacthor.so");

overload  + (a, b){
	return stringify(a) + stringify(b);
}

print reacthor;

let input = "";

fn p(){
	while(true){
		if(input != "") {
			print "from other thread: " + input;
			input = "";
		}
	}
}

let thread = reacthor.start_thread(p);

//print "hello";

let number = 0;

while(true){

	input = get_input("give me some input please?");	
}

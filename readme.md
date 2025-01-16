# Reacthor - Threads for Thor

Reacthor is a library for multithreading and parallelism for thorlang.


Reacthor is designed to be as simple as possible and enforces this through simple function based execution.

## Installation
---

You can either build from source: 

```bash
git clone https://github.com/Adotweb/reacthor 
cd reacthor
chmod +x ./install.sh 
./install.sh 
cp ./reacthor.so /path/to/your/directory
```

or download the latest version from the github repositories `reacthor.so` file;


## Usage 

### Starting Threads 
To run a thread you can use the `reacthor.start_thread()` function;

```thorlang

//import reacthor 
let reacthor = import_lib("reacthor.so");

fn do_something(){
    print "hello";
}

//starts a decoupled thread
reacthor.start_thread(do_something);

//do something else
```

> **Note** since main threads don't controle whether or not your code has finished, they only run for as long as the main thread is alive. (see *loops in off-threads*)

### Loops in off-threads 
When running loops in main threads, they will not keep the main loop alive, even if the arent finished running themselves, this means that the following example:

```thorlang 
//import reacthor 
let reacthor = import_lib("reacthor.so");

fn do_something(){
    while(true){
        //do something in here that loops
    }
}

//starts a decoupled thread
reacthor.start_thread(do_something);

```
, will instantly exit. What you need to do to keep this loop running is introduce a keepalive, through something simple like this: 

```thorlang 
//import reacthor 
let reacthor = import_lib("reacthor.so");

fn do_something(){
    while(true){
        //do something in here that loops
    }
}

//starts a decoupled thread
reacthor.start_thread(do_something);


//add a keepalive
while(true){
    //do nothing, this just keeps the loop from do_something alive
}
```


### Variables
Since the functions running in off-threads are just simple functions, they can both access and mutate values from outside themselves. 


```thorlang 
let reacthor = import_lib("reacthor.so");

let value = "hello";

fn mutate_value(){
    value = 6;
}

//this will still mutate value;
reacthor.start_thread(mutate_value);

while(true){
    //at some point (when the off-thread is done running) this will print the new value;
    print value;
}
```

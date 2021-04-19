// For more comments about what's going on here, check out the `hello_world`
// example
const rust = import('./pkg');
rust
  .then(({add, Machine}) => {

    console.log('1 + 2 = ' + add(1, 2));
    console.log("Joshy was here");
    // console.log(greet("Joshy"));

    let thing = new Machine();
    console.log("Machine Created");
    console.log(thing.toJSON());

    for( let i = 0; i < 10; i++ ) {
      // console.log(`Loop iteration ${i}`);
      thing.inc_a();
      console.log(thing.a);
    }

    console.log(`The inner storage item is ${thing.get_inner_storage_item()}`)
  })
  .catch(console.error);

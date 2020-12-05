# bc-rust
### Instruction on how to run:
- git clone blake's branch
- cargo build + cargo run
- open "localhost:8080" in any browser (preferabbly Chrome, some functions might break in IE, but really I trust you not to use IE). 
- on the browser, start creating new block, change preference .etc
- please note that "mine" button is not functional yet (for now it only send the new block data and block id to Rust. the blockchain isn't mined again yet)

### How everything is connected together: 
- all commands from front-end (html + JavaScript) are passed to rust using HTML REQUEST (Trevor: you should not worry about any of this, I have all variables you'd need already parsed back to Rust for you to just pickup and use)
- all of the data from rust to html will be parsed as JSON file 'output.json' at the root directory
- Rust will keep on overwritting JSON file to add new block or refresh data
- front end will always loop thru the json file as it gets updated and display everything inside.

### What needs to be done next:
@Trevor: "mine_from" method for struct Blockchain: whenever the "Mine" button is pressed, the current "block data" will be sent to Rust. Rust will then mine that block and all of the blocks following it.
  - you will need mine_id and mine_content of the clicked block to get the new content as well as the box id (see comment on main.rs, from line 280).
@Chaewon or Blake: Disable "Mine" button on each block's form on default. Button is activated only if form's val.mined === False or content of block_data is edited
@Chaewon or Blake: If leading zeros was changed, remove all blocks from front-end

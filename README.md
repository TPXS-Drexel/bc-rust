# bc-rust
### Instruction on how to run:
- git clone blake's branch
- cargo build + cargo run
- open "localhost:8080" in any browser (preferabbly Chrome, some functions might break in IE, but really I trust you not to use IE). 
- on the browser, start creating new block, change preference .etc

### How everything is connected together: 
- all commands from front-end (html + JavaScript) are passed to rust using HTML REQUEST 
- all of the data from rust to html will be parsed as JSON file 'output.json' at the root directory
- Rust will keep on overwritting JSON file to add new block or refresh data
- front end will always loop thru the json file as it gets updated and display everything inside.

### What needs to be done next:


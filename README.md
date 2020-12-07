# bc-rust
### Instruction on how to run:
- git clone main branch
In the project directory:
- sudo docker build . -t rust_bc:v2 
- docker run -d -p 80:8080 rust_bc:v2
- open "localhost:80" in any browser (preferabbly Chrome/Safari, some functions might break in IE, but really I trust you not to use IE). 
- on the browser, start creating new block, change preference .etc

### How everything is connected together: 
- all commands from front-end (html + JavaScript) are passed to rust using HTML REQUEST 
- all of the data from rust to html will be parsed as JSON file 'output.json' at the root directory
- Rust will keep on overwritting JSON file to add new block or refresh data
- front end will always loop thru the json file as it gets updated and display everything inside.



### Docker Steps:
- sudo docker build . -t rust_bc:v2 
- docker run -d -p 80:8080 rust_bc:v2

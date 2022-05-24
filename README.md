# cat_time
The meowing of cats reminds us that life is fleeting. 

## Functionality
The program will play a random cat sound from a folder containing meow_x.mp3s. 
For more fun, cron suggestion provided to make it into an office hourly alarm. 

## To Run
4) Download files.
5.a) Check your version of Rust with:
```
rustc --version
```
5.b)If it doesn't exist, please consult your package manager of choice, then proceed.
6) In the project direct test:
```
cargo run
```

### For EXTRA Fun
1) Create a recurring schedule for the cat clock to run
```
crontab -l
```
2) Here is my recommendation for an office clock:
```
0 9-17 * * 1-5 cargo run
```

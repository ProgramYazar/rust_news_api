# Rust Language Restfull News API


###  <span style="color: orange; margin-right: 20px"> Please read: </span> Of course, I have not implemented all the functions such as paging, ... but in the source code you can find place holders for this purpose. This is just a demo project designed to interview only. 

<br><br>

# Project 

<br/>


* This project use Rocket Framework for api design, http requests like json request,...
* i've created a very basic file cache system for caching
* I use GNews api for backend


## Instructions:
* Install rust via 
```shell 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
```


## Run Project


### For development test command: 
```shell 
API_KEY="blablabla" cargo run 
```
-----------

### For release test command: 
```shell 
API_KEY="blablabla" cargo run --release
```
-----------


\
\
\
Restfull api  endpoints for local development machine:


```shell 
$ curl "http://127.0.0.1:8000/searchByTitle/Best%20Programmer"
$ curl "http://127.0.0.1:8000/searchByDescription/Best%20Programmer"
$ curl "http://127.0.0.1:8000/searchByContent/Best%20Programmer"
```

\
\
Ofcouse, I didnt implement all functions like paging, ... but in source code you can find place holders for this purpose.
This is only demo. 



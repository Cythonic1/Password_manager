use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, Write};
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Derive Clone for Creds so it can be cloned when passed by reference
pub struct Locked;
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Derive Clone for Creds so it can be cloned when passed by reference
pub struct Unlocked;
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Derive Clone for Creds so it can be cloned when passed by reference
// Keep in mind change this to locked by default.
pub struct Creds{
    pub username: String,
    pub password: String,
    pub platform: String,
    pub description: String,
}

const MYPATH:&str ="/home/pythonic/Desktop/rust/password_manager/test";
#[derive(Debug, Clone)] // Derive Clone for Creds so it can be cloned when passed by reference
pub enum Errors{
    InvalidPassword,
    Invalidformat,
    NotCompeleteData
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Derive Clone for Creds so it can be cloned when passed by reference
pub struct Values<State=Locked>{
    pub value: Vec<Creds>,
    pub state: PhantomData<State>    
}

impl Values<Unlocked>{
    pub fn add(&mut self,cred: &Creds){
        self.value.push(cred.clone())
    }
    pub fn list(&self) {
        for cred in self.value.iter(){
            println!("{:#?}", cred)
        }
    }
    pub fn write_json(&self) {
        let json_data = serde_json::to_string_pretty(self).expect("can not convert to json");
        let path = format!("{MYPATH}/test.json");
        match File::create(&path) {
            Ok(mut file) => {
                let _ = file.write_all(json_data.as_bytes());
            }
            Err(e) => println!("{e:?}"),
        }
    }
    pub fn read_json() ->  Result<Values<Unlocked>, Errors>{
        let path = format!("{MYPATH}/test.json");
        let file = match  File::open(&path){
            Ok(file) => file,
            Err(_) => return Err(Errors::Invalidformat)
        };

        let reader = BufReader::new(file);
        let data: Values<Unlocked> = serde_json::from_reader(reader).map_err(|_| Errors::Invalidformat)?;
        println!("Read Sessusfuly");

        Ok(data)
    }


    pub fn lock(self) -> Values<Locked> {
         Values{
            value:self.value,
            state:PhantomData
        }
    }
}

impl Values<Locked>{
   pub fn unlock(self, key: String) -> Result<Values<Unlocked>,Errors>  {
        if key == "Hello".to_string(){
            Ok(Values {
                value : self.value,
                state: PhantomData
            })

        }else{
           Err(Errors::InvalidPassword)
        }
    }
    pub fn new() -> Values<Locked>{
        Values { 
            value: Vec::new(), 
            state: PhantomData
        }
    }
}

impl Creds {
    pub fn new(username: String, password: String, platform: String, description: String) -> Result<Self, Errors> {
        if !username.is_empty() && !password.is_empty() && !platform.is_empty() && !description.is_empty() {
            Ok(Creds{
                username,
                password,
                platform,
                description,
            })
        }else{
            Err(Errors::NotCompeleteData)
        }
    }
}

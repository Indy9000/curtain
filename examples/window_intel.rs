extern crate rustorm;
extern crate curtain;

use rustorm::codegen;
use rustorm::database::DatabaseDev;
use rustorm::db::postgres::Postgres;
use curtain::window;

fn main(){
    let pg:Result<Postgres,&str> = Postgres::new("postgres://postgres:p0stgr3s@localhost/bazaar_v5");
    match pg{
        Ok(pg) => {
            derive_all_windows(&pg);
        }
        Err(error) =>{
            println!("{}",error);
        }
    }
}


pub fn derive_all_windows<T:DatabaseDev>(db_dev:&T){
    let all_tables = codegen::get_all_tables(db_dev);
    window::extract_windows(&all_tables);
}
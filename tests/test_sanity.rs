extern crate supattrs;
use supattrs::rename;

#[rename]
fn test_run(){
    assert_eq!(2,2);
}
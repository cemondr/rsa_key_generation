use toy_rsa_lib::*;
#[allow(dead_code)]


fn lambda(p: u64, q: u64) -> u64{
    lcm(p-1,q-1)
}

#[test]
fn test_lambda(){
    assert_eq!(lambda(9,13),24)
}

#[test]
fn test_labmda_2(){
    assert_eq!(lambda(22,24),483)
}
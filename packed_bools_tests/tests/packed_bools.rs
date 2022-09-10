use packed_bools::PackedBooleans;

#[derive(Default, PackedBooleans)]
pub struct FooStruct<'a, T = ()> {
    #[pack_bools(active, admin, discount, premium, og, frozen, kyc, tos_agree)]
    pub booleans: u8,
    pub an_option: Option<&'a T>,
}

#[derive(Default, PackedBooleans)]
pub struct BarStruct<'a, T = ()> {
    #[pack_bools(active, tos_agree)]
    pub booleans: u8,
    pub an_option: Option<&'a T>,
}

#[test]
fn test_set_bools() {
    let mut account = FooStruct::<()>::default();

    account.set_active(true);
    account.set_admin(false);
    account.set_discount(true);
    account.set_premium(false);
    account.set_og(false);
    account.set_frozen(true);
    account.set_kyc(false);
    account.set_tos_agree(true);

    assert_eq!(account.get_active(), true);
    assert_eq!(account.get_admin(), false);
    assert_eq!(account.get_discount(), true);
    assert_eq!(account.get_premium(), false);
    assert_eq!(account.get_og(), false);
    assert_eq!(account.get_frozen(), true);
    assert_eq!(account.get_kyc(), false);
    assert_eq!(account.get_tos_agree(), true);
}

#[test]
fn test_set_bools_small_set() {
    let mut account = BarStruct::<()>::default();

    account.set_active(true);
    account.set_tos_agree(true);

    assert_eq!(account.get_active(), true);
    assert_eq!(account.get_tos_agree(), true);
}

#[test]
fn test_double_set_bools() {
    let mut account = FooStruct::<()>::default();

    account.set_active(true);
    account.set_admin(false);
    account.set_discount(true);
    account.set_premium(false);
    account.set_og(false);
    account.set_frozen(true);
    account.set_kyc(false);
    account.set_tos_agree(true);

    account.set_active(true);
    account.set_admin(false);
    account.set_discount(true);
    account.set_premium(false);
    account.set_og(false);
    account.set_frozen(true);
    account.set_kyc(false);
    account.set_tos_agree(true);

    assert_eq!(account.get_active(), true);
    assert_eq!(account.get_admin(), false);
    assert_eq!(account.get_discount(), true);
    assert_eq!(account.get_premium(), false);
    assert_eq!(account.get_og(), false);
    assert_eq!(account.get_frozen(), true);
    assert_eq!(account.get_kyc(), false);
    assert_eq!(account.get_tos_agree(), true);
}

#[test]
fn test_get_bools() {
    let account = FooStruct::<()>::default();
    assert_eq!(account.get_active(), false);
    assert_eq!(account.get_admin(), false);
    assert_eq!(account.get_discount(), false);
}

#[test]
fn test_create_struct() {
    let account = FooStruct::<()>::default();
    assert_eq!(account.booleans, 0);
    let account = BarStruct::<()>::default();
    assert_eq!(account.booleans, 0);
}

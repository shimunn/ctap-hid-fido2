//
// cargo test -- --test-threads=1
//

use ctap_hid_fido2::str_buf::StrBuf;
use ctap_hid_fido2::util;
use ctap_hid_fido2::*;
use ring::digest;
use std::convert::TryFrom;

#[test]
fn test_get_hid_devices() {
    get_hid_devices();
    assert!(true);
}

#[test]
fn test_wink() {
    wink(&Cfg::init()).unwrap();
    assert!(true);
}

#[test]
fn test_get_info() {
    get_info(&Cfg::init()).unwrap();
    assert!(true);
}

#[test]
fn test_get_info_u2f() {
    match ctap_hid_fido2::enable_info_param(&Cfg::init(), &InfoParam::VersionsU2Fv2) {
        Ok(result) => {
            if !result {
                // Skip
                return;
            }
        }
        Err(_) => assert!(false),
    };

    get_info_u2f(&Cfg::init()).unwrap();
    assert!(true);
}

#[test]
fn test_client_pin_get_retries() {
    let retry = get_pin_retries(&Cfg::init());
    println!("- retries = {:?}", retry);
    assert!(true);
}

#[test]
fn test_make_credential_with_pin_non_rk() {
    // parameter
    let rpid = "test.com";
    let challenge = b"this is challenge".to_vec();
    let pin = "1234";

    let att = make_credential(&Cfg::init(), rpid, &challenge, Some(pin)).unwrap();
    println!("Attestation");
    println!("{}", att);

    let ass = get_assertion(
        &Cfg::init(),
        rpid,
        &challenge,
        &att.credential_descriptor.id,
        Some(pin),
    )
    .unwrap();
    println!("Assertion");
    println!("{}", ass);

    assert!(true);
}

#[test]
fn test_credential_management_get_creds_metadata() {
    match ctap_hid_fido2::enable_info_param(&Cfg::init(), &InfoParam::VersionsFido21Pre) {
        Ok(result) => {
            if !result {
                // Skip
                return;
            }
        }
        Err(_) => assert!(false),
    };

    let pin = "1234";
    match ctap_hid_fido2::credential_management_get_creds_metadata(&Cfg::init(), Some(pin)) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_credential_management_enumerate_rps() {
    match ctap_hid_fido2::enable_info_param(&Cfg::init(), &InfoParam::VersionsFido21Pre) {
        Ok(result) => {
            if !result {
                // Skip
                return;
            }
        }
        Err(_) => assert!(false),
    };

    let pin = "1234";
    match ctap_hid_fido2::credential_management_enumerate_rps(&Cfg::init(), Some(pin)) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_bio_enrollment_get_fingerprint_sensor_info() {
    let mut skip = true;
    match ctap_hid_fido2::enable_info_option(&Cfg::init(), &InfoOption::UserVerificationMgmtPreview)
    {
        Ok(result) => {
            //println!("result = {:?}", result);
            if let Some(v) = result {
                //println!("some value = {}", v);
                if v {
                    skip = false
                };
            }
        }
        Err(_) => assert!(false),
    };

    // skip
    if skip {
        return;
    };

    match ctap_hid_fido2::bio_enrollment_get_fingerprint_sensor_info(&Cfg::init()) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_bio_enrollment_enumerate_enrollments() {
    let mut skip = true;
    match ctap_hid_fido2::enable_info_option(&Cfg::init(), &InfoOption::UserVerificationMgmtPreview)
    {
        Ok(result) => {
            if let Some(v) = result {
                if v {
                    skip = false
                };
            }
        }
        Err(_) => assert!(false),
    };

    if skip {
        return;
    };

    let pin = "1234";
    match ctap_hid_fido2::bio_enrollment_enumerate_enrollments(&Cfg::init(), pin) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    };
}

#[test]
fn test_enc_hmac_sha_256() {
    let key_str = "this is key.";
    let hasher = digest::digest(&digest::SHA256, &key_str.as_bytes());
    let key = <[u8; 32]>::try_from(hasher.as_ref()).unwrap();

    let message = "this is message.";
    let sig = enc_hmac_sha_256::authenticate(&key, message.as_bytes());
    print!("{}", StrBuf::bufh("- hmac signature", &sig));
    assert_eq!(
        sig,
        util::to_str_hex("BF3D3FCFC4462CDCBEBBBC8AF82EA38B7B5ED4259B2061322C57B5CA696D6080")
    );
}

#[test]
fn test_enc_aes256_cbc() {
    let key_str = "this is key.";
    let hasher = digest::digest(&digest::SHA256, &key_str.as_bytes());
    let key = <[u8; 32]>::try_from(hasher.as_ref()).unwrap();

    let message = "this is message.";
    let enc_data = enc_aes256_cbc::encrypt_message_str(&key, message);
    print!("{}", StrBuf::bufh("- enc_data", &enc_data));
    assert_eq!(
        enc_data,
        util::to_str_hex("37455A8392187439EFAA249617AAB5C2")
    );

    let dec_data = enc_aes256_cbc::decrypt_message_str(&key, &enc_data);
    print!("- dec_data = {}", dec_data);
    assert_eq!(dec_data, message);
}

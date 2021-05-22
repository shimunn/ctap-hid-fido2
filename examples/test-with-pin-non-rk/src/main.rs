use anyhow::Result;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use ctap_hid_fido2::make_credential_params::Extension as Mext;
use ctap_hid_fido2::get_assertion_params::Extension as Gext;
use ctap_hid_fido2::str_buf::StrBuf;
use ctap_hid_fido2::verifier;
use ctap_hid_fido2::HidParam;

fn main() -> Result<()> {
    println!("----- test-with-pin-non-rk start -----");

    // parameter
    let rpid = "test.com";
    let pin = "1234";
    let challenge = verifier::create_challenge();

    // Register
    let mut strbuf = StrBuf::new(30);
    println!(
        "{}",
        strbuf
            .appent("Register - make_credential()")
            .append("- rpid", &rpid)
            .appenh("- challenge", &challenge)
            .build()
    );

    /*
    let att = ctap_hid_fido2::make_credential(
        &HidParam::get_default_params(),
        rpid,
        &challenge,
        Some(pin),
    )?;
    */

    // with extensions
    let ext = Mext::HmacSecret(Some(true));
    let att = ctap_hid_fido2::make_credential_with_extensions(
        &HidParam::get_default_params(),
        rpid,
        &challenge,
        Some(pin),
        Some(&vec![ext]),
    )?;

    println!("- Register Success!!");
    println!("Attestation");
    println!("{}", att);

    println!("Verify");
    let verify_result = verifier::verify_attestation(rpid, &challenge, &att);

    let mut strbuf = StrBuf::new(30);
    println!(
        "{}",
        strbuf
            .append("- is_success", &verify_result.is_success)
            .appenh("- credential_id", &verify_result.credential_id)
            .build()
    );

    // PEND
    let message = "this is test.";
    let mut salt = [0u8; 32];
    let mut digest = Sha256::new();
    digest.input(&message.as_bytes());
    digest.result(&mut salt);
    let mut strbuf = StrBuf::new(30);
    println!("{}", strbuf.appenh("- salt", &salt).build());
    let ext = Gext::HmacSecret(Some(salt));
    //

    // Authenticate
    println!("Authenticate - get_assertion_with_pin()");
    let challenge = verifier::create_challenge();
    println!("{}", strbuf.appenh("- challenge", &challenge).build());

    let ass = ctap_hid_fido2::get_assertion(
        &HidParam::get_default_params(),
        rpid,
        &challenge,
        &verify_result.credential_id,
        Some(pin),
    )?;
    /*
    let ass = ctap_hid_fido2::get_assertion_with_extensios(
        &HidParam::get_default_params(),
        rpid,
        &challenge,
        &verify_result.credential_id,
        Some(pin),
        Some(&vec![ext]),
    )?;
    */

    println!("- Authenticate Success!!");
    println!("Assertion");
    println!("{}", ass);

    println!("Verify");
    let is_success = verifier::verify_assertion(
        rpid,
        &verify_result.credential_publickey_der,
        &challenge,
        &ass,
    );
    println!("- is_success = {:?}", is_success);

    println!("----- test-with-pin-non-rk end -----");
    Ok(())
}

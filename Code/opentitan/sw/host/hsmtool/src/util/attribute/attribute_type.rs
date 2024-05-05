// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// This file was autogenerated by `//sw/host/hsmtool/scripts/pkcs11_consts.py`.
// Do not edit.'

use cryptoki_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::convert::TryFrom;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    IntoPrimitive,
    FromPrimitive,
    serde::Serialize,
    serde::Deserialize,
    strum::Display,
    strum::EnumString,
    strum::EnumIter,
)]
#[repr(u64)]
pub enum AttributeType {
    #[serde(rename = "CKA_CLASS")]
    #[strum(serialize = "CKA_CLASS", serialize = "Class", serialize = "class")]
    Class = CKA_CLASS,
    #[serde(rename = "CKA_TOKEN")]
    #[strum(serialize = "CKA_TOKEN", serialize = "Token", serialize = "token")]
    Token = CKA_TOKEN,
    #[serde(rename = "CKA_PRIVATE")]
    #[strum(
        serialize = "CKA_PRIVATE",
        serialize = "Private",
        serialize = "private"
    )]
    Private = CKA_PRIVATE,
    #[serde(rename = "CKA_LABEL")]
    #[strum(serialize = "CKA_LABEL", serialize = "Label", serialize = "label")]
    Label = CKA_LABEL,
    #[serde(rename = "CKA_APPLICATION")]
    #[strum(
        serialize = "CKA_APPLICATION",
        serialize = "Application",
        serialize = "application"
    )]
    Application = CKA_APPLICATION,
    #[serde(rename = "CKA_VALUE")]
    #[strum(serialize = "CKA_VALUE", serialize = "Value", serialize = "value")]
    Value = CKA_VALUE,
    #[serde(rename = "CKA_OBJECT_ID")]
    #[strum(
        serialize = "CKA_OBJECT_ID",
        serialize = "ObjectId",
        serialize = "object_id"
    )]
    ObjectId = CKA_OBJECT_ID,
    #[serde(rename = "CKA_CERTIFICATE_TYPE")]
    #[strum(
        serialize = "CKA_CERTIFICATE_TYPE",
        serialize = "CertificateType",
        serialize = "certificate_type"
    )]
    CertificateType = CKA_CERTIFICATE_TYPE,
    #[serde(rename = "CKA_ISSUER")]
    #[strum(serialize = "CKA_ISSUER", serialize = "Issuer", serialize = "issuer")]
    Issuer = CKA_ISSUER,
    #[serde(rename = "CKA_SERIAL_NUMBER")]
    #[strum(
        serialize = "CKA_SERIAL_NUMBER",
        serialize = "SerialNumber",
        serialize = "serial_number"
    )]
    SerialNumber = CKA_SERIAL_NUMBER,
    #[serde(rename = "CKA_AC_ISSUER")]
    #[strum(
        serialize = "CKA_AC_ISSUER",
        serialize = "AcIssuer",
        serialize = "ac_issuer"
    )]
    AcIssuer = CKA_AC_ISSUER,
    #[serde(rename = "CKA_OWNER")]
    #[strum(serialize = "CKA_OWNER", serialize = "Owner", serialize = "owner")]
    Owner = CKA_OWNER,
    #[serde(rename = "CKA_ATTR_TYPES")]
    #[strum(
        serialize = "CKA_ATTR_TYPES",
        serialize = "AttrTypes",
        serialize = "attr_types"
    )]
    AttrTypes = CKA_ATTR_TYPES,
    #[serde(rename = "CKA_TRUSTED")]
    #[strum(
        serialize = "CKA_TRUSTED",
        serialize = "Trusted",
        serialize = "trusted"
    )]
    Trusted = CKA_TRUSTED,
    #[serde(rename = "CKA_CERTIFICATE_CATEGORY")]
    #[strum(
        serialize = "CKA_CERTIFICATE_CATEGORY",
        serialize = "CertificateCategory",
        serialize = "certificate_category"
    )]
    CertificateCategory = CKA_CERTIFICATE_CATEGORY,
    #[serde(rename = "CKA_JAVA_MIDP_SECURITY_DOMAIN")]
    #[strum(
        serialize = "CKA_JAVA_MIDP_SECURITY_DOMAIN",
        serialize = "JavaMidpSecurityDomain",
        serialize = "java_midp_security_domain"
    )]
    JavaMidpSecurityDomain = CKA_JAVA_MIDP_SECURITY_DOMAIN,
    #[serde(rename = "CKA_URL")]
    #[strum(serialize = "CKA_URL", serialize = "Url", serialize = "url")]
    Url = CKA_URL,
    #[serde(rename = "CKA_HASH_OF_SUBJECT_PUBLIC_KEY")]
    #[strum(
        serialize = "CKA_HASH_OF_SUBJECT_PUBLIC_KEY",
        serialize = "HashOfSubjectPublicKey",
        serialize = "hash_of_subject_public_key"
    )]
    HashOfSubjectPublicKey = CKA_HASH_OF_SUBJECT_PUBLIC_KEY,
    #[serde(rename = "CKA_HASH_OF_ISSUER_PUBLIC_KEY")]
    #[strum(
        serialize = "CKA_HASH_OF_ISSUER_PUBLIC_KEY",
        serialize = "HashOfIssuerPublicKey",
        serialize = "hash_of_issuer_public_key"
    )]
    HashOfIssuerPublicKey = CKA_HASH_OF_ISSUER_PUBLIC_KEY,
    #[serde(rename = "CKA_NAME_HASH_ALGORITHM")]
    #[strum(
        serialize = "CKA_NAME_HASH_ALGORITHM",
        serialize = "NameHashAlgorithm",
        serialize = "name_hash_algorithm"
    )]
    NameHashAlgorithm = CKA_NAME_HASH_ALGORITHM,
    #[serde(rename = "CKA_CHECK_VALUE")]
    #[strum(
        serialize = "CKA_CHECK_VALUE",
        serialize = "CheckValue",
        serialize = "check_value"
    )]
    CheckValue = CKA_CHECK_VALUE,
    #[serde(rename = "CKA_KEY_TYPE")]
    #[strum(
        serialize = "CKA_KEY_TYPE",
        serialize = "KeyType",
        serialize = "key_type"
    )]
    KeyType = CKA_KEY_TYPE,
    #[serde(rename = "CKA_SUBJECT")]
    #[strum(
        serialize = "CKA_SUBJECT",
        serialize = "Subject",
        serialize = "subject"
    )]
    Subject = CKA_SUBJECT,
    #[serde(rename = "CKA_ID")]
    #[strum(serialize = "CKA_ID", serialize = "Id", serialize = "id")]
    Id = CKA_ID,
    #[serde(rename = "CKA_SENSITIVE")]
    #[strum(
        serialize = "CKA_SENSITIVE",
        serialize = "Sensitive",
        serialize = "sensitive"
    )]
    Sensitive = CKA_SENSITIVE,
    #[serde(rename = "CKA_ENCRYPT")]
    #[strum(
        serialize = "CKA_ENCRYPT",
        serialize = "Encrypt",
        serialize = "encrypt"
    )]
    Encrypt = CKA_ENCRYPT,
    #[serde(rename = "CKA_DECRYPT")]
    #[strum(
        serialize = "CKA_DECRYPT",
        serialize = "Decrypt",
        serialize = "decrypt"
    )]
    Decrypt = CKA_DECRYPT,
    #[serde(rename = "CKA_WRAP")]
    #[strum(serialize = "CKA_WRAP", serialize = "Wrap", serialize = "wrap")]
    Wrap = CKA_WRAP,
    #[serde(rename = "CKA_UNWRAP")]
    #[strum(serialize = "CKA_UNWRAP", serialize = "Unwrap", serialize = "unwrap")]
    Unwrap = CKA_UNWRAP,
    #[serde(rename = "CKA_SIGN")]
    #[strum(serialize = "CKA_SIGN", serialize = "Sign", serialize = "sign")]
    Sign = CKA_SIGN,
    #[serde(rename = "CKA_SIGN_RECOVER")]
    #[strum(
        serialize = "CKA_SIGN_RECOVER",
        serialize = "SignRecover",
        serialize = "sign_recover"
    )]
    SignRecover = CKA_SIGN_RECOVER,
    #[serde(rename = "CKA_VERIFY")]
    #[strum(serialize = "CKA_VERIFY", serialize = "Verify", serialize = "verify")]
    Verify = CKA_VERIFY,
    #[serde(rename = "CKA_VERIFY_RECOVER")]
    #[strum(
        serialize = "CKA_VERIFY_RECOVER",
        serialize = "VerifyRecover",
        serialize = "verify_recover"
    )]
    VerifyRecover = CKA_VERIFY_RECOVER,
    #[serde(rename = "CKA_DERIVE")]
    #[strum(serialize = "CKA_DERIVE", serialize = "Derive", serialize = "derive")]
    Derive = CKA_DERIVE,
    #[serde(rename = "CKA_START_DATE")]
    #[strum(
        serialize = "CKA_START_DATE",
        serialize = "StartDate",
        serialize = "start_date"
    )]
    StartDate = CKA_START_DATE,
    #[serde(rename = "CKA_END_DATE")]
    #[strum(
        serialize = "CKA_END_DATE",
        serialize = "EndDate",
        serialize = "end_date"
    )]
    EndDate = CKA_END_DATE,
    #[serde(rename = "CKA_MODULUS")]
    #[strum(
        serialize = "CKA_MODULUS",
        serialize = "Modulus",
        serialize = "modulus"
    )]
    Modulus = CKA_MODULUS,
    #[serde(rename = "CKA_MODULUS_BITS")]
    #[strum(
        serialize = "CKA_MODULUS_BITS",
        serialize = "ModulusBits",
        serialize = "modulus_bits"
    )]
    ModulusBits = CKA_MODULUS_BITS,
    #[serde(rename = "CKA_PUBLIC_EXPONENT")]
    #[strum(
        serialize = "CKA_PUBLIC_EXPONENT",
        serialize = "PublicExponent",
        serialize = "public_exponent"
    )]
    PublicExponent = CKA_PUBLIC_EXPONENT,
    #[serde(rename = "CKA_PRIVATE_EXPONENT")]
    #[strum(
        serialize = "CKA_PRIVATE_EXPONENT",
        serialize = "PrivateExponent",
        serialize = "private_exponent"
    )]
    PrivateExponent = CKA_PRIVATE_EXPONENT,
    #[serde(rename = "CKA_PRIME_1")]
    #[strum(serialize = "CKA_PRIME_1", serialize = "Prime1", serialize = "prime_1")]
    Prime1 = CKA_PRIME_1,
    #[serde(rename = "CKA_PRIME_2")]
    #[strum(serialize = "CKA_PRIME_2", serialize = "Prime2", serialize = "prime_2")]
    Prime2 = CKA_PRIME_2,
    #[serde(rename = "CKA_EXPONENT_1")]
    #[strum(
        serialize = "CKA_EXPONENT_1",
        serialize = "Exponent1",
        serialize = "exponent_1"
    )]
    Exponent1 = CKA_EXPONENT_1,
    #[serde(rename = "CKA_EXPONENT_2")]
    #[strum(
        serialize = "CKA_EXPONENT_2",
        serialize = "Exponent2",
        serialize = "exponent_2"
    )]
    Exponent2 = CKA_EXPONENT_2,
    #[serde(rename = "CKA_COEFFICIENT")]
    #[strum(
        serialize = "CKA_COEFFICIENT",
        serialize = "Coefficient",
        serialize = "coefficient"
    )]
    Coefficient = CKA_COEFFICIENT,
    #[serde(rename = "CKA_PUBLIC_KEY_INFO")]
    #[strum(
        serialize = "CKA_PUBLIC_KEY_INFO",
        serialize = "PublicKeyInfo",
        serialize = "public_key_info"
    )]
    PublicKeyInfo = CKA_PUBLIC_KEY_INFO,
    #[serde(rename = "CKA_PRIME")]
    #[strum(serialize = "CKA_PRIME", serialize = "Prime", serialize = "prime")]
    Prime = CKA_PRIME,
    #[serde(rename = "CKA_SUBPRIME")]
    #[strum(
        serialize = "CKA_SUBPRIME",
        serialize = "Subprime",
        serialize = "subprime"
    )]
    Subprime = CKA_SUBPRIME,
    #[serde(rename = "CKA_BASE")]
    #[strum(serialize = "CKA_BASE", serialize = "Base", serialize = "base")]
    Base = CKA_BASE,
    #[serde(rename = "CKA_PRIME_BITS")]
    #[strum(
        serialize = "CKA_PRIME_BITS",
        serialize = "PrimeBits",
        serialize = "prime_bits"
    )]
    PrimeBits = CKA_PRIME_BITS,
    #[serde(rename = "CKA_SUB_PRIME_BITS")]
    #[strum(
        serialize = "CKA_SUB_PRIME_BITS",
        serialize = "SubPrimeBits",
        serialize = "sub_prime_bits"
    )]
    SubPrimeBits = CKA_SUB_PRIME_BITS,
    #[serde(rename = "CKA_VALUE_BITS")]
    #[strum(
        serialize = "CKA_VALUE_BITS",
        serialize = "ValueBits",
        serialize = "value_bits"
    )]
    ValueBits = CKA_VALUE_BITS,
    #[serde(rename = "CKA_VALUE_LEN")]
    #[strum(
        serialize = "CKA_VALUE_LEN",
        serialize = "ValueLen",
        serialize = "value_len"
    )]
    ValueLen = CKA_VALUE_LEN,
    #[serde(rename = "CKA_EXTRACTABLE")]
    #[strum(
        serialize = "CKA_EXTRACTABLE",
        serialize = "Extractable",
        serialize = "extractable"
    )]
    Extractable = CKA_EXTRACTABLE,
    #[serde(rename = "CKA_LOCAL")]
    #[strum(serialize = "CKA_LOCAL", serialize = "Local", serialize = "local")]
    Local = CKA_LOCAL,
    #[serde(rename = "CKA_NEVER_EXTRACTABLE")]
    #[strum(
        serialize = "CKA_NEVER_EXTRACTABLE",
        serialize = "NeverExtractable",
        serialize = "never_extractable"
    )]
    NeverExtractable = CKA_NEVER_EXTRACTABLE,
    #[serde(rename = "CKA_ALWAYS_SENSITIVE")]
    #[strum(
        serialize = "CKA_ALWAYS_SENSITIVE",
        serialize = "AlwaysSensitive",
        serialize = "always_sensitive"
    )]
    AlwaysSensitive = CKA_ALWAYS_SENSITIVE,
    #[serde(rename = "CKA_KEY_GEN_MECHANISM")]
    #[strum(
        serialize = "CKA_KEY_GEN_MECHANISM",
        serialize = "KeyGenMechanism",
        serialize = "key_gen_mechanism"
    )]
    KeyGenMechanism = CKA_KEY_GEN_MECHANISM,
    #[serde(rename = "CKA_MODIFIABLE")]
    #[strum(
        serialize = "CKA_MODIFIABLE",
        serialize = "Modifiable",
        serialize = "modifiable"
    )]
    Modifiable = CKA_MODIFIABLE,
    #[serde(rename = "CKA_COPYABLE")]
    #[strum(
        serialize = "CKA_COPYABLE",
        serialize = "Copyable",
        serialize = "copyable"
    )]
    Copyable = CKA_COPYABLE,
    #[serde(rename = "CKA_DESTROYABLE")]
    #[strum(
        serialize = "CKA_DESTROYABLE",
        serialize = "Destroyable",
        serialize = "destroyable"
    )]
    Destroyable = CKA_DESTROYABLE,
    #[serde(rename = "CKA_EC_PARAMS")]
    #[strum(
        serialize = "CKA_EC_PARAMS",
        serialize = "EcParams",
        serialize = "ec_params"
    )]
    EcParams = CKA_EC_PARAMS,
    #[serde(rename = "CKA_EC_POINT")]
    #[strum(
        serialize = "CKA_EC_POINT",
        serialize = "EcPoint",
        serialize = "ec_point"
    )]
    EcPoint = CKA_EC_POINT,
    #[serde(rename = "CKA_SECONDARY_AUTH")]
    #[strum(
        serialize = "CKA_SECONDARY_AUTH",
        serialize = "SecondaryAuth",
        serialize = "secondary_auth"
    )]
    SecondaryAuth = CKA_SECONDARY_AUTH,
    #[serde(rename = "CKA_AUTH_PIN_FLAGS")]
    #[strum(
        serialize = "CKA_AUTH_PIN_FLAGS",
        serialize = "AuthPinFlags",
        serialize = "auth_pin_flags"
    )]
    AuthPinFlags = CKA_AUTH_PIN_FLAGS,
    #[serde(rename = "CKA_ALWAYS_AUTHENTICATE")]
    #[strum(
        serialize = "CKA_ALWAYS_AUTHENTICATE",
        serialize = "AlwaysAuthenticate",
        serialize = "always_authenticate"
    )]
    AlwaysAuthenticate = CKA_ALWAYS_AUTHENTICATE,
    #[serde(rename = "CKA_WRAP_WITH_TRUSTED")]
    #[strum(
        serialize = "CKA_WRAP_WITH_TRUSTED",
        serialize = "WrapWithTrusted",
        serialize = "wrap_with_trusted"
    )]
    WrapWithTrusted = CKA_WRAP_WITH_TRUSTED,
    #[serde(rename = "CKA_OTP_FORMAT")]
    #[strum(
        serialize = "CKA_OTP_FORMAT",
        serialize = "OtpFormat",
        serialize = "otp_format"
    )]
    OtpFormat = CKA_OTP_FORMAT,
    #[serde(rename = "CKA_OTP_LENGTH")]
    #[strum(
        serialize = "CKA_OTP_LENGTH",
        serialize = "OtpLength",
        serialize = "otp_length"
    )]
    OtpLength = CKA_OTP_LENGTH,
    #[serde(rename = "CKA_OTP_TIME_INTERVAL")]
    #[strum(
        serialize = "CKA_OTP_TIME_INTERVAL",
        serialize = "OtpTimeInterval",
        serialize = "otp_time_interval"
    )]
    OtpTimeInterval = CKA_OTP_TIME_INTERVAL,
    #[serde(rename = "CKA_OTP_USER_FRIENDLY_MODE")]
    #[strum(
        serialize = "CKA_OTP_USER_FRIENDLY_MODE",
        serialize = "OtpUserFriendlyMode",
        serialize = "otp_user_friendly_mode"
    )]
    OtpUserFriendlyMode = CKA_OTP_USER_FRIENDLY_MODE,
    #[serde(rename = "CKA_OTP_CHALLENGE_REQUIREMENT")]
    #[strum(
        serialize = "CKA_OTP_CHALLENGE_REQUIREMENT",
        serialize = "OtpChallengeRequirement",
        serialize = "otp_challenge_requirement"
    )]
    OtpChallengeRequirement = CKA_OTP_CHALLENGE_REQUIREMENT,
    #[serde(rename = "CKA_OTP_TIME_REQUIREMENT")]
    #[strum(
        serialize = "CKA_OTP_TIME_REQUIREMENT",
        serialize = "OtpTimeRequirement",
        serialize = "otp_time_requirement"
    )]
    OtpTimeRequirement = CKA_OTP_TIME_REQUIREMENT,
    #[serde(rename = "CKA_OTP_COUNTER_REQUIREMENT")]
    #[strum(
        serialize = "CKA_OTP_COUNTER_REQUIREMENT",
        serialize = "OtpCounterRequirement",
        serialize = "otp_counter_requirement"
    )]
    OtpCounterRequirement = CKA_OTP_COUNTER_REQUIREMENT,
    #[serde(rename = "CKA_OTP_PIN_REQUIREMENT")]
    #[strum(
        serialize = "CKA_OTP_PIN_REQUIREMENT",
        serialize = "OtpPinRequirement",
        serialize = "otp_pin_requirement"
    )]
    OtpPinRequirement = CKA_OTP_PIN_REQUIREMENT,
    #[serde(rename = "CKA_OTP_USER_IDENTIFIER")]
    #[strum(
        serialize = "CKA_OTP_USER_IDENTIFIER",
        serialize = "OtpUserIdentifier",
        serialize = "otp_user_identifier"
    )]
    OtpUserIdentifier = CKA_OTP_USER_IDENTIFIER,
    #[serde(rename = "CKA_OTP_SERVICE_IDENTIFIER")]
    #[strum(
        serialize = "CKA_OTP_SERVICE_IDENTIFIER",
        serialize = "OtpServiceIdentifier",
        serialize = "otp_service_identifier"
    )]
    OtpServiceIdentifier = CKA_OTP_SERVICE_IDENTIFIER,
    #[serde(rename = "CKA_OTP_SERVICE_LOGO")]
    #[strum(
        serialize = "CKA_OTP_SERVICE_LOGO",
        serialize = "OtpServiceLogo",
        serialize = "otp_service_logo"
    )]
    OtpServiceLogo = CKA_OTP_SERVICE_LOGO,
    #[serde(rename = "CKA_OTP_SERVICE_LOGO_TYPE")]
    #[strum(
        serialize = "CKA_OTP_SERVICE_LOGO_TYPE",
        serialize = "OtpServiceLogoType",
        serialize = "otp_service_logo_type"
    )]
    OtpServiceLogoType = CKA_OTP_SERVICE_LOGO_TYPE,
    #[serde(rename = "CKA_OTP_COUNTER")]
    #[strum(
        serialize = "CKA_OTP_COUNTER",
        serialize = "OtpCounter",
        serialize = "otp_counter"
    )]
    OtpCounter = CKA_OTP_COUNTER,
    #[serde(rename = "CKA_OTP_TIME")]
    #[strum(
        serialize = "CKA_OTP_TIME",
        serialize = "OtpTime",
        serialize = "otp_time"
    )]
    OtpTime = CKA_OTP_TIME,
    #[serde(rename = "CKA_GOSTR3410_PARAMS")]
    #[strum(
        serialize = "CKA_GOSTR3410_PARAMS",
        serialize = "Gostr3410Params",
        serialize = "gostr3410_params"
    )]
    Gostr3410Params = CKA_GOSTR3410_PARAMS,
    #[serde(rename = "CKA_GOSTR3411_PARAMS")]
    #[strum(
        serialize = "CKA_GOSTR3411_PARAMS",
        serialize = "Gostr3411Params",
        serialize = "gostr3411_params"
    )]
    Gostr3411Params = CKA_GOSTR3411_PARAMS,
    #[serde(rename = "CKA_GOST28147_PARAMS")]
    #[strum(
        serialize = "CKA_GOST28147_PARAMS",
        serialize = "Gost28147Params",
        serialize = "gost28147_params"
    )]
    Gost28147Params = CKA_GOST28147_PARAMS,
    #[serde(rename = "CKA_HW_FEATURE_TYPE")]
    #[strum(
        serialize = "CKA_HW_FEATURE_TYPE",
        serialize = "HwFeatureType",
        serialize = "hw_feature_type"
    )]
    HwFeatureType = CKA_HW_FEATURE_TYPE,
    #[serde(rename = "CKA_RESET_ON_INIT")]
    #[strum(
        serialize = "CKA_RESET_ON_INIT",
        serialize = "ResetOnInit",
        serialize = "reset_on_init"
    )]
    ResetOnInit = CKA_RESET_ON_INIT,
    #[serde(rename = "CKA_HAS_RESET")]
    #[strum(
        serialize = "CKA_HAS_RESET",
        serialize = "HasReset",
        serialize = "has_reset"
    )]
    HasReset = CKA_HAS_RESET,
    #[serde(rename = "CKA_PIXEL_X")]
    #[strum(serialize = "CKA_PIXEL_X", serialize = "PixelX", serialize = "pixel_x")]
    PixelX = CKA_PIXEL_X,
    #[serde(rename = "CKA_PIXEL_Y")]
    #[strum(serialize = "CKA_PIXEL_Y", serialize = "PixelY", serialize = "pixel_y")]
    PixelY = CKA_PIXEL_Y,
    #[serde(rename = "CKA_RESOLUTION")]
    #[strum(
        serialize = "CKA_RESOLUTION",
        serialize = "Resolution",
        serialize = "resolution"
    )]
    Resolution = CKA_RESOLUTION,
    #[serde(rename = "CKA_CHAR_ROWS")]
    #[strum(
        serialize = "CKA_CHAR_ROWS",
        serialize = "CharRows",
        serialize = "char_rows"
    )]
    CharRows = CKA_CHAR_ROWS,
    #[serde(rename = "CKA_CHAR_COLUMNS")]
    #[strum(
        serialize = "CKA_CHAR_COLUMNS",
        serialize = "CharColumns",
        serialize = "char_columns"
    )]
    CharColumns = CKA_CHAR_COLUMNS,
    #[serde(rename = "CKA_COLOR")]
    #[strum(serialize = "CKA_COLOR", serialize = "Color", serialize = "color")]
    Color = CKA_COLOR,
    #[serde(rename = "CKA_BITS_PER_PIXEL")]
    #[strum(
        serialize = "CKA_BITS_PER_PIXEL",
        serialize = "BitsPerPixel",
        serialize = "bits_per_pixel"
    )]
    BitsPerPixel = CKA_BITS_PER_PIXEL,
    #[serde(rename = "CKA_CHAR_SETS")]
    #[strum(
        serialize = "CKA_CHAR_SETS",
        serialize = "CharSets",
        serialize = "char_sets"
    )]
    CharSets = CKA_CHAR_SETS,
    #[serde(rename = "CKA_ENCODING_METHODS")]
    #[strum(
        serialize = "CKA_ENCODING_METHODS",
        serialize = "EncodingMethods",
        serialize = "encoding_methods"
    )]
    EncodingMethods = CKA_ENCODING_METHODS,
    #[serde(rename = "CKA_MIME_TYPES")]
    #[strum(
        serialize = "CKA_MIME_TYPES",
        serialize = "MimeTypes",
        serialize = "mime_types"
    )]
    MimeTypes = CKA_MIME_TYPES,
    #[serde(rename = "CKA_MECHANISM_TYPE")]
    #[strum(
        serialize = "CKA_MECHANISM_TYPE",
        serialize = "MechanismType",
        serialize = "mechanism_type"
    )]
    MechanismType = CKA_MECHANISM_TYPE,
    #[serde(rename = "CKA_REQUIRED_CMS_ATTRIBUTES")]
    #[strum(
        serialize = "CKA_REQUIRED_CMS_ATTRIBUTES",
        serialize = "RequiredCmsAttributes",
        serialize = "required_cms_attributes"
    )]
    RequiredCmsAttributes = CKA_REQUIRED_CMS_ATTRIBUTES,
    #[serde(rename = "CKA_DEFAULT_CMS_ATTRIBUTES")]
    #[strum(
        serialize = "CKA_DEFAULT_CMS_ATTRIBUTES",
        serialize = "DefaultCmsAttributes",
        serialize = "default_cms_attributes"
    )]
    DefaultCmsAttributes = CKA_DEFAULT_CMS_ATTRIBUTES,
    #[serde(rename = "CKA_SUPPORTED_CMS_ATTRIBUTES")]
    #[strum(
        serialize = "CKA_SUPPORTED_CMS_ATTRIBUTES",
        serialize = "SupportedCmsAttributes",
        serialize = "supported_cms_attributes"
    )]
    SupportedCmsAttributes = CKA_SUPPORTED_CMS_ATTRIBUTES,
    #[serde(rename = "CKA_WRAP_TEMPLATE")]
    #[strum(
        serialize = "CKA_WRAP_TEMPLATE",
        serialize = "WrapTemplate",
        serialize = "wrap_template"
    )]
    WrapTemplate = CKA_WRAP_TEMPLATE,
    #[serde(rename = "CKA_UNWRAP_TEMPLATE")]
    #[strum(
        serialize = "CKA_UNWRAP_TEMPLATE",
        serialize = "UnwrapTemplate",
        serialize = "unwrap_template"
    )]
    UnwrapTemplate = CKA_UNWRAP_TEMPLATE,
    #[serde(rename = "CKA_DERIVE_TEMPLATE")]
    #[strum(
        serialize = "CKA_DERIVE_TEMPLATE",
        serialize = "DeriveTemplate",
        serialize = "derive_template"
    )]
    DeriveTemplate = CKA_DERIVE_TEMPLATE,
    #[serde(rename = "CKA_ALLOWED_MECHANISMS")]
    #[strum(
        serialize = "CKA_ALLOWED_MECHANISMS",
        serialize = "AllowedMechanisms",
        serialize = "allowed_mechanisms"
    )]
    AllowedMechanisms = CKA_ALLOWED_MECHANISMS,
    #[num_enum(catch_all)]
    UnknownAttributeType(u64) = u64::MAX,
}

impl From<cryptoki::object::AttributeType> for AttributeType {
    fn from(val: cryptoki::object::AttributeType) -> Self {
        let val = CK_ATTRIBUTE_TYPE::from(val);
        Self::from(val)
    }
}

impl TryFrom<AttributeType> for cryptoki::object::AttributeType {
    type Error = cryptoki::error::Error;
    fn try_from(val: AttributeType) -> Result<Self, Self::Error> {
        let val = CK_ATTRIBUTE_TYPE::from(val);
        cryptoki::object::AttributeType::try_from(val)
    }
}
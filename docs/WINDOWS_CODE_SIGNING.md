# eDesk Windows code signing

eDesk supports optional production Authenticode signing through SSL.com eSigner.
The normal build remains unsigned until signing is deliberately enabled; no
self-signed or locally generated production certificate is used.

## Recommended certificate

For a registered business, obtain an **OV Code Signing** certificate from
SSL.com and enroll it in **eSigner for Code**. If eDesk is published by an
individual who does not represent a registered organization, ask SSL.com about
an **IV Code Signing** certificate instead. Confirm that the selected certificate
is eligible for eSigner cloud enrollment before purchase.

SSL.com currently lists OV Code Signing at US$129/year and its eSigner Tier 1
at US$15/month (240 signing operations). Taxes, exchange rates, enrollment,
and regional availability may change the final amount; confirm the checkout
total with SSL.com before paying.

## GitHub configuration (after certificate enrollment)

In repository Settings -> Secrets and variables -> Actions:

Add these **repository secrets**:
- `ESIGNER_USERNAME`: dedicated SSL.com automation account username
- `ESIGNER_PASSWORD`: SSL.com account password
- `ESIGNER_TOTP_SECRET`: eSigner Authenticator TOTP secret (not a current one-time code)

Add this **repository variable**:
- `EDESK_CODE_SIGNING_ENABLED` = `true`

Use a dedicated eSigner account containing exactly one code-signing certificate.
The workflow intentionally fails if it finds zero or multiple code-signing
certificates, rather than signing with an unintended identity.

## What the workflow signs

When enabled, the Windows build:
1. Initializes SSL.com eSigner CKA on the ephemeral Windows runner.
2. Signs and verifies `edesk-baseline/edesk.exe` before it is bundled.
3. Signs and verifies `dist/eDesk-Setup.exe` and the self-update executable.
4. Publishes those signed files through the existing release process.

The existing `EDESK_UPDATE_SIGNING_KEY` remains separate and unchanged. It
continues to sign eDesk's update metadata; it is not a Windows code-signing key.

Do not upload a PFX, private key, TOTP seed, or account password to the repository.
The eSigner private key remains in SSL.com's cloud HSM. Signing is skipped unless
the repository variable is explicitly enabled.

## Important limitations

Authenticode signing identifies the publisher and makes file tampering detectable.
It does not guarantee immediate Microsoft SmartScreen reputation or automatic
clearance by 360 Total Security. Submit any remaining false-positive detections
to the antivirus vendor for review. For 360, submit the installer and flagged
`edesk.exe` at https://www.360totalsecurity.com/en/suspicion/ or email
support@360safe.com.

Official references:
- https://www.ssl.com/products/software-integrity/code-signing/ov/
- https://www.ssl.com/products/software-integrity/signing-service/
- https://www.ssl.com/how-to/integrating-esigner-with-ci-cd-pipelines-a-complete-setup-and-configuration-guide/
- https://www.ssl.com/how-to/how-to-install-ssl-com-esigner-cloud-key-adapter-cka/

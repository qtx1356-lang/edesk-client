# Licensing and Attribution Checklist

eDesk is planned as a modified distribution based on RustDesk. RustDesk's client repository identifies its license as GNU Affero General Public License version 3 (AGPL-3.0). Confirm the license files and notices at the exact upstream revision imported into this repository before making or distributing a build.

## Required project practices

- Preserve upstream license files, copyright notices, and attribution.
- Keep notices for third-party libraries and assets.
- Track the exact upstream commit and the changes made by eDesk.
- Review AGPL-3.0 obligations for modified and network-accessible versions, including source-code availability requirements where applicable.
- If binaries are distributed, provide the corresponding source and notices in the manner required by the applicable licenses.
- Do not remove or obscure RustDesk attribution required by its license.
- Do not use RustDesk trademarks or branding in a way that suggests eDesk is an official product or endorsed by the RustDesk project.

## Before public release

- [ ] Confirm the client license at the pinned commit.
- [ ] Review all bundled dependency licenses.
- [ ] Preserve required notices in the application and distribution package.
- [ ] Publish the corresponding source for the exact released build as required.
- [ ] Include a third-party notices file.
- [ ] Obtain qualified legal advice for distribution and hosted-service questions.

This document is a project checklist, not legal advice.

Upstream client: https://github.com/rustdesk/rustdesk
Upstream server: https://github.com/rustdesk/rustdesk-server

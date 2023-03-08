# Smoothening the invite process

From [ISSUE-997](https://github.com/Scille/parsec-cloud/issues/997)

This is a meta-issue, tracking the issues for implementing a smoother invitation process.

## A full smooth invitation process - Alice inviting Bob

### 1. Alice section - about 1 minute

- Alice goes to the Invitation Management Panel
- Alice provides Bob's email address
- A mail is automatically sent to Bob
- A new pending invite appear in the invite list
- Alice can get back to other activities

### 2. Bob section - about 5 minutes

- Bob receives the invitation email, showing two steps:
  - Parsec Installation
  - Parsec Registration
- Bob clicks on the first step and the latest parsec installer is downloaded
- Bob runs the installer and is prompted for administrator rights
- Bob goes through the installation process, winfsp is installed silently
- Parsec gets started automatically, Bob fills up his name (as seen by others)
- Bob then clicks on "Join an organization", and is asked to get back to the mail
- Bob gets back to the mail and clicks the parsec registration link
- Parsec opens in the foreground and asks Bob to reach Alice and ask for the token
- Bob calls Alice

### 3. Alice and Bob joined section - about 3 minutes

- Bob asks Alice for the token
- Alice opens parsec and goes to the Invitation Management Panel
- Alice finds the pending invite and generate the token
- Alice spells the 6-digit token over the phone
- Bob enters the token and validates
- Alice is prompted with a confirmation dialog summing up Bob's information
- In particular, Alice checks that Bob correctly entered his name
- Alice validates the invite and Bob is successfully invited
- Bob gets automatically logged in

### Note

- It doesn't require any copy-pasting
- It's about 10 minutes, although Alice can invite several user in parallel

### Alternative ideas

- the mail contains the parsec installer
- the mail contains a link to an html page with the instruction for the two step process
- it might be possible to skip the step where Bob gets back to the mail

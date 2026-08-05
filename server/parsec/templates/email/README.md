# README

This directory contains email templates used by Parsec Server.

The templates are written in [Jinja format](https://jinja.palletsprojects.com/en/stable/templates/).

## Email templates in plain text

For plain text templates, extend the base template and define the following jinja blocks:

- `main_content`: the main content of the email

## Email templates in HTML

For HTML templates, extend the base template and define the following jinja blocks:

- `document_title`: the title for the `<head>` element, in the browser's title bar or the page's tab
- `preheader`: ???
- `title`: the title for the email content (not to be confused with the email subject)
- `description`: a short description of the email purpose
- `main_content`: the main content of the email

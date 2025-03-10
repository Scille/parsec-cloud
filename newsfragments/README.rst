About news fragment
===================

This directory collects "news fragments": short files containing a snippet of
ReST-formatted text that will be added to the next release notes.

News fragments should contain a description of a change that is relevant to the
user. This differs from a commit message or PR description, which are a intended
for developers.

The filename should be ``<ISSUE>.<TYPE>.rst``, where ``<ISSUE>`` is an issue
number, and ``<TYPE>`` is one of:

  * ``feature``
  * ``bugfix``
  * ``doc``
  * ``removal``
  * ``api``
  * ``misc``
  * ``empty``

So for example: ``123.feature.rst``, ``456.bugfix.rst``

If your PR fixes an issue, use the issue number here. If there is no issue, then
submit the PR first, and then use its number to add a news fragment to the PR.

To check how the release nots will look like for the current version, run::

  python misc/releaser.py history

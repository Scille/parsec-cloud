Documentation
=============


see http://www.sphinx-doc.org/en/master/usage/advanced/intl.html

Update translation
------------------

1) Extract text and create/update .po files
```
$ make gettext
$ sphinx-intl update -p _build/locale -l fr
```

2) Translate .po files

3) Build .po -> .mo
```
$ make -e SPHINXOPTS="-D language='fr'" html
```

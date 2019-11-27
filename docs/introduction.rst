.. _doc_introduction:

============
Introduction
============


Parsec is a free software (AGPL v3) aiming at easily work and share your
data in the cloud with a total privacy thanks to cryptographic security.


.. image:: parsec_snapshot.png
    :align: center


Key features:

- Works as a virtual drive on you computer. You can access and modify all the data
  stored in Parsec with your regular softwares just like you would on your local
  hard-drive.
- Never lose any data. Synchronization with the remote server never destroy anything,
  hence you can browse data history and recover from any point in time.
- Client-side cryptographic security. Data and metadata are only visible by you
  and the ones you choose to share with.
- Cryptographic signature. Each modification is signed by it author making trivial
  to identify modifications.
- Simplified enrolment. New user enrolment is simple as sharing a link and a token code.
- Simple to host. Parsec server only requires a PostgreSQL DB and S3 or OpenStack
  Swift object storage.


Overview
========


Parsec is divided between a client (responsible to expose data to the user and
provide an encryption layer) and a server (storing the encrypted data and notifying clients about
other users activity such as data modification or new sharing).

.. figure:: architecture_diagram.svg
    :align: center
    :alt: Parsec single server, multi organizations showcase

    Parsec single server, multi organizations showcase

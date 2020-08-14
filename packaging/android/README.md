Purpose
=======

This code aims at demonstrating an use of services in python-for-android, and
communication between services and a kivy front end.

That examples uses the OSC protocol for simplicity and historical reasons,
(since an implementation was shipped with kivy historically, and a better one
is now available as third party). The OSC protocol makes things simple because
it's unconnected, you just send a message, and can forget about it. You bind
functions to messages you receive. It's simple enough for a lot of things, and
avoids the burden of maintaining a connection when both the service and front
end can be restarted any time.

The app is composed of the front-end, defined in src/main.py, and the back-end defined in src/service.py.

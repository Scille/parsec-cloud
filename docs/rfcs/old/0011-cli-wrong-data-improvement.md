# CLI wrong data improvement

From [ISSUE-1075](https://github.com/Scille/parsec-cloud/issues/1075)

Parsing of the parsec URL is not done properly.

```shell
$ parsec core create_organization MYOrg -B parsec://:80 -T 12345
...
assert not hostname.startswith("parsec://")
AttributeError: 'NoneType' object has no attribute 'startswith'

```

I propose to check in the [from_url](https://github.com/Scille/parsec-cloud/blob/0e4e9cb3de402dd551362b07024ac86b3e9711e2/parsec/core/types/backend_address.py#L53) method that the host name is not None or zero-length string.

```python
if not split.hostname:
    raise ValueError(f"hostname missing")
```

and eventually combined with isinstance(split.hostname, str)

But worst, the argument are read in an async and weird ways

```shell
parsec@SRV:~$ parsec core create_organization MYOrg -B parsec://a:80&uname&a -T 12345
[1] 5876
[2] 5877
Linux
a: command not found
[2]+  Done                    uname
Usage: parsec core create_organization [OPTIONS] NAME
Try "parsec core create_organization --help" for help.

Error: Missing option "--administration-token" / "-T".

```

For this point, the only argument is use_ssl, read from the argument no_ssl. So the argument parsing is kind of overkill for a single argument. Plus, the way it is performed with kwargs in class method creation is a very bad security practice. My guess here is that this issue is limited to the user who is running the "core" client, so as he starts the parsec client, he can also run directly the "evil" command elsewhere in his session. Still, a user can execute command from the CLI in the argument of an argument of an option, and this is not so good, as it escape the program boundary, run system command where it is totally not needed.

Normally a URL starts with the protocol to use, and the argument in the data are just a payload. So for the no_ssl case, this should be parsec:// and parsecs:// (parsec over TLS). Or parnotsec:// and parsec://. I know this change has huge impact on the code base, but this will solve this argument parsing in a classy way, there won't be any argument to handle for create_organization. And the call will simply create a BackendAddr with use_ssl True or False (depending of the protocol scheme).

An other way can be to pass the params dictionary in the class creator, and read/decode this dict in the init of the BackendAddr.\__init__ method (creator). Then this creator will read and set the _use_ssl property accordingly.
Something like this :

```python
class BackendAddr:
    def __init__(self, hostname: str, port: Optional[int] = None, params={}):
        assert not hostname.startswith("parsec://")
        self._hostname = hostname
        self._port, _ = self._parse_port(port, use_ssl)
        self._use_ssl = True
        if "no_ssl" in params and params["no_ssl"]=="true":
            self._use_ssl = False
...
@classmethod
    def from_url(cls, url: str):
    ...
    return cls(hostname=split.hostname, port=split.port, params)
```

I didn't checked at this point if the BackendAddr creator is called with others code parts and what it can break. This is for the idea.

This is also valid for stats_organization and status_organization. In these others CLI commands,  I haven't checked  if the only argument is no_ssl.

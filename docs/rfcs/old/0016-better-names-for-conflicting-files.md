# Better names for conflicting files

From [ISSUE-1820](https://github.com/Scille/parsec-cloud/issues/1820)

> Using `remote_device_name` correspond to a `DeviceID` which doesn't seems like a good idea given the user has no idea who this correspond to.
> In such case it would be much better to do (by order of simplicity):
>
> 1) `conflict`, `conflict (2)` etc.
> 2) `conflict 2000-01-01T12:12:12Z`, `conflict 2000-01-01T12:12:12Z (2)` etc.
> 3) `conflict with John` (using user label)
>
> As a matter of fact I'm not really sure what 2) and 3) brings to the table given modification date and author are supposed to be easy to see with the history feature.
> So I would vote for the much simpler solution 1) (and wait for a real user push before going more complex, especially considering nobody complained so far that `conflict <uuid>@<uuid>` is not that helpful :laughing: )
>
> [@touilleMan on PR-1819](https://github.com/Scille/parsec-cloud/pull/1816#discussion_r697645008)

There's also the problem of localization: the names should take into account the language configured in the global configuration.

My choice would go to either 1 or 3:

- 1 because it's simple and usually a good a idea to wait for user feedback
- 3 because the user label seems to be the most useful information to display
